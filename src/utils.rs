// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use arrow::array::StringArray;
use arrow::array::{
    Array, ArrayRef, AsArray, BooleanArray, Datum, GenericByteArray, ListArray, NullArray,
    OffsetSizeTrait, PrimitiveArray,
};
use arrow::compute::{cast, date_part, DatePart};
use arrow::datatypes::{ArrowPrimitiveType, DataType, GenericBinaryType, GenericStringType};
use arrow::error::Result as ArrowResult;
use datafusion::common::cast::{as_fixed_size_list_array, as_string_array};
use datafusion::common::{Result, ScalarValue};
use datafusion::error::DataFusionError;
use datafusion::logical_expr::{ColumnarValue, ScalarFunctionImplementation};
use datafusion::physical_expr::functions::Hint;
use std::sync::Arc;

/// Creates a scalar function implementation for the given function.
/// * `inner` - the function to be executed
/// * `hints` - hints to be used when expanding scalars to arrays
pub(super) fn make_scalar_function<F>(inner: F, hints: Vec<Hint>) -> ScalarFunctionImplementation
where
    F: Fn(&[ArrayRef]) -> Result<ArrayRef> + Sync + Send + 'static,
{
    Arc::new(move |args: &[ColumnarValue]| {
        // first, identify if any of the arguments is an Array. If yes, store its `len`,
        // as any scalar will need to be converted to an array of len `len`.
        let len = args
            .iter()
            .fold(Option::<usize>::None, |acc, arg| match arg {
                ColumnarValue::Scalar(_) => acc,
                ColumnarValue::Array(a) => Some(a.len()),
            });

        let is_scalar = len.is_none();

        let inferred_length = len.unwrap_or(1);
        let args = args
            .iter()
            .zip(hints.iter().chain(std::iter::repeat(&Hint::Pad)))
            .map(|(arg, hint)| {
                // Decide on the length to expand this scalar to depending
                // on the given hints.
                let expansion_len = match hint {
                    Hint::AcceptsSingular => 1,
                    Hint::Pad => inferred_length,
                };
                arg.clone().into_array(expansion_len)
            })
            .collect::<datafusion::common::Result<Vec<_>>>()?;

        let result = (inner)(&args);
        if is_scalar {
            // If all inputs are scalar, keeps output as scalar
            let result = result.and_then(|arr| ScalarValue::try_from_array(&arr, 0));
            result.map(ColumnarValue::Scalar)
        } else {
            result.map(ColumnarValue::Array)
        }
    })
}

/// Applies a unary computational kernel to a columnar value.
/// `arg` - a Datafusion columnar value to be processed
/// `kernel` - a unary function on Arrow arrays
pub(super) fn apply_unary_kernel<F>(arg: &ColumnarValue, kernel: F) -> Result<ColumnarValue>
where
    F: Fn(&ArrayRef) -> ArrowResult<ArrayRef> + Sync + Send + 'static,
{
    match arg {
        ColumnarValue::Array(array) => {
            let res = kernel(array)?;
            Ok(ColumnarValue::Array(res))
        }
        ColumnarValue::Scalar(scalar) => {
            let array = scalar.to_array()?;
            let res = kernel(&array)?;
            let scalar = ScalarValue::try_from_array(&res, 0)?;
            Ok(ColumnarValue::Scalar(scalar))
        }
    }
}

/// Applies Arrow `date_part` kernel to a temporal columnar value.
/// Takes care of adjusting the output type to Trino's BIGINT.
/// `arg` - a Datafusion columnar value to be processed; temporal datatypes for DATE, TIME, TIMESTAMP are supported
/// `part` - the date part to extract
pub(super) fn apply_datepart_kernel(arg: &ColumnarValue, part: DatePart) -> Result<ColumnarValue> {
    apply_unary_kernel(arg, move |arr| {
        let res = date_part(arr, part)?;
        // DF date_part returns Int32, but Trino needs Int64
        Ok(cast(&res, &DataType::Int64)?)
    })
}

/// Use Arrow `date_part` kernel to compute day of week,
/// with Monday being 1 through Sunday being 7, per Trino convention..
pub(super) fn apply_dow_kernel(arg: &ColumnarValue) -> Result<ColumnarValue> {
    apply_unary_kernel(arg, move |arr| {
        let monday0 = date_part(arr, DatePart::DayOfWeekMonday0)?;
        let one = arrow::array::Int32Array::new_scalar(1);
        let monday1 = arrow::compute::kernels::numeric::add(&monday0, &one)?;
        // DF date_part returns Int32, but Trino needs Int64
        Ok(cast(&monday1, &DataType::Int64)?)
    })
}

/// Convert a vector from Datafusion to Arrow representation, accounting for a singleton vector case.
pub(super) fn columnar_to_datum(col: &ColumnarValue) -> Box<dyn Datum + '_> {
    match col {
        ColumnarValue::Scalar(scalar) => {
            // Result::unwrap() here can panic only due to a bug in Datafusion or Arrow
            Box::new(scalar.to_scalar().unwrap())
        }
        ColumnarValue::Array(array) => Box::new(array.as_ref()),
    }
}

/// Wrap as a DataFusion ColumnarValue, recognizing a singleton vector.
pub(super) fn array_to_columnar(array: ArrayRef) -> ColumnarValue {
    if array.len() == 1 {
        match ScalarValue::try_from_array(&array, 0) {
            Ok(scalar) => ColumnarValue::Scalar(scalar),
            Err(_) => ColumnarValue::Array(array), // Should not happen; can just panic instead
        }
    } else {
        ColumnarValue::Array(array)
    }
}

/// Extract the underlying string array from an array of SDF "distinct" type.
/// (The distinct type is FixedSizeList with size 1.)
/// TODO: This implementation iterates and reconstructs into a new array;
/// it could be possible to exploit the size=1 circumstance to achieve no-copy implementation.
pub(super) fn distinct_to_string_array(distinct_arr: &Arc<dyn Array>) -> Result<StringArray> {
    let fxsize_arr = as_fixed_size_list_array(distinct_arr)?;
    let res = fxsize_arr
        .iter()
        .map(|lst_opt| {
            lst_opt
                .map(|lst| {
                    let inner = as_string_array(&lst)?;
                    Ok(inner.value(0).to_owned())
                })
                .transpose()
        })
        .collect::<Result<StringArray>>()?;
    Ok(res)
}

/// Means to lift a suite of kernels on typed arrays to operate on a list array (an array of lists).
/// Given a collection of kernels that operate on primitive, string, binary, and boolean arrays,
/// this trait provides machinery to apply these kernels to each list in a list array.
/// The prototypical example that motivated this construction is the `array_min` and `array_max` UDFs:
/// they have identical implememntations, differs only in which kernels (min or max) are used on typed lists.
pub trait KernelLifter {
    // The feature (such as a UDF) that this lifter helps to implement, for error messages.
    const FEATURE_NAME: &'static str;

    // The kernel to apply to lists of primitves.
    fn primitive_kernel<T>(array: &PrimitiveArray<T>) -> Option<T::Native>
    where
        T: ArrowPrimitiveType;

    // The kernel to apply to lists of strings.
    fn string_kernel<O>(array: &GenericByteArray<GenericStringType<O>>) -> Option<&str>
    where
        O: OffsetSizeTrait;

    // The kernel to apply to lists of binaries.
    fn binary_kernel<O>(array: &GenericByteArray<GenericBinaryType<O>>) -> Option<&[u8]>
    where
        O: OffsetSizeTrait;

    // The kernel to apply to lists of booleans.
    fn boolean_kernel(array: &BooleanArray) -> Option<bool>;

    // Deals with a column of lists none of which contains a real item -- every list is either empty of contains only NULLs.
    fn lift_null(list_array: &ListArray) -> ArrayRef {
        Arc::new(NullArray::new(list_array.len()))
    }

    // TODO: Find a way to reduce remaining duplication across the four lift_xxx_kernel methods.

    fn lift_primitive_kernel<T>(list_array: &ListArray) -> ArrayRef
    where
        T: ArrowPrimitiveType,
    {
        let result: PrimitiveArray<T> = list_array
            .iter()
            .map(|list| match list {
                None => None, // NULL in place of a list in the column
                Some(lst) => {
                    let typ_lst = lst.as_primitive::<T>();
                    if typ_lst.null_count() > 0 {
                        None // A Trino quirk? If a NULL is present, array_min,max are NULL.
                    } else {
                        Self::primitive_kernel(typ_lst)
                    }
                }
            })
            .collect();
        Arc::new(result)
    }

    fn lift_string_kernel<O>(list_array: &ListArray) -> ArrayRef
    where
        O: OffsetSizeTrait,
    {
        let result: GenericByteArray<GenericStringType<O>> = list_array
            .iter()
            .map(|list| match list {
                None => None, // NULL in place of a list in the column
                Some(lst) => {
                    let typ_lst = lst.as_string::<O>();
                    if typ_lst.null_count() > 0 {
                        None // A Trino quirk? If a NULL is present, array_min,max are NULL.
                    } else {
                        Self::string_kernel(typ_lst).map(|v| v.to_owned())
                    }
                }
            })
            .collect();
        Arc::new(result)
    }

    fn lift_binary_kernel<O>(list_array: &ListArray) -> ArrayRef
    where
        O: OffsetSizeTrait,
    {
        let result: GenericByteArray<GenericBinaryType<O>> = list_array
            .iter()
            .map(|list| match list {
                None => None, // NULL in place of a list in the column
                Some(lst) => {
                    let typ_lst = lst.as_binary::<O>();
                    if typ_lst.null_count() > 0 {
                        None // A Trino quirk? If a NULL is present, array_min,max are NULL.
                    } else {
                        Self::binary_kernel(typ_lst).map(|v| v.to_owned())
                    }
                }
            })
            .collect();
        Arc::new(result)
    }

    fn lift_boolean_kernel(list_array: &ListArray) -> ArrayRef {
        let result: BooleanArray = list_array
            .iter()
            .map(|list| match list {
                None => None, // NULL in place of a list in the column
                Some(lst) => {
                    let typ_lst = lst.as_boolean();
                    if typ_lst.null_count() > 0 {
                        return None; // A Trino quirk? If a NULL is present, array_min,max are NULL.
                    } else {
                        Self::boolean_kernel(typ_lst)
                    }
                }
            })
            .collect();
        Arc::new(result)
    }

    /// Lifts the kernels supplied to this trait to operate on a list array
    /// (as long its underlying list type is supported by one of the kernels).
    fn lift_all_kernels(array: ArrayRef) -> Result<ArrayRef> {
        use arrow::datatypes as adt;
        use arrow::datatypes::{IntervalUnit, TimeUnit};

        // Currently, only `array` that is a ListArray is supported.
        // If we run into a LargeListArray in practice, the following can be relatively easily generalized
        // to have `list_array` at type GenericListArray<O> instead of ListArray (aka GenericListArray<i32>),
        // similar to how it is done with generic_list_cardinality in DF's cardinality UDF.
        // Running into FixedSizeListArray does not appear likely here and would require good deal more adjusting.
        let list_array: &ListArray = datafusion::common::cast::as_list_array(&array)?;
        let elem_dt = list_array.value_type();
        let res: ArrayRef = match elem_dt {
            DataType::Null => Self::lift_null(list_array),
            // numeric primitive arrays
            DataType::UInt8 => Self::lift_primitive_kernel::<adt::UInt8Type>(list_array),
            DataType::UInt16 => Self::lift_primitive_kernel::<adt::UInt16Type>(list_array),
            DataType::UInt32 => Self::lift_primitive_kernel::<adt::UInt32Type>(list_array),
            DataType::UInt64 => Self::lift_primitive_kernel::<adt::UInt64Type>(list_array),
            DataType::Int8 => Self::lift_primitive_kernel::<adt::Int8Type>(list_array),
            DataType::Int16 => Self::lift_primitive_kernel::<adt::Int16Type>(list_array),
            DataType::Int32 => Self::lift_primitive_kernel::<adt::Int32Type>(list_array),
            DataType::Int64 => Self::lift_primitive_kernel::<adt::Int64Type>(list_array),
            DataType::Float16 => Self::lift_primitive_kernel::<adt::Float16Type>(list_array),
            DataType::Float32 => Self::lift_primitive_kernel::<adt::Float32Type>(list_array),
            DataType::Float64 => Self::lift_primitive_kernel::<adt::Float64Type>(list_array),
            DataType::Decimal128(_, _) => {
                Self::lift_primitive_kernel::<adt::Decimal128Type>(list_array)
            }
            DataType::Decimal256(_, _) => {
                Self::lift_primitive_kernel::<adt::Decimal256Type>(list_array)
            }
            // temporal primitive types
            DataType::Date32 => Self::lift_primitive_kernel::<adt::Date32Type>(list_array),
            DataType::Date64 => Self::lift_primitive_kernel::<adt::Date64Type>(list_array),
            DataType::Timestamp(TimeUnit::Second, _) => {
                Self::lift_primitive_kernel::<adt::TimestampSecondType>(list_array)
            }
            DataType::Timestamp(TimeUnit::Millisecond, _) => {
                Self::lift_primitive_kernel::<adt::TimestampMillisecondType>(list_array)
            }
            DataType::Timestamp(TimeUnit::Microsecond, _) => {
                Self::lift_primitive_kernel::<adt::TimestampMicrosecondType>(list_array)
            }
            DataType::Timestamp(TimeUnit::Nanosecond, _) => {
                Self::lift_primitive_kernel::<adt::TimestampNanosecondType>(list_array)
            }
            DataType::Time32(TimeUnit::Second) => {
                Self::lift_primitive_kernel::<adt::Time32SecondType>(list_array)
            }
            DataType::Time32(TimeUnit::Millisecond) => {
                Self::lift_primitive_kernel::<adt::Time32MillisecondType>(list_array)
            }
            DataType::Time64(TimeUnit::Microsecond) => {
                Self::lift_primitive_kernel::<adt::Time64MicrosecondType>(list_array)
            }
            DataType::Time64(TimeUnit::Nanosecond) => {
                Self::lift_primitive_kernel::<adt::Time64NanosecondType>(list_array)
            }
            DataType::Duration(TimeUnit::Second) => {
                Self::lift_primitive_kernel::<adt::DurationSecondType>(list_array)
            }
            DataType::Duration(TimeUnit::Millisecond) => {
                Self::lift_primitive_kernel::<adt::DurationMillisecondType>(list_array)
            }
            DataType::Duration(TimeUnit::Microsecond) => {
                Self::lift_primitive_kernel::<adt::DurationMicrosecondType>(list_array)
            }
            DataType::Duration(TimeUnit::Nanosecond) => {
                Self::lift_primitive_kernel::<adt::DurationNanosecondType>(list_array)
            }
            DataType::Interval(IntervalUnit::DayTime) => {
                Self::lift_primitive_kernel::<adt::IntervalDayTimeType>(list_array)
            }
            DataType::Interval(IntervalUnit::MonthDayNano) => {
                Self::lift_primitive_kernel::<adt::IntervalMonthDayNanoType>(list_array)
            }
            DataType::Interval(IntervalUnit::YearMonth) => {
                Self::lift_primitive_kernel::<adt::IntervalYearMonthType>(list_array)
            }

            // string arrays
            DataType::Utf8 => Self::lift_string_kernel::<i32>(list_array),
            DataType::LargeUtf8 => Self::lift_string_kernel::<i64>(list_array),

            // binary arrays
            DataType::Binary => Self::lift_binary_kernel::<i32>(list_array),
            DataType::LargeBinary => Self::lift_binary_kernel::<i64>(list_array),

            // boolean arrays
            DataType::Boolean => Self::lift_boolean_kernel(list_array),

            _ => {
                return Err(DataFusionError::NotImplemented(format!(
                    "Not implemented: {} on a list array of {} elements. {}:{}",
                    Self::FEATURE_NAME,
                    elem_dt,
                    file!(),
                    line!()
                )))
            }
        };
        Ok(res)
    }

    /// Given the Arrow DataType of the array to be passed to `lift_all_kernels`,
    /// determines whether `lift_all_kernels` is applicable and what the DataType of the result array will be.
    fn return_type(arg: &DataType) -> Result<DataType> {
        let field = match arg {
            DataType::List(f) => Ok(f),
            DataType::LargeList(_) | DataType::FixedSizeList(_, _) => {
                Err(DataFusionError::NotImplemented(format!(
                    "Not implemented: {} on LargeList or FixedSizeList. {}:{}",
                    Self::FEATURE_NAME,
                    file!(),
                    line!()
                )))
            }
            _ => Err(DataFusionError::Plan(format!(
                "The {} function can only accept List/LargeList/FixedSizeList.",
                Self::FEATURE_NAME
            ))),
        }?;
        let dt = field.data_type();
        match dt {
            DataType::Null => Ok(dt.clone()),
            dt if dt.is_primitive() => Ok(dt.clone()),
            DataType::Utf8 | DataType::LargeUtf8 => Ok(dt.clone()),
            DataType::Binary | DataType::LargeBinary => Ok(dt.clone()),
            DataType::Boolean => Ok(dt.clone()),
            dt => Err(DataFusionError::NotImplemented(format!(
                "Not implemented: {} on arrays with items of type: {}. {}:{}",
                Self::FEATURE_NAME,
                dt,
                file!(),
                line!()
            ))),
        }
    }
}

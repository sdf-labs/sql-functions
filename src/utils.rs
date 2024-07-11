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

use arrow::array::{ArrayRef, Datum};
use arrow::compute::{cast, date_part, DatePart};
use arrow::datatypes::DataType;
use arrow::error::Result as ArrowResult;
use datafusion::common::{Result, ScalarValue};
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
            Err(_) => ColumnarValue::Array(array),
        }
    } else {
        ColumnarValue::Array(array)
    }
}

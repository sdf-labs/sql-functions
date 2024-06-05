// start implementing uses
use arrow::array::PrimitiveArray;
use arrow::compute::DatePart;
use arrow::datatypes::{DataType, Int32Type};
use chrono::{Datelike, Duration, NaiveDate};
use datafusion::common::cast::as_date32_array;
use datafusion::common::{exec_err, Result};
use datafusion::logical_expr::simplify::{ExprSimplifyResult, SimplifyInfo};
use datafusion::logical_expr::{ColumnarValue, Expr, ScalarUDFImpl, Signature, Volatility};
use datafusion::scalar::ScalarValue;
use std::any::Any;
use std::sync::Arc;
// end implementing uses

#[derive(Debug)]
pub(super) struct Func {
    signature: Signature,
}

impl Func {
    pub fn new() -> Self {        
        // start implementing constructor
        Self {
            signature: Signature::exact(
                vec![DataType::Utf8, DataType::Date32, DataType::Date32],
                Volatility::Immutable,
            ),
        }
        // end implementing constructor
    }
}

impl ScalarUDFImpl for Func {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "date_diff"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }

    // start implementing return_type
    fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType> {
        Ok(DataType::Int32)
    }
    // end implementing return_type

    // start implementing invoke
    fn invoke(&self, args: &[ColumnarValue]) -> Result<ColumnarValue> {
        assert_eq!(args.len(), 3);
        let part = if let ColumnarValue::Scalar(ScalarValue::Utf8(Some(v))) = args[0].to_owned() {
            v
        } else {
            return exec_err!("First argument of `DATE_PART` must be non-null scalar Utf8");
        };

        let args = ColumnarValue::values_to_arrays(args)?;

        let start = args[1].to_owned();
        let end = args[2].to_owned();

        let part = match part.as_str() {
            "year" => DatePart::Year,
            "month" => DatePart::Month,
            "week" => DatePart::Week,
            "day" => DatePart::Day,
            "hour" => DatePart::Hour,
            "minute" => DatePart::Minute,
            "second" => DatePart::Second,
            "millisecond" => DatePart::Millisecond,
            "microsecond" => DatePart::Microsecond,
            "nanosecond" => DatePart::Nanosecond,
            _ => return exec_err!("Date part '{part}' not supported"),
        };

        let array = as_date32_array(&start)?
            .into_iter()
            .zip(as_date32_array(&end)?.into_iter())
            .map(|(start, end)| {
                if let (Some(start), Some(end)) = (start, end) {
                    match part {
                        DatePart::Year => {
                            let (start, end, sign) = if start < end {
                                (start, end, 1)
                            } else {
                                (end, start, -1)
                            };
                            let start = NaiveDate::default() + Duration::days(start as i64);
                            let end = NaiveDate::default() + Duration::days(end as i64);
                            let year = end.year() - start.year();
                            let start_without_year = (start.month0(), start.day0());
                            let end_without_year = (end.month0(), end.day0());

                            let diff = Some(if end_without_year < start_without_year {
                                year - 1
                            } else {
                                year
                            });
                            Ok(diff.map(|diff| diff * sign))
                        }
                        DatePart::Month => {
                            let (start, end, sign) = if start < end {
                                (start, end, 1)
                            } else {
                                (end, start, -1)
                            };
                            let start = NaiveDate::default() + Duration::days(start as i64);
                            let end = NaiveDate::default() + Duration::days(end as i64);
                            let month = (end.year() - start.year()) * 12
                                + (end.month() as i32 - start.month() as i32);
                            let diff = Some(if end.day0() < start.day0() {
                                month - 1
                            } else {
                                month
                            });
                            Ok(diff.map(|diff| diff * sign))
                        }
                        DatePart::Week => {
                            let start = NaiveDate::default() + Duration::days(start as i64);
                            let end = NaiveDate::default() + Duration::days(end as i64);
                            let diff = end - start;
                            let weeks = diff.num_days() / 7;
                            Ok(Some(weeks as i32))
                        }
                        DatePart::Day => Ok(Some(end - start)),
                        DatePart::Hour
                        | DatePart::Minute
                        | DatePart::Second
                        | DatePart::Millisecond
                        | DatePart::Microsecond
                        | DatePart::Nanosecond => {
                            exec_err!("Date part '{part}' is not a valid DATE field")
                        }
                        _ => Ok(None),
                    }
                } else {
                    Ok(None)
                }
            })
            .collect::<Result<PrimitiveArray<Int32Type>>>()?;
        Ok(ColumnarValue::Array(Arc::new(array)))
    }
    // end implementing invoke

    // start implementing simplify
    fn simplify(&self, args: Vec<Expr>, _info: &dyn SimplifyInfo) -> Result<ExprSimplifyResult> {
        Ok(ExprSimplifyResult::Original(args))
    }
    // end implementing simplify
}

// start implementing footer
// end implementing footer

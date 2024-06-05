// start implementing uses
use arrow::array::PrimitiveArray;
use arrow::compute::DatePart;
use arrow::datatypes::{DataType, Int64Type, TimeUnit};
use datafusion::common::cast::{as_time32_millisecond_array, as_time32_second_array, as_time64_microsecond_array, as_time64_nanosecond_array};
use datafusion::common::{exec_err, Result};
use datafusion::logical_expr::simplify::{ExprSimplifyResult, SimplifyInfo};
use datafusion::logical_expr::TypeSignature::Exact;
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
        use arrow::datatypes::TimeUnit::*;
        use DataType::*;
        Self {
            signature: Signature::one_of(
                vec![
                    Exact(vec![Utf8, Time32(Second), Time32(Second)]),
                    Exact(vec![Utf8, Time32(Second), Time32(Millisecond)]),
                    Exact(vec![Utf8, Time32(Second), Time64(Microsecond)]),
                    Exact(vec![Utf8, Time32(Second), Time64(Nanosecond)]),
                    Exact(vec![Utf8, Time32(Millisecond), Time32(Second)]),
                    Exact(vec![Utf8, Time32(Millisecond), Time32(Millisecond)]),
                    Exact(vec![Utf8, Time32(Millisecond), Time64(Microsecond)]),
                    Exact(vec![Utf8, Time32(Millisecond), Time64(Nanosecond)]),
                    Exact(vec![Utf8, Time64(Microsecond), Time32(Second)]),
                    Exact(vec![Utf8, Time64(Microsecond), Time32(Millisecond)]),
                    Exact(vec![Utf8, Time64(Microsecond), Time64(Microsecond)]),
                    Exact(vec![Utf8, Time64(Microsecond), Time64(Nanosecond)]),
                    Exact(vec![Utf8, Time64(Nanosecond), Time32(Second)]),
                    Exact(vec![Utf8, Time64(Nanosecond), Time32(Millisecond)]),
                    Exact(vec![Utf8, Time64(Nanosecond), Time64(Microsecond)]),
                    Exact(vec![Utf8, Time64(Nanosecond), Time64(Nanosecond)]),
                ],
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
        Ok(DataType::Int64)
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
            "quarter" => DatePart::Quarter,
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

        let start = match start.data_type() {
            DataType::Time32(TimeUnit::Second) => as_time32_second_array(start.as_ref())?
                .into_iter()
                .map(|v| v.map(|v| v as i64 * 1_000))
                .collect::<Vec<_>>(),
            DataType::Time32(TimeUnit::Millisecond) => as_time32_millisecond_array(start.as_ref())?
                .into_iter()
                .map(|v| v.map(|v| v as i64 * 1))
                .collect::<Vec<_>>(),
            DataType::Time64(TimeUnit::Microsecond) => as_time64_microsecond_array(start.as_ref())?
                .into_iter()
                .map(|v| v.map(|v| v / 1_000))
                .collect::<Vec<_>>(),
            DataType::Time64(TimeUnit::Nanosecond) => as_time64_nanosecond_array(start.as_ref())?
                .into_iter()
                .map(|v| v.map(|v| v / 1_000_000))
                .collect::<Vec<_>>(),
            _ => unreachable!("Invalid data type"),
        };

        let end = match end.data_type() {
            DataType::Time32(TimeUnit::Second) => as_time32_second_array(end.as_ref())?
                .into_iter()
                .map(|v| v.map(|v| v as i64 * 1_000))
                .collect::<Vec<_>>(),
            DataType::Time32(TimeUnit::Millisecond) => as_time32_millisecond_array(end.as_ref())?
                .into_iter()
                .map(|v| v.map(|v| v as i64 * 1))
                .collect::<Vec<_>>(),
            DataType::Time64(TimeUnit::Microsecond) => as_time64_microsecond_array(end.as_ref())?
                .into_iter()
                .map(|v| v.map(|v| v / 1_000))
                .collect::<Vec<_>>(),
            DataType::Time64(TimeUnit::Nanosecond) => as_time64_nanosecond_array(end.as_ref())?
                .into_iter()
                .map(|v| v.map(|v| v / 1_000_000))
                .collect::<Vec<_>>(),
            _ => unreachable!("Invalid data type"),
        };

        let array = start
            .into_iter()
            .zip(end.into_iter())
            .map(|(start, end)| {
                if let (Some(start), Some(end)) = (start, end) {
                    match part {
                        DatePart::Year
                        | DatePart::Month
                        | DatePart::Week
                        | DatePart::Day
                        | DatePart::Microsecond
                        | DatePart::Nanosecond => {
                            exec_err!("Date part '{part}' is not a valid TIME field")
                        }
                        DatePart::Hour => Ok(Some((end - start) / 3_600_000)),
                        DatePart::Minute => Ok(Some((end - start) / 60_000)),
                        DatePart::Second => Ok(Some((end - start) / 1_000)),
                        DatePart::Millisecond => Ok(Some((end - start) / 1)),
                        _ => Ok(None),
                    }
                } else {
                    Ok(None)
                }
            })
            .collect::<Result<PrimitiveArray<Int64Type>>>()?;
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

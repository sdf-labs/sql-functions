#![allow(non_camel_case_types)]
use arrow::array::PrimitiveArray;
use arrow::compute::DatePart;
use arrow::datatypes::{DataType, Int32Type, Int64Type, TimeUnit};
use chrono::{DateTime, Datelike, Duration, NaiveDate, Timelike};
use datafusion::common::cast::{as_date32_array, as_time32_millisecond_array, as_time32_second_array, as_time64_microsecond_array, as_time64_nanosecond_array, as_timestamp_microsecond_array, as_timestamp_millisecond_array, as_timestamp_nanosecond_array, as_timestamp_second_array};
use datafusion::common::{exec_err, Result};
use datafusion::logical_expr::simplify::{ExprSimplifyResult, SimplifyInfo};
use datafusion::logical_expr::{ColumnarValue, Expr, ScalarUDFImpl, Signature, Volatility};
use datafusion::scalar::ScalarValue;
use std::any::Any;
use std::sync::Arc;

fn date_diff_varchar_date_date_invoke(args: &[ColumnarValue]) -> Result<ColumnarValue> {
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

fn date_diff_varchar_date_date_return_type(_arg_types: &[DataType]) -> Result<DataType> {
    Ok(DataType::Int32)
}

fn date_diff_varchar_date_date_simplify(
    args: Vec<Expr>,
    _info: &dyn SimplifyInfo,
) -> Result<ExprSimplifyResult> {
    Ok(ExprSimplifyResult::Original(args))
}

fn date_diff_varchar_time_p_time_p_invoke(args: &[ColumnarValue]) -> Result<ColumnarValue> {
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

fn date_diff_varchar_time_p_time_p_return_type(_arg_types: &[DataType]) -> Result<DataType> {
    Ok(DataType::Int64)
}

fn date_diff_varchar_time_p_time_p_simplify(
    args: Vec<Expr>,
    _info: &dyn SimplifyInfo,
) -> Result<ExprSimplifyResult> {
    Ok(ExprSimplifyResult::Original(args))
}

fn date_diff_varchar_timestamp_p_timestamp_p_invoke(
    args: &[ColumnarValue],
) -> Result<ColumnarValue> {
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
        DataType::Timestamp(TimeUnit::Second, _) => as_timestamp_second_array(start.as_ref())?
            .into_iter()
            .map(|v| v.map(|v| v * 1_000))
            .collect::<Vec<_>>(),
        DataType::Timestamp(TimeUnit::Millisecond, _) => {
            as_timestamp_millisecond_array(start.as_ref())?
                .into_iter()
                .map(|v| v.map(|v| v * 1))
                .collect::<Vec<_>>()
        }
        DataType::Timestamp(TimeUnit::Microsecond, _) => {
            as_timestamp_microsecond_array(start.as_ref())?
                .into_iter()
                .map(|v| v.map(|v| v / 1_000))
                .collect::<Vec<_>>()
        }
        DataType::Timestamp(TimeUnit::Nanosecond, _) => {
            as_timestamp_nanosecond_array(start.as_ref())?
                .into_iter()
                .map(|v| v.map(|v| v / 1_000_000))
                .collect::<Vec<_>>()
        }
        _ => unreachable!("Invalid data type"),
    };

    let end = match end.data_type() {
        DataType::Timestamp(TimeUnit::Second, _) => as_timestamp_second_array(end.as_ref())?
            .into_iter()
            .map(|v| v.map(|v| v * 1_000))
            .collect::<Vec<_>>(),
        DataType::Timestamp(TimeUnit::Millisecond, _) => {
            as_timestamp_millisecond_array(end.as_ref())?
                .into_iter()
                .map(|v| v.map(|v| v * 1))
                .collect::<Vec<_>>()
        }
        DataType::Timestamp(TimeUnit::Microsecond, _) => {
            as_timestamp_microsecond_array(end.as_ref())?
                .into_iter()
                .map(|v| v.map(|v| v / 1_000))
                .collect::<Vec<_>>()
        }
        DataType::Timestamp(TimeUnit::Nanosecond, _) => {
            as_timestamp_nanosecond_array(end.as_ref())?
                .into_iter()
                .map(|v| v.map(|v| v / 1_000_000))
                .collect::<Vec<_>>()
        }
        _ => unreachable!("Invalid data type"),
    };

    let array = start
        .into_iter()
        .zip(end.into_iter())
        .map(|(start, end)| {
            if let (Some(start), Some(end)) = (start, end) {
                match part {
                    DatePart::Year => {
                        let (start, end, sign) = if start < end {
                            (start, end, 1)
                        } else {
                            (end, start, -1)
                        };

                        let start = DateTime::from_timestamp(
                            start / 1_000,
                            (start % 1_000) as u32 * 1_000_000,
                        );
                        let end = DateTime::from_timestamp(
                            end / 1_000,
                            (end % 1_000) as u32 * 1_000_000,
                        );
                        let diff = if let (Some(start), Some(end)) = (start, end) {
                            let year = end.year() - start.year();
                            let start_without_year = (
                                start.month(),
                                start.day(),
                                start.hour(),
                                start.minute(),
                                start.second(),
                                start.nanosecond(),
                            );
                            let end_without_year = (
                                end.month(),
                                end.day(),
                                end.hour(),
                                end.minute(),
                                end.second(),
                                end.nanosecond(),
                            );
                            if end_without_year < start_without_year {
                                Some(year - 1)
                            } else {
                                Some(year)
                            }
                        } else {
                            None
                        };
                        Ok(diff.map(|diff| (diff * sign) as i64))
                    }
                    DatePart::Month => {
                        let (start, end, sign) = if start < end {
                            (start, end, 1)
                        } else {
                            (end, start, -1)
                        };

                        let start = DateTime::from_timestamp(
                            start / 1_000,
                            (start % 1_000) as u32 * 1_000_000,
                        );
                        let end = DateTime::from_timestamp(
                            end / 1_000,
                            (end % 1_000) as u32 * 1_000_000,
                        );
                        let diff = if let (Some(start), Some(end)) = (start, end) {
                            let month = (end.year() - start.year()) * 12
                                + (end.month() as i32 - start.month() as i32);
                            let start_without_month = (
                                start.day(),
                                start.hour(),
                                start.minute(),
                                start.second(),
                                start.nanosecond(),
                            );
                            let end_without_month = (
                                end.day(),
                                end.hour(),
                                end.minute(),
                                end.second(),
                                end.nanosecond(),
                            );
                            Some(if end_without_month < start_without_month {
                                month - 1
                            } else {
                                month
                            })
                        } else {
                            None
                        };
                        Ok(diff.map(|diff| (diff * sign) as i64))
                    }
                    DatePart::Week => Ok(Some((end - start) / 604_800_000)),
                    DatePart::Day => Ok(Some((end - start) / 86_400_000)),
                    DatePart::Hour => Ok(Some((end - start) / 3_600_000)),
                    DatePart::Minute => Ok(Some((end - start) / 60_000)),
                    DatePart::Second => Ok(Some((end - start) / 1_000)),
                    DatePart::Millisecond => Ok(Some((end - start) / 1)),
                    DatePart::Microsecond | DatePart::Nanosecond => {
                        exec_err!("Date part '{part}' is not a valid TIMESTAMP field")
                    }
                    _ => Ok(None),
                }
            } else {
                Ok(None)
            }
        })
        .collect::<Result<PrimitiveArray<Int64Type>>>()?;
    Ok(ColumnarValue::Array(Arc::new(array)))
}

fn date_diff_varchar_timestamp_p_timestamp_p_return_type(
    _arg_types: &[DataType],
) -> Result<DataType> {
    Ok(DataType::Int64)
}

fn date_diff_varchar_timestamp_p_timestamp_p_simplify(
    args: Vec<Expr>,
    _info: &dyn SimplifyInfo,
) -> Result<ExprSimplifyResult> {
    Ok(ExprSimplifyResult::Original(args))
}

/// ========== Above are user code/Below are generated code ==========


#[derive(Debug)]
pub(super) struct date_diff_varchar_date_dateFunc {
    signature: Signature,
}

impl date_diff_varchar_date_dateFunc {
    pub fn new() -> Self {        
        Self {
            signature: Signature::any(3, Volatility::Immutable),
        }
    }
}

impl ScalarUDFImpl for date_diff_varchar_date_dateFunc {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "date_diff"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }


    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        date_diff_varchar_date_date_return_type(arg_types)
    }

    fn invoke(&self, args: &[ColumnarValue]) -> Result<ColumnarValue> {
        date_diff_varchar_date_date_invoke(args)
    }

    fn simplify(
        &self,
        args: Vec<Expr>,
        info: &dyn SimplifyInfo,
    ) -> Result<ExprSimplifyResult> {
        date_diff_varchar_date_date_simplify(args, info)
    }

}

#[derive(Debug)]
pub(super) struct date_diff_varchar_time_p_time_pFunc {
    signature: Signature,
}

impl date_diff_varchar_time_p_time_pFunc {
    pub fn new() -> Self {        
        Self {
            signature: Signature::any(3, Volatility::Immutable),
        }
    }
}

impl ScalarUDFImpl for date_diff_varchar_time_p_time_pFunc {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "date_diff"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }


    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        date_diff_varchar_time_p_time_p_return_type(arg_types)
    }

    fn invoke(&self, args: &[ColumnarValue]) -> Result<ColumnarValue> {
        date_diff_varchar_time_p_time_p_invoke(args)
    }

    fn simplify(
        &self,
        args: Vec<Expr>,
        info: &dyn SimplifyInfo,
    ) -> Result<ExprSimplifyResult> {
        date_diff_varchar_time_p_time_p_simplify(args, info)
    }

}

#[derive(Debug)]
pub(super) struct date_diff_varchar_timestamp_p_timestamp_pFunc {
    signature: Signature,
}

impl date_diff_varchar_timestamp_p_timestamp_pFunc {
    pub fn new() -> Self {        
        Self {
            signature: Signature::any(3, Volatility::Immutable),
        }
    }
}

impl ScalarUDFImpl for date_diff_varchar_timestamp_p_timestamp_pFunc {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "date_diff"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }


    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        date_diff_varchar_timestamp_p_timestamp_p_return_type(arg_types)
    }

    fn invoke(&self, args: &[ColumnarValue]) -> Result<ColumnarValue> {
        date_diff_varchar_timestamp_p_timestamp_p_invoke(args)
    }

    fn simplify(
        &self,
        args: Vec<Expr>,
        info: &dyn SimplifyInfo,
    ) -> Result<ExprSimplifyResult> {
        date_diff_varchar_timestamp_p_timestamp_p_simplify(args, info)
    }

}

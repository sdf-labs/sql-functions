// start implementing uses
use arrow::array::{ArrayRef, ListArray, StringArray};
use arrow::buffer::{NullBuffer, OffsetBuffer};
use arrow::datatypes::{DataType, Field};
use datafusion::common::cast::{as_int64_array, as_string_array};
use datafusion::common::Result;
use datafusion::logical_expr::simplify::{ExprSimplifyResult, SimplifyInfo};
use datafusion::logical_expr::{ColumnarValue, Expr, ScalarUDFImpl, Signature, Volatility};
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
        // Example
        Self {
            signature:
            Signature::any(3, Volatility::Immutable),
        }

        // end implementing constructor
    }
}

impl ScalarUDFImpl for Func {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "split"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }

    // start implementing return_type
    fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType> {
        Ok(DataType::List(Arc::new(Field::new("item", DataType::Utf8, true))))
    }
    // end implementing return_type

    // start implementing invoke
    fn invoke(&self, args: &[ColumnarValue]) -> Result<ColumnarValue> {
        let args = ColumnarValue::values_to_arrays(args)?;
        let string_arr = as_string_array(&args[0])?;
        let delimiter_arr = as_string_array(&args[1])?;
        let limit_arr = as_int64_array(&args[2])?;

        let mut values = vec![];
        let mut nulls = vec![];
        let mut offsets = vec![];
        string_arr
            .iter()
            .zip(delimiter_arr.iter())
            .zip(limit_arr.iter())
            .for_each(
                |((string, delimiter), limit)| match (string, delimiter, limit) {
                    (Some(string), Some(delimiter), Some(limit)) => {
                        let split = string
                            .splitn(limit as usize, delimiter)
                            .map(|s| s.to_string())
                            .collect::<Vec<_>>();
                        offsets.push(split.len());
                        values.extend(split);
                        nulls.push(true);
                    }
                    _ => {
                        offsets.push(1);
                        values.push("".to_string());
                        nulls.push(false);
                    }
                },
            );

        let element_type = Arc::new(Field::new("item", DataType::Utf8, true));
        let array = Arc::new(ListArray::try_new(
            Arc::clone(&element_type),
            OffsetBuffer::<i32>::from_lengths(offsets),
            Arc::new(StringArray::from(values)),
            Some(NullBuffer::from(nulls)),
        )?) as ArrayRef;
        Ok(ColumnarValue::Array(array))

    }
    // end implementing invoke

    // start implementing simplify
    fn simplify(
        &self,
        args: Vec<Expr>,
        _info: &dyn SimplifyInfo,
    ) -> Result<ExprSimplifyResult> {
        Ok(ExprSimplifyResult::Original(args))
    }
    // end implementing simplify
}

// start implementing footer
// end implementing footer
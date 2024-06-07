// start implementing uses
use arrow::array::{Array, BooleanArray, ListArray};
use arrow::datatypes::DataType;
use datafusion::common::cast::{as_generic_list_array, as_string_array};
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
            signature: Signature::any(2, Volatility::Immutable),
        }

        // end implementing constructor
    }
}

impl ScalarUDFImpl for Func {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "contains_array_varchar"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }

    // start implementing return_type
    fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType> {
        Ok(DataType::Boolean)
    }
    // end implementing return_type

    // start implementing invoke
    fn invoke(&self, args: &[ColumnarValue]) -> Result<ColumnarValue> {
        let args: Vec<std::sync::Arc<dyn Array>> = ColumnarValue::values_to_arrays(args)?;
        let array: &ListArray = as_generic_list_array(&args[0])?;
        let target = as_string_array(&args[1])?;

        let array = array
            .iter()
            .zip(target.iter())
            .map(|(array, target)| match (array, target) {
                (Some(array), Some(target)) => {
                    let array = as_string_array(&array)?;
                    for i in 0..array.len() {
                        if array.value(i).contains(target) {
                            return Ok(Some(true));
                        }
                    }
                    Ok(Some(false))
                }
                _ => Ok(None),
            })
            .collect::<Result<BooleanArray>>()?;

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

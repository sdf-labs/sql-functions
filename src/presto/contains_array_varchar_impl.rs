#![allow(non_camel_case_types)]
use arrow::array::{Array, BooleanArray, ListArray};
use arrow::datatypes::DataType;
use datafusion::common::cast::{as_generic_list_array, as_string_array};
use datafusion::common::Result;
use datafusion::logical_expr::simplify::{ExprSimplifyResult, SimplifyInfo};
use datafusion::logical_expr::{ColumnarValue, Expr, ScalarUDFImpl, Signature, Volatility};
use std::any::Any;
use std::sync::Arc;

fn contains_array_varchar_array_varchar_varchar_invoke(args: &[ColumnarValue]) -> Result<ColumnarValue> {
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

fn contains_array_varchar_array_varchar_varchar_return_type(_arg_types: &[DataType]) -> Result<DataType> {
    Ok(DataType::Boolean)
}

fn contains_array_varchar_array_varchar_varchar_simplify(args: Vec<Expr>, _info: &dyn SimplifyInfo) -> Result<ExprSimplifyResult> {
    Ok(ExprSimplifyResult::Original(args))
}


/// ========== Above are user code/Below are generated code ==========


#[derive(Debug)]
pub(super) struct contains_array_varchar_array_varchar_varcharFunc {
    signature: Signature,
}

impl contains_array_varchar_array_varchar_varcharFunc {
    pub fn new() -> Self {        
        Self {
            signature: Signature::any(2, Volatility::Immutable),
        }
    }
}

impl ScalarUDFImpl for contains_array_varchar_array_varchar_varcharFunc {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "contains_array_varchar"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }


    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        contains_array_varchar_array_varchar_varchar_return_type(arg_types)
    }

    fn invoke(&self, args: &[ColumnarValue]) -> Result<ColumnarValue> {
        contains_array_varchar_array_varchar_varchar_invoke(args)
    }

    fn simplify(
        &self,
        args: Vec<Expr>,
        info: &dyn SimplifyInfo,
    ) -> Result<ExprSimplifyResult> {
        contains_array_varchar_array_varchar_varchar_simplify(args, info)
    }

}

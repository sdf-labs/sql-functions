#![allow(non_camel_case_types)]
use arrow::datatypes::DataType;
use datafusion::common::Result;
use datafusion::error::DataFusionError;
use datafusion::logical_expr::simplify::{ExprSimplifyResult, SimplifyInfo};
use datafusion::logical_expr::{ColumnarValue, Expr, ScalarUDFImpl, Signature, Volatility};
use std::any::Any;


fn contains_sequence_array_1_array_1_invoke(_args: &[ColumnarValue]) -> Result<ColumnarValue> {
    Err(DataFusionError::NotImplemented("todo".to_string()))
}

fn contains_sequence_array_1_array_1_return_type(_arg_types: &[DataType]) -> Result<DataType> {
    Err(DataFusionError::NotImplemented("todo".to_string()))
}

fn contains_sequence_array_1_array_1_simplify(args: Vec<Expr>, _info: &dyn SimplifyInfo) -> Result<ExprSimplifyResult> {
    Ok(ExprSimplifyResult::Original(args))
}


/// ========== Above are user code/Below are generated code ==========


#[derive(Debug)]
pub(super) struct contains_sequence_array_1_array_1Func {
    signature: Signature,
}

impl contains_sequence_array_1_array_1Func {
    pub fn new() -> Self {        
        Self {
            signature: Signature::any(2, Volatility::Immutable),
        }
    }
}

impl ScalarUDFImpl for contains_sequence_array_1_array_1Func {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "contains_sequence"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }


    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        contains_sequence_array_1_array_1_return_type(arg_types)
    }

    fn invoke(&self, args: &[ColumnarValue]) -> Result<ColumnarValue> {
        contains_sequence_array_1_array_1_invoke(args)
    }

    fn simplify(
        &self,
        args: Vec<Expr>,
        info: &dyn SimplifyInfo,
    ) -> Result<ExprSimplifyResult> {
        contains_sequence_array_1_array_1_simplify(args, info)
    }

}

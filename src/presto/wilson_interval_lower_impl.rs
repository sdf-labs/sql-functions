#![allow(non_camel_case_types)]
use arrow::datatypes::DataType;
use datafusion::common::Result;
use datafusion::error::DataFusionError;
use datafusion::logical_expr::simplify::{ExprSimplifyResult, SimplifyInfo};
use datafusion::logical_expr::{ColumnarValue, Expr, ScalarUDFImpl, Signature, Volatility};
use std::any::Any;


fn wilson_interval_lower_bigint_bigint_double_invoke(_args: &[ColumnarValue]) -> Result<ColumnarValue> {
    Err(DataFusionError::NotImplemented("todo".to_string()))
}

fn wilson_interval_lower_bigint_bigint_double_return_type(_arg_types: &[DataType]) -> Result<DataType> {
    Err(DataFusionError::NotImplemented("todo".to_string()))
}

fn wilson_interval_lower_bigint_bigint_double_simplify(args: Vec<Expr>, _info: &dyn SimplifyInfo) -> Result<ExprSimplifyResult> {
    Ok(ExprSimplifyResult::Original(args))
}


/// ========== Above are user code/Below are generated code ==========


#[derive(Debug)]
pub(super) struct wilson_interval_lower_bigint_bigint_doubleFunc {
    signature: Signature,
}

impl wilson_interval_lower_bigint_bigint_doubleFunc {
    pub fn new() -> Self {        
        Self {
            signature: Signature::any(3, Volatility::Immutable),
        }
    }
}

impl ScalarUDFImpl for wilson_interval_lower_bigint_bigint_doubleFunc {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "wilson_interval_lower"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }


    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        wilson_interval_lower_bigint_bigint_double_return_type(arg_types)
    }

    fn invoke(&self, args: &[ColumnarValue]) -> Result<ColumnarValue> {
        wilson_interval_lower_bigint_bigint_double_invoke(args)
    }

    fn simplify(
        &self,
        args: Vec<Expr>,
        info: &dyn SimplifyInfo,
    ) -> Result<ExprSimplifyResult> {
        wilson_interval_lower_bigint_bigint_double_simplify(args, info)
    }

}

#![allow(non_camel_case_types)]
use arrow::datatypes::DataType;
use datafusion::common::Result;
use datafusion::error::DataFusionError;
use datafusion::logical_expr::simplify::{ExprSimplifyResult, SimplifyInfo};
use datafusion::logical_expr::{ColumnarValue, Expr, ScalarUDFImpl, Signature, Volatility};
use std::any::Any;


fn st_polygon_varchar_invoke(_args: &[ColumnarValue]) -> Result<ColumnarValue> {
    Err(DataFusionError::NotImplemented("todo".to_string()))
}

fn st_polygon_varchar_return_type(_arg_types: &[DataType]) -> Result<DataType> {
    Err(DataFusionError::NotImplemented("todo".to_string()))
}

fn st_polygon_varchar_simplify(args: Vec<Expr>, _info: &dyn SimplifyInfo) -> Result<ExprSimplifyResult> {
    Ok(ExprSimplifyResult::Original(args))
}


/// ========== Above are user code/Below are generated code ==========


#[derive(Debug)]
pub(super) struct st_polygon_varcharFunc {
    signature: Signature,
}

impl st_polygon_varcharFunc {
    pub fn new() -> Self {        
        Self {
            signature: Signature::any(1, Volatility::Immutable),
        }
    }
}

impl ScalarUDFImpl for st_polygon_varcharFunc {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "st_polygon"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }


    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        st_polygon_varchar_return_type(arg_types)
    }

    fn invoke(&self, args: &[ColumnarValue]) -> Result<ColumnarValue> {
        st_polygon_varchar_invoke(args)
    }

    fn simplify(
        &self,
        args: Vec<Expr>,
        info: &dyn SimplifyInfo,
    ) -> Result<ExprSimplifyResult> {
        st_polygon_varchar_simplify(args, info)
    }

}

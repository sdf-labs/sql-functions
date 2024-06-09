#![allow(non_camel_case_types)]
use arrow::compute::kernels::zip::zip;
use arrow::datatypes::DataType;
use datafusion::common::cast::as_boolean_array;
use datafusion::common::Result;
use datafusion::logical_expr::simplify::{ExprSimplifyResult, SimplifyInfo};
use datafusion::logical_expr::{ColumnarValue, Expr, ScalarUDFImpl, Signature, Volatility};
use std::any::Any;
use std::sync::Arc;


fn if_boolean_1_1_invoke(args: &[ColumnarValue]) -> Result<ColumnarValue> {
    let args = ColumnarValue::values_to_arrays(args)?;
        let conditions = as_boolean_array(&args[0])?;
        let left = args[1].to_owned();
        let right = args[2].to_owned();
        let result = zip(&conditions, &left, &right)?;
        Ok(ColumnarValue::Array(Arc::new(result)))
}

fn if_boolean_1_1_return_type(arg_types: &[DataType]) -> Result<DataType> {
    Ok(arg_types[1].clone())
}

fn if_boolean_1_1_simplify(args: Vec<Expr>, _info: &dyn SimplifyInfo) -> Result<ExprSimplifyResult> {
    Ok(ExprSimplifyResult::Original(args))
}


/// ========== Above are user code/Below are generated code ==========


#[derive(Debug)]
pub(super) struct if_boolean_1_1Func {
    signature: Signature,
}

impl if_boolean_1_1Func {
    pub fn new() -> Self {        
        Self {
            signature: Signature::any(3, Volatility::Immutable),
        }
    }
}

impl ScalarUDFImpl for if_boolean_1_1Func {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "if"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }


    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        if_boolean_1_1_return_type(arg_types)
    }

    fn invoke(&self, args: &[ColumnarValue]) -> Result<ColumnarValue> {
        if_boolean_1_1_invoke(args)
    }

    fn simplify(
        &self,
        args: Vec<Expr>,
        info: &dyn SimplifyInfo,
    ) -> Result<ExprSimplifyResult> {
        if_boolean_1_1_simplify(args, info)
    }

}

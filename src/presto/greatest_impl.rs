#![allow(non_camel_case_types)]
use arrow::compute::kernels::cmp::lt;
use arrow::compute::kernels::zip::zip;
use arrow::datatypes::DataType;
use datafusion::common::{exec_err, Result};
use datafusion::logical_expr::simplify::{ExprSimplifyResult, SimplifyInfo};
use datafusion::logical_expr::{ColumnarValue, Expr, ScalarUDFImpl, Signature, Volatility};
use std::any::Any;


fn greatest_3_invoke(args: &[ColumnarValue]) -> Result<ColumnarValue> {
    let args = ColumnarValue::values_to_arrays(args)?;
    let result = args.into_iter().fold(Ok(None), |acc, cur| {
        let acc = acc?;
        if let Some(acc) = acc {
            let comparison = lt(&acc, &cur)?;
            zip(&comparison, &cur, &acc).map(Some)
        } else {
            Ok(Some(cur))
        }
    })?;
    if let Some(result) = result {
        Ok(ColumnarValue::Array(result))
    } else {
        exec_err!("greatest expects at least one argument")
    }
}

fn greatest_3_return_type(arg_types: &[DataType]) -> Result<DataType> {
    Ok(arg_types[0].clone())
}

fn greatest_3_simplify(args: Vec<Expr>, _info: &dyn SimplifyInfo) -> Result<ExprSimplifyResult> {
    Ok(ExprSimplifyResult::Original(args))
}


/// ========== Above are user code/Below are generated code ==========


#[derive(Debug)]
pub(super) struct greatest_3Func {
    signature: Signature,
}

impl greatest_3Func {
    pub fn new() -> Self {        
        Self {
            signature: Signature::variadic_equal(Volatility::Immutable),
        }
    }
}

impl ScalarUDFImpl for greatest_3Func {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "greatest"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }


    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        greatest_3_return_type(arg_types)
    }

    fn invoke(&self, args: &[ColumnarValue]) -> Result<ColumnarValue> {
        greatest_3_invoke(args)
    }

    fn simplify(
        &self,
        args: Vec<Expr>,
        info: &dyn SimplifyInfo,
    ) -> Result<ExprSimplifyResult> {
        greatest_3_simplify(args, info)
    }

}

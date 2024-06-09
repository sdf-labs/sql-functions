#![allow(non_camel_case_types)]
use arrow::array::{ArrayRef, GenericStringArray};
use arrow::datatypes::DataType;
use datafusion::common::cast::as_binary_array;
use datafusion::common::Result;
use datafusion::logical_expr::simplify::{ExprSimplifyResult, SimplifyInfo};
use datafusion::logical_expr::{ColumnarValue, Expr, ScalarUDFImpl, Signature, Volatility};
use std::any::Any;
use std::sync::Arc;

use crate::utils::make_scalar_function;


fn to_hex_varbinary_invoke(args: &[ColumnarValue]) -> Result<ColumnarValue> {
    make_scalar_function(to_hex, vec![])(args)
}

fn to_hex(args: &[ArrayRef]) -> Result<ArrayRef> {
    let binary_array = as_binary_array(&args[0])?;
    let result = binary_array
        .iter()
        .map(|binary| {
            let hex = binary.map(|binary| {
                let hex = binary
                    .iter()
                    .map(|byte| format!("{:02x}", byte))
                    .collect::<String>();
                hex
            });
            Ok(hex)
        })
        .collect::<Result<GenericStringArray<i32>>>()?;
    Ok(Arc::new(result) as ArrayRef)
}

fn to_hex_varbinary_return_type(_arg_types: &[DataType]) -> Result<DataType> {
    Ok(DataType::Utf8)
}

fn to_hex_varbinary_simplify(args: Vec<Expr>, _info: &dyn SimplifyInfo) -> Result<ExprSimplifyResult> {
    Ok(ExprSimplifyResult::Original(args))
}


/// ========== Above are user code/Below are generated code ==========


#[derive(Debug)]
pub(super) struct to_hex_varbinaryFunc {
    signature: Signature,
}

impl to_hex_varbinaryFunc {
    pub fn new() -> Self {        
        Self {
            signature: Signature::any(1, Volatility::Immutable),
        }
    }
}

impl ScalarUDFImpl for to_hex_varbinaryFunc {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "to_hex"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }


    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        to_hex_varbinary_return_type(arg_types)
    }

    fn invoke(&self, args: &[ColumnarValue]) -> Result<ColumnarValue> {
        to_hex_varbinary_invoke(args)
    }

    fn simplify(
        &self,
        args: Vec<Expr>,
        info: &dyn SimplifyInfo,
    ) -> Result<ExprSimplifyResult> {
        to_hex_varbinary_simplify(args, info)
    }

}

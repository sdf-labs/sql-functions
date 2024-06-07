// start implementing uses
use arrow::array::{ArrayRef, GenericStringArray};
use arrow::datatypes::DataType;
use datafusion::common::cast::as_binary_array;
use datafusion::common::Result;
use datafusion::logical_expr::simplify::{ExprSimplifyResult, SimplifyInfo};
use datafusion::logical_expr::{ColumnarValue, Expr, ScalarUDFImpl, Signature, Volatility};
use std::any::Any;
use std::sync::Arc;

use crate::utils::make_scalar_function;

// end implementing uses

#[derive(Debug)]
pub(super) struct Func {
    signature: Signature,
}

impl Func {
    pub fn new() -> Self {        
        // start implementing constructor
        Self {
            signature: Signature::exact(vec![DataType::Binary], Volatility::Immutable),
        }
        // end implementing constructor
    }
}

impl ScalarUDFImpl for Func {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "to_hex"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }

    // start implementing return_type
    fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType> {
        Ok(DataType::Utf8)
    }
    // end implementing return_type

    // start implementing invoke
    fn invoke(&self, args: &[ColumnarValue]) -> Result<ColumnarValue> {
        make_scalar_function(to_hex, vec![])(args)
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
 
// end implementing footer

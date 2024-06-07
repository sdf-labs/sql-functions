// start implementing uses
use arrow::datatypes::DataType;
use datafusion::common::Result;
use datafusion::logical_expr::simplify::{ExprSimplifyResult, SimplifyInfo};
use datafusion::logical_expr::{ColumnarValue, Expr, ScalarUDFImpl, Signature, Volatility};
use std::any::Any;
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
            Signature::any(1, Volatility::Immutable),
        }

        // end implementing constructor
    }
}

impl ScalarUDFImpl for Func {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "st_dimension"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }

    // start implementing return_type
    fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType> {
        todo!()
    }
    // end implementing return_type

    // start implementing invoke
    fn invoke(&self, _args: &[ColumnarValue]) -> Result<ColumnarValue> {
        todo!()
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
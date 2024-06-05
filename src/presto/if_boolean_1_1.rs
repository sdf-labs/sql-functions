// start implementing uses
use arrow::compute::kernels::zip::zip;
use arrow::datatypes::DataType;
use datafusion::common::cast::as_boolean_array;
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
        Self {
            signature: Signature::any(3, Volatility::Immutable),
        }
        // end implementing constructor
    }
}

impl ScalarUDFImpl for Func {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "if"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }

    // start implementing return_type
    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        if arg_types.len() != 3 {
            return Err(datafusion::error::DataFusionError::Internal(
                "if expects 3 arguments".to_string(),
            ));
        }
        if arg_types[0] != DataType::Boolean {
            return Err(datafusion::error::DataFusionError::Internal(
                "if expects the first argument to be a boolean".to_string(),
            ));
        }
        Ok(arg_types[1].clone())
    }
    // end implementing return_type

    // start implementing invoke
    fn invoke(&self, args: &[ColumnarValue]) -> Result<ColumnarValue> {
        let args = ColumnarValue::values_to_arrays(args)?;
        let conditions = as_boolean_array(&args[0])?;
        let left = args[1].to_owned();
        let right = args[2].to_owned();
        let result = zip(&conditions, &left, &right)?;
        Ok(ColumnarValue::Array(Arc::new(result)))
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

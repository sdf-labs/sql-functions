// start implementing uses
use arrow::compute::kernels::cmp::lt;
use arrow::compute::kernels::zip::zip;
use arrow::datatypes::DataType;
use datafusion::common::{exec_err, Result};
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
        Self {
            signature: Signature::variadic_equal(Volatility::Immutable),
        }
        // end implementing constructor
    }
}

impl ScalarUDFImpl for Func {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "least"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }

    // start implementing return_type
    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        Ok(arg_types[0].clone())
    }
    // end implementing return_type

    // start implementing invoke
    fn invoke(&self, args: &[ColumnarValue]) -> Result<ColumnarValue> {
        let args = ColumnarValue::values_to_arrays(args)?;
        let result = args.into_iter().fold(Ok(None), |acc, cur| {
            let acc = acc?;
            if let Some(acc) = acc {
                let comparison = lt(&acc, &cur)?;
                zip(&comparison, &acc, &cur).map(Some)
            } else {
                Ok(Some(cur))
            }
        })?;
        if let Some(result) = result {
            Ok(ColumnarValue::Array(result))
        } else {
            exec_err!("least expects at least one argument")
        }
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

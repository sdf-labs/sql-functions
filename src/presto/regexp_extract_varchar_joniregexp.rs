// start implementing uses
use arrow::array::{as_fixed_size_list_array, StringArray};
use arrow::datatypes::DataType;
use datafusion::common::cast::{as_list_array, as_string_array};
use datafusion::common::Result;
use datafusion::functions::regex::regexpmatch::regexp_match;
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
            signature: Signature::any(2, Volatility::Immutable),
        }
        // end implementing constructor
    }
}

impl ScalarUDFImpl for Func {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "regexp_extract"
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
        let args = ColumnarValue::values_to_arrays(args)?;
        let value = args[0].to_owned();
        let pattern = args[1].to_owned();
        let pattern = as_fixed_size_list_array(&pattern);
        let pattern = pattern
            .iter()
            .map(|x| {
                x.map(|x| {
                    let inner = as_string_array(&x)?;
                    Ok(inner.value(0).to_owned())
                })
                .transpose()
            })
            .collect::<Result<StringArray>>()?;

        let arrays = vec![value, Arc::new(pattern)];
        let result = regexp_match::<i32>(&arrays)?;
        let result = as_list_array(result.as_ref())?;
        let result = result
            .iter()
            .map(|x| {
                x.map(|x| {
                    let inner = as_string_array(&x)?;
                    Ok(inner.value(0).to_owned())
                })
                .transpose()
            })
            .collect::<Result<StringArray>>()?;

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

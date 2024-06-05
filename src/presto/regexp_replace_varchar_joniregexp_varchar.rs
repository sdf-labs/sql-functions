// start implementing uses
use arrow::array::{as_fixed_size_list_array, StringArray};
use arrow::datatypes::DataType;
use datafusion::common::cast::as_string_array;
use datafusion::common::Result;
use datafusion::functions::regex::regexpreplace::specialize_regexp_replace;
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
        "regexp_replace"
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
        // let pattern = cast(&pattern, &DataType::Utf8)?;
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
        let replacement = args[2].to_owned();

        let columnar_values = vec![
            ColumnarValue::Array(value),
            ColumnarValue::Array(Arc::new(pattern)),
            ColumnarValue::Array(replacement),
        ];
        let array = specialize_regexp_replace::<i32>(&columnar_values)?;
        Ok(ColumnarValue::Array(array))
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

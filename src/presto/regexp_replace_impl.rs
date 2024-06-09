#![allow(non_camel_case_types)]
use arrow::array::{as_fixed_size_list_array, ArrayRef, StringArray};
use arrow::datatypes::DataType;
use datafusion::common::cast::as_string_array;
use datafusion::common::Result;
use datafusion::error::DataFusionError;
use datafusion::functions::regex::regexpreplace::specialize_regexp_replace;
use datafusion::logical_expr::simplify::{ExprSimplifyResult, SimplifyInfo};
use datafusion::logical_expr::{ColumnarValue, Expr, ScalarUDFImpl, Signature, Volatility};
use std::any::Any;
use std::sync::Arc;


fn regexp_replace_varchar_joniregexp_function_array_varchar_varchar_invoke(_args: &[ColumnarValue]) -> Result<ColumnarValue> {
    Err(DataFusionError::NotImplemented("todo".to_string()))
}

fn regexp_replace_varchar_joniregexp_function_array_varchar_varchar_return_type(_arg_types: &[DataType]) -> Result<DataType> {
    Err(DataFusionError::NotImplemented("todo".to_string()))
}

fn regexp_replace_varchar_joniregexp_function_array_varchar_varchar_simplify(args: Vec<Expr>, _info: &dyn SimplifyInfo) -> Result<ExprSimplifyResult> {
    Ok(ExprSimplifyResult::Original(args))
}

fn regexp_replace_varchar_joniregexp_invoke(args: &[ColumnarValue]) -> Result<ColumnarValue> {
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
        let replacement = {
            let array = StringArray::from(vec![""; value.len()]);
            Arc::new(array) as ArrayRef
        };

        let columnar_values = vec![
            ColumnarValue::Array(value),
            ColumnarValue::Array(Arc::new(pattern)),
            ColumnarValue::Array(replacement),
        ];
        let array = specialize_regexp_replace::<i32>(&columnar_values)?;
        Ok(ColumnarValue::Array(array))
}

fn regexp_replace_varchar_joniregexp_return_type(_arg_types: &[DataType]) -> Result<DataType> {
    Ok(DataType::Utf8)
}

fn regexp_replace_varchar_joniregexp_simplify(args: Vec<Expr>, _info: &dyn SimplifyInfo) -> Result<ExprSimplifyResult> {
    Ok(ExprSimplifyResult::Original(args))
}

fn regexp_replace_varchar_joniregexp_varchar_invoke(args: &[ColumnarValue]) -> Result<ColumnarValue> {
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

fn regexp_replace_varchar_joniregexp_varchar_return_type(_arg_types: &[DataType]) -> Result<DataType> {
    Ok(DataType::Utf8)
}

fn regexp_replace_varchar_joniregexp_varchar_simplify(args: Vec<Expr>, _info: &dyn SimplifyInfo) -> Result<ExprSimplifyResult> {
    Ok(ExprSimplifyResult::Original(args))
}


/// ========== Above are user code/Below are generated code ==========


#[derive(Debug)]
pub(super) struct regexp_replace_varchar_joniregexp_function_array_varchar_varcharFunc {
    signature: Signature,
}

impl regexp_replace_varchar_joniregexp_function_array_varchar_varcharFunc {
    pub fn new() -> Self {        
        Self {
            signature: Signature::any(3, Volatility::Immutable),
        }
    }
}

impl ScalarUDFImpl for regexp_replace_varchar_joniregexp_function_array_varchar_varcharFunc {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "regexp_replace"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }


    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        regexp_replace_varchar_joniregexp_function_array_varchar_varchar_return_type(arg_types)
    }

    fn invoke(&self, args: &[ColumnarValue]) -> Result<ColumnarValue> {
        regexp_replace_varchar_joniregexp_function_array_varchar_varchar_invoke(args)
    }

    fn simplify(
        &self,
        args: Vec<Expr>,
        info: &dyn SimplifyInfo,
    ) -> Result<ExprSimplifyResult> {
        regexp_replace_varchar_joniregexp_function_array_varchar_varchar_simplify(args, info)
    }

}

#[derive(Debug)]
pub(super) struct regexp_replace_varchar_joniregexpFunc {
    signature: Signature,
}

impl regexp_replace_varchar_joniregexpFunc {
    pub fn new() -> Self {        
        Self {
            signature: Signature::any(2, Volatility::Immutable),
        }
    }
}

impl ScalarUDFImpl for regexp_replace_varchar_joniregexpFunc {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "regexp_replace"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }


    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        regexp_replace_varchar_joniregexp_return_type(arg_types)
    }

    fn invoke(&self, args: &[ColumnarValue]) -> Result<ColumnarValue> {
        regexp_replace_varchar_joniregexp_invoke(args)
    }

    fn simplify(
        &self,
        args: Vec<Expr>,
        info: &dyn SimplifyInfo,
    ) -> Result<ExprSimplifyResult> {
        regexp_replace_varchar_joniregexp_simplify(args, info)
    }

}

#[derive(Debug)]
pub(super) struct regexp_replace_varchar_joniregexp_varcharFunc {
    signature: Signature,
}

impl regexp_replace_varchar_joniregexp_varcharFunc {
    pub fn new() -> Self {        
        Self {
            signature: Signature::any(3, Volatility::Immutable),
        }
    }
}

impl ScalarUDFImpl for regexp_replace_varchar_joniregexp_varcharFunc {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "regexp_replace"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }


    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        regexp_replace_varchar_joniregexp_varchar_return_type(arg_types)
    }

    fn invoke(&self, args: &[ColumnarValue]) -> Result<ColumnarValue> {
        regexp_replace_varchar_joniregexp_varchar_invoke(args)
    }

    fn simplify(
        &self,
        args: Vec<Expr>,
        info: &dyn SimplifyInfo,
    ) -> Result<ExprSimplifyResult> {
        regexp_replace_varchar_joniregexp_varchar_simplify(args, info)
    }

}

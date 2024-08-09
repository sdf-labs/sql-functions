// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

#![allow(non_camel_case_types)]
use arrow::datatypes::DataType;
use datafusion::common::Result;
use datafusion::error::DataFusionError;
use datafusion::logical_expr::simplify::{ExprSimplifyResult, SimplifyInfo};
use datafusion::logical_expr::{ColumnarValue, Expr, ScalarUDFImpl, Signature, Volatility};
use regex::Regex;
use std::any::Any;
use std::sync::Arc;

use crate::utils_regexp::{map_rowfun__pat_hay_str_to_string, map_rowfun__pat_hay_to_string};

fn regexp_replace_varchar_joniregexp_function_array_varchar_varchar_invoke(
    _args: &[ColumnarValue],
) -> Result<ColumnarValue> {
    Err(DataFusionError::NotImplemented("todo".to_string()))
}

fn regexp_replace_varchar_joniregexp_function_array_varchar_varchar_return_type(
    _arg_types: &[DataType],
) -> Result<DataType> {
    Err(DataFusionError::NotImplemented("todo".to_string()))
}

fn regexp_replace_varchar_joniregexp_function_array_varchar_varchar_simplify(
    args: Vec<Expr>,
    _info: &dyn SimplifyInfo,
) -> Result<ExprSimplifyResult> {
    Ok(ExprSimplifyResult::Original(args))
}

fn regexp_replace_varchar_joniregexp_invoke(args: &[ColumnarValue]) -> Result<ColumnarValue> {
    map_rowfun__pat_hay_to_string(&args[1], &args[0], Arc::new(regexp_replace__rowfun))
}

fn regexp_replace__rowfun(pat: &str) -> Result<Arc<dyn Fn(/*hay:*/ &str) -> String>> {
    let re = Regex::new(pat).map_err(|e| {
        DataFusionError::Execution(format!(
            "Regular expression for regexp_count did not compile: {e:?}"
        ))
    })?;
    let rowfun = move |str: &str| re.replace_all(str, "").into_owned();
    Ok(Arc::new(rowfun))
}

fn regexp_replace_varchar_joniregexp_return_type(_arg_types: &[DataType]) -> Result<DataType> {
    Ok(DataType::Utf8)
}

fn regexp_replace_varchar_joniregexp_simplify(
    args: Vec<Expr>,
    _info: &dyn SimplifyInfo,
) -> Result<ExprSimplifyResult> {
    Ok(ExprSimplifyResult::Original(args))
}

fn regexp_replace_varchar_joniregexp_varchar_invoke(
    args: &[ColumnarValue],
) -> Result<ColumnarValue> {
    map_rowfun__pat_hay_str_to_string(
        &args[1],
        &args[0],
        &args[2],
        Arc::new(regexp_replace__rowfun2),
    )
}

fn regexp_replace__rowfun2(
    pat: &str,
) -> Result<Arc<dyn Fn(/*hay:*/ &str, /*str1:*/ &str) -> String>> {
    let re = Regex::new(pat).map_err(|e| {
        DataFusionError::Execution(format!(
            "Regular expression for regexp_count did not compile: {e:?}"
        ))
    })?;
    let rowfun = move |str: &str, str1: &str| re.replace_all(str, str1).into_owned();
    Ok(Arc::new(rowfun))
}

fn regexp_replace_varchar_joniregexp_varchar_return_type(
    _arg_types: &[DataType],
) -> Result<DataType> {
    Ok(DataType::Utf8)
}

fn regexp_replace_varchar_joniregexp_varchar_simplify(
    args: Vec<Expr>,
    _info: &dyn SimplifyInfo,
) -> Result<ExprSimplifyResult> {
    Ok(ExprSimplifyResult::Original(args))
}

// ========== Generated template below this line ==========
// Do *NOT* edit below this line: all changes will be overwritten
// when template is regenerated!


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

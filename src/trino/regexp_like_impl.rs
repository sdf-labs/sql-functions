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
use arrow::array::{as_fixed_size_list_array, StringArray};
use arrow::datatypes::DataType;
use datafusion::common::cast::as_string_array;
use datafusion::common::Result;
use datafusion::functions::regex::regexplike::regexp_like;
use datafusion::logical_expr::simplify::{ExprSimplifyResult, SimplifyInfo};
use datafusion::logical_expr::{ColumnarValue, Expr, ScalarUDFImpl, Signature, Volatility};
use std::any::Any;
use std::sync::Arc;

fn regexp_like_varchar_joniregexp_invoke(args: &[ColumnarValue]) -> Result<ColumnarValue> {
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

    let args = vec![value, Arc::new(pattern)];
    let array = regexp_like::<i32>(&args)?;
    Ok(ColumnarValue::Array(array))
}

fn regexp_like_varchar_joniregexp_return_type(_arg_types: &[DataType]) -> Result<DataType> {
    Ok(DataType::Boolean)
}

fn regexp_like_varchar_joniregexp_simplify(
    args: Vec<Expr>,
    _info: &dyn SimplifyInfo,
) -> Result<ExprSimplifyResult> {
    Ok(ExprSimplifyResult::Original(args))
}

// ========== Generated template below this line ==========
// Do *NOT* edit below this line: all changes will be overwritten
// when template is regenerated!

#[derive(Debug)]
pub(super) struct regexp_like_varchar_joniregexpFunc {
    signature: Signature,
}

impl regexp_like_varchar_joniregexpFunc {
    pub fn new() -> Self {
        Self {
            signature: Signature::any(2, Volatility::Immutable),
        }
    }
}

impl ScalarUDFImpl for regexp_like_varchar_joniregexpFunc {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "regexp_like"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }

    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        regexp_like_varchar_joniregexp_return_type(arg_types)
    }

    fn invoke(&self, args: &[ColumnarValue]) -> Result<ColumnarValue> {
        regexp_like_varchar_joniregexp_invoke(args)
    }

    fn simplify(&self, args: Vec<Expr>, info: &dyn SimplifyInfo) -> Result<ExprSimplifyResult> {
        regexp_like_varchar_joniregexp_simplify(args, info)
    }
}

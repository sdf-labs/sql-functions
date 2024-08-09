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
use arrow::compute::kernels::cmp::lt;
use arrow::compute::kernels::zip::zip;
use arrow::datatypes::DataType;
use datafusion::common::{exec_err, Result};
use datafusion::logical_expr::simplify::{ExprSimplifyResult, SimplifyInfo};
use datafusion::logical_expr::{ColumnarValue, Expr, ScalarUDFImpl, Signature, Volatility};
use std::any::Any;

fn greatest_3_invoke(args: &[ColumnarValue]) -> Result<ColumnarValue> {
    let args = ColumnarValue::values_to_arrays(args)?;
    let result = args.into_iter().fold(Ok(None), |acc, cur| {
        let acc = acc?;
        if let Some(acc) = acc {
            let comparison = lt(&acc, &cur)?;
            zip(&comparison, &cur, &acc).map(Some)
        } else {
            Ok(Some(cur))
        }
    })?;
    if let Some(result) = result {
        Ok(ColumnarValue::Array(result))
    } else {
        exec_err!("greatest expects at least one argument")
    }
}

fn greatest_3_return_type(arg_types: &[DataType]) -> Result<DataType> {
    Ok(arg_types[0].clone())
}

fn greatest_3_simplify(args: Vec<Expr>, _info: &dyn SimplifyInfo) -> Result<ExprSimplifyResult> {
    Ok(ExprSimplifyResult::Original(args))
}

// ========== Generated template below this line ==========
// Do *NOT* edit below this line: all changes will be overwritten
// when template is regenerated!

#[derive(Debug)]
pub(super) struct greatest_3Func {
    signature: Signature,
}

impl greatest_3Func {
    pub fn new() -> Self {
        Self {
            signature: Signature::variadic_equal(Volatility::Immutable),
        }
    }
}

impl ScalarUDFImpl for greatest_3Func {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "greatest"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }

    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        greatest_3_return_type(arg_types)
    }

    fn invoke(&self, args: &[ColumnarValue]) -> Result<ColumnarValue> {
        greatest_3_invoke(args)
    }

    fn simplify(&self, args: Vec<Expr>, info: &dyn SimplifyInfo) -> Result<ExprSimplifyResult> {
        greatest_3_simplify(args, info)
    }
}

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
use std::any::Any;

fn ceil_bigint_invoke(args: &[ColumnarValue]) -> Result<ColumnarValue> {
    Ok(args[0].clone())
}

fn ceil_bigint_return_type(_arg_types: &[DataType]) -> Result<DataType> {
    Ok(DataType::Int64)
}

fn ceil_bigint_simplify(args: Vec<Expr>, _info: &dyn SimplifyInfo) -> Result<ExprSimplifyResult> {
    Ok(ExprSimplifyResult::Original(args))
}

fn ceil_decimal_p_s_invoke(_args: &[ColumnarValue]) -> Result<ColumnarValue> {
    Err(DataFusionError::NotImplemented(format!(
        "Not implemented {}:{}",
        file!(),
        line!()
    )))
}

fn ceil_decimal_p_s_return_type(_arg_types: &[DataType]) -> Result<DataType> {
    Err(DataFusionError::NotImplemented(format!(
        "Not implemented {}:{}",
        file!(),
        line!()
    )))
}

fn ceil_decimal_p_s_simplify(
    args: Vec<Expr>,
    _info: &dyn SimplifyInfo,
) -> Result<ExprSimplifyResult> {
    Ok(ExprSimplifyResult::Original(args))
}

fn ceil_double_invoke(_args: &[ColumnarValue]) -> Result<ColumnarValue> {
    Err(DataFusionError::NotImplemented(format!(
        "Not implemented {}:{}",
        file!(),
        line!()
    )))
}

fn ceil_double_return_type(_arg_types: &[DataType]) -> Result<DataType> {
    Err(DataFusionError::NotImplemented(format!(
        "Not implemented {}:{}",
        file!(),
        line!()
    )))
}

fn ceil_double_simplify(args: Vec<Expr>, _info: &dyn SimplifyInfo) -> Result<ExprSimplifyResult> {
    Ok(ExprSimplifyResult::Original(args))
}

fn ceil_integer_invoke(_args: &[ColumnarValue]) -> Result<ColumnarValue> {
    Err(DataFusionError::NotImplemented(format!(
        "Not implemented {}:{}",
        file!(),
        line!()
    )))
}

fn ceil_integer_return_type(_arg_types: &[DataType]) -> Result<DataType> {
    Err(DataFusionError::NotImplemented(format!(
        "Not implemented {}:{}",
        file!(),
        line!()
    )))
}

fn ceil_integer_simplify(args: Vec<Expr>, _info: &dyn SimplifyInfo) -> Result<ExprSimplifyResult> {
    Ok(ExprSimplifyResult::Original(args))
}

fn ceil_real_invoke(_args: &[ColumnarValue]) -> Result<ColumnarValue> {
    Err(DataFusionError::NotImplemented(format!(
        "Not implemented {}:{}",
        file!(),
        line!()
    )))
}

fn ceil_real_return_type(_arg_types: &[DataType]) -> Result<DataType> {
    Err(DataFusionError::NotImplemented(format!(
        "Not implemented {}:{}",
        file!(),
        line!()
    )))
}

fn ceil_real_simplify(args: Vec<Expr>, _info: &dyn SimplifyInfo) -> Result<ExprSimplifyResult> {
    Ok(ExprSimplifyResult::Original(args))
}

fn ceil_smallint_invoke(_args: &[ColumnarValue]) -> Result<ColumnarValue> {
    Err(DataFusionError::NotImplemented(format!(
        "Not implemented {}:{}",
        file!(),
        line!()
    )))
}

fn ceil_smallint_return_type(_arg_types: &[DataType]) -> Result<DataType> {
    Err(DataFusionError::NotImplemented(format!(
        "Not implemented {}:{}",
        file!(),
        line!()
    )))
}

fn ceil_smallint_simplify(args: Vec<Expr>, _info: &dyn SimplifyInfo) -> Result<ExprSimplifyResult> {
    Ok(ExprSimplifyResult::Original(args))
}

fn ceil_tinyint_invoke(_args: &[ColumnarValue]) -> Result<ColumnarValue> {
    Err(DataFusionError::NotImplemented(format!(
        "Not implemented {}:{}",
        file!(),
        line!()
    )))
}

fn ceil_tinyint_return_type(_arg_types: &[DataType]) -> Result<DataType> {
    Err(DataFusionError::NotImplemented(format!(
        "Not implemented {}:{}",
        file!(),
        line!()
    )))
}

fn ceil_tinyint_simplify(args: Vec<Expr>, _info: &dyn SimplifyInfo) -> Result<ExprSimplifyResult> {
    Ok(ExprSimplifyResult::Original(args))
}

// ========== Generated template below this line ==========
// Do *NOT* edit below this line: all changes will be overwritten
// when template is regenerated!

#[derive(Debug)]
pub(super) struct ceil_bigintFunc {
    signature: Signature,
}

impl ceil_bigintFunc {
    pub fn new() -> Self {
        Self {
            signature: Signature::any(1, Volatility::Immutable),
        }
    }
}

impl ScalarUDFImpl for ceil_bigintFunc {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "ceil"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }

    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        ceil_bigint_return_type(arg_types)
    }

    fn invoke(&self, args: &[ColumnarValue]) -> Result<ColumnarValue> {
        ceil_bigint_invoke(args)
    }

    fn simplify(&self, args: Vec<Expr>, info: &dyn SimplifyInfo) -> Result<ExprSimplifyResult> {
        ceil_bigint_simplify(args, info)
    }
}

#[derive(Debug)]
pub(super) struct ceil_decimal_p_sFunc {
    signature: Signature,
}

impl ceil_decimal_p_sFunc {
    pub fn new() -> Self {
        Self {
            signature: Signature::any(1, Volatility::Immutable),
        }
    }
}

impl ScalarUDFImpl for ceil_decimal_p_sFunc {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "ceil"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }

    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        ceil_decimal_p_s_return_type(arg_types)
    }

    fn invoke(&self, args: &[ColumnarValue]) -> Result<ColumnarValue> {
        ceil_decimal_p_s_invoke(args)
    }

    fn simplify(&self, args: Vec<Expr>, info: &dyn SimplifyInfo) -> Result<ExprSimplifyResult> {
        ceil_decimal_p_s_simplify(args, info)
    }
}

#[derive(Debug)]
pub(super) struct ceil_doubleFunc {
    signature: Signature,
}

impl ceil_doubleFunc {
    pub fn new() -> Self {
        Self {
            signature: Signature::any(1, Volatility::Immutable),
        }
    }
}

impl ScalarUDFImpl for ceil_doubleFunc {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "ceil"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }

    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        ceil_double_return_type(arg_types)
    }

    fn invoke(&self, args: &[ColumnarValue]) -> Result<ColumnarValue> {
        ceil_double_invoke(args)
    }

    fn simplify(&self, args: Vec<Expr>, info: &dyn SimplifyInfo) -> Result<ExprSimplifyResult> {
        ceil_double_simplify(args, info)
    }
}

#[derive(Debug)]
pub(super) struct ceil_integerFunc {
    signature: Signature,
}

impl ceil_integerFunc {
    pub fn new() -> Self {
        Self {
            signature: Signature::any(1, Volatility::Immutable),
        }
    }
}

impl ScalarUDFImpl for ceil_integerFunc {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "ceil"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }

    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        ceil_integer_return_type(arg_types)
    }

    fn invoke(&self, args: &[ColumnarValue]) -> Result<ColumnarValue> {
        ceil_integer_invoke(args)
    }

    fn simplify(&self, args: Vec<Expr>, info: &dyn SimplifyInfo) -> Result<ExprSimplifyResult> {
        ceil_integer_simplify(args, info)
    }
}

#[derive(Debug)]
pub(super) struct ceil_realFunc {
    signature: Signature,
}

impl ceil_realFunc {
    pub fn new() -> Self {
        Self {
            signature: Signature::any(1, Volatility::Immutable),
        }
    }
}

impl ScalarUDFImpl for ceil_realFunc {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "ceil"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }

    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        ceil_real_return_type(arg_types)
    }

    fn invoke(&self, args: &[ColumnarValue]) -> Result<ColumnarValue> {
        ceil_real_invoke(args)
    }

    fn simplify(&self, args: Vec<Expr>, info: &dyn SimplifyInfo) -> Result<ExprSimplifyResult> {
        ceil_real_simplify(args, info)
    }
}

#[derive(Debug)]
pub(super) struct ceil_smallintFunc {
    signature: Signature,
}

impl ceil_smallintFunc {
    pub fn new() -> Self {
        Self {
            signature: Signature::any(1, Volatility::Immutable),
        }
    }
}

impl ScalarUDFImpl for ceil_smallintFunc {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "ceil"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }

    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        ceil_smallint_return_type(arg_types)
    }

    fn invoke(&self, args: &[ColumnarValue]) -> Result<ColumnarValue> {
        ceil_smallint_invoke(args)
    }

    fn simplify(&self, args: Vec<Expr>, info: &dyn SimplifyInfo) -> Result<ExprSimplifyResult> {
        ceil_smallint_simplify(args, info)
    }
}

#[derive(Debug)]
pub(super) struct ceil_tinyintFunc {
    signature: Signature,
}

impl ceil_tinyintFunc {
    pub fn new() -> Self {
        Self {
            signature: Signature::any(1, Volatility::Immutable),
        }
    }
}

impl ScalarUDFImpl for ceil_tinyintFunc {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "ceil"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }

    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        ceil_tinyint_return_type(arg_types)
    }

    fn invoke(&self, args: &[ColumnarValue]) -> Result<ColumnarValue> {
        ceil_tinyint_invoke(args)
    }

    fn simplify(&self, args: Vec<Expr>, info: &dyn SimplifyInfo) -> Result<ExprSimplifyResult> {
        ceil_tinyint_simplify(args, info)
    }
}

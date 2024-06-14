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

fn format_varchar_1_invoke(_args: &[ColumnarValue]) -> Result<ColumnarValue> {
    Err(DataFusionError::NotImplemented(format!(
        "Not implemented {}:{}",
        file!(),
        line!()
    )))
}

fn format_varchar_1_return_type(_arg_types: &[DataType]) -> Result<DataType> {
    Err(DataFusionError::NotImplemented(format!(
        "Not implemented {}:{}",
        file!(),
        line!()
    )))
}

fn format_varchar_1_simplify(
    args: Vec<Expr>,
    _info: &dyn SimplifyInfo,
) -> Result<ExprSimplifyResult> {
    Ok(ExprSimplifyResult::Original(args))
}

fn format_varchar_1_2_invoke(_args: &[ColumnarValue]) -> Result<ColumnarValue> {
    Err(DataFusionError::NotImplemented(format!(
        "Not implemented {}:{}",
        file!(),
        line!()
    )))
}

fn format_varchar_1_2_return_type(_arg_types: &[DataType]) -> Result<DataType> {
    Err(DataFusionError::NotImplemented(format!(
        "Not implemented {}:{}",
        file!(),
        line!()
    )))
}

fn format_varchar_1_2_simplify(
    args: Vec<Expr>,
    _info: &dyn SimplifyInfo,
) -> Result<ExprSimplifyResult> {
    Ok(ExprSimplifyResult::Original(args))
}

fn format_varchar_1_2_3_invoke(_args: &[ColumnarValue]) -> Result<ColumnarValue> {
    Err(DataFusionError::NotImplemented(format!(
        "Not implemented {}:{}",
        file!(),
        line!()
    )))
}

fn format_varchar_1_2_3_return_type(_arg_types: &[DataType]) -> Result<DataType> {
    Err(DataFusionError::NotImplemented(format!(
        "Not implemented {}:{}",
        file!(),
        line!()
    )))
}

fn format_varchar_1_2_3_simplify(
    args: Vec<Expr>,
    _info: &dyn SimplifyInfo,
) -> Result<ExprSimplifyResult> {
    Ok(ExprSimplifyResult::Original(args))
}

fn format_varchar_1_2_3_4_invoke(_args: &[ColumnarValue]) -> Result<ColumnarValue> {
    Err(DataFusionError::NotImplemented(format!(
        "Not implemented {}:{}",
        file!(),
        line!()
    )))
}

fn format_varchar_1_2_3_4_return_type(_arg_types: &[DataType]) -> Result<DataType> {
    Err(DataFusionError::NotImplemented(format!(
        "Not implemented {}:{}",
        file!(),
        line!()
    )))
}

fn format_varchar_1_2_3_4_simplify(
    args: Vec<Expr>,
    _info: &dyn SimplifyInfo,
) -> Result<ExprSimplifyResult> {
    Ok(ExprSimplifyResult::Original(args))
}

fn format_varchar_1_2_3_4_5_invoke(_args: &[ColumnarValue]) -> Result<ColumnarValue> {
    Err(DataFusionError::NotImplemented(format!(
        "Not implemented {}:{}",
        file!(),
        line!()
    )))
}

fn format_varchar_1_2_3_4_5_return_type(_arg_types: &[DataType]) -> Result<DataType> {
    Err(DataFusionError::NotImplemented(format!(
        "Not implemented {}:{}",
        file!(),
        line!()
    )))
}

fn format_varchar_1_2_3_4_5_simplify(
    args: Vec<Expr>,
    _info: &dyn SimplifyInfo,
) -> Result<ExprSimplifyResult> {
    Ok(ExprSimplifyResult::Original(args))
}

// ========== Generated template below this line ==========
// Do *NOT* edit below this line: all changes will be overwritten
// when template is regenerated!

#[derive(Debug)]
pub(super) struct format_varchar_1Func {
    signature: Signature,
}

impl format_varchar_1Func {
    pub fn new() -> Self {
        Self {
            signature: Signature::any(2, Volatility::Immutable),
        }
    }
}

impl ScalarUDFImpl for format_varchar_1Func {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "format"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }

    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        format_varchar_1_return_type(arg_types)
    }

    fn invoke(&self, args: &[ColumnarValue]) -> Result<ColumnarValue> {
        format_varchar_1_invoke(args)
    }

    fn simplify(&self, args: Vec<Expr>, info: &dyn SimplifyInfo) -> Result<ExprSimplifyResult> {
        format_varchar_1_simplify(args, info)
    }
}

#[derive(Debug)]
pub(super) struct format_varchar_1_2Func {
    signature: Signature,
}

impl format_varchar_1_2Func {
    pub fn new() -> Self {
        Self {
            signature: Signature::any(3, Volatility::Immutable),
        }
    }
}

impl ScalarUDFImpl for format_varchar_1_2Func {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "format"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }

    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        format_varchar_1_2_return_type(arg_types)
    }

    fn invoke(&self, args: &[ColumnarValue]) -> Result<ColumnarValue> {
        format_varchar_1_2_invoke(args)
    }

    fn simplify(&self, args: Vec<Expr>, info: &dyn SimplifyInfo) -> Result<ExprSimplifyResult> {
        format_varchar_1_2_simplify(args, info)
    }
}

#[derive(Debug)]
pub(super) struct format_varchar_1_2_3Func {
    signature: Signature,
}

impl format_varchar_1_2_3Func {
    pub fn new() -> Self {
        Self {
            signature: Signature::any(4, Volatility::Immutable),
        }
    }
}

impl ScalarUDFImpl for format_varchar_1_2_3Func {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "format"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }

    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        format_varchar_1_2_3_return_type(arg_types)
    }

    fn invoke(&self, args: &[ColumnarValue]) -> Result<ColumnarValue> {
        format_varchar_1_2_3_invoke(args)
    }

    fn simplify(&self, args: Vec<Expr>, info: &dyn SimplifyInfo) -> Result<ExprSimplifyResult> {
        format_varchar_1_2_3_simplify(args, info)
    }
}

#[derive(Debug)]
pub(super) struct format_varchar_1_2_3_4Func {
    signature: Signature,
}

impl format_varchar_1_2_3_4Func {
    pub fn new() -> Self {
        Self {
            signature: Signature::any(5, Volatility::Immutable),
        }
    }
}

impl ScalarUDFImpl for format_varchar_1_2_3_4Func {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "format"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }

    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        format_varchar_1_2_3_4_return_type(arg_types)
    }

    fn invoke(&self, args: &[ColumnarValue]) -> Result<ColumnarValue> {
        format_varchar_1_2_3_4_invoke(args)
    }

    fn simplify(&self, args: Vec<Expr>, info: &dyn SimplifyInfo) -> Result<ExprSimplifyResult> {
        format_varchar_1_2_3_4_simplify(args, info)
    }
}

#[derive(Debug)]
pub(super) struct format_varchar_1_2_3_4_5Func {
    signature: Signature,
}

impl format_varchar_1_2_3_4_5Func {
    pub fn new() -> Self {
        Self {
            signature: Signature::any(6, Volatility::Immutable),
        }
    }
}

impl ScalarUDFImpl for format_varchar_1_2_3_4_5Func {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "format"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }

    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        format_varchar_1_2_3_4_5_return_type(arg_types)
    }

    fn invoke(&self, args: &[ColumnarValue]) -> Result<ColumnarValue> {
        format_varchar_1_2_3_4_5_invoke(args)
    }

    fn simplify(&self, args: Vec<Expr>, info: &dyn SimplifyInfo) -> Result<ExprSimplifyResult> {
        format_varchar_1_2_3_4_5_simplify(args, info)
    }
}

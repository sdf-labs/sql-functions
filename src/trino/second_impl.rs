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

fn second_intervaldaytosecond_invoke(_args: &[ColumnarValue]) -> Result<ColumnarValue> {
    Err(DataFusionError::NotImplemented(format!(
        "Not implemented {}:{}",
        file!(),
        line!()
    )))
}

fn second_intervaldaytosecond_return_type(_arg_types: &[DataType]) -> Result<DataType> {
    Err(DataFusionError::NotImplemented(format!(
        "Not implemented {}:{}",
        file!(),
        line!()
    )))
}

fn second_intervaldaytosecond_simplify(
    args: Vec<Expr>,
    _info: &dyn SimplifyInfo,
) -> Result<ExprSimplifyResult> {
    Ok(ExprSimplifyResult::Original(args))
}

fn second_time_p_invoke(_args: &[ColumnarValue]) -> Result<ColumnarValue> {
    Err(DataFusionError::NotImplemented(format!(
        "Not implemented {}:{}",
        file!(),
        line!()
    )))
}

fn second_time_p_return_type(_arg_types: &[DataType]) -> Result<DataType> {
    Err(DataFusionError::NotImplemented(format!(
        "Not implemented {}:{}",
        file!(),
        line!()
    )))
}

fn second_time_p_simplify(args: Vec<Expr>, _info: &dyn SimplifyInfo) -> Result<ExprSimplifyResult> {
    Ok(ExprSimplifyResult::Original(args))
}

fn second_timestamp_p_invoke(_args: &[ColumnarValue]) -> Result<ColumnarValue> {
    Err(DataFusionError::NotImplemented(format!(
        "Not implemented {}:{}",
        file!(),
        line!()
    )))
}

fn second_timestamp_p_return_type(_arg_types: &[DataType]) -> Result<DataType> {
    Err(DataFusionError::NotImplemented(format!(
        "Not implemented {}:{}",
        file!(),
        line!()
    )))
}

fn second_timestamp_p_simplify(
    args: Vec<Expr>,
    _info: &dyn SimplifyInfo,
) -> Result<ExprSimplifyResult> {
    Ok(ExprSimplifyResult::Original(args))
}

// ========== Generated template below this line ==========
// Do *NOT* edit below this line: all changes will be overwritten
// when template is regenerated!

#[derive(Debug)]
pub(super) struct second_intervaldaytosecondFunc {
    signature: Signature,
}

impl second_intervaldaytosecondFunc {
    pub fn new() -> Self {
        Self {
            signature: Signature::any(1, Volatility::Immutable),
        }
    }
}

impl ScalarUDFImpl for second_intervaldaytosecondFunc {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "second"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }

    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        second_intervaldaytosecond_return_type(arg_types)
    }

    fn invoke(&self, args: &[ColumnarValue]) -> Result<ColumnarValue> {
        second_intervaldaytosecond_invoke(args)
    }

    fn simplify(&self, args: Vec<Expr>, info: &dyn SimplifyInfo) -> Result<ExprSimplifyResult> {
        second_intervaldaytosecond_simplify(args, info)
    }
}

#[derive(Debug)]
pub(super) struct second_time_pFunc {
    signature: Signature,
}

impl second_time_pFunc {
    pub fn new() -> Self {
        Self {
            signature: Signature::any(1, Volatility::Immutable),
        }
    }
}

impl ScalarUDFImpl for second_time_pFunc {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "second"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }

    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        second_time_p_return_type(arg_types)
    }

    fn invoke(&self, args: &[ColumnarValue]) -> Result<ColumnarValue> {
        second_time_p_invoke(args)
    }

    fn simplify(&self, args: Vec<Expr>, info: &dyn SimplifyInfo) -> Result<ExprSimplifyResult> {
        second_time_p_simplify(args, info)
    }
}

#[derive(Debug)]
pub(super) struct second_timestamp_pFunc {
    signature: Signature,
}

impl second_timestamp_pFunc {
    pub fn new() -> Self {
        Self {
            signature: Signature::any(1, Volatility::Immutable),
        }
    }
}

impl ScalarUDFImpl for second_timestamp_pFunc {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "second"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }

    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        second_timestamp_p_return_type(arg_types)
    }

    fn invoke(&self, args: &[ColumnarValue]) -> Result<ColumnarValue> {
        second_timestamp_p_invoke(args)
    }

    fn simplify(&self, args: Vec<Expr>, info: &dyn SimplifyInfo) -> Result<ExprSimplifyResult> {
        second_timestamp_p_simplify(args, info)
    }
}

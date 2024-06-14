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
use arrow::array::{ArrayRef, ListArray, StringArray};
use arrow::buffer::{NullBuffer, OffsetBuffer};
use arrow::datatypes::{DataType, Field};
use datafusion::common::cast::{as_int64_array, as_string_array};
use datafusion::common::Result;
use datafusion::error::DataFusionError;
use datafusion::logical_expr::simplify::{ExprSimplifyResult, SimplifyInfo};
use datafusion::logical_expr::{ColumnarValue, Expr, ScalarUDFImpl, Signature, Volatility};
use std::any::Any;
use std::sync::Arc;

fn split_varchar_varchar_invoke(_args: &[ColumnarValue]) -> Result<ColumnarValue> {
    Err(DataFusionError::NotImplemented("todo".to_string()))
}

fn split_varchar_varchar_return_type(_arg_types: &[DataType]) -> Result<DataType> {
    Err(DataFusionError::NotImplemented("todo".to_string()))
}

fn split_varchar_varchar_simplify(
    args: Vec<Expr>,
    _info: &dyn SimplifyInfo,
) -> Result<ExprSimplifyResult> {
    Ok(ExprSimplifyResult::Original(args))
}

fn split_varchar_varchar_bigint_invoke(args: &[ColumnarValue]) -> Result<ColumnarValue> {
    let args = ColumnarValue::values_to_arrays(args)?;
    let string_arr = as_string_array(&args[0])?;
    let delimiter_arr = as_string_array(&args[1])?;
    let limit_arr = as_int64_array(&args[2])?;

    let mut values = vec![];
    let mut nulls = vec![];
    let mut offsets = vec![];
    string_arr
        .iter()
        .zip(delimiter_arr.iter())
        .zip(limit_arr.iter())
        .for_each(
            |((string, delimiter), limit)| match (string, delimiter, limit) {
                (Some(string), Some(delimiter), Some(limit)) => {
                    let split = string
                        .splitn(limit as usize, delimiter)
                        .map(|s| s.to_string())
                        .collect::<Vec<_>>();
                    offsets.push(split.len());
                    values.extend(split);
                    nulls.push(true);
                }
                _ => {
                    offsets.push(1);
                    values.push("".to_string());
                    nulls.push(false);
                }
            },
        );

    let element_type = Arc::new(Field::new("item", DataType::Utf8, true));
    let array = Arc::new(ListArray::try_new(
        Arc::clone(&element_type),
        OffsetBuffer::<i32>::from_lengths(offsets),
        Arc::new(StringArray::from(values)),
        Some(NullBuffer::from(nulls)),
    )?) as ArrayRef;
    Ok(ColumnarValue::Array(array))
}

fn split_varchar_varchar_bigint_return_type(_arg_types: &[DataType]) -> Result<DataType> {
    Ok(DataType::List(Arc::new(Field::new(
        "item",
        DataType::Utf8,
        true,
    ))))
}

fn split_varchar_varchar_bigint_simplify(
    args: Vec<Expr>,
    _info: &dyn SimplifyInfo,
) -> Result<ExprSimplifyResult> {
    Ok(ExprSimplifyResult::Original(args))
}

// ========== Generated template below this line ==========
// Do *NOT* edit below this line: all changes will be overwritten
// when template is regenerated!

#[derive(Debug)]
pub(super) struct split_varchar_varcharFunc {
    signature: Signature,
}

impl split_varchar_varcharFunc {
    pub fn new() -> Self {
        Self {
            signature: Signature::any(2, Volatility::Immutable),
        }
    }
}

impl ScalarUDFImpl for split_varchar_varcharFunc {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "split"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }

    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        split_varchar_varchar_return_type(arg_types)
    }

    fn invoke(&self, args: &[ColumnarValue]) -> Result<ColumnarValue> {
        split_varchar_varchar_invoke(args)
    }

    fn simplify(&self, args: Vec<Expr>, info: &dyn SimplifyInfo) -> Result<ExprSimplifyResult> {
        split_varchar_varchar_simplify(args, info)
    }
}

#[derive(Debug)]
pub(super) struct split_varchar_varchar_bigintFunc {
    signature: Signature,
}

impl split_varchar_varchar_bigintFunc {
    pub fn new() -> Self {
        Self {
            signature: Signature::any(3, Volatility::Immutable),
        }
    }
}

impl ScalarUDFImpl for split_varchar_varchar_bigintFunc {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "split"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }

    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        split_varchar_varchar_bigint_return_type(arg_types)
    }

    fn invoke(&self, args: &[ColumnarValue]) -> Result<ColumnarValue> {
        split_varchar_varchar_bigint_invoke(args)
    }

    fn simplify(&self, args: Vec<Expr>, info: &dyn SimplifyInfo) -> Result<ExprSimplifyResult> {
        split_varchar_varchar_bigint_simplify(args, info)
    }
}

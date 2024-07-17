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
use arrow::array::{BooleanArray, GenericByteArray, OffsetSizeTrait, PrimitiveArray};
use arrow::datatypes::{ArrowPrimitiveType, DataType, GenericBinaryType, GenericStringType};
use datafusion::common::Result;
use datafusion::logical_expr::simplify::{ExprSimplifyResult, SimplifyInfo};
use datafusion::logical_expr::{ColumnarValue, Expr, ScalarUDFImpl, Signature, Volatility};
use std::any::Any;

use crate::utils::KernelLifter;

struct MaxLifter;

impl KernelLifter for MaxLifter {
    const FEATURE_NAME: &'static str = "array_max";

    fn primitive_kernel<T>(array: &PrimitiveArray<T>) -> Option<T::Native>
    where
        T: ArrowPrimitiveType,
    {
        arrow::compute::kernels::aggregate::max(array)
    }

    fn string_kernel<O>(array: &GenericByteArray<GenericStringType<O>>) -> Option<&str>
    where
        O: OffsetSizeTrait,
    {
        arrow::compute::kernels::aggregate::max_string(array)
    }

    fn binary_kernel<O>(array: &GenericByteArray<GenericBinaryType<O>>) -> Option<&[u8]>
    where
        O: OffsetSizeTrait,
    {
        arrow::compute::kernels::aggregate::max_binary(array)
    }

    fn boolean_kernel(array: &BooleanArray) -> Option<bool> {
        arrow::compute::kernels::aggregate::max_boolean(array)
    }
}

fn array_max_array_1_invoke(args: &[ColumnarValue]) -> Result<ColumnarValue> {
    let array = args[0].clone().into_array(1).unwrap();
    MaxLifter::lift_all_kernels(array).map(ColumnarValue::Array)
}

fn array_max_array_1_return_type(arg_types: &[DataType]) -> Result<DataType> {
    assert!(arg_types.len() == 1);
    MaxLifter::return_type(&arg_types[0])
}

fn array_max_array_1_simplify(
    args: Vec<Expr>,
    _info: &dyn SimplifyInfo,
) -> Result<ExprSimplifyResult> {
    Ok(ExprSimplifyResult::Original(args))
}

// ========== Generated template below this line ==========
// Do *NOT* edit below this line: all changes will be overwritten
// when template is regenerated!

#[derive(Debug)]
pub(super) struct array_max_array_1Func {
    signature: Signature,
}

impl array_max_array_1Func {
    pub fn new() -> Self {
        Self {
            signature: Signature::any(1, Volatility::Immutable),
        }
    }
}

impl ScalarUDFImpl for array_max_array_1Func {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "array_max"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }

    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        array_max_array_1_return_type(arg_types)
    }

    fn invoke(&self, args: &[ColumnarValue]) -> Result<ColumnarValue> {
        array_max_array_1_invoke(args)
    }

    fn simplify(&self, args: Vec<Expr>, info: &dyn SimplifyInfo) -> Result<ExprSimplifyResult> {
        array_max_array_1_simplify(args, info)
    }
}

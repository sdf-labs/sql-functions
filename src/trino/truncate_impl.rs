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
use arrow::array::Decimal128Array;
use arrow::datatypes::DataType;
use datafusion::common::cast::as_decimal128_array;
use datafusion::common::{exec_err, Result};
use datafusion::error::DataFusionError;
use datafusion::logical_expr::simplify::{ExprSimplifyResult, SimplifyInfo};
use datafusion::logical_expr::{ColumnarValue, Expr, ScalarUDFImpl, Signature, Volatility};
use datafusion::scalar::ScalarValue;
use std::any::Any;
use std::sync::Arc;

fn truncate_decimal_p_s_bigint_invoke(args: &[ColumnarValue]) -> Result<ColumnarValue> {
    let precision = if let ColumnarValue::Scalar(ScalarValue::Int64(Some(v))) = args[1].to_owned() {
        v
    } else {
        return exec_err!("Second argument of `truncate` must be non-null scalar bigint");
    };

    let num = truncate(args, precision)?;

    Ok(ColumnarValue::Array(Arc::new(num)))
}

fn truncate(args: &[ColumnarValue], precision: i64) -> Result<Decimal128Array> {
    let args = ColumnarValue::values_to_arrays(args)?;
    let num = as_decimal128_array(&args[0])?;
    let p = num.precision();
    let s = num.scale();
    let num = num
        .into_iter()
        .map(|num| {
            num.map(|num| {
                let factor = 10i128.pow((s - precision as i8).max(0) as u32);
                let num = num / factor * factor;
                num
            })
        })
        .collect::<Decimal128Array>()
        .with_precision_and_scale(p, s)?;
    Ok(num)
}

fn truncate_decimal_p_s_bigint_return_type(arg_types: &[DataType]) -> Result<DataType> {
    Ok(arg_types[0].clone())
}

fn truncate_decimal_p_s_bigint_simplify(
    args: Vec<Expr>,
    _info: &dyn SimplifyInfo,
) -> Result<ExprSimplifyResult> {
    Ok(ExprSimplifyResult::Original(args))
}

fn truncate_decimal_p_s_invoke(args: &[ColumnarValue]) -> Result<ColumnarValue> {
    let num = truncate(args, 0)?;

    Ok(ColumnarValue::Array(Arc::new(num)))
}

fn truncate_decimal_p_s_return_type(arg_types: &[DataType]) -> Result<DataType> {
    Ok(arg_types[0].clone())
}

fn truncate_decimal_p_s_simplify(
    args: Vec<Expr>,
    _info: &dyn SimplifyInfo,
) -> Result<ExprSimplifyResult> {
    Ok(ExprSimplifyResult::Original(args))
}

fn truncate_double_invoke(_args: &[ColumnarValue]) -> Result<ColumnarValue> {
    Err(DataFusionError::NotImplemented(format!(
        "Not implemented {}:{}",
        file!(),
        line!()
    )))
}

fn truncate_double_return_type(_arg_types: &[DataType]) -> Result<DataType> {
    Err(DataFusionError::NotImplemented(format!(
        "Not implemented {}:{}",
        file!(),
        line!()
    )))
}

fn truncate_double_simplify(
    args: Vec<Expr>,
    _info: &dyn SimplifyInfo,
) -> Result<ExprSimplifyResult> {
    Ok(ExprSimplifyResult::Original(args))
}

fn truncate_real_invoke(_args: &[ColumnarValue]) -> Result<ColumnarValue> {
    Err(DataFusionError::NotImplemented(format!(
        "Not implemented {}:{}",
        file!(),
        line!()
    )))
}

fn truncate_real_return_type(_arg_types: &[DataType]) -> Result<DataType> {
    Err(DataFusionError::NotImplemented(format!(
        "Not implemented {}:{}",
        file!(),
        line!()
    )))
}

fn truncate_real_simplify(args: Vec<Expr>, _info: &dyn SimplifyInfo) -> Result<ExprSimplifyResult> {
    Ok(ExprSimplifyResult::Original(args))
}

// ========== Generated template below this line ==========
// Do *NOT* edit below this line: all changes will be overwritten
// when template is regenerated!


#[derive(Debug)]
pub(super) struct truncate_decimal_p_s_bigintFunc {
    signature: Signature,
}

impl truncate_decimal_p_s_bigintFunc {
    pub fn new() -> Self {        
        Self {
            signature: Signature::any(2, Volatility::Immutable),
        }
    }
}

impl ScalarUDFImpl for truncate_decimal_p_s_bigintFunc {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "truncate"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }


    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        truncate_decimal_p_s_bigint_return_type(arg_types)
    }

    fn invoke(&self, args: &[ColumnarValue]) -> Result<ColumnarValue> {
        truncate_decimal_p_s_bigint_invoke(args)
    }

    fn simplify(
        &self,
        args: Vec<Expr>,
        info: &dyn SimplifyInfo,
    ) -> Result<ExprSimplifyResult> {
        truncate_decimal_p_s_bigint_simplify(args, info)
    }

}

#[derive(Debug)]
pub(super) struct truncate_decimal_p_sFunc {
    signature: Signature,
}

impl truncate_decimal_p_sFunc {
    pub fn new() -> Self {        
        Self {
            signature: Signature::any(1, Volatility::Immutable),
        }
    }
}

impl ScalarUDFImpl for truncate_decimal_p_sFunc {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "truncate"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }


    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        truncate_decimal_p_s_return_type(arg_types)
    }

    fn invoke(&self, args: &[ColumnarValue]) -> Result<ColumnarValue> {
        truncate_decimal_p_s_invoke(args)
    }

    fn simplify(
        &self,
        args: Vec<Expr>,
        info: &dyn SimplifyInfo,
    ) -> Result<ExprSimplifyResult> {
        truncate_decimal_p_s_simplify(args, info)
    }

}

#[derive(Debug)]
pub(super) struct truncate_doubleFunc {
    signature: Signature,
}

impl truncate_doubleFunc {
    pub fn new() -> Self {        
        Self {
            signature: Signature::any(1, Volatility::Immutable),
        }
    }
}

impl ScalarUDFImpl for truncate_doubleFunc {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "truncate"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }


    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        truncate_double_return_type(arg_types)
    }

    fn invoke(&self, args: &[ColumnarValue]) -> Result<ColumnarValue> {
        truncate_double_invoke(args)
    }

    fn simplify(
        &self,
        args: Vec<Expr>,
        info: &dyn SimplifyInfo,
    ) -> Result<ExprSimplifyResult> {
        truncate_double_simplify(args, info)
    }

}

#[derive(Debug)]
pub(super) struct truncate_realFunc {
    signature: Signature,
}

impl truncate_realFunc {
    pub fn new() -> Self {        
        Self {
            signature: Signature::any(1, Volatility::Immutable),
        }
    }
}

impl ScalarUDFImpl for truncate_realFunc {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "truncate"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }


    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        truncate_real_return_type(arg_types)
    }

    fn invoke(&self, args: &[ColumnarValue]) -> Result<ColumnarValue> {
        truncate_real_invoke(args)
    }

    fn simplify(
        &self,
        args: Vec<Expr>,
        info: &dyn SimplifyInfo,
    ) -> Result<ExprSimplifyResult> {
        truncate_real_simplify(args, info)
    }

}

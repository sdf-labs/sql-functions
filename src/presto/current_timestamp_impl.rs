#![allow(non_camel_case_types)]
use arrow::datatypes::{DataType, TimeUnit};
use datafusion::common::Result;
use datafusion::error::DataFusionError;
use datafusion::logical_expr::simplify::{ExprSimplifyResult, SimplifyInfo};
use datafusion::logical_expr::{ColumnarValue, Expr, ScalarUDFImpl, Signature, Volatility};
use datafusion::scalar::ScalarValue;
use std::any::Any;


fn current_timestamp_invoke(_args: &[ColumnarValue]) -> Result<ColumnarValue> {
    Err(DataFusionError::NotImplemented("todo".to_string()))
}

fn current_timestamp_return_type(_arg_types: &[DataType]) -> Result<DataType> {
    Ok(DataType::Timestamp(TimeUnit::Millisecond, None))
}

fn current_timestamp_simplify(_args: Vec<Expr>, info: &dyn SimplifyInfo) -> Result<ExprSimplifyResult> {
    let now_ts = info.execution_props().query_execution_start_time;
        let millis = now_ts.timestamp_millis();
        
        Ok(ExprSimplifyResult::Simplified(Expr::Literal(
            ScalarValue::TimestampMillisecond(Some(millis), None)
        )))
}

fn current_timestamp_bigint_0_invoke(_args: &[ColumnarValue]) -> Result<ColumnarValue> {
    Err(DataFusionError::NotImplemented("todo".to_string()))
}

fn current_timestamp_bigint_0_return_type(_arg_types: &[DataType]) -> Result<DataType> {
    Ok(DataType::Timestamp(TimeUnit::Second, None))
}

fn current_timestamp_bigint_0_simplify(args: Vec<Expr>, _info: &dyn SimplifyInfo) -> Result<ExprSimplifyResult> {
    Ok(ExprSimplifyResult::Original(args))
}

fn current_timestamp_bigint_3_invoke(_args: &[ColumnarValue]) -> Result<ColumnarValue> {
    Err(DataFusionError::NotImplemented("todo".to_string()))
}

fn current_timestamp_bigint_3_return_type(_arg_types: &[DataType]) -> Result<DataType> {
    Err(DataFusionError::NotImplemented("todo".to_string()))
}

fn current_timestamp_bigint_3_simplify(args: Vec<Expr>, _info: &dyn SimplifyInfo) -> Result<ExprSimplifyResult> {
    Ok(ExprSimplifyResult::Original(args))
}

fn current_timestamp_bigint_6_invoke(_args: &[ColumnarValue]) -> Result<ColumnarValue> {
    Err(DataFusionError::NotImplemented("todo".to_string()))
}

fn current_timestamp_bigint_6_return_type(_arg_types: &[DataType]) -> Result<DataType> {
    Err(DataFusionError::NotImplemented("todo".to_string()))
}

fn current_timestamp_bigint_6_simplify(args: Vec<Expr>, _info: &dyn SimplifyInfo) -> Result<ExprSimplifyResult> {
    Ok(ExprSimplifyResult::Original(args))
}

fn current_timestamp_bigint_9_invoke(_args: &[ColumnarValue]) -> Result<ColumnarValue> {
    Err(DataFusionError::NotImplemented("todo".to_string()))
}

fn current_timestamp_bigint_9_return_type(_arg_types: &[DataType]) -> Result<DataType> {
    Err(DataFusionError::NotImplemented("todo".to_string()))
}

fn current_timestamp_bigint_9_simplify(args: Vec<Expr>, _info: &dyn SimplifyInfo) -> Result<ExprSimplifyResult> {
    Ok(ExprSimplifyResult::Original(args))
}


/// ========== Above are user code/Below are generated code ==========


#[derive(Debug)]
pub(super) struct current_timestampFunc {
    signature: Signature,
}

impl current_timestampFunc {
    pub fn new() -> Self {        
        Self {
            signature: Signature::any(0, Volatility::Immutable),
        }
    }
}

impl ScalarUDFImpl for current_timestampFunc {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "current_timestamp"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }


    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        current_timestamp_return_type(arg_types)
    }

    fn invoke(&self, args: &[ColumnarValue]) -> Result<ColumnarValue> {
        current_timestamp_invoke(args)
    }

    fn simplify(
        &self,
        args: Vec<Expr>,
        info: &dyn SimplifyInfo,
    ) -> Result<ExprSimplifyResult> {
        current_timestamp_simplify(args, info)
    }

}

#[derive(Debug)]
pub(super) struct current_timestamp_bigint_0Func {
    signature: Signature,
}

impl current_timestamp_bigint_0Func {
    pub fn new() -> Self {        
        Self {
            signature: Signature::any(1, Volatility::Immutable),
        }
    }
}

impl ScalarUDFImpl for current_timestamp_bigint_0Func {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "current_timestamp"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }


    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        current_timestamp_bigint_0_return_type(arg_types)
    }

    fn invoke(&self, args: &[ColumnarValue]) -> Result<ColumnarValue> {
        current_timestamp_bigint_0_invoke(args)
    }

    fn simplify(
        &self,
        args: Vec<Expr>,
        info: &dyn SimplifyInfo,
    ) -> Result<ExprSimplifyResult> {
        current_timestamp_bigint_0_simplify(args, info)
    }

}

#[derive(Debug)]
pub(super) struct current_timestamp_bigint_3Func {
    signature: Signature,
}

impl current_timestamp_bigint_3Func {
    pub fn new() -> Self {        
        Self {
            signature: Signature::any(1, Volatility::Immutable),
        }
    }
}

impl ScalarUDFImpl for current_timestamp_bigint_3Func {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "current_timestamp"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }


    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        current_timestamp_bigint_3_return_type(arg_types)
    }

    fn invoke(&self, args: &[ColumnarValue]) -> Result<ColumnarValue> {
        current_timestamp_bigint_3_invoke(args)
    }

    fn simplify(
        &self,
        args: Vec<Expr>,
        info: &dyn SimplifyInfo,
    ) -> Result<ExprSimplifyResult> {
        current_timestamp_bigint_3_simplify(args, info)
    }

}

#[derive(Debug)]
pub(super) struct current_timestamp_bigint_6Func {
    signature: Signature,
}

impl current_timestamp_bigint_6Func {
    pub fn new() -> Self {        
        Self {
            signature: Signature::any(1, Volatility::Immutable),
        }
    }
}

impl ScalarUDFImpl for current_timestamp_bigint_6Func {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "current_timestamp"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }


    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        current_timestamp_bigint_6_return_type(arg_types)
    }

    fn invoke(&self, args: &[ColumnarValue]) -> Result<ColumnarValue> {
        current_timestamp_bigint_6_invoke(args)
    }

    fn simplify(
        &self,
        args: Vec<Expr>,
        info: &dyn SimplifyInfo,
    ) -> Result<ExprSimplifyResult> {
        current_timestamp_bigint_6_simplify(args, info)
    }

}

#[derive(Debug)]
pub(super) struct current_timestamp_bigint_9Func {
    signature: Signature,
}

impl current_timestamp_bigint_9Func {
    pub fn new() -> Self {        
        Self {
            signature: Signature::any(1, Volatility::Immutable),
        }
    }
}

impl ScalarUDFImpl for current_timestamp_bigint_9Func {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "current_timestamp"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }


    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        current_timestamp_bigint_9_return_type(arg_types)
    }

    fn invoke(&self, args: &[ColumnarValue]) -> Result<ColumnarValue> {
        current_timestamp_bigint_9_invoke(args)
    }

    fn simplify(
        &self,
        args: Vec<Expr>,
        info: &dyn SimplifyInfo,
    ) -> Result<ExprSimplifyResult> {
        current_timestamp_bigint_9_simplify(args, info)
    }

}

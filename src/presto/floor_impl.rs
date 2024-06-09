#![allow(non_camel_case_types)]
use arrow::datatypes::DataType;
use datafusion::common::Result;
use datafusion::error::DataFusionError;
use datafusion::logical_expr::simplify::{ExprSimplifyResult, SimplifyInfo};
use datafusion::logical_expr::{ColumnarValue, Expr, ScalarUDFImpl, Signature, Volatility};
use std::any::Any;


fn floor_bigint_invoke(_args: &[ColumnarValue]) -> Result<ColumnarValue> {
    Err(DataFusionError::NotImplemented("todo".to_string()))
}

fn floor_bigint_return_type(_arg_types: &[DataType]) -> Result<DataType> {
    Err(DataFusionError::NotImplemented("todo".to_string()))
}

fn floor_bigint_simplify(args: Vec<Expr>, _info: &dyn SimplifyInfo) -> Result<ExprSimplifyResult> {
    Ok(ExprSimplifyResult::Original(args))
}

fn floor_decimal_p_s_invoke(_args: &[ColumnarValue]) -> Result<ColumnarValue> {
    Err(DataFusionError::NotImplemented("todo".to_string()))
}

fn floor_decimal_p_s_return_type(_arg_types: &[DataType]) -> Result<DataType> {
    Err(DataFusionError::NotImplemented("todo".to_string()))
}

fn floor_decimal_p_s_simplify(args: Vec<Expr>, _info: &dyn SimplifyInfo) -> Result<ExprSimplifyResult> {
    Ok(ExprSimplifyResult::Original(args))
}

fn floor_double_invoke(_args: &[ColumnarValue]) -> Result<ColumnarValue> {
    Err(DataFusionError::NotImplemented("todo".to_string()))
}

fn floor_double_return_type(_arg_types: &[DataType]) -> Result<DataType> {
    Err(DataFusionError::NotImplemented("todo".to_string()))
}

fn floor_double_simplify(args: Vec<Expr>, _info: &dyn SimplifyInfo) -> Result<ExprSimplifyResult> {
    Ok(ExprSimplifyResult::Original(args))
}

fn floor_integer_invoke(_args: &[ColumnarValue]) -> Result<ColumnarValue> {
    Err(DataFusionError::NotImplemented("todo".to_string()))
}

fn floor_integer_return_type(_arg_types: &[DataType]) -> Result<DataType> {
    Err(DataFusionError::NotImplemented("todo".to_string()))
}

fn floor_integer_simplify(args: Vec<Expr>, _info: &dyn SimplifyInfo) -> Result<ExprSimplifyResult> {
    Ok(ExprSimplifyResult::Original(args))
}

fn floor_real_invoke(_args: &[ColumnarValue]) -> Result<ColumnarValue> {
    Err(DataFusionError::NotImplemented("todo".to_string()))
}

fn floor_real_return_type(_arg_types: &[DataType]) -> Result<DataType> {
    Err(DataFusionError::NotImplemented("todo".to_string()))
}

fn floor_real_simplify(args: Vec<Expr>, _info: &dyn SimplifyInfo) -> Result<ExprSimplifyResult> {
    Ok(ExprSimplifyResult::Original(args))
}

fn floor_smallint_invoke(_args: &[ColumnarValue]) -> Result<ColumnarValue> {
    Err(DataFusionError::NotImplemented("todo".to_string()))
}

fn floor_smallint_return_type(_arg_types: &[DataType]) -> Result<DataType> {
    Err(DataFusionError::NotImplemented("todo".to_string()))
}

fn floor_smallint_simplify(args: Vec<Expr>, _info: &dyn SimplifyInfo) -> Result<ExprSimplifyResult> {
    Ok(ExprSimplifyResult::Original(args))
}

fn floor_tinyint_invoke(_args: &[ColumnarValue]) -> Result<ColumnarValue> {
    Err(DataFusionError::NotImplemented("todo".to_string()))
}

fn floor_tinyint_return_type(_arg_types: &[DataType]) -> Result<DataType> {
    Err(DataFusionError::NotImplemented("todo".to_string()))
}

fn floor_tinyint_simplify(args: Vec<Expr>, _info: &dyn SimplifyInfo) -> Result<ExprSimplifyResult> {
    Ok(ExprSimplifyResult::Original(args))
}


/// ========== Above are user code/Below are generated code ==========


#[derive(Debug)]
pub(super) struct floor_bigintFunc {
    signature: Signature,
}

impl floor_bigintFunc {
    pub fn new() -> Self {        
        Self {
            signature: Signature::any(1, Volatility::Immutable),
        }
    }
}

impl ScalarUDFImpl for floor_bigintFunc {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "floor"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }


    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        floor_bigint_return_type(arg_types)
    }

    fn invoke(&self, args: &[ColumnarValue]) -> Result<ColumnarValue> {
        floor_bigint_invoke(args)
    }

    fn simplify(
        &self,
        args: Vec<Expr>,
        info: &dyn SimplifyInfo,
    ) -> Result<ExprSimplifyResult> {
        floor_bigint_simplify(args, info)
    }

}

#[derive(Debug)]
pub(super) struct floor_decimal_p_sFunc {
    signature: Signature,
}

impl floor_decimal_p_sFunc {
    pub fn new() -> Self {        
        Self {
            signature: Signature::any(1, Volatility::Immutable),
        }
    }
}

impl ScalarUDFImpl for floor_decimal_p_sFunc {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "floor"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }


    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        floor_decimal_p_s_return_type(arg_types)
    }

    fn invoke(&self, args: &[ColumnarValue]) -> Result<ColumnarValue> {
        floor_decimal_p_s_invoke(args)
    }

    fn simplify(
        &self,
        args: Vec<Expr>,
        info: &dyn SimplifyInfo,
    ) -> Result<ExprSimplifyResult> {
        floor_decimal_p_s_simplify(args, info)
    }

}

#[derive(Debug)]
pub(super) struct floor_doubleFunc {
    signature: Signature,
}

impl floor_doubleFunc {
    pub fn new() -> Self {        
        Self {
            signature: Signature::any(1, Volatility::Immutable),
        }
    }
}

impl ScalarUDFImpl for floor_doubleFunc {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "floor"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }


    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        floor_double_return_type(arg_types)
    }

    fn invoke(&self, args: &[ColumnarValue]) -> Result<ColumnarValue> {
        floor_double_invoke(args)
    }

    fn simplify(
        &self,
        args: Vec<Expr>,
        info: &dyn SimplifyInfo,
    ) -> Result<ExprSimplifyResult> {
        floor_double_simplify(args, info)
    }

}

#[derive(Debug)]
pub(super) struct floor_integerFunc {
    signature: Signature,
}

impl floor_integerFunc {
    pub fn new() -> Self {        
        Self {
            signature: Signature::any(1, Volatility::Immutable),
        }
    }
}

impl ScalarUDFImpl for floor_integerFunc {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "floor"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }


    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        floor_integer_return_type(arg_types)
    }

    fn invoke(&self, args: &[ColumnarValue]) -> Result<ColumnarValue> {
        floor_integer_invoke(args)
    }

    fn simplify(
        &self,
        args: Vec<Expr>,
        info: &dyn SimplifyInfo,
    ) -> Result<ExprSimplifyResult> {
        floor_integer_simplify(args, info)
    }

}

#[derive(Debug)]
pub(super) struct floor_realFunc {
    signature: Signature,
}

impl floor_realFunc {
    pub fn new() -> Self {        
        Self {
            signature: Signature::any(1, Volatility::Immutable),
        }
    }
}

impl ScalarUDFImpl for floor_realFunc {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "floor"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }


    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        floor_real_return_type(arg_types)
    }

    fn invoke(&self, args: &[ColumnarValue]) -> Result<ColumnarValue> {
        floor_real_invoke(args)
    }

    fn simplify(
        &self,
        args: Vec<Expr>,
        info: &dyn SimplifyInfo,
    ) -> Result<ExprSimplifyResult> {
        floor_real_simplify(args, info)
    }

}

#[derive(Debug)]
pub(super) struct floor_smallintFunc {
    signature: Signature,
}

impl floor_smallintFunc {
    pub fn new() -> Self {        
        Self {
            signature: Signature::any(1, Volatility::Immutable),
        }
    }
}

impl ScalarUDFImpl for floor_smallintFunc {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "floor"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }


    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        floor_smallint_return_type(arg_types)
    }

    fn invoke(&self, args: &[ColumnarValue]) -> Result<ColumnarValue> {
        floor_smallint_invoke(args)
    }

    fn simplify(
        &self,
        args: Vec<Expr>,
        info: &dyn SimplifyInfo,
    ) -> Result<ExprSimplifyResult> {
        floor_smallint_simplify(args, info)
    }

}

#[derive(Debug)]
pub(super) struct floor_tinyintFunc {
    signature: Signature,
}

impl floor_tinyintFunc {
    pub fn new() -> Self {        
        Self {
            signature: Signature::any(1, Volatility::Immutable),
        }
    }
}

impl ScalarUDFImpl for floor_tinyintFunc {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "floor"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }


    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        floor_tinyint_return_type(arg_types)
    }

    fn invoke(&self, args: &[ColumnarValue]) -> Result<ColumnarValue> {
        floor_tinyint_invoke(args)
    }

    fn simplify(
        &self,
        args: Vec<Expr>,
        info: &dyn SimplifyInfo,
    ) -> Result<ExprSimplifyResult> {
        floor_tinyint_simplify(args, info)
    }

}

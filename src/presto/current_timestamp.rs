// start implementing uses
use arrow::datatypes::{DataType, TimeUnit};
use datafusion::common::Result;
use datafusion::logical_expr::simplify::{ExprSimplifyResult, SimplifyInfo};
use datafusion::logical_expr::{ColumnarValue, Expr, ScalarUDFImpl, Signature, Volatility};
use datafusion::scalar::ScalarValue;
use std::any::Any;
// end implementing uses

#[derive(Debug)]
pub(super) struct Func {
    signature: Signature,
}

impl Func {
    pub fn new() -> Self {        
        // start implementing constructor
        Self {
            signature:
            Signature::exact(vec![], Volatility::Immutable)
        }
        // end implementing constructor
    }
}

impl ScalarUDFImpl for Func {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "current_timestamp"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }

    // start implementing return_type
    fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType> {
        Ok(DataType::Timestamp(TimeUnit::Millisecond, None))
    }
    // end implementing return_type

    // start implementing invoke
    fn invoke(&self, _args: &[ColumnarValue]) -> Result<ColumnarValue> {
        todo!()
    }
    // end implementing invoke

    // start implementing simplify
    fn simplify(
        &self,
        _args: Vec<Expr>,
        info: &dyn SimplifyInfo,
    ) -> Result<ExprSimplifyResult> {
        let now_ts = info.execution_props().query_execution_start_time;
        let millis = now_ts.timestamp_millis();
        dbg!(millis);
        Ok(ExprSimplifyResult::Simplified(Expr::Literal(
            ScalarValue::TimestampMillisecond(Some(millis), None)
        )))
    }
    // end implementing simplify
}

// start implementing footer
// end implementing footer
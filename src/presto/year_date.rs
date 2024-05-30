// start implementing uses
use arrow::array::ArrayRef;
use arrow::compute::{cast, date_part, DatePart};
use arrow::datatypes::DataType;
use datafusion::common::Result;
use datafusion::logical_expr::{ColumnarValue, ScalarUDFImpl, Signature, Volatility};
use std::any::Any;

use crate::utils::make_scalar_function;
// end implementing uses

#[derive(Debug)]
pub(super) struct Func {
    signature: Signature,
}

impl Func {
    pub fn new() -> Self {        
        // start implementing constructor
        Self {
            signature: Signature::exact(vec![DataType::Date32], Volatility::Immutable),
        }
        // end implementing constructor
    }
}

impl ScalarUDFImpl for Func {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "year"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }

    // start implementing return_type
    fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType> {
        Ok(DataType::Int64)
    }
    // end implementing return_type

    // start implementing invoke
    fn invoke(&self, args: &[ColumnarValue]) -> Result<ColumnarValue> {
        make_scalar_function(year, vec![])(args)
    }
    // end implementing invoke
}

// start implementing footer
fn year(args: &[ArrayRef]) -> Result<ArrayRef> {
    let array = &args[0];
    Ok(cast(date_part(array, DatePart::Year)?.as_ref(), &DataType::Int64)?)
}
// end implementing footer
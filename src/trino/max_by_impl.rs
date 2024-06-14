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
use arrow::array::ArrayRef;
use arrow::datatypes::{DataType, Field};
use datafusion::common::Result;
use datafusion::error::DataFusionError;
use datafusion::logical_expr::function::AccumulatorArgs;
use datafusion::logical_expr::utils::format_state_name;
use datafusion::logical_expr::{Accumulator, AggregateUDFImpl, Signature, Volatility};
use datafusion::physical_plan::expressions::StatsType;
use datafusion::scalar::ScalarValue;
use std::any::Any;

fn max_by_5_4_bigint_return_type(_arg_types: &[DataType]) -> Result<DataType> {
    Err(DataFusionError::NotImplemented(format!(
        "Not implemented {}:{}",
        file!(),
        line!()
    )))
}

fn max_by_5_4_bigint_state_fields(
    _name: &str,
    _value_type: DataType,
    _ordering_fields: Vec<Field>,
) -> Result<Vec<Field>> {
    Err(DataFusionError::NotImplemented(format!(
        "Not implemented {}:{}",
        file!(),
        line!()
    )))
}

fn max_by_5_4_bigint_accumulator(_acc_args: AccumulatorArgs) -> Result<Box<dyn Accumulator>> {
    Err(DataFusionError::NotImplemented(format!(
        "Not implemented {}:{}",
        file!(),
        line!()
    )))
}

#[derive(Debug)]
pub struct max_by_5_4_bigint_Accumulator {}

impl max_by_5_4_bigint_Accumulator {
    pub fn _try_new(_s_type: StatsType) -> Result<Self> {
        Err(DataFusionError::NotImplemented(format!(
            "Not implemented {}:{}",
            file!(),
            line!()
        )))
    }
}

impl Accumulator for max_by_5_4_bigint_Accumulator {
    fn state(&mut self) -> Result<Vec<ScalarValue>> {
        Err(DataFusionError::NotImplemented(format!(
            "Not implemented {}:{}",
            file!(),
            line!()
        )))
    }

    fn update_batch(&mut self, _values: &[ArrayRef]) -> Result<()> {
        Err(DataFusionError::NotImplemented(format!(
            "Not implemented {}:{}",
            file!(),
            line!()
        )))
    }

    fn retract_batch(&mut self, _values: &[ArrayRef]) -> Result<()> {
        Err(DataFusionError::NotImplemented(format!(
            "Not implemented {}:{}",
            file!(),
            line!()
        )))
    }

    fn merge_batch(&mut self, _states: &[ArrayRef]) -> Result<()> {
        Err(DataFusionError::NotImplemented(format!(
            "Not implemented {}:{}",
            file!(),
            line!()
        )))
    }

    fn evaluate(&mut self) -> Result<ScalarValue> {
        Err(DataFusionError::NotImplemented(format!(
            "Not implemented {}:{}",
            file!(),
            line!()
        )))
    }

    fn size(&self) -> usize {
        // TODO
        0
    }
}

fn max_by_5_4_return_type(arg_types: &[DataType]) -> Result<DataType> {
    Ok(arg_types[0].clone())
}

fn max_by_5_4_state_fields(
    name: &str,
    value_type: DataType,
    ordering_fields: Vec<Field>,
) -> Result<Vec<Field>> {
    Ok(vec![
        Field::new(format_state_name(name, "target"), self.target_datatype.clone(), true),
        Field::new(format_state_name(name, "value"), self.value_datatype.clone(), true),
    ])
}

fn max_by_5_4_accumulator(_acc_args: AccumulatorArgs) -> Result<Box<dyn Accumulator>> {
    Ok(Box::new(max_by_5_4_Accumulator::try_new()?))
}

#[derive(Debug)]
pub struct max_by_5_4_Accumulator {
    target: ScalarValue,
    max: ScalarValue,
}

impl max_by_5_4_Accumulator {
    pub fn try_new(datatype: &DataType) -> Result<Self> {
        Ok(Self {
            target: 0,
            value: 0.0,
        })
    }
}

impl Accumulator for max_by_5_4_Accumulator {
    fn state(&mut self) -> Result<Vec<ScalarValue>> {
        Ok(vec![
            ScalarValue::from(self.target),
            ScalarValue::from(self.value),
        ])
    }

    fn update_batch(&mut self, values: &[ArrayRef]) -> Result<()> {
        // values = [
        //     StringArray
        //     [
        //     "Alice",
        //     "Bob",
        //     "Alice",
        //     "Bob",
        //     ],
        //     PrimitiveArray<Decimal128(10, 2)>
        //     [
        //     150000,
        //     200000,
        //     250000,
        //     300000,
        //     ],
        // ]
        dbg!(values);
        todo!()
    }

    fn retract_batch(&mut self, values: &[ArrayRef]) -> Result<()> {
        dbg!(values);
        todo!()
    }

    fn merge_batch(&mut self, states: &[ArrayRef]) -> Result<()> {
        dbg!(states);
        todo!()
    }

    fn evaluate(&mut self) -> Result<ScalarValue> {
        todo!()
    }

    fn size(&self) -> usize {
        std::mem::size_of_val(self)
    }
}

// ========== Generated template below this line ==========
// Do *NOT* edit below this line: all changes will be overwritten
// when template is regenerated!

#[derive(Debug)]
pub(super) struct max_by_5_4_bigintFunc {
    signature: Signature,
}

impl max_by_5_4_bigintFunc {
    pub fn new() -> Self {
        Self {
            signature: Signature::any(3, Volatility::Immutable),
        }
    }
}

impl AggregateUDFImpl for max_by_5_4_bigintFunc {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "max_by"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }

    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        max_by_5_4_bigint_return_type(arg_types)
    }

    fn state_fields(
        &self,
        name: &str,
        value_type: DataType,
        ordering_fields: Vec<Field>,
    ) -> Result<Vec<Field>> {
        max_by_5_4_bigint_state_fields(name, value_type, ordering_fields)
    }

    fn accumulator(&self, acc_args: AccumulatorArgs) -> Result<Box<dyn Accumulator>> {
        max_by_5_4_bigint_accumulator(acc_args)
    }
}

#[derive(Debug)]
pub(super) struct max_by_5_4Func {
    signature: Signature,
}

impl max_by_5_4Func {
    pub fn new() -> Self {
        Self {
            signature: Signature::any(2, Volatility::Immutable),
        }
    }
}

impl AggregateUDFImpl for max_by_5_4Func {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "max_by"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }

    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        max_by_5_4_return_type(arg_types)
    }

    fn state_fields(
        &self,
        name: &str,
        value_type: DataType,
        ordering_fields: Vec<Field>,
    ) -> Result<Vec<Field>> {
        max_by_5_4_state_fields(name, value_type, ordering_fields)
    }

    fn accumulator(&self, acc_args: AccumulatorArgs) -> Result<Box<dyn Accumulator>> {
        max_by_5_4_accumulator(acc_args)
    }
}

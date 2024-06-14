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
use datafusion::logical_expr::{Accumulator, AggregateUDFImpl, Signature, Volatility};
use datafusion::physical_plan::expressions::StatsType;
use datafusion::scalar::ScalarValue;
use std::any::Any;




fn numeric_histogram_bigint_double_return_type(_arg_types: &[DataType]) -> Result<DataType> {
    Err(DataFusionError::NotImplemented(format!("Not implemented {}:{}", file!(), line!())))
}

fn numeric_histogram_bigint_double_state_fields(_name: &str, _value_type: DataType, _ordering_fields: Vec<Field>) -> Result<Vec<Field>> {
    Err(DataFusionError::NotImplemented(format!("Not implemented {}:{}", file!(), line!())))
}

fn numeric_histogram_bigint_double_accumulator(_acc_args: AccumulatorArgs) -> Result<Box<dyn Accumulator>> {
    Err(DataFusionError::NotImplemented(format!("Not implemented {}:{}", file!(), line!())))
}

#[derive(Debug)]
pub struct numeric_histogram_bigint_double_Accumulator {}

impl numeric_histogram_bigint_double_Accumulator {
    pub fn _try_new(_s_type: StatsType) -> Result<Self> {
        Err(DataFusionError::NotImplemented(format!("Not implemented {}:{}", file!(), line!())))
    }
}

impl Accumulator for numeric_histogram_bigint_double_Accumulator {
    fn state(&mut self) -> Result<Vec<ScalarValue>> {
        Err(DataFusionError::NotImplemented(format!("Not implemented {}:{}", file!(), line!())))
    }

    fn update_batch(&mut self, _values: &[ArrayRef]) -> Result<()> {
        Err(DataFusionError::NotImplemented(format!("Not implemented {}:{}", file!(), line!())))
    }

    fn retract_batch(&mut self, _values: &[ArrayRef]) -> Result<()> {
        Err(DataFusionError::NotImplemented(format!("Not implemented {}:{}", file!(), line!())))
    }

    fn merge_batch(&mut self, _states: &[ArrayRef]) -> Result<()> {
        Err(DataFusionError::NotImplemented(format!("Not implemented {}:{}", file!(), line!())))
    }

    fn evaluate(&mut self) -> Result<ScalarValue> {
        Err(DataFusionError::NotImplemented(format!("Not implemented {}:{}", file!(), line!())))
    }

    fn size(&self) -> usize {
        // TODO
        0
    }
}



fn numeric_histogram_bigint_double_double_return_type(_arg_types: &[DataType]) -> Result<DataType> {
    Err(DataFusionError::NotImplemented(format!("Not implemented {}:{}", file!(), line!())))
}

fn numeric_histogram_bigint_double_double_state_fields(_name: &str, _value_type: DataType, _ordering_fields: Vec<Field>) -> Result<Vec<Field>> {
    Err(DataFusionError::NotImplemented(format!("Not implemented {}:{}", file!(), line!())))
}

fn numeric_histogram_bigint_double_double_accumulator(_acc_args: AccumulatorArgs) -> Result<Box<dyn Accumulator>> {
    Err(DataFusionError::NotImplemented(format!("Not implemented {}:{}", file!(), line!())))
}

#[derive(Debug)]
pub struct numeric_histogram_bigint_double_double_Accumulator {}

impl numeric_histogram_bigint_double_double_Accumulator {
    pub fn _try_new(_s_type: StatsType) -> Result<Self> {
        Err(DataFusionError::NotImplemented(format!("Not implemented {}:{}", file!(), line!())))
    }
}

impl Accumulator for numeric_histogram_bigint_double_double_Accumulator {
    fn state(&mut self) -> Result<Vec<ScalarValue>> {
        Err(DataFusionError::NotImplemented(format!("Not implemented {}:{}", file!(), line!())))
    }

    fn update_batch(&mut self, _values: &[ArrayRef]) -> Result<()> {
        Err(DataFusionError::NotImplemented(format!("Not implemented {}:{}", file!(), line!())))
    }

    fn retract_batch(&mut self, _values: &[ArrayRef]) -> Result<()> {
        Err(DataFusionError::NotImplemented(format!("Not implemented {}:{}", file!(), line!())))
    }

    fn merge_batch(&mut self, _states: &[ArrayRef]) -> Result<()> {
        Err(DataFusionError::NotImplemented(format!("Not implemented {}:{}", file!(), line!())))
    }

    fn evaluate(&mut self) -> Result<ScalarValue> {
        Err(DataFusionError::NotImplemented(format!("Not implemented {}:{}", file!(), line!())))
    }

    fn size(&self) -> usize {
        // TODO
        0
    }
}



fn numeric_histogram_bigint_real_return_type(_arg_types: &[DataType]) -> Result<DataType> {
    Err(DataFusionError::NotImplemented(format!("Not implemented {}:{}", file!(), line!())))
}

fn numeric_histogram_bigint_real_state_fields(_name: &str, _value_type: DataType, _ordering_fields: Vec<Field>) -> Result<Vec<Field>> {
    Err(DataFusionError::NotImplemented(format!("Not implemented {}:{}", file!(), line!())))
}

fn numeric_histogram_bigint_real_accumulator(_acc_args: AccumulatorArgs) -> Result<Box<dyn Accumulator>> {
    Err(DataFusionError::NotImplemented(format!("Not implemented {}:{}", file!(), line!())))
}

#[derive(Debug)]
pub struct numeric_histogram_bigint_real_Accumulator {}

impl numeric_histogram_bigint_real_Accumulator {
    pub fn _try_new(_s_type: StatsType) -> Result<Self> {
        Err(DataFusionError::NotImplemented(format!("Not implemented {}:{}", file!(), line!())))
    }
}

impl Accumulator for numeric_histogram_bigint_real_Accumulator {
    fn state(&mut self) -> Result<Vec<ScalarValue>> {
        Err(DataFusionError::NotImplemented(format!("Not implemented {}:{}", file!(), line!())))
    }

    fn update_batch(&mut self, _values: &[ArrayRef]) -> Result<()> {
        Err(DataFusionError::NotImplemented(format!("Not implemented {}:{}", file!(), line!())))
    }

    fn retract_batch(&mut self, _values: &[ArrayRef]) -> Result<()> {
        Err(DataFusionError::NotImplemented(format!("Not implemented {}:{}", file!(), line!())))
    }

    fn merge_batch(&mut self, _states: &[ArrayRef]) -> Result<()> {
        Err(DataFusionError::NotImplemented(format!("Not implemented {}:{}", file!(), line!())))
    }

    fn evaluate(&mut self) -> Result<ScalarValue> {
        Err(DataFusionError::NotImplemented(format!("Not implemented {}:{}", file!(), line!())))
    }

    fn size(&self) -> usize {
        // TODO
        0
    }
}



fn numeric_histogram_bigint_real_double_return_type(_arg_types: &[DataType]) -> Result<DataType> {
    Err(DataFusionError::NotImplemented(format!("Not implemented {}:{}", file!(), line!())))
}

fn numeric_histogram_bigint_real_double_state_fields(_name: &str, _value_type: DataType, _ordering_fields: Vec<Field>) -> Result<Vec<Field>> {
    Err(DataFusionError::NotImplemented(format!("Not implemented {}:{}", file!(), line!())))
}

fn numeric_histogram_bigint_real_double_accumulator(_acc_args: AccumulatorArgs) -> Result<Box<dyn Accumulator>> {
    Err(DataFusionError::NotImplemented(format!("Not implemented {}:{}", file!(), line!())))
}

#[derive(Debug)]
pub struct numeric_histogram_bigint_real_double_Accumulator {}

impl numeric_histogram_bigint_real_double_Accumulator {
    pub fn _try_new(_s_type: StatsType) -> Result<Self> {
        Err(DataFusionError::NotImplemented(format!("Not implemented {}:{}", file!(), line!())))
    }
}

impl Accumulator for numeric_histogram_bigint_real_double_Accumulator {
    fn state(&mut self) -> Result<Vec<ScalarValue>> {
        Err(DataFusionError::NotImplemented(format!("Not implemented {}:{}", file!(), line!())))
    }

    fn update_batch(&mut self, _values: &[ArrayRef]) -> Result<()> {
        Err(DataFusionError::NotImplemented(format!("Not implemented {}:{}", file!(), line!())))
    }

    fn retract_batch(&mut self, _values: &[ArrayRef]) -> Result<()> {
        Err(DataFusionError::NotImplemented(format!("Not implemented {}:{}", file!(), line!())))
    }

    fn merge_batch(&mut self, _states: &[ArrayRef]) -> Result<()> {
        Err(DataFusionError::NotImplemented(format!("Not implemented {}:{}", file!(), line!())))
    }

    fn evaluate(&mut self) -> Result<ScalarValue> {
        Err(DataFusionError::NotImplemented(format!("Not implemented {}:{}", file!(), line!())))
    }

    fn size(&self) -> usize {
        // TODO
        0
    }
}



// ========== Generated template below this line ==========
// Do *NOT* edit below this line: all changes will be overwritten
// when template is regenerated!


#[derive(Debug)]
pub(super) struct numeric_histogram_bigint_doubleFunc {
    signature: Signature,
}

impl numeric_histogram_bigint_doubleFunc {
    pub fn new() -> Self {        
        Self {
            signature: Signature::any(2, Volatility::Immutable),
        }
    }
}

impl AggregateUDFImpl for numeric_histogram_bigint_doubleFunc {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "numeric_histogram"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }


    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        numeric_histogram_bigint_double_return_type(arg_types)
    }

    fn state_fields(
        &self,
        name: &str,
        value_type: DataType,
        ordering_fields: Vec<Field>,
    ) -> Result<Vec<Field>> {
        numeric_histogram_bigint_double_state_fields(name, value_type, ordering_fields)
    }

    fn accumulator(&self, acc_args: AccumulatorArgs) -> Result<Box<dyn Accumulator>> {
        numeric_histogram_bigint_double_accumulator(acc_args)
    }
}

#[derive(Debug)]
pub(super) struct numeric_histogram_bigint_double_doubleFunc {
    signature: Signature,
}

impl numeric_histogram_bigint_double_doubleFunc {
    pub fn new() -> Self {        
        Self {
            signature: Signature::any(3, Volatility::Immutable),
        }
    }
}

impl AggregateUDFImpl for numeric_histogram_bigint_double_doubleFunc {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "numeric_histogram"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }


    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        numeric_histogram_bigint_double_double_return_type(arg_types)
    }

    fn state_fields(
        &self,
        name: &str,
        value_type: DataType,
        ordering_fields: Vec<Field>,
    ) -> Result<Vec<Field>> {
        numeric_histogram_bigint_double_double_state_fields(name, value_type, ordering_fields)
    }

    fn accumulator(&self, acc_args: AccumulatorArgs) -> Result<Box<dyn Accumulator>> {
        numeric_histogram_bigint_double_double_accumulator(acc_args)
    }
}

#[derive(Debug)]
pub(super) struct numeric_histogram_bigint_realFunc {
    signature: Signature,
}

impl numeric_histogram_bigint_realFunc {
    pub fn new() -> Self {        
        Self {
            signature: Signature::any(2, Volatility::Immutable),
        }
    }
}

impl AggregateUDFImpl for numeric_histogram_bigint_realFunc {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "numeric_histogram"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }


    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        numeric_histogram_bigint_real_return_type(arg_types)
    }

    fn state_fields(
        &self,
        name: &str,
        value_type: DataType,
        ordering_fields: Vec<Field>,
    ) -> Result<Vec<Field>> {
        numeric_histogram_bigint_real_state_fields(name, value_type, ordering_fields)
    }

    fn accumulator(&self, acc_args: AccumulatorArgs) -> Result<Box<dyn Accumulator>> {
        numeric_histogram_bigint_real_accumulator(acc_args)
    }
}

#[derive(Debug)]
pub(super) struct numeric_histogram_bigint_real_doubleFunc {
    signature: Signature,
}

impl numeric_histogram_bigint_real_doubleFunc {
    pub fn new() -> Self {        
        Self {
            signature: Signature::any(3, Volatility::Immutable),
        }
    }
}

impl AggregateUDFImpl for numeric_histogram_bigint_real_doubleFunc {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "numeric_histogram"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }


    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        numeric_histogram_bigint_real_double_return_type(arg_types)
    }

    fn state_fields(
        &self,
        name: &str,
        value_type: DataType,
        ordering_fields: Vec<Field>,
    ) -> Result<Vec<Field>> {
        numeric_histogram_bigint_real_double_state_fields(name, value_type, ordering_fields)
    }

    fn accumulator(&self, acc_args: AccumulatorArgs) -> Result<Box<dyn Accumulator>> {
        numeric_histogram_bigint_real_double_accumulator(acc_args)
    }
}

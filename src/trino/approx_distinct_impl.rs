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




fn approx_distinct_boolean_return_type(_arg_types: &[DataType]) -> Result<DataType> {
    Err(DataFusionError::NotImplemented(format!("Not implemented {}:{}", file!(), line!())))
}

fn approx_distinct_boolean_state_fields(_name: &str, _value_type: DataType, _ordering_fields: Vec<Field>) -> Result<Vec<Field>> {
    Err(DataFusionError::NotImplemented(format!("Not implemented {}:{}", file!(), line!())))
}

fn approx_distinct_boolean_accumulator(_acc_args: AccumulatorArgs) -> Result<Box<dyn Accumulator>> {
    Err(DataFusionError::NotImplemented(format!("Not implemented {}:{}", file!(), line!())))
}

#[derive(Debug)]
pub struct approx_distinct_boolean_Accumulator {}

impl approx_distinct_boolean_Accumulator {
    pub fn _try_new(_s_type: StatsType) -> Result<Self> {
        Err(DataFusionError::NotImplemented(format!("Not implemented {}:{}", file!(), line!())))
    }
}

impl Accumulator for approx_distinct_boolean_Accumulator {
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



fn approx_distinct_boolean_double_return_type(_arg_types: &[DataType]) -> Result<DataType> {
    Err(DataFusionError::NotImplemented(format!("Not implemented {}:{}", file!(), line!())))
}

fn approx_distinct_boolean_double_state_fields(_name: &str, _value_type: DataType, _ordering_fields: Vec<Field>) -> Result<Vec<Field>> {
    Err(DataFusionError::NotImplemented(format!("Not implemented {}:{}", file!(), line!())))
}

fn approx_distinct_boolean_double_accumulator(_acc_args: AccumulatorArgs) -> Result<Box<dyn Accumulator>> {
    Err(DataFusionError::NotImplemented(format!("Not implemented {}:{}", file!(), line!())))
}

#[derive(Debug)]
pub struct approx_distinct_boolean_double_Accumulator {}

impl approx_distinct_boolean_double_Accumulator {
    pub fn _try_new(_s_type: StatsType) -> Result<Self> {
        Err(DataFusionError::NotImplemented(format!("Not implemented {}:{}", file!(), line!())))
    }
}

impl Accumulator for approx_distinct_boolean_double_Accumulator {
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



fn approx_distinct_1_return_type(_arg_types: &[DataType]) -> Result<DataType> {
    Err(DataFusionError::NotImplemented(format!("Not implemented {}:{}", file!(), line!())))
}

fn approx_distinct_1_state_fields(_name: &str, _value_type: DataType, _ordering_fields: Vec<Field>) -> Result<Vec<Field>> {
    Err(DataFusionError::NotImplemented(format!("Not implemented {}:{}", file!(), line!())))
}

fn approx_distinct_1_accumulator(_acc_args: AccumulatorArgs) -> Result<Box<dyn Accumulator>> {
    Err(DataFusionError::NotImplemented(format!("Not implemented {}:{}", file!(), line!())))
}

#[derive(Debug)]
pub struct approx_distinct_1_Accumulator {}

impl approx_distinct_1_Accumulator {
    pub fn _try_new(_s_type: StatsType) -> Result<Self> {
        Err(DataFusionError::NotImplemented(format!("Not implemented {}:{}", file!(), line!())))
    }
}

impl Accumulator for approx_distinct_1_Accumulator {
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



fn approx_distinct_1_double_return_type(_arg_types: &[DataType]) -> Result<DataType> {
    Err(DataFusionError::NotImplemented(format!("Not implemented {}:{}", file!(), line!())))
}

fn approx_distinct_1_double_state_fields(_name: &str, _value_type: DataType, _ordering_fields: Vec<Field>) -> Result<Vec<Field>> {
    Err(DataFusionError::NotImplemented(format!("Not implemented {}:{}", file!(), line!())))
}

fn approx_distinct_1_double_accumulator(_acc_args: AccumulatorArgs) -> Result<Box<dyn Accumulator>> {
    Err(DataFusionError::NotImplemented(format!("Not implemented {}:{}", file!(), line!())))
}

#[derive(Debug)]
pub struct approx_distinct_1_double_Accumulator {}

impl approx_distinct_1_double_Accumulator {
    pub fn _try_new(_s_type: StatsType) -> Result<Self> {
        Err(DataFusionError::NotImplemented(format!("Not implemented {}:{}", file!(), line!())))
    }
}

impl Accumulator for approx_distinct_1_double_Accumulator {
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



fn approx_distinct_unknown_return_type(_arg_types: &[DataType]) -> Result<DataType> {
    Err(DataFusionError::NotImplemented(format!("Not implemented {}:{}", file!(), line!())))
}

fn approx_distinct_unknown_state_fields(_name: &str, _value_type: DataType, _ordering_fields: Vec<Field>) -> Result<Vec<Field>> {
    Err(DataFusionError::NotImplemented(format!("Not implemented {}:{}", file!(), line!())))
}

fn approx_distinct_unknown_accumulator(_acc_args: AccumulatorArgs) -> Result<Box<dyn Accumulator>> {
    Err(DataFusionError::NotImplemented(format!("Not implemented {}:{}", file!(), line!())))
}

#[derive(Debug)]
pub struct approx_distinct_unknown_Accumulator {}

impl approx_distinct_unknown_Accumulator {
    pub fn _try_new(_s_type: StatsType) -> Result<Self> {
        Err(DataFusionError::NotImplemented(format!("Not implemented {}:{}", file!(), line!())))
    }
}

impl Accumulator for approx_distinct_unknown_Accumulator {
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



fn approx_distinct_unknown_double_return_type(_arg_types: &[DataType]) -> Result<DataType> {
    Err(DataFusionError::NotImplemented(format!("Not implemented {}:{}", file!(), line!())))
}

fn approx_distinct_unknown_double_state_fields(_name: &str, _value_type: DataType, _ordering_fields: Vec<Field>) -> Result<Vec<Field>> {
    Err(DataFusionError::NotImplemented(format!("Not implemented {}:{}", file!(), line!())))
}

fn approx_distinct_unknown_double_accumulator(_acc_args: AccumulatorArgs) -> Result<Box<dyn Accumulator>> {
    Err(DataFusionError::NotImplemented(format!("Not implemented {}:{}", file!(), line!())))
}

#[derive(Debug)]
pub struct approx_distinct_unknown_double_Accumulator {}

impl approx_distinct_unknown_double_Accumulator {
    pub fn _try_new(_s_type: StatsType) -> Result<Self> {
        Err(DataFusionError::NotImplemented(format!("Not implemented {}:{}", file!(), line!())))
    }
}

impl Accumulator for approx_distinct_unknown_double_Accumulator {
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
pub(super) struct approx_distinct_booleanFunc {
    signature: Signature,
}

impl approx_distinct_booleanFunc {
    pub fn new() -> Self {        
        Self {
            signature: Signature::any(1, Volatility::Immutable),
        }
    }
}

impl AggregateUDFImpl for approx_distinct_booleanFunc {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "approx_distinct"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }


    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        approx_distinct_boolean_return_type(arg_types)
    }

    fn state_fields(
        &self,
        name: &str,
        value_type: DataType,
        ordering_fields: Vec<Field>,
    ) -> Result<Vec<Field>> {
        approx_distinct_boolean_state_fields(name, value_type, ordering_fields)
    }

    fn accumulator(&self, acc_args: AccumulatorArgs) -> Result<Box<dyn Accumulator>> {
        approx_distinct_boolean_accumulator(acc_args)
    }
}

#[derive(Debug)]
pub(super) struct approx_distinct_boolean_doubleFunc {
    signature: Signature,
}

impl approx_distinct_boolean_doubleFunc {
    pub fn new() -> Self {        
        Self {
            signature: Signature::any(2, Volatility::Immutable),
        }
    }
}

impl AggregateUDFImpl for approx_distinct_boolean_doubleFunc {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "approx_distinct"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }


    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        approx_distinct_boolean_double_return_type(arg_types)
    }

    fn state_fields(
        &self,
        name: &str,
        value_type: DataType,
        ordering_fields: Vec<Field>,
    ) -> Result<Vec<Field>> {
        approx_distinct_boolean_double_state_fields(name, value_type, ordering_fields)
    }

    fn accumulator(&self, acc_args: AccumulatorArgs) -> Result<Box<dyn Accumulator>> {
        approx_distinct_boolean_double_accumulator(acc_args)
    }
}

#[derive(Debug)]
pub(super) struct approx_distinct_1Func {
    signature: Signature,
}

impl approx_distinct_1Func {
    pub fn new() -> Self {        
        Self {
            signature: Signature::any(1, Volatility::Immutable),
        }
    }
}

impl AggregateUDFImpl for approx_distinct_1Func {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "approx_distinct"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }


    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        approx_distinct_1_return_type(arg_types)
    }

    fn state_fields(
        &self,
        name: &str,
        value_type: DataType,
        ordering_fields: Vec<Field>,
    ) -> Result<Vec<Field>> {
        approx_distinct_1_state_fields(name, value_type, ordering_fields)
    }

    fn accumulator(&self, acc_args: AccumulatorArgs) -> Result<Box<dyn Accumulator>> {
        approx_distinct_1_accumulator(acc_args)
    }
}

#[derive(Debug)]
pub(super) struct approx_distinct_1_doubleFunc {
    signature: Signature,
}

impl approx_distinct_1_doubleFunc {
    pub fn new() -> Self {        
        Self {
            signature: Signature::any(2, Volatility::Immutable),
        }
    }
}

impl AggregateUDFImpl for approx_distinct_1_doubleFunc {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "approx_distinct"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }


    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        approx_distinct_1_double_return_type(arg_types)
    }

    fn state_fields(
        &self,
        name: &str,
        value_type: DataType,
        ordering_fields: Vec<Field>,
    ) -> Result<Vec<Field>> {
        approx_distinct_1_double_state_fields(name, value_type, ordering_fields)
    }

    fn accumulator(&self, acc_args: AccumulatorArgs) -> Result<Box<dyn Accumulator>> {
        approx_distinct_1_double_accumulator(acc_args)
    }
}

#[derive(Debug)]
pub(super) struct approx_distinct_unknownFunc {
    signature: Signature,
}

impl approx_distinct_unknownFunc {
    pub fn new() -> Self {        
        Self {
            signature: Signature::any(1, Volatility::Immutable),
        }
    }
}

impl AggregateUDFImpl for approx_distinct_unknownFunc {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "approx_distinct"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }


    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        approx_distinct_unknown_return_type(arg_types)
    }

    fn state_fields(
        &self,
        name: &str,
        value_type: DataType,
        ordering_fields: Vec<Field>,
    ) -> Result<Vec<Field>> {
        approx_distinct_unknown_state_fields(name, value_type, ordering_fields)
    }

    fn accumulator(&self, acc_args: AccumulatorArgs) -> Result<Box<dyn Accumulator>> {
        approx_distinct_unknown_accumulator(acc_args)
    }
}

#[derive(Debug)]
pub(super) struct approx_distinct_unknown_doubleFunc {
    signature: Signature,
}

impl approx_distinct_unknown_doubleFunc {
    pub fn new() -> Self {        
        Self {
            signature: Signature::any(2, Volatility::Immutable),
        }
    }
}

impl AggregateUDFImpl for approx_distinct_unknown_doubleFunc {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "approx_distinct"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }


    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        approx_distinct_unknown_double_return_type(arg_types)
    }

    fn state_fields(
        &self,
        name: &str,
        value_type: DataType,
        ordering_fields: Vec<Field>,
    ) -> Result<Vec<Field>> {
        approx_distinct_unknown_double_state_fields(name, value_type, ordering_fields)
    }

    fn accumulator(&self, acc_args: AccumulatorArgs) -> Result<Box<dyn Accumulator>> {
        approx_distinct_unknown_double_accumulator(acc_args)
    }
}

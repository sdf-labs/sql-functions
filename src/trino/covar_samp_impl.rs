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
use arrow::array::{ArrayRef, Float64Array, UInt64Array};
use arrow::datatypes::{DataType, Field};
use datafusion::common::{downcast_value, unwrap_or_internal_err, Result};
use datafusion::error::DataFusionError;
use datafusion::logical_expr::function::AccumulatorArgs;
use datafusion::logical_expr::utils::format_state_name;
use datafusion::logical_expr::{Accumulator, AggregateUDFImpl, Signature, Volatility};
use datafusion::physical_plan::expressions::StatsType;
use datafusion::scalar::ScalarValue;
use std::any::Any;

use std::fmt::Debug;

use arrow::compute::kernels::cast;

fn covar_samp_double_double_return_type(_arg_types: &[DataType]) -> Result<DataType> {
    Ok(DataType::Float64)
}

fn covar_samp_double_double_state_fields(
    name: &str,
    _value_type: DataType,
    _ordering_fields: Vec<Field>,
) -> Result<Vec<Field>> {
    Ok(vec![
        Field::new(format_state_name(name, "count"), DataType::UInt64, true),
        Field::new(format_state_name(name, "mean1"), DataType::Float64, true),
        Field::new(format_state_name(name, "mean2"), DataType::Float64, true),
        Field::new(
            format_state_name(name, "algo_const"),
            DataType::Float64,
            true,
        ),
    ])
}

fn covar_samp_double_double_accumulator(
    _acc_args: AccumulatorArgs,
) -> Result<Box<dyn Accumulator>> {
    Ok(Box::new(CovarianceAccumulator::try_new(StatsType::Sample)?))
}

fn covar_samp_real_real_return_type(_arg_types: &[DataType]) -> Result<DataType> {
    Err(DataFusionError::NotImplemented(format!(
        "Not implemented {}:{}",
        file!(),
        line!()
    )))

}

fn covar_samp_real_real_state_fields(
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

fn covar_samp_real_real_accumulator(_acc_args: AccumulatorArgs) -> Result<Box<dyn Accumulator>> {
    Err(DataFusionError::NotImplemented(format!(
        "Not implemented {}:{}",
        file!(),
        line!()
    )))
}

#[derive(Debug)]
pub struct CovarianceAccumulator {
    algo_const: f64,
    mean1: f64,
    mean2: f64,
    count: u64,
    stats_type: StatsType,
}

impl CovarianceAccumulator {
    /// Creates a new `CovarianceAccumulator`
    pub fn try_new(s_type: StatsType) -> Result<Self> {
        Ok(Self {
            algo_const: 0_f64,
            mean1: 0_f64,
            mean2: 0_f64,
            count: 0_u64,
            stats_type: s_type,
        })
    }

}

impl Accumulator for CovarianceAccumulator {
    fn state(&mut self) -> Result<Vec<ScalarValue>> {
        Ok(vec![
            ScalarValue::from(self.count),
            ScalarValue::from(self.mean1),
            ScalarValue::from(self.mean2),
            ScalarValue::from(self.algo_const),
        ])
    }

    fn update_batch(&mut self, values: &[ArrayRef]) -> Result<()> {
        let values1 = &cast(&values[0], &DataType::Float64)?;
        let values2 = &cast(&values[1], &DataType::Float64)?;

        let mut arr1 = downcast_value!(values1, Float64Array).iter().flatten();
        let mut arr2 = downcast_value!(values2, Float64Array).iter().flatten();

        for i in 0..values1.len() {
            let value1 = if values1.is_valid(i) {
                arr1.next()
            } else {
                None
            };
            let value2 = if values2.is_valid(i) {
                arr2.next()
            } else {
                None
            };

            if value1.is_none() || value2.is_none() {
                continue;
            }

            let value1 = unwrap_or_internal_err!(value1);
            let value2 = unwrap_or_internal_err!(value2);
            let new_count = self.count + 1;
            let delta1 = value1 - self.mean1;
            let new_mean1 = delta1 / new_count as f64 + self.mean1;
            let delta2 = value2 - self.mean2;
            let new_mean2 = delta2 / new_count as f64 + self.mean2;
            let new_c = delta1 * (value2 - new_mean2) + self.algo_const;

            self.count += 1;
            self.mean1 = new_mean1;
            self.mean2 = new_mean2;
            self.algo_const = new_c;
        }

        Ok(())
    }

    fn retract_batch(&mut self, values: &[ArrayRef]) -> Result<()> {
        let values1 = &cast(&values[0], &DataType::Float64)?;
        let values2 = &cast(&values[1], &DataType::Float64)?;
        let mut arr1 = downcast_value!(values1, Float64Array).iter().flatten();
        let mut arr2 = downcast_value!(values2, Float64Array).iter().flatten();

        for i in 0..values1.len() {
            let value1 = if values1.is_valid(i) {
                arr1.next()
            } else {
                None
            };
            let value2 = if values2.is_valid(i) {
                arr2.next()
            } else {
                None
            };

            if value1.is_none() || value2.is_none() {
                continue;
            }

            let value1 = unwrap_or_internal_err!(value1);
            let value2 = unwrap_or_internal_err!(value2);

            let new_count = self.count - 1;
            let delta1 = self.mean1 - value1;
            let new_mean1 = delta1 / new_count as f64 + self.mean1;
            let delta2 = self.mean2 - value2;
            let new_mean2 = delta2 / new_count as f64 + self.mean2;
            let new_c = self.algo_const - delta1 * (new_mean2 - value2);

            self.count -= 1;
            self.mean1 = new_mean1;
            self.mean2 = new_mean2;
            self.algo_const = new_c;
        }

        Ok(())
    }

    fn merge_batch(&mut self, states: &[ArrayRef]) -> Result<()> {
        let counts = downcast_value!(states[0], UInt64Array);
        let means1 = downcast_value!(states[1], Float64Array);
        let means2 = downcast_value!(states[2], Float64Array);
        let cs = downcast_value!(states[3], Float64Array);

        for i in 0..counts.len() {
            let c = counts.value(i);
            if c == 0_u64 {
                continue;
            }
            let new_count = self.count + c;
            let new_mean1 = self.mean1 * self.count as f64 / new_count as f64
                + means1.value(i) * c as f64 / new_count as f64;
            let new_mean2 = self.mean2 * self.count as f64 / new_count as f64
                + means2.value(i) * c as f64 / new_count as f64;
            let delta1 = self.mean1 - means1.value(i);
            let delta2 = self.mean2 - means2.value(i);
            let new_c = self.algo_const
                + cs.value(i)
                + delta1 * delta2 * self.count as f64 * c as f64 / new_count as f64;

            self.count = new_count;
            self.mean1 = new_mean1;
            self.mean2 = new_mean2;
            self.algo_const = new_c;
        }
        Ok(())
    }

    fn evaluate(&mut self) -> Result<ScalarValue> {
        let count = match self.stats_type {
            StatsType::Population => self.count,
            StatsType::Sample => {
                if self.count > 0 {
                    self.count - 1
                } else {
                    self.count
                }
            }
        };

        if count == 0 {
            Ok(ScalarValue::Float64(None))
        } else {
            Ok(ScalarValue::Float64(Some(self.algo_const / count as f64)))
        }
    }

    fn size(&self) -> usize {
        std::mem::size_of_val(self)
    }
}

// ========== Generated template below this line ==========
// Do *NOT* edit below this line: all changes will be overwritten
// when template is regenerated!

#[derive(Debug)]
pub(super) struct covar_samp_double_doubleFunc {
    signature: Signature,
}

impl covar_samp_double_doubleFunc {
    pub fn new() -> Self {
        Self {
            signature: Signature::any(2, Volatility::Immutable),
        }
    }
}

impl AggregateUDFImpl for covar_samp_double_doubleFunc {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "covar_samp"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }

    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        covar_samp_double_double_return_type(arg_types)
    }

    fn state_fields(
        &self,
        name: &str,
        value_type: DataType,
        ordering_fields: Vec<Field>,
    ) -> Result<Vec<Field>> {
        covar_samp_double_double_state_fields(name, value_type, ordering_fields)
    }

    fn accumulator(&self, acc_args: AccumulatorArgs) -> Result<Box<dyn Accumulator>> {
        covar_samp_double_double_accumulator(acc_args)
    }
}

#[derive(Debug)]
pub(super) struct covar_samp_real_realFunc {
    signature: Signature,
}

impl covar_samp_real_realFunc {
    pub fn new() -> Self {
        Self {
            signature: Signature::any(2, Volatility::Immutable),
        }
    }
}

impl AggregateUDFImpl for covar_samp_real_realFunc {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn name(&self) -> &str {
        "covar_samp"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }

    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        covar_samp_real_real_return_type(arg_types)
    }

    fn state_fields(
        &self,
        name: &str,
        value_type: DataType,
        ordering_fields: Vec<Field>,
    ) -> Result<Vec<Field>> {
        covar_samp_real_real_state_fields(name, value_type, ordering_fields)
    }

    fn accumulator(&self, acc_args: AccumulatorArgs) -> Result<Box<dyn Accumulator>> {
        covar_samp_real_real_accumulator(acc_args)
    }
}

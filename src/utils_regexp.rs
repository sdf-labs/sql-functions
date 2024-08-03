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

//! Helper functors for creating Regexp-based UDFs.
//! As with UDF functors in general, the idea is to have a generic implementation of
//! vectorization machinery that can accept a simple function defined over fields of a row
//! and map it over `ColumnarValue` inputs.
//! The functors here address Regexp specifics as follows:
//! - The case of the pattern column being a scalar is recognized and optimized for by compiling the
//!   pattern only once. (This case corresponds to a literal-string pattern in an SQL query.)
//! - Row functions are curried, with the first argument being a string for the regexp pattern.
//!   (This is different from the argument order in UDFs, but is better suited the regexp pre-compiling.)
//! - The pattern column (the first column) is assumed to be an SDF "distinct" type `joniregexp`
//!   and is handled suitably to extract the underlying string array.
//! Functor names are of the form map_rowfun__aaa_bbb_ccc_to_zzz,
//! where aaa, bbb, ccc, ... (up to `to`) indicate the types of the columns and of the row function's arguments,
//! while zzz indicates the output type.

#![allow(non_snake_case)]
use arrow::array::{Array, ArrayRef, Int64Array};
use datafusion::common::cast::as_string_array;
use datafusion::common::Result;
use datafusion::logical_expr::ColumnarValue;
use std::sync::Arc;

use crate::utils::{array_to_columnar, distinct_to_string_array};

/// Map a curried row function that accepts a pattern and a string and returns an i64
/// over two columns, of jonigexp and haystack/string.  
pub(super) fn map_rowfun__pat_hay_to_i64<F>(
    joni_col: &ColumnarValue,
    hay_col: &ColumnarValue,
    rowfun: Arc<F>,
) -> Result<ColumnarValue>
where
    F: (Fn(/*pat:*/ &str) -> Result<Arc<dyn Fn(/*hay:*/ &str) -> i64>>) + Sync + Send + 'static,
{
    let res_arr = match joni_col {
        ColumnarValue::Array(joni_arr) => {
            let hay_arr = hay_col.to_owned().into_array(joni_arr.len())?;
            let pattern = distinct_to_string_array(joni_arr)?;
            let haystack = as_string_array(&hay_arr)?;
            let res = pattern
                .iter()
                .zip(haystack.iter())
                .map(|(pat_opt, hay_opt)| match (pat_opt, hay_opt) {
                    (Some(pat), Some(hay)) => {
                        let regfun = rowfun(pat)?;
                        Ok(Some(regfun(hay)))
                    }
                    _ => Ok(None),
                })
                .collect::<Result<Int64Array>>()?;
            Arc::new(res) as ArrayRef
        }
        ColumnarValue::Scalar(joni_scalar) => {
            let hay_arr = hay_col.to_owned().into_array(1)?; // NB: 1 only triggers when hay_col is Scalar
            let joni_arr = joni_scalar.to_array()?;
            let pat_arr = distinct_to_string_array(&joni_arr)?;
            if pat_arr.is_null(0) {
                arrow::array::new_null_array(hay_arr.data_type(), hay_arr.len())
            } else {
                let pat = pat_arr.value(0);
                let regfun = rowfun(pat)?;
                let haystack = as_string_array(&hay_arr)?;
                let res = haystack
                    .iter()
                    .map(|hay_opt| match hay_opt {
                        Some(hay) => {
                            // core action
                            Some(regfun(hay))
                        }
                        _ => None,
                    })
                    .collect::<Int64Array>();
                Arc::new(res) as ArrayRef
            }
        }
    };
    Ok(array_to_columnar(res_arr))
}

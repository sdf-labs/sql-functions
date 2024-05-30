use std::{collections::HashMap, sync::Arc};

use datafusion::logical_expr::ScalarUDF;
use rust_embed::RustEmbed;

#[macro_use]
pub mod macros;

make_package!(presto, "presto_expressions", "Presto functions.");

/// Fluent-style API for creating `Expr`s
pub mod expr_fn {
    pub use super::presto::expr_fn::*;
}

/// Registers all enabled packages with a [`FunctionRegistry`]
pub fn get_all_functions() -> Vec<(String, Arc<ScalarUDF>)> {
    presto::functions()
}


#[derive(RustEmbed)]
#[folder = "assets/"]
pub struct Asset;

impl Asset {
    pub fn load_raw_ymls() -> HashMap<String, Vec<String>> {
        let mut result = HashMap::new();
        for dialect in vec!["presto", "bigquery", "snowflake", "redshift"] {
            let mut raw_ymls = vec![];
            for filename in ["functions.sdf.yml", "functions_extra.sdf.yml"] {
                let filename = format!("{}/{}", dialect, filename);

                let asset = Self::get(filename.as_ref());
                if asset.is_some() {
                    let asset = asset.unwrap();

                    let input = std::str::from_utf8(&asset.data)
                        .unwrap_or_else(|_| panic!("{}:: corrupted asset: non UTF-8", filename));

                    raw_ymls.push(input.to_owned());
                }
            }
            result.insert(dialect.to_string(), raw_ymls);
        }
        result
    }
}
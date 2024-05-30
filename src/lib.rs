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
    pub fn load_file(filename: &str) -> Option<String> {
        let asset = Self::get(filename.as_ref());
        if let Some(asset) = asset {
            let input = std::str::from_utf8(&asset.data)
                .unwrap_or_else(|_| panic!("{}:: corrupted asset: non UTF-8", filename));

            Some(input.to_string())
        } else {
            None
        }
    }
}
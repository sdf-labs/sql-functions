use std::sync::Arc;

use datafusion::logical_expr::ScalarUDF;
use rust_embed::{EmbeddedFile, RustEmbed};

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
    pub fn load_file(filename: &str) -> Option<EmbeddedFile> {
        Self::get(filename.as_ref())
    }
}

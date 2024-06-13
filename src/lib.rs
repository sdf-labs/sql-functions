use std::sync::Arc;

use datafusion::logical_expr::{AggregateUDF, ScalarUDF};
use rust_embed::{EmbeddedFile, RustEmbed};

#[macro_use]
pub mod macros;

make_package!(trino, "trino_expressions", "trino functions.");


/// Registers all enabled packages with a [`FunctionRegistry`]
pub fn get_all_functions() -> Vec<(String, Arc<ScalarUDF>)> {
    trino::udfs()
}

pub fn get_all_udafs() -> Vec<(String, Arc<AggregateUDF>)> {
    trino::udafs()
}

#[derive(RustEmbed)]
#[folder = "assets/"]
pub struct Asset;

impl Asset {
    pub fn load_file(filename: &str) -> Option<EmbeddedFile> {
        Self::get(filename.as_ref())
    }
}

mod utils;

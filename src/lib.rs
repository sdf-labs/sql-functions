use std::sync::Arc;

use datafusion::logical_expr::ScalarUDF;

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

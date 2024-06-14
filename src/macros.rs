macro_rules! export_functions {
    ($(($DIALECT:ident, $FUNC:ident,  $($arg:ident)*, $DOC:expr)),* $(,)?) => {
        pub mod expr_fn {
            $(
                #[doc = $DOC]
                /// Return $name(arg)
                pub fn $FUNC($($arg: datafusion::logical_expr::Expr),*) -> datafusion::logical_expr::Expr {
                    super::$FUNC().call(vec![$($arg),*],)
                }
            )*
        }

        /// Return a list of all functions in this package
        pub fn functions() -> Vec<(String, std::sync::Arc<datafusion::logical_expr::ScalarUDF>)> {

            vec![
                $(
                    (format!("{}::{}", stringify!($DIALECT), stringify!($FUNC)), $FUNC()),
                )*
            ]

        }
    };
}

/// Creates a singleton `ScalarUDF` of the `$UDF` function named `$GNAME` and a
/// function named `$NAME` which returns that function named $NAME.
///
/// This is used to ensure creating the list of `ScalarUDF` only happens once.
macro_rules! make_udf_function {
    ($UDF:ty, $GNAME:ident, $NAME:ident) => {
        /// Singleton instance of the function
        static $GNAME: std::sync::OnceLock<std::sync::Arc<datafusion::logical_expr::ScalarUDF>> =
            std::sync::OnceLock::new();

        /// Return a [`ScalarUDF`] for [`$UDF`]
        ///
        /// [`ScalarUDF`]: datafusion::logical_expr::ScalarUDF
        fn $NAME() -> std::sync::Arc<datafusion::logical_expr::ScalarUDF> {
            $GNAME
                .get_or_init(|| {
                    std::sync::Arc::new(datafusion::logical_expr::ScalarUDF::new_from_impl(
                        <$UDF>::new(),
                    ))
                })
                .clone()
        }
    };
}

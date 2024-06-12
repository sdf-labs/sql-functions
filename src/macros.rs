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

/// Macro creates the named module if the feature is enabled
/// otherwise creates a stub
///
/// Which returns:
///
/// 1. The list of actual function implementation when the relevant
/// feature is activated,
///
/// 2. A list of stub function when the feature is not activated that produce
/// a runtime error (and explain what feature flag is needed to activate them).
///
/// The rationale for providing stub functions is to help users to configure datafusion
/// properly (so they get an error telling them why a function is not available)
/// instead of getting a cryptic "no function found" message at runtime.

macro_rules! make_package {
    ($name:ident, $feature:literal, $DOC:expr) => {
        #[cfg(feature = $feature)]
        #[doc = $DOC ]
        #[doc = concat!("Enabled via feature flag `", $feature, "`")]
        pub mod $name;

        #[cfg(not(feature = $feature))]
        #[doc = concat!("Disabled. Enable via feature flag `", $feature, "`")]
        pub mod $name {
            use datafusion::logical_expr::ScalarUDF;
            use log::debug;
            use std::sync::Arc;

            /// Returns an empty list of functions when the feature is not enabled
            pub fn functions() -> Vec<Arc<ScalarUDF>> {
                debug!("{} functions disabled", stringify!($name));
                vec![]
            }
        }
    };
}

macro_rules! make_udaf_expr_and_func {
    ($UDAF:ty, $EXPR_FN:ident, $($arg:ident)*, $DOC:expr, $AGGREGATE_UDF_FN:ident) => {
        // "fluent expr_fn" style function
        #[doc = $DOC]
        pub fn $EXPR_FN(
            $($arg: datafusion::logical_expr::Expr,)*
            distinct: bool,
            filter: Option<Box<datafusion::logical_expr::Expr>>,
            order_by: Option<Vec<datafusion::logical_expr::Expr>>
        ) -> datafusion::logical_expr::Expr {
            datafusion::logical_expr::Expr::AggregateFunction(datafusion::logical_expr::expr::AggregateFunction::new_udf(
                $AGGREGATE_UDF_FN(),
                vec![$($arg),*],
                distinct,
                filter,
                order_by,
                None,
            ))
        }
        create_func!($UDAF, $AGGREGATE_UDF_FN);
    };
    ($UDAF:ty, $EXPR_FN:ident, $DOC:expr, $AGGREGATE_UDF_FN:ident) => {
        // "fluent expr_fn" style function
        #[doc = $DOC]
        pub fn $EXPR_FN(
            args: Vec<datafusion::logical_expr::Expr>,
            distinct: bool,
            filter: Option<Box<datafusion::logical_expr::Expr>>,
            order_by: Option<Vec<datafusion::logical_expr::Expr>>,
            null_treatment: Option<sqlparser::ast::NullTreatment>
        ) -> datafusion::logical_expr::Expr {
            datafusion::logical_expr::Expr::AggregateFunction(datafusion::logical_expr::expr::AggregateFunction::new_udf(
                $AGGREGATE_UDF_FN(),
                args,
                distinct,
                filter,
                order_by,
                null_treatment,
            ))
        }
        create_func!($UDAF, $AGGREGATE_UDF_FN);
    };
}

macro_rules! create_func {
    ($UDAF:ty, $AGGREGATE_UDF_FN:ident) => {
        paste::paste! {
            /// Singleton instance of [$UDAF], ensures the UDAF is only created once
            /// named STATIC_$(UDAF). For example `STATIC_FirstValue`
            #[allow(non_upper_case_globals)]
            static [< STATIC_ $UDAF >]: std::sync::OnceLock<std::sync::Arc<datafusion::logical_expr::AggregateUDF>> =
            std::sync::OnceLock::new();

            /// AggregateFunction that returns a [AggregateUDF] for [$UDAF]
            ///
            /// [AggregateUDF]: datafusion::logical_expr::AggregateUDF
            pub fn $AGGREGATE_UDF_FN() -> std::sync::Arc<datafusion::logical_expr::AggregateUDF> {
                [< STATIC_ $UDAF >]
                    .get_or_init(|| {
                        std::sync::Arc::new(datafusion::logical_expr::AggregateUDF::from(<$UDAF>::default()))
                    })
                    .clone()
            }
        }
    }
}

/// Creates a singleton `ScalarUDF` of the `$UDF` function named `$GNAME` and a
/// function named `$NAME` which returns that function named $NAME.
///
/// This is used to ensure creating the list of `ScalarUDF` only happens once.
macro_rules! create_udf {
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

macro_rules! create_udaf {
    ($UDF:ty, $GNAME:ident, $NAME:ident) => {
        /// Singleton instance of the function
        static $GNAME: std::sync::OnceLock<std::sync::Arc<datafusion::logical_expr::AggregateUDF>> =
            std::sync::OnceLock::new();

        /// Return a [`ScalarUDF`] for [`$UDF`]
        ///
        /// [`ScalarUDF`]: datafusion::logical_expr::ScalarUDF
        fn $NAME() -> std::sync::Arc<datafusion::logical_expr::AggregateUDF> {
            $GNAME
                .get_or_init(|| {
                    std::sync::Arc::new(datafusion::logical_expr::AggregateUDF::new_from_impl(
                        <$UDF>::new(),
                    ))
                })
                .clone()
        }
    };
}

mod nans;

// create  UDFs
make_udf_function!(nans::IsNanFunc, ISNAN, isnan1);

pub use datafusion::functions::math::abs;
pub use datafusion::functions::math::acos;
pub use datafusion::functions::math::round;

// Export the functions out of this package, both as expr_fn as well as a list of functions
export_functions!(
    (
        presto,
        isnan1,
        num,
        "returns true if a given number is +NaN or -NaN otherwise returns false"
    ),
    (presto, abs, num, "TODO: abs doc comment"),
    (presto, acos, num, "TODO: acos doc comment"),
    (presto, round, num, "TODO: round doc comment")
);

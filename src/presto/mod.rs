mod nans;

// create  UDFs
make_udf_function!(nans::IsNanFunc, ISNAN, isnan1);

pub use datafusion::functions::math::abs;
pub use datafusion::functions::math::acos;
pub use datafusion::functions::math::round;

pub use datafusion::functions::unicode::character_length as length;

pub use datafusion::functions_array::length::array_length_udf as length_array;

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
    (presto, round, num, "TODO: round doc comment"),
    (presto, length_array, arr, "TODO: length_array doc comment"),
    (presto, length, str, "TODO: length(varchar) doc comment")
);

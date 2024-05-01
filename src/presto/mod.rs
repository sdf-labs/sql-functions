
mod nans;

// create  UDFs
make_udf_function!(nans::IsNanFunc, ISNAN, isnan1);

// Export the functions out of this package, both as expr_fn as well as a list of functions
export_functions!(
    (presto, isnan1, num, "returns true if a given number is +NaN or -NaN otherwise returns false")
);

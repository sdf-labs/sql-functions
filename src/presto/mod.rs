#![allow(non_snake_case)]
mod day_date;
mod month_date;
mod to_hex_varbinary;
mod year_date;


// create  UDFs
make_udf_function!(day_date::Func, DAY_DATE, day_date);
make_udf_function!(month_date::Func, MONTH_DATE, month_date);
make_udf_function!(to_hex_varbinary::Func, TO_HEX_VARBINARY, to_hex_varbinary);
make_udf_function!(year_date::Func, YEAR_DATE, year_date);


// Export the functions out of this package, both as expr_fn as well as a list of functions
export_functions!(
    (presto, day_date, arg1, "function doc"),
    (presto, month_date, arg1, "function doc"),
    (presto, to_hex_varbinary, arg1, "function doc"),
    (presto, year_date, arg1, "function doc"),
);
#![allow(non_snake_case)]
mod date_diff_varchar_date_date;
mod date_diff_varchar_time_p_time_p;
mod date_diff_varchar_timestamp_p_timestamp_p;
mod day_date;
mod greatest_3;
mod least_3;
mod month_date;
mod regexp_extract_varchar_joniregexp;
mod regexp_like_varchar_joniregexp;
mod regexp_replace_varchar_joniregexp;
mod regexp_replace_varchar_joniregexp_varchar;
mod to_hex_varbinary;
mod year_date;
mod if_boolean_1_1;
mod current_timestamp;


// create  UDFs
make_udf_function!(date_diff_varchar_date_date::Func, DATE_DIFF_VARCHAR_DATE_DATE, date_diff_varchar_date_date);
make_udf_function!(date_diff_varchar_time_p_time_p::Func, DATE_DIFF_VARCHAR_TIME_P_TIME_P, date_diff_varchar_time_p_time_p);
make_udf_function!(date_diff_varchar_timestamp_p_timestamp_p::Func, DATE_DIFF_VARCHAR_TIMESTAMP_P_TIMESTAMP_P, date_diff_varchar_timestamp_p_timestamp_p);
make_udf_function!(day_date::Func, DAY_DATE, day_date);
make_udf_function!(greatest_3::Func, GREATEST_3, greatest_3);
make_udf_function!(least_3::Func, LEAST_3, least_3);
make_udf_function!(month_date::Func, MONTH_DATE, month_date);
make_udf_function!(regexp_extract_varchar_joniregexp::Func, REGEXP_EXTRACT_VARCHAR_JONIREGEXP, regexp_extract_varchar_joniregexp);
make_udf_function!(regexp_like_varchar_joniregexp::Func, REGEXP_LIKE_VARCHAR_JONIREGEXP, regexp_like_varchar_joniregexp);
make_udf_function!(regexp_replace_varchar_joniregexp::Func, REGEXP_REPLACE_VARCHAR_JONIREGEXP, regexp_replace_varchar_joniregexp);
make_udf_function!(regexp_replace_varchar_joniregexp_varchar::Func, REGEXP_REPLACE_VARCHAR_JONIREGEXP_VARCHAR, regexp_replace_varchar_joniregexp_varchar);
make_udf_function!(to_hex_varbinary::Func, TO_HEX_VARBINARY, to_hex_varbinary);
make_udf_function!(year_date::Func, YEAR_DATE, year_date);
make_udf_function!(if_boolean_1_1::Func, IF_BOOLEAN_1_1, if_boolean_1_1);
make_udf_function!(current_timestamp::Func, CURRENT_TIMESTAMP, current_timestamp);


// Export the functions out of this package, both as expr_fn as well as a list of functions
export_functions!(
    (presto, date_diff_varchar_date_date, arg1 arg2 arg3, "function doc"),
    (presto, date_diff_varchar_time_p_time_p, arg1 arg2 arg3, "function doc"),
    (presto, date_diff_varchar_timestamp_p_timestamp_p, arg1 arg2 arg3, "function doc"),
    (presto, day_date, arg1, "function doc"),
    (presto, greatest_3, arg1, "function doc"),
    (presto, least_3, arg1, "function doc"),
    (presto, month_date, arg1, "function doc"),
    (presto, regexp_extract_varchar_joniregexp, arg1 arg2, "function doc"),
    (presto, regexp_like_varchar_joniregexp, arg1 arg2, "function doc"),
    (presto, regexp_replace_varchar_joniregexp, arg1 arg2, "function doc"),
    (presto, regexp_replace_varchar_joniregexp_varchar, arg1 arg2 arg3, "function doc"),
    (presto, to_hex_varbinary, arg1, "function doc"),
    (presto, year_date, arg1, "function doc"),
    (presto, if_boolean_1_1, arg1 arg2 arg3, "function doc"),
    (presto, current_timestamp, , "function doc"),
);
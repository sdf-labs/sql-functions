#![allow(non_snake_case)]
mod abs_tinyint;
mod abs_smallint;
mod abs_bigint;
mod abs_double;
mod abs_decimal_p_s;
mod abs_real;
mod acos_double;
mod all_match_array_1_function_1_boolean;
mod any_match_array_1_function_1_boolean;
mod array_distinct_array_3;
mod array_except_array_3_array_3;
mod array_intersect_array_3_array_3;
mod array_join_array_1_varchar;
mod array_join_array_1_varchar_varchar;
mod array_max_array_1;
mod array_min_array_1;
mod array_position_array_1_1;
mod array_remove_array_3_3;
mod array_sort_array_3;
mod array_sort_array_1_function_1_1_bigint;
mod array_union_array_3_array_3;
mod arrays_overlap_array_3_array_3;
mod asin_double;
mod at_timezone_timestamp_p_varchar;
mod atan_double;
mod atan2_double_double;
mod bar_double_bigint;
mod bar_double_bigint_color_color;
mod beta_cdf_double_double_double;
mod bing_tile_bigint_bigint_bigint;
mod bing_tile_varchar;
mod bing_tile_at_double_double_bigint;
mod bing_tile_coordinates_bingtile;
mod bing_tile_polygon_bingtile;
mod bing_tile_quadkey_bingtile;
mod bing_tile_zoom_level_bingtile;
mod bing_tiles_around_double_double_bigint;
mod bing_tiles_around_double_double_bigint_double;
mod bit_count_bigint_bigint;
mod bitwise_and_bigint_bigint;
mod bitwise_left_shift_bigint_bigint;
mod bitwise_left_shift_integer_bigint;
mod bitwise_left_shift_smallint_bigint;
mod bitwise_left_shift_tinyint_bigint;
mod bitwise_not_bigint;
mod bitwise_or_bigint_bigint;
mod bitwise_right_shift_bigint_bigint;
mod bitwise_right_shift_integer_bigint;
mod bitwise_right_shift_smallint_bigint;
mod bitwise_right_shift_tinyint_bigint;
mod bitwise_right_shift_arithmetic_bigint_bigint;
mod bitwise_right_shift_arithmetic_integer_bigint;
mod bitwise_right_shift_arithmetic_smallint_bigint;
mod bitwise_right_shift_arithmetic_tinyint_bigint;
mod bitwise_xor_bigint_bigint;
mod cardinality_array_3;
mod cardinality_hyperloglog;
mod cardinality_map_4_5;
mod cardinality_setdigest;
mod cbrt_double;
mod ceil_bigint;
mod ceil_decimal_p_s;
mod ceil_double;
mod ceil_integer;
mod ceil_real;
mod ceil_smallint;
mod ceil_tinyint;
mod ceiling_bigint;
mod ceiling_decimal_p_s;
mod ceiling_double;
mod ceiling_integer;
mod ceiling_real;
mod ceiling_smallint;
mod ceiling_tinyint;
mod char2hexint_varchar;
mod chr_bigint;
mod classify_map_bigint_double_classifier;
mod coalesce_1;
mod codepoint_varchar;
mod color_double_color_color;
mod color_double_double_double_color_color;
mod color_varchar;
mod combinations_array_1_bigint;
mod concat_3_array_3;
mod concat_array_3;
mod concat_array_3_3;
mod concat_varchar_varchar;
mod concat_varchar;
mod concat_varbinary;
mod concat_ws_varchar_array_varchar;
mod concat_ws_varchar;
mod contains_array_1_1;
mod contains_varchar_ipaddress;
mod contains_sequence_array_1_array_1;
mod cos_double;
mod cosh_double;
mod cosine_similarity_map_varchar_double_map_varchar_double;
mod crc32_varbinary;
mod current_date;
mod current_groups;
mod current_timezone;
mod date_timestamp_p;
mod date_varchar;
mod date_add_varchar_bigint_date;
mod date_add_varchar_bigint_time_p;
mod date_add_varchar_bigint_timestamp_p;
mod date_diff_varchar_date_date;
mod date_diff_varchar_time_p_time_p;
mod date_diff_varchar_timestamp_p_timestamp_p;
mod date_format_timestamp_p_varchar;
mod date_parse_varchar_varchar;
mod date_trunc_varchar_time_p;
mod date_trunc_varchar_timestamp_p;
mod date_trunc_varchar_date;
mod day_date;
mod day_intervaldaytosecond;
mod day_timestamp_p;
mod day_of_month_date;
mod day_of_month_intervaldaytosecond;
mod day_of_month_timestamp_p;
mod day_of_week_date;
mod day_of_week_timestamp_p;
mod day_of_year_date;
mod day_of_year_timestamp_p;
mod degrees_double;
mod dow_date;
mod dow_timestamp_p;
mod doy_date;
mod doy_timestamp_p;
mod e;
mod element_at_map_4_5_4;
mod element_at_array_3_bigint;
mod empty_approx_set;
mod exp_double;
mod features_double;
mod features_double_double;
mod features_double_double_double;
mod features_double_double_double_double;
mod features_double_double_double_double_double;
mod features_double_double_double_double_double_double;
mod features_double_double_double_double_double_double_double;
mod features_double_double_double_double_double_double_double_double;
mod features_double_double_double_double_double_double_double_double_double;
mod features_double_double_double_double_double_double_double_double_double_double;
mod filter_array_1_function_1_boolean;
mod flatten_array_array_3;
mod floor_bigint;
mod floor_decimal_p_s;
mod floor_double;
mod floor_integer;
mod floor_real;
mod floor_smallint;
mod floor_tinyint;
mod format_datetime_timestamp_p_varchar;
mod format_number_bigint;
mod format_number_double;
mod from_base_varchar_bigint;
mod from_base32_varbinary;
mod from_base32_varchar;
mod from_base64_varbinary;
mod from_base64_varchar;
mod from_base64url_varbinary;
mod from_base64url_varchar;
mod from_big_endian_32_varbinary;
mod from_big_endian_64_varbinary;
mod from_encoded_polyline_varchar;
mod from_geojson_geometry_varchar;
mod from_hex_varbinary;
mod from_hex_varchar;
mod from_ieee754_32_varbinary;
mod from_ieee754_64_varbinary;
mod from_iso8601_date_varchar;
mod from_iso8601_timestamp_varchar;
mod from_iso8601_timestamp_nanos_varchar;
mod from_unixtime_bigint;
mod from_unixtime_bigint_bigint_bigint;
mod from_unixtime_bigint_varchar;
mod from_unixtime_nanos_bigint;
mod from_unixtime_nanos_decimal_p_s;
mod from_utf8_varbinary;
mod from_utf8_varbinary_bigint;
mod from_utf8_varbinary_varchar;
mod geometry_from_hadoop_shape_varbinary;
mod geometry_invalid_reason_geometry;
mod geometry_nearest_points_geometry_geometry;
mod geometry_to_bing_tiles_geometry_bigint;
mod geometry_union_array_geometry;
mod great_circle_distance_double_double_double_double;
mod greatest_3;
mod hamming_distance_varchar_varchar;
mod hash_counts_setdigest;
mod hmac_md5_varbinary_varbinary;
mod hmac_sha1_varbinary_varbinary;
mod hmac_sha256_varbinary_varbinary;
mod hmac_sha512_varbinary_varbinary;
mod hour_intervaldaytosecond;
mod hour_time_p;
mod hour_timestamp_p;
mod human_readable_seconds_double;
mod index_varchar_varchar;
mod infinity;
mod intersection_cardinality_setdigest_setdigest;
mod inverse_beta_cdf_double_double_double;
mod inverse_normal_cdf_double_double_double;
mod is_finite_double;
mod is_infinite_double;
mod is_json_scalar_json;
mod is_json_scalar_varchar;
mod is_nan_double;
mod is_nan_real;
mod jaccard_index_setdigest_setdigest;
mod json_array_contains_json_bigint;
mod json_array_contains_json_boolean;
mod json_array_contains_json_double;
mod json_array_contains_json_varchar;
mod json_array_contains_varchar_bigint;
mod json_array_contains_varchar_boolean;
mod json_array_contains_varchar_double;
mod json_array_contains_varchar_varchar;
mod json_array_get_json_bigint;
mod json_array_get_varchar_bigint;
mod json_array_length_json;
mod json_array_length_varchar;
mod json_extract_json_jsonpath;
mod json_extract_varchar_jsonpath;
mod json_extract_scalar_json_jsonpath;
mod json_extract_scalar_varchar_jsonpath;
mod json_format_json;
mod json_parse_varchar;
mod json_size_json_jsonpath;
mod json_size_varchar_jsonpath;
mod last_day_of_month_date;
mod last_day_of_month_timestamp_p;
mod least_3;
mod length_varchar;
mod length_varbinary;
mod length_array_1;
mod levenshtein_distance_varchar_varchar;
mod line_interpolate_point_geometry_double;
mod line_interpolate_points_geometry_double;
mod line_locate_point_geometry_geometry;
mod ln_double;
mod log_double_double;
mod log10_double;
mod log2_double;
mod lower_varchar;
mod lpad_varbinary_bigint_varbinary;
mod lpad_varchar_bigint_varchar;
mod ltrim_varchar;
mod ltrim_varchar_codepoints;
mod luhn_check_varchar;
mod map_array_4_array_5;
mod map;
mod map_concat_map_4_5;
mod map_entries_map_4_5;
mod map_filter_map_4_5_function_4_5_boolean;
mod map_from_entries_array_row_c04_c15;
mod map_keys_map_4_5;
mod map_values_map_4_5;
mod map_zip_with_map_4_8_map_4_7_function_4_8_7_6;
mod md5_varbinary;
mod millisecond_intervaldaytosecond;
mod millisecond_time_p;
mod millisecond_timestamp_p;
mod minute_intervaldaytosecond;
mod minute_time_p;
mod minute_timestamp_p;
mod mod_bigint_bigint;
mod mod_decimal_a_precision_a_scale_decimal_b_precision_b_scale;
mod mod_double_double;
mod mod_integer_integer;
mod mod_real_real;
mod mod_smallint_smallint;
mod mod_tinyint_tinyint;
mod month_date;
mod month_intervalyeartomonth;
mod month_timestamp_p;
mod multimap_from_entries_array_row_c04_c15;
mod murmur3_varbinary;
mod nan;
mod ngrams_array_1_bigint;
mod none_match_array_1_function_1_boolean;
mod normal_cdf_double_double_double;
mod normalize_varchar_varchar;
mod now;
mod nullif_1_1;
mod objectid;
mod objectid_varchar;
mod objectid_timestamp_objectid;
mod parse_data_size_varchar;
mod parse_datetime_varchar_varchar;
mod parse_duration_varchar;
mod parse_presto_data_size_varchar;
mod pi;
mod pow_double_double;
mod power_double_double;
mod quantile_at_value_qdigest_bigint;
mod quantile_at_value_qdigest_double;
mod quantile_at_value_qdigest_real;
mod quarter_date;
mod quarter_timestamp_p;
mod radians_double;
mod rand_bigint;
mod rand_bigint_bigint;
mod rand;
mod rand_integer;
mod rand_integer_integer;
mod rand_smallint;
mod rand_smallint_smallint;
mod rand_tinyint;
mod rand_tinyint_tinyint;
mod random_bigint;
mod random_bigint_bigint;
mod random;
mod random_integer;
mod random_integer_integer;
mod random_smallint;
mod random_smallint_smallint;
mod random_tinyint;
mod random_tinyint_tinyint;
mod reduce_array_1_10_function_10_1_10_function_10_9;
mod regexp_count_varchar_joniregexp;
mod regexp_extract_varchar_joniregexp;
mod regexp_extract_varchar_joniregexp_bigint;
mod regexp_extract_all_varchar_joniregexp;
mod regexp_extract_all_varchar_joniregexp_bigint;
mod regexp_like_varchar_joniregexp;
mod regexp_position_varchar_joniregexp;
mod regexp_position_varchar_joniregexp_bigint;
mod regexp_position_varchar_joniregexp_bigint_bigint;
mod regexp_replace_varchar_joniregexp_function_array_varchar_varchar;
mod regexp_replace_varchar_joniregexp;
mod regexp_replace_varchar_joniregexp_varchar;
mod regexp_split_varchar_joniregexp;
mod regress_map_bigint_double_regressor;
mod render_boolean;
mod render_bigint_color;
mod render_double_color;
mod render_varchar_color;
mod repeat_1_bigint;
mod replace_varchar_varchar_varchar;
mod replace_varchar_varchar;
mod reverse_array_3;
mod reverse_varbinary;
mod reverse_varchar;
mod rgb_bigint_bigint_bigint;
mod round_double;
mod round_double_bigint;
mod round_real;
mod round_real_bigint;
mod round_integer;
mod round_integer_integer;
mod round_decimal_p_s;
mod round_decimal_p_s_bigint;
mod round_bigint;
mod round_bigint_bigint;
mod round_smallint;
mod round_smallint_bigint;
mod round_tinyint;
mod round_tinyint_bigint;
mod rpad_varbinary_bigint_varbinary;
mod rpad_varchar_bigint_varchar;
mod rtrim_varchar;
mod rtrim_varchar_codepoints;
mod second_intervaldaytosecond;
mod second_time_p;
mod second_timestamp_p;
mod sequence_bigint_bigint;
mod sequence_bigint_bigint_bigint;
mod sequence_date_date;
mod sequence_date_date_intervaldaytosecond;
mod sequence_date_date_intervalyeartomonth;
mod sequence_timestamp_p_timestamp_p_intervaldaytosecond;
mod sha1_varbinary;
mod sha256_varbinary;
mod sha512_varbinary;
mod shuffle_array_3;
mod sign_bigint;
mod sign_decimal_p_s;
mod sign_double;
mod sign_integer;
mod sign_real;
mod sign_smallint;
mod sign_tinyint;
mod simplify_geometry_geometry_double;
mod sin_double;
mod sinh_double;
mod slice_array_3_bigint_bigint;
mod soundex_varchar;
mod spatial_partitions_kdbtree_geometry;
mod spatial_partitions_kdbtree_geometry_double;
mod split_varchar_varchar;
mod split_varchar_varchar_bigint;
mod split_part_varchar_varchar_bigint;
mod split_to_map_varchar_varchar_varchar;
mod split_to_multimap_varchar_varchar_varchar;
mod spooky_hash_v2_32_varbinary;
mod spooky_hash_v2_64_varbinary;
mod sqrt_double;
mod st_area_geometry;
mod st_area_sphericalgeography;
mod st_asbinary_geometry;
mod st_astext_geometry;
mod st_boundary_geometry;
mod st_buffer_geometry_double;
mod st_centroid_geometry;
mod st_contains_geometry_geometry;
mod st_convexhull_geometry;
mod st_coorddim_geometry;
mod st_crosses_geometry_geometry;
mod st_difference_geometry_geometry;
mod st_dimension_geometry;
mod st_disjoint_geometry_geometry;
mod st_distance_geometry_geometry;
mod st_distance_sphericalgeography_sphericalgeography;
mod st_endpoint_geometry;
mod st_envelope_geometry;
mod st_envelopeaspts_geometry;
mod st_equals_geometry_geometry;
mod st_exteriorring_geometry;
mod st_geometries_geometry;
mod st_geometryfromtext_varchar;
mod st_geometryn_geometry_bigint;
mod st_geometrytype_geometry;
mod st_geomfrombinary_varbinary;
mod st_interiorringn_geometry_bigint;
mod st_interiorrings_geometry;
mod st_intersection_geometry_geometry;
mod st_intersects_geometry_geometry;
mod st_isclosed_geometry;
mod st_isempty_geometry;
mod st_isring_geometry;
mod st_issimple_geometry;
mod st_isvalid_geometry;
mod st_length_geometry;
mod st_length_sphericalgeography;
mod st_linefromtext_varchar;
mod st_linestring_array_geometry;
mod st_multipoint_array_geometry;
mod st_numgeometries_geometry;
mod st_numinteriorring_geometry;
mod st_numpoints_geometry;
mod st_overlaps_geometry_geometry;
mod st_point_double_double;
mod st_pointn_geometry_bigint;
mod st_points_geometry;
mod st_polygon_varchar;
mod st_relate_geometry_geometry_varchar;
mod st_startpoint_geometry;
mod st_symdifference_geometry_geometry;
mod st_touches_geometry_geometry;
mod st_union_geometry_geometry;
mod st_within_geometry_geometry;
mod st_x_geometry;
mod st_xmax_geometry;
mod st_xmin_geometry;
mod st_y_geometry;
mod st_ymax_geometry;
mod st_ymin_geometry;
mod starts_with_varchar_varchar;
mod strpos_varchar_varchar;
mod strpos_varchar_varchar_bigint;
mod substr_varchar_bigint;
mod substr_varchar_bigint_bigint;
mod substr_varbinary_bigint;
mod substr_varbinary_bigint_bigint;
mod substring_varchar_bigint;
mod substring_varchar_bigint_bigint;
mod tan_double;
mod tanh_double;
mod timestamp_objectid_timestamp_0;
mod timezone_hour_time_p;
mod timezone_hour_timestamp_p;
mod timezone_minute_time_p;
mod timezone_minute_timestamp_p;
mod to_base_bigint_bigint;
mod to_base32_varbinary;
mod to_base64_varbinary;
mod to_base64url_varbinary;
mod to_big_endian_32_bigint;
mod to_big_endian_64_bigint;
mod to_char_timestamp_p_varchar;
mod to_date_varchar_varchar;
mod to_encoded_polyline_geometry;
mod to_geojson_geometry_sphericalgeography;
mod to_geometry_sphericalgeography;
mod to_hex_varbinary;
mod to_ieee754_32_real;
mod to_ieee754_64_double;
mod to_iso8601_date;
mod to_iso8601_timestamp_p;
mod to_milliseconds_intervaldaytosecond;
mod to_spherical_geography_geometry;
mod to_timestamp_varchar_varchar;
mod to_unixtime_timestamp_p;
mod to_utf8_varchar;
mod transform_array_1_function_1_11;
mod transform_keys_map_13_5_function_13_5_12;
mod transform_values_map_4_8_function_4_8_7;
mod translate_varchar_varchar_varchar;
mod trim_varchar;
mod trim_varchar_codepoints;
mod trim_array_array_3_bigint;
mod truncate_decimal_p_s_bigint;
mod truncate_decimal_p_s;
mod truncate_double;
mod truncate_real;
mod typeof_1;
mod upper_varchar;
mod url_decode_varchar;
mod url_encode_varchar;
mod url_extract_fragment_varchar;
mod url_extract_host_varchar;
mod url_extract_parameter_varchar_varchar;
mod url_extract_path_varchar;
mod url_extract_port_varchar;
mod url_extract_protocol_varchar;
mod url_extract_query_varchar;
mod uuid;
mod value_at_quantile_qdigest_double;
mod value_at_quantile_tdigest_double;
mod values_at_quantiles_qdigest_array_double;
mod values_at_quantiles_tdigest_array_double;
mod week_date;
mod week_timestamp_p;
mod week_of_year_date;
mod week_of_year_timestamp_p;
mod width_bucket_double_array_double;
mod width_bucket_double_double_double_bigint;
mod wilson_interval_lower_bigint_bigint_double;
mod wilson_interval_upper_bigint_bigint_double;
mod with_timezone_timestamp_p_varchar;
mod word_stem_varchar;
mod word_stem_varchar_varchar;
mod xxhash64_varbinary;
mod year_date;
mod year_intervalyeartomonth;
mod year_timestamp_p;
mod year_of_week_date;
mod year_of_week_timestamp_p;
mod yow_date;
mod yow_timestamp_p;
mod zip_array_14_array_15;
mod zip_array_14_array_15_array_16;
mod zip_array_14_array_15_array_16_array_17;
mod zip_array_14_array_15_array_16_array_17_array_18;
mod zip_with_array_1_array_11_function_1_11_9;
mod if_boolean_1_1;
mod try_1;
mod current_time;
mod current_timestamp;
mod current_timestamp_bigint_0;
mod current_timestamp_bigint_3;
mod current_timestamp_bigint_6;
mod current_timestamp_bigint_9;
mod date_part_varchar_time_p;
mod date_part_varchar_timestamp_p;
mod date_part_varchar_date;
mod localtime;
mod localtimestamp;
mod localtimestamp_bigint_0;
mod localtimestamp_bigint_3;
mod localtimestamp_bigint_6;
mod localtimestamp_bigint_9;
mod current_user;
mod current_catalog;
mod current_schema;
mod format_varchar_1;
mod format_varchar_1_2;
mod format_varchar_1_2_3;
mod format_varchar_1_2_3_4;
mod format_varchar_1_2_3_4_5;
mod reclassify_T_varchar_varchar;
mod reclassify_T_varchar;
mod btrim_varchar;
mod btrim_varchar_varchar;
mod to_timestamp_seconds_1;
mod make_array_1;
mod length_array_array_1;


// create  UDFs
make_udf_function!(abs_tinyint::Func, ABS_TINYINT, abs_tinyint);
make_udf_function!(abs_smallint::Func, ABS_SMALLINT, abs_smallint);
make_udf_function!(abs_bigint::Func, ABS_BIGINT, abs_bigint);
make_udf_function!(abs_double::Func, ABS_DOUBLE, abs_double);
make_udf_function!(abs_decimal_p_s::Func, ABS_DECIMAL_P_S, abs_decimal_p_s);
make_udf_function!(abs_real::Func, ABS_REAL, abs_real);
make_udf_function!(acos_double::Func, ACOS_DOUBLE, acos_double);
make_udf_function!(all_match_array_1_function_1_boolean::Func, ALL_MATCH_ARRAY_1_FUNCTION_1_BOOLEAN, all_match_array_1_function_1_boolean);
make_udf_function!(any_match_array_1_function_1_boolean::Func, ANY_MATCH_ARRAY_1_FUNCTION_1_BOOLEAN, any_match_array_1_function_1_boolean);
make_udf_function!(array_distinct_array_3::Func, ARRAY_DISTINCT_ARRAY_3, array_distinct_array_3);
make_udf_function!(array_except_array_3_array_3::Func, ARRAY_EXCEPT_ARRAY_3_ARRAY_3, array_except_array_3_array_3);
make_udf_function!(array_intersect_array_3_array_3::Func, ARRAY_INTERSECT_ARRAY_3_ARRAY_3, array_intersect_array_3_array_3);
make_udf_function!(array_join_array_1_varchar::Func, ARRAY_JOIN_ARRAY_1_VARCHAR, array_join_array_1_varchar);
make_udf_function!(array_join_array_1_varchar_varchar::Func, ARRAY_JOIN_ARRAY_1_VARCHAR_VARCHAR, array_join_array_1_varchar_varchar);
make_udf_function!(array_max_array_1::Func, ARRAY_MAX_ARRAY_1, array_max_array_1);
make_udf_function!(array_min_array_1::Func, ARRAY_MIN_ARRAY_1, array_min_array_1);
make_udf_function!(array_position_array_1_1::Func, ARRAY_POSITION_ARRAY_1_1, array_position_array_1_1);
make_udf_function!(array_remove_array_3_3::Func, ARRAY_REMOVE_ARRAY_3_3, array_remove_array_3_3);
make_udf_function!(array_sort_array_3::Func, ARRAY_SORT_ARRAY_3, array_sort_array_3);
make_udf_function!(array_sort_array_1_function_1_1_bigint::Func, ARRAY_SORT_ARRAY_1_FUNCTION_1_1_BIGINT, array_sort_array_1_function_1_1_bigint);
make_udf_function!(array_union_array_3_array_3::Func, ARRAY_UNION_ARRAY_3_ARRAY_3, array_union_array_3_array_3);
make_udf_function!(arrays_overlap_array_3_array_3::Func, ARRAYS_OVERLAP_ARRAY_3_ARRAY_3, arrays_overlap_array_3_array_3);
make_udf_function!(asin_double::Func, ASIN_DOUBLE, asin_double);
make_udf_function!(at_timezone_timestamp_p_varchar::Func, AT_TIMEZONE_TIMESTAMP_P_VARCHAR, at_timezone_timestamp_p_varchar);
make_udf_function!(atan_double::Func, ATAN_DOUBLE, atan_double);
make_udf_function!(atan2_double_double::Func, ATAN2_DOUBLE_DOUBLE, atan2_double_double);
make_udf_function!(bar_double_bigint::Func, BAR_DOUBLE_BIGINT, bar_double_bigint);
make_udf_function!(bar_double_bigint_color_color::Func, BAR_DOUBLE_BIGINT_COLOR_COLOR, bar_double_bigint_color_color);
make_udf_function!(beta_cdf_double_double_double::Func, BETA_CDF_DOUBLE_DOUBLE_DOUBLE, beta_cdf_double_double_double);
make_udf_function!(bing_tile_bigint_bigint_bigint::Func, BING_TILE_BIGINT_BIGINT_BIGINT, bing_tile_bigint_bigint_bigint);
make_udf_function!(bing_tile_varchar::Func, BING_TILE_VARCHAR, bing_tile_varchar);
make_udf_function!(bing_tile_at_double_double_bigint::Func, BING_TILE_AT_DOUBLE_DOUBLE_BIGINT, bing_tile_at_double_double_bigint);
make_udf_function!(bing_tile_coordinates_bingtile::Func, BING_TILE_COORDINATES_BINGTILE, bing_tile_coordinates_bingtile);
make_udf_function!(bing_tile_polygon_bingtile::Func, BING_TILE_POLYGON_BINGTILE, bing_tile_polygon_bingtile);
make_udf_function!(bing_tile_quadkey_bingtile::Func, BING_TILE_QUADKEY_BINGTILE, bing_tile_quadkey_bingtile);
make_udf_function!(bing_tile_zoom_level_bingtile::Func, BING_TILE_ZOOM_LEVEL_BINGTILE, bing_tile_zoom_level_bingtile);
make_udf_function!(bing_tiles_around_double_double_bigint::Func, BING_TILES_AROUND_DOUBLE_DOUBLE_BIGINT, bing_tiles_around_double_double_bigint);
make_udf_function!(bing_tiles_around_double_double_bigint_double::Func, BING_TILES_AROUND_DOUBLE_DOUBLE_BIGINT_DOUBLE, bing_tiles_around_double_double_bigint_double);
make_udf_function!(bit_count_bigint_bigint::Func, BIT_COUNT_BIGINT_BIGINT, bit_count_bigint_bigint);
make_udf_function!(bitwise_and_bigint_bigint::Func, BITWISE_AND_BIGINT_BIGINT, bitwise_and_bigint_bigint);
make_udf_function!(bitwise_left_shift_bigint_bigint::Func, BITWISE_LEFT_SHIFT_BIGINT_BIGINT, bitwise_left_shift_bigint_bigint);
make_udf_function!(bitwise_left_shift_integer_bigint::Func, BITWISE_LEFT_SHIFT_INTEGER_BIGINT, bitwise_left_shift_integer_bigint);
make_udf_function!(bitwise_left_shift_smallint_bigint::Func, BITWISE_LEFT_SHIFT_SMALLINT_BIGINT, bitwise_left_shift_smallint_bigint);
make_udf_function!(bitwise_left_shift_tinyint_bigint::Func, BITWISE_LEFT_SHIFT_TINYINT_BIGINT, bitwise_left_shift_tinyint_bigint);
make_udf_function!(bitwise_not_bigint::Func, BITWISE_NOT_BIGINT, bitwise_not_bigint);
make_udf_function!(bitwise_or_bigint_bigint::Func, BITWISE_OR_BIGINT_BIGINT, bitwise_or_bigint_bigint);
make_udf_function!(bitwise_right_shift_bigint_bigint::Func, BITWISE_RIGHT_SHIFT_BIGINT_BIGINT, bitwise_right_shift_bigint_bigint);
make_udf_function!(bitwise_right_shift_integer_bigint::Func, BITWISE_RIGHT_SHIFT_INTEGER_BIGINT, bitwise_right_shift_integer_bigint);
make_udf_function!(bitwise_right_shift_smallint_bigint::Func, BITWISE_RIGHT_SHIFT_SMALLINT_BIGINT, bitwise_right_shift_smallint_bigint);
make_udf_function!(bitwise_right_shift_tinyint_bigint::Func, BITWISE_RIGHT_SHIFT_TINYINT_BIGINT, bitwise_right_shift_tinyint_bigint);
make_udf_function!(bitwise_right_shift_arithmetic_bigint_bigint::Func, BITWISE_RIGHT_SHIFT_ARITHMETIC_BIGINT_BIGINT, bitwise_right_shift_arithmetic_bigint_bigint);
make_udf_function!(bitwise_right_shift_arithmetic_integer_bigint::Func, BITWISE_RIGHT_SHIFT_ARITHMETIC_INTEGER_BIGINT, bitwise_right_shift_arithmetic_integer_bigint);
make_udf_function!(bitwise_right_shift_arithmetic_smallint_bigint::Func, BITWISE_RIGHT_SHIFT_ARITHMETIC_SMALLINT_BIGINT, bitwise_right_shift_arithmetic_smallint_bigint);
make_udf_function!(bitwise_right_shift_arithmetic_tinyint_bigint::Func, BITWISE_RIGHT_SHIFT_ARITHMETIC_TINYINT_BIGINT, bitwise_right_shift_arithmetic_tinyint_bigint);
make_udf_function!(bitwise_xor_bigint_bigint::Func, BITWISE_XOR_BIGINT_BIGINT, bitwise_xor_bigint_bigint);
make_udf_function!(cardinality_array_3::Func, CARDINALITY_ARRAY_3, cardinality_array_3);
make_udf_function!(cardinality_hyperloglog::Func, CARDINALITY_HYPERLOGLOG, cardinality_hyperloglog);
make_udf_function!(cardinality_map_4_5::Func, CARDINALITY_MAP_4_5, cardinality_map_4_5);
make_udf_function!(cardinality_setdigest::Func, CARDINALITY_SETDIGEST, cardinality_setdigest);
make_udf_function!(cbrt_double::Func, CBRT_DOUBLE, cbrt_double);
make_udf_function!(ceil_bigint::Func, CEIL_BIGINT, ceil_bigint);
make_udf_function!(ceil_decimal_p_s::Func, CEIL_DECIMAL_P_S, ceil_decimal_p_s);
make_udf_function!(ceil_double::Func, CEIL_DOUBLE, ceil_double);
make_udf_function!(ceil_integer::Func, CEIL_INTEGER, ceil_integer);
make_udf_function!(ceil_real::Func, CEIL_REAL, ceil_real);
make_udf_function!(ceil_smallint::Func, CEIL_SMALLINT, ceil_smallint);
make_udf_function!(ceil_tinyint::Func, CEIL_TINYINT, ceil_tinyint);
make_udf_function!(ceiling_bigint::Func, CEILING_BIGINT, ceiling_bigint);
make_udf_function!(ceiling_decimal_p_s::Func, CEILING_DECIMAL_P_S, ceiling_decimal_p_s);
make_udf_function!(ceiling_double::Func, CEILING_DOUBLE, ceiling_double);
make_udf_function!(ceiling_integer::Func, CEILING_INTEGER, ceiling_integer);
make_udf_function!(ceiling_real::Func, CEILING_REAL, ceiling_real);
make_udf_function!(ceiling_smallint::Func, CEILING_SMALLINT, ceiling_smallint);
make_udf_function!(ceiling_tinyint::Func, CEILING_TINYINT, ceiling_tinyint);
make_udf_function!(char2hexint_varchar::Func, CHAR2HEXINT_VARCHAR, char2hexint_varchar);
make_udf_function!(chr_bigint::Func, CHR_BIGINT, chr_bigint);
make_udf_function!(classify_map_bigint_double_classifier::Func, CLASSIFY_MAP_BIGINT_DOUBLE_CLASSIFIER, classify_map_bigint_double_classifier);
make_udf_function!(coalesce_1::Func, COALESCE_1, coalesce_1);
make_udf_function!(codepoint_varchar::Func, CODEPOINT_VARCHAR, codepoint_varchar);
make_udf_function!(color_double_color_color::Func, COLOR_DOUBLE_COLOR_COLOR, color_double_color_color);
make_udf_function!(color_double_double_double_color_color::Func, COLOR_DOUBLE_DOUBLE_DOUBLE_COLOR_COLOR, color_double_double_double_color_color);
make_udf_function!(color_varchar::Func, COLOR_VARCHAR, color_varchar);
make_udf_function!(combinations_array_1_bigint::Func, COMBINATIONS_ARRAY_1_BIGINT, combinations_array_1_bigint);
make_udf_function!(concat_3_array_3::Func, CONCAT_3_ARRAY_3, concat_3_array_3);
make_udf_function!(concat_array_3::Func, CONCAT_ARRAY_3, concat_array_3);
make_udf_function!(concat_array_3_3::Func, CONCAT_ARRAY_3_3, concat_array_3_3);
make_udf_function!(concat_varchar_varchar::Func, CONCAT_VARCHAR_VARCHAR, concat_varchar_varchar);
make_udf_function!(concat_varchar::Func, CONCAT_VARCHAR, concat_varchar);
make_udf_function!(concat_varbinary::Func, CONCAT_VARBINARY, concat_varbinary);
make_udf_function!(concat_ws_varchar_array_varchar::Func, CONCAT_WS_VARCHAR_ARRAY_VARCHAR, concat_ws_varchar_array_varchar);
make_udf_function!(concat_ws_varchar::Func, CONCAT_WS_VARCHAR, concat_ws_varchar);
make_udf_function!(contains_array_1_1::Func, CONTAINS_ARRAY_1_1, contains_array_1_1);
make_udf_function!(contains_varchar_ipaddress::Func, CONTAINS_VARCHAR_IPADDRESS, contains_varchar_ipaddress);
make_udf_function!(contains_sequence_array_1_array_1::Func, CONTAINS_SEQUENCE_ARRAY_1_ARRAY_1, contains_sequence_array_1_array_1);
make_udf_function!(cos_double::Func, COS_DOUBLE, cos_double);
make_udf_function!(cosh_double::Func, COSH_DOUBLE, cosh_double);
make_udf_function!(cosine_similarity_map_varchar_double_map_varchar_double::Func, COSINE_SIMILARITY_MAP_VARCHAR_DOUBLE_MAP_VARCHAR_DOUBLE, cosine_similarity_map_varchar_double_map_varchar_double);
make_udf_function!(crc32_varbinary::Func, CRC32_VARBINARY, crc32_varbinary);
make_udf_function!(current_date::Func, CURRENT_DATE, current_date);
make_udf_function!(current_groups::Func, CURRENT_GROUPS, current_groups);
make_udf_function!(current_timezone::Func, CURRENT_TIMEZONE, current_timezone);
make_udf_function!(date_timestamp_p::Func, DATE_TIMESTAMP_P, date_timestamp_p);
make_udf_function!(date_varchar::Func, DATE_VARCHAR, date_varchar);
make_udf_function!(date_add_varchar_bigint_date::Func, DATE_ADD_VARCHAR_BIGINT_DATE, date_add_varchar_bigint_date);
make_udf_function!(date_add_varchar_bigint_time_p::Func, DATE_ADD_VARCHAR_BIGINT_TIME_P, date_add_varchar_bigint_time_p);
make_udf_function!(date_add_varchar_bigint_timestamp_p::Func, DATE_ADD_VARCHAR_BIGINT_TIMESTAMP_P, date_add_varchar_bigint_timestamp_p);
make_udf_function!(date_diff_varchar_date_date::Func, DATE_DIFF_VARCHAR_DATE_DATE, date_diff_varchar_date_date);
make_udf_function!(date_diff_varchar_time_p_time_p::Func, DATE_DIFF_VARCHAR_TIME_P_TIME_P, date_diff_varchar_time_p_time_p);
make_udf_function!(date_diff_varchar_timestamp_p_timestamp_p::Func, DATE_DIFF_VARCHAR_TIMESTAMP_P_TIMESTAMP_P, date_diff_varchar_timestamp_p_timestamp_p);
make_udf_function!(date_format_timestamp_p_varchar::Func, DATE_FORMAT_TIMESTAMP_P_VARCHAR, date_format_timestamp_p_varchar);
make_udf_function!(date_parse_varchar_varchar::Func, DATE_PARSE_VARCHAR_VARCHAR, date_parse_varchar_varchar);
make_udf_function!(date_trunc_varchar_time_p::Func, DATE_TRUNC_VARCHAR_TIME_P, date_trunc_varchar_time_p);
make_udf_function!(date_trunc_varchar_timestamp_p::Func, DATE_TRUNC_VARCHAR_TIMESTAMP_P, date_trunc_varchar_timestamp_p);
make_udf_function!(date_trunc_varchar_date::Func, DATE_TRUNC_VARCHAR_DATE, date_trunc_varchar_date);
make_udf_function!(day_date::Func, DAY_DATE, day_date);
make_udf_function!(day_intervaldaytosecond::Func, DAY_INTERVALDAYTOSECOND, day_intervaldaytosecond);
make_udf_function!(day_timestamp_p::Func, DAY_TIMESTAMP_P, day_timestamp_p);
make_udf_function!(day_of_month_date::Func, DAY_OF_MONTH_DATE, day_of_month_date);
make_udf_function!(day_of_month_intervaldaytosecond::Func, DAY_OF_MONTH_INTERVALDAYTOSECOND, day_of_month_intervaldaytosecond);
make_udf_function!(day_of_month_timestamp_p::Func, DAY_OF_MONTH_TIMESTAMP_P, day_of_month_timestamp_p);
make_udf_function!(day_of_week_date::Func, DAY_OF_WEEK_DATE, day_of_week_date);
make_udf_function!(day_of_week_timestamp_p::Func, DAY_OF_WEEK_TIMESTAMP_P, day_of_week_timestamp_p);
make_udf_function!(day_of_year_date::Func, DAY_OF_YEAR_DATE, day_of_year_date);
make_udf_function!(day_of_year_timestamp_p::Func, DAY_OF_YEAR_TIMESTAMP_P, day_of_year_timestamp_p);
make_udf_function!(degrees_double::Func, DEGREES_DOUBLE, degrees_double);
make_udf_function!(dow_date::Func, DOW_DATE, dow_date);
make_udf_function!(dow_timestamp_p::Func, DOW_TIMESTAMP_P, dow_timestamp_p);
make_udf_function!(doy_date::Func, DOY_DATE, doy_date);
make_udf_function!(doy_timestamp_p::Func, DOY_TIMESTAMP_P, doy_timestamp_p);
make_udf_function!(e::Func, E, e);
make_udf_function!(element_at_map_4_5_4::Func, ELEMENT_AT_MAP_4_5_4, element_at_map_4_5_4);
make_udf_function!(element_at_array_3_bigint::Func, ELEMENT_AT_ARRAY_3_BIGINT, element_at_array_3_bigint);
make_udf_function!(empty_approx_set::Func, EMPTY_APPROX_SET, empty_approx_set);
make_udf_function!(exp_double::Func, EXP_DOUBLE, exp_double);
make_udf_function!(features_double::Func, FEATURES_DOUBLE, features_double);
make_udf_function!(features_double_double::Func, FEATURES_DOUBLE_DOUBLE, features_double_double);
make_udf_function!(features_double_double_double::Func, FEATURES_DOUBLE_DOUBLE_DOUBLE, features_double_double_double);
make_udf_function!(features_double_double_double_double::Func, FEATURES_DOUBLE_DOUBLE_DOUBLE_DOUBLE, features_double_double_double_double);
make_udf_function!(features_double_double_double_double_double::Func, FEATURES_DOUBLE_DOUBLE_DOUBLE_DOUBLE_DOUBLE, features_double_double_double_double_double);
make_udf_function!(features_double_double_double_double_double_double::Func, FEATURES_DOUBLE_DOUBLE_DOUBLE_DOUBLE_DOUBLE_DOUBLE, features_double_double_double_double_double_double);
make_udf_function!(features_double_double_double_double_double_double_double::Func, FEATURES_DOUBLE_DOUBLE_DOUBLE_DOUBLE_DOUBLE_DOUBLE_DOUBLE, features_double_double_double_double_double_double_double);
make_udf_function!(features_double_double_double_double_double_double_double_double::Func, FEATURES_DOUBLE_DOUBLE_DOUBLE_DOUBLE_DOUBLE_DOUBLE_DOUBLE_DOUBLE, features_double_double_double_double_double_double_double_double);
make_udf_function!(features_double_double_double_double_double_double_double_double_double::Func, FEATURES_DOUBLE_DOUBLE_DOUBLE_DOUBLE_DOUBLE_DOUBLE_DOUBLE_DOUBLE_DOUBLE, features_double_double_double_double_double_double_double_double_double);
make_udf_function!(features_double_double_double_double_double_double_double_double_double_double::Func, FEATURES_DOUBLE_DOUBLE_DOUBLE_DOUBLE_DOUBLE_DOUBLE_DOUBLE_DOUBLE_DOUBLE_DOUBLE, features_double_double_double_double_double_double_double_double_double_double);
make_udf_function!(filter_array_1_function_1_boolean::Func, FILTER_ARRAY_1_FUNCTION_1_BOOLEAN, filter_array_1_function_1_boolean);
make_udf_function!(flatten_array_array_3::Func, FLATTEN_ARRAY_ARRAY_3, flatten_array_array_3);
make_udf_function!(floor_bigint::Func, FLOOR_BIGINT, floor_bigint);
make_udf_function!(floor_decimal_p_s::Func, FLOOR_DECIMAL_P_S, floor_decimal_p_s);
make_udf_function!(floor_double::Func, FLOOR_DOUBLE, floor_double);
make_udf_function!(floor_integer::Func, FLOOR_INTEGER, floor_integer);
make_udf_function!(floor_real::Func, FLOOR_REAL, floor_real);
make_udf_function!(floor_smallint::Func, FLOOR_SMALLINT, floor_smallint);
make_udf_function!(floor_tinyint::Func, FLOOR_TINYINT, floor_tinyint);
make_udf_function!(format_datetime_timestamp_p_varchar::Func, FORMAT_DATETIME_TIMESTAMP_P_VARCHAR, format_datetime_timestamp_p_varchar);
make_udf_function!(format_number_bigint::Func, FORMAT_NUMBER_BIGINT, format_number_bigint);
make_udf_function!(format_number_double::Func, FORMAT_NUMBER_DOUBLE, format_number_double);
make_udf_function!(from_base_varchar_bigint::Func, FROM_BASE_VARCHAR_BIGINT, from_base_varchar_bigint);
make_udf_function!(from_base32_varbinary::Func, FROM_BASE32_VARBINARY, from_base32_varbinary);
make_udf_function!(from_base32_varchar::Func, FROM_BASE32_VARCHAR, from_base32_varchar);
make_udf_function!(from_base64_varbinary::Func, FROM_BASE64_VARBINARY, from_base64_varbinary);
make_udf_function!(from_base64_varchar::Func, FROM_BASE64_VARCHAR, from_base64_varchar);
make_udf_function!(from_base64url_varbinary::Func, FROM_BASE64URL_VARBINARY, from_base64url_varbinary);
make_udf_function!(from_base64url_varchar::Func, FROM_BASE64URL_VARCHAR, from_base64url_varchar);
make_udf_function!(from_big_endian_32_varbinary::Func, FROM_BIG_ENDIAN_32_VARBINARY, from_big_endian_32_varbinary);
make_udf_function!(from_big_endian_64_varbinary::Func, FROM_BIG_ENDIAN_64_VARBINARY, from_big_endian_64_varbinary);
make_udf_function!(from_encoded_polyline_varchar::Func, FROM_ENCODED_POLYLINE_VARCHAR, from_encoded_polyline_varchar);
make_udf_function!(from_geojson_geometry_varchar::Func, FROM_GEOJSON_GEOMETRY_VARCHAR, from_geojson_geometry_varchar);
make_udf_function!(from_hex_varbinary::Func, FROM_HEX_VARBINARY, from_hex_varbinary);
make_udf_function!(from_hex_varchar::Func, FROM_HEX_VARCHAR, from_hex_varchar);
make_udf_function!(from_ieee754_32_varbinary::Func, FROM_IEEE754_32_VARBINARY, from_ieee754_32_varbinary);
make_udf_function!(from_ieee754_64_varbinary::Func, FROM_IEEE754_64_VARBINARY, from_ieee754_64_varbinary);
make_udf_function!(from_iso8601_date_varchar::Func, FROM_ISO8601_DATE_VARCHAR, from_iso8601_date_varchar);
make_udf_function!(from_iso8601_timestamp_varchar::Func, FROM_ISO8601_TIMESTAMP_VARCHAR, from_iso8601_timestamp_varchar);
make_udf_function!(from_iso8601_timestamp_nanos_varchar::Func, FROM_ISO8601_TIMESTAMP_NANOS_VARCHAR, from_iso8601_timestamp_nanos_varchar);
make_udf_function!(from_unixtime_bigint::Func, FROM_UNIXTIME_BIGINT, from_unixtime_bigint);
make_udf_function!(from_unixtime_bigint_bigint_bigint::Func, FROM_UNIXTIME_BIGINT_BIGINT_BIGINT, from_unixtime_bigint_bigint_bigint);
make_udf_function!(from_unixtime_bigint_varchar::Func, FROM_UNIXTIME_BIGINT_VARCHAR, from_unixtime_bigint_varchar);
make_udf_function!(from_unixtime_nanos_bigint::Func, FROM_UNIXTIME_NANOS_BIGINT, from_unixtime_nanos_bigint);
make_udf_function!(from_unixtime_nanos_decimal_p_s::Func, FROM_UNIXTIME_NANOS_DECIMAL_P_S, from_unixtime_nanos_decimal_p_s);
make_udf_function!(from_utf8_varbinary::Func, FROM_UTF8_VARBINARY, from_utf8_varbinary);
make_udf_function!(from_utf8_varbinary_bigint::Func, FROM_UTF8_VARBINARY_BIGINT, from_utf8_varbinary_bigint);
make_udf_function!(from_utf8_varbinary_varchar::Func, FROM_UTF8_VARBINARY_VARCHAR, from_utf8_varbinary_varchar);
make_udf_function!(geometry_from_hadoop_shape_varbinary::Func, GEOMETRY_FROM_HADOOP_SHAPE_VARBINARY, geometry_from_hadoop_shape_varbinary);
make_udf_function!(geometry_invalid_reason_geometry::Func, GEOMETRY_INVALID_REASON_GEOMETRY, geometry_invalid_reason_geometry);
make_udf_function!(geometry_nearest_points_geometry_geometry::Func, GEOMETRY_NEAREST_POINTS_GEOMETRY_GEOMETRY, geometry_nearest_points_geometry_geometry);
make_udf_function!(geometry_to_bing_tiles_geometry_bigint::Func, GEOMETRY_TO_BING_TILES_GEOMETRY_BIGINT, geometry_to_bing_tiles_geometry_bigint);
make_udf_function!(geometry_union_array_geometry::Func, GEOMETRY_UNION_ARRAY_GEOMETRY, geometry_union_array_geometry);
make_udf_function!(great_circle_distance_double_double_double_double::Func, GREAT_CIRCLE_DISTANCE_DOUBLE_DOUBLE_DOUBLE_DOUBLE, great_circle_distance_double_double_double_double);
make_udf_function!(greatest_3::Func, GREATEST_3, greatest_3);
make_udf_function!(hamming_distance_varchar_varchar::Func, HAMMING_DISTANCE_VARCHAR_VARCHAR, hamming_distance_varchar_varchar);
make_udf_function!(hash_counts_setdigest::Func, HASH_COUNTS_SETDIGEST, hash_counts_setdigest);
make_udf_function!(hmac_md5_varbinary_varbinary::Func, HMAC_MD5_VARBINARY_VARBINARY, hmac_md5_varbinary_varbinary);
make_udf_function!(hmac_sha1_varbinary_varbinary::Func, HMAC_SHA1_VARBINARY_VARBINARY, hmac_sha1_varbinary_varbinary);
make_udf_function!(hmac_sha256_varbinary_varbinary::Func, HMAC_SHA256_VARBINARY_VARBINARY, hmac_sha256_varbinary_varbinary);
make_udf_function!(hmac_sha512_varbinary_varbinary::Func, HMAC_SHA512_VARBINARY_VARBINARY, hmac_sha512_varbinary_varbinary);
make_udf_function!(hour_intervaldaytosecond::Func, HOUR_INTERVALDAYTOSECOND, hour_intervaldaytosecond);
make_udf_function!(hour_time_p::Func, HOUR_TIME_P, hour_time_p);
make_udf_function!(hour_timestamp_p::Func, HOUR_TIMESTAMP_P, hour_timestamp_p);
make_udf_function!(human_readable_seconds_double::Func, HUMAN_READABLE_SECONDS_DOUBLE, human_readable_seconds_double);
make_udf_function!(index_varchar_varchar::Func, INDEX_VARCHAR_VARCHAR, index_varchar_varchar);
make_udf_function!(infinity::Func, INFINITY, infinity);
make_udf_function!(intersection_cardinality_setdigest_setdigest::Func, INTERSECTION_CARDINALITY_SETDIGEST_SETDIGEST, intersection_cardinality_setdigest_setdigest);
make_udf_function!(inverse_beta_cdf_double_double_double::Func, INVERSE_BETA_CDF_DOUBLE_DOUBLE_DOUBLE, inverse_beta_cdf_double_double_double);
make_udf_function!(inverse_normal_cdf_double_double_double::Func, INVERSE_NORMAL_CDF_DOUBLE_DOUBLE_DOUBLE, inverse_normal_cdf_double_double_double);
make_udf_function!(is_finite_double::Func, IS_FINITE_DOUBLE, is_finite_double);
make_udf_function!(is_infinite_double::Func, IS_INFINITE_DOUBLE, is_infinite_double);
make_udf_function!(is_json_scalar_json::Func, IS_JSON_SCALAR_JSON, is_json_scalar_json);
make_udf_function!(is_json_scalar_varchar::Func, IS_JSON_SCALAR_VARCHAR, is_json_scalar_varchar);
make_udf_function!(is_nan_double::Func, IS_NAN_DOUBLE, is_nan_double);
make_udf_function!(is_nan_real::Func, IS_NAN_REAL, is_nan_real);
make_udf_function!(jaccard_index_setdigest_setdigest::Func, JACCARD_INDEX_SETDIGEST_SETDIGEST, jaccard_index_setdigest_setdigest);
make_udf_function!(json_array_contains_json_bigint::Func, JSON_ARRAY_CONTAINS_JSON_BIGINT, json_array_contains_json_bigint);
make_udf_function!(json_array_contains_json_boolean::Func, JSON_ARRAY_CONTAINS_JSON_BOOLEAN, json_array_contains_json_boolean);
make_udf_function!(json_array_contains_json_double::Func, JSON_ARRAY_CONTAINS_JSON_DOUBLE, json_array_contains_json_double);
make_udf_function!(json_array_contains_json_varchar::Func, JSON_ARRAY_CONTAINS_JSON_VARCHAR, json_array_contains_json_varchar);
make_udf_function!(json_array_contains_varchar_bigint::Func, JSON_ARRAY_CONTAINS_VARCHAR_BIGINT, json_array_contains_varchar_bigint);
make_udf_function!(json_array_contains_varchar_boolean::Func, JSON_ARRAY_CONTAINS_VARCHAR_BOOLEAN, json_array_contains_varchar_boolean);
make_udf_function!(json_array_contains_varchar_double::Func, JSON_ARRAY_CONTAINS_VARCHAR_DOUBLE, json_array_contains_varchar_double);
make_udf_function!(json_array_contains_varchar_varchar::Func, JSON_ARRAY_CONTAINS_VARCHAR_VARCHAR, json_array_contains_varchar_varchar);
make_udf_function!(json_array_get_json_bigint::Func, JSON_ARRAY_GET_JSON_BIGINT, json_array_get_json_bigint);
make_udf_function!(json_array_get_varchar_bigint::Func, JSON_ARRAY_GET_VARCHAR_BIGINT, json_array_get_varchar_bigint);
make_udf_function!(json_array_length_json::Func, JSON_ARRAY_LENGTH_JSON, json_array_length_json);
make_udf_function!(json_array_length_varchar::Func, JSON_ARRAY_LENGTH_VARCHAR, json_array_length_varchar);
make_udf_function!(json_extract_json_jsonpath::Func, JSON_EXTRACT_JSON_JSONPATH, json_extract_json_jsonpath);
make_udf_function!(json_extract_varchar_jsonpath::Func, JSON_EXTRACT_VARCHAR_JSONPATH, json_extract_varchar_jsonpath);
make_udf_function!(json_extract_scalar_json_jsonpath::Func, JSON_EXTRACT_SCALAR_JSON_JSONPATH, json_extract_scalar_json_jsonpath);
make_udf_function!(json_extract_scalar_varchar_jsonpath::Func, JSON_EXTRACT_SCALAR_VARCHAR_JSONPATH, json_extract_scalar_varchar_jsonpath);
make_udf_function!(json_format_json::Func, JSON_FORMAT_JSON, json_format_json);
make_udf_function!(json_parse_varchar::Func, JSON_PARSE_VARCHAR, json_parse_varchar);
make_udf_function!(json_size_json_jsonpath::Func, JSON_SIZE_JSON_JSONPATH, json_size_json_jsonpath);
make_udf_function!(json_size_varchar_jsonpath::Func, JSON_SIZE_VARCHAR_JSONPATH, json_size_varchar_jsonpath);
make_udf_function!(last_day_of_month_date::Func, LAST_DAY_OF_MONTH_DATE, last_day_of_month_date);
make_udf_function!(last_day_of_month_timestamp_p::Func, LAST_DAY_OF_MONTH_TIMESTAMP_P, last_day_of_month_timestamp_p);
make_udf_function!(least_3::Func, LEAST_3, least_3);
make_udf_function!(length_varchar::Func, LENGTH_VARCHAR, length_varchar);
make_udf_function!(length_varbinary::Func, LENGTH_VARBINARY, length_varbinary);
make_udf_function!(length_array_1::Func, LENGTH_ARRAY_1, length_array_1);
make_udf_function!(levenshtein_distance_varchar_varchar::Func, LEVENSHTEIN_DISTANCE_VARCHAR_VARCHAR, levenshtein_distance_varchar_varchar);
make_udf_function!(line_interpolate_point_geometry_double::Func, LINE_INTERPOLATE_POINT_GEOMETRY_DOUBLE, line_interpolate_point_geometry_double);
make_udf_function!(line_interpolate_points_geometry_double::Func, LINE_INTERPOLATE_POINTS_GEOMETRY_DOUBLE, line_interpolate_points_geometry_double);
make_udf_function!(line_locate_point_geometry_geometry::Func, LINE_LOCATE_POINT_GEOMETRY_GEOMETRY, line_locate_point_geometry_geometry);
make_udf_function!(ln_double::Func, LN_DOUBLE, ln_double);
make_udf_function!(log_double_double::Func, LOG_DOUBLE_DOUBLE, log_double_double);
make_udf_function!(log10_double::Func, LOG10_DOUBLE, log10_double);
make_udf_function!(log2_double::Func, LOG2_DOUBLE, log2_double);
make_udf_function!(lower_varchar::Func, LOWER_VARCHAR, lower_varchar);
make_udf_function!(lpad_varbinary_bigint_varbinary::Func, LPAD_VARBINARY_BIGINT_VARBINARY, lpad_varbinary_bigint_varbinary);
make_udf_function!(lpad_varchar_bigint_varchar::Func, LPAD_VARCHAR_BIGINT_VARCHAR, lpad_varchar_bigint_varchar);
make_udf_function!(ltrim_varchar::Func, LTRIM_VARCHAR, ltrim_varchar);
make_udf_function!(ltrim_varchar_codepoints::Func, LTRIM_VARCHAR_CODEPOINTS, ltrim_varchar_codepoints);
make_udf_function!(luhn_check_varchar::Func, LUHN_CHECK_VARCHAR, luhn_check_varchar);
make_udf_function!(map_array_4_array_5::Func, MAP_ARRAY_4_ARRAY_5, map_array_4_array_5);
make_udf_function!(map::Func, MAP, map);
make_udf_function!(map_concat_map_4_5::Func, MAP_CONCAT_MAP_4_5, map_concat_map_4_5);
make_udf_function!(map_entries_map_4_5::Func, MAP_ENTRIES_MAP_4_5, map_entries_map_4_5);
make_udf_function!(map_filter_map_4_5_function_4_5_boolean::Func, MAP_FILTER_MAP_4_5_FUNCTION_4_5_BOOLEAN, map_filter_map_4_5_function_4_5_boolean);
make_udf_function!(map_from_entries_array_row_c04_c15::Func, MAP_FROM_ENTRIES_ARRAY_ROW_C04_C15, map_from_entries_array_row_c04_c15);
make_udf_function!(map_keys_map_4_5::Func, MAP_KEYS_MAP_4_5, map_keys_map_4_5);
make_udf_function!(map_values_map_4_5::Func, MAP_VALUES_MAP_4_5, map_values_map_4_5);
make_udf_function!(map_zip_with_map_4_8_map_4_7_function_4_8_7_6::Func, MAP_ZIP_WITH_MAP_4_8_MAP_4_7_FUNCTION_4_8_7_6, map_zip_with_map_4_8_map_4_7_function_4_8_7_6);
make_udf_function!(md5_varbinary::Func, MD5_VARBINARY, md5_varbinary);
make_udf_function!(millisecond_intervaldaytosecond::Func, MILLISECOND_INTERVALDAYTOSECOND, millisecond_intervaldaytosecond);
make_udf_function!(millisecond_time_p::Func, MILLISECOND_TIME_P, millisecond_time_p);
make_udf_function!(millisecond_timestamp_p::Func, MILLISECOND_TIMESTAMP_P, millisecond_timestamp_p);
make_udf_function!(minute_intervaldaytosecond::Func, MINUTE_INTERVALDAYTOSECOND, minute_intervaldaytosecond);
make_udf_function!(minute_time_p::Func, MINUTE_TIME_P, minute_time_p);
make_udf_function!(minute_timestamp_p::Func, MINUTE_TIMESTAMP_P, minute_timestamp_p);
make_udf_function!(mod_bigint_bigint::Func, MOD_BIGINT_BIGINT, mod_bigint_bigint);
make_udf_function!(mod_decimal_a_precision_a_scale_decimal_b_precision_b_scale::Func, MOD_DECIMAL_A_PRECISION_A_SCALE_DECIMAL_B_PRECISION_B_SCALE, mod_decimal_a_precision_a_scale_decimal_b_precision_b_scale);
make_udf_function!(mod_double_double::Func, MOD_DOUBLE_DOUBLE, mod_double_double);
make_udf_function!(mod_integer_integer::Func, MOD_INTEGER_INTEGER, mod_integer_integer);
make_udf_function!(mod_real_real::Func, MOD_REAL_REAL, mod_real_real);
make_udf_function!(mod_smallint_smallint::Func, MOD_SMALLINT_SMALLINT, mod_smallint_smallint);
make_udf_function!(mod_tinyint_tinyint::Func, MOD_TINYINT_TINYINT, mod_tinyint_tinyint);
make_udf_function!(month_date::Func, MONTH_DATE, month_date);
make_udf_function!(month_intervalyeartomonth::Func, MONTH_INTERVALYEARTOMONTH, month_intervalyeartomonth);
make_udf_function!(month_timestamp_p::Func, MONTH_TIMESTAMP_P, month_timestamp_p);
make_udf_function!(multimap_from_entries_array_row_c04_c15::Func, MULTIMAP_FROM_ENTRIES_ARRAY_ROW_C04_C15, multimap_from_entries_array_row_c04_c15);
make_udf_function!(murmur3_varbinary::Func, MURMUR3_VARBINARY, murmur3_varbinary);
make_udf_function!(nan::Func, NAN, nan);
make_udf_function!(ngrams_array_1_bigint::Func, NGRAMS_ARRAY_1_BIGINT, ngrams_array_1_bigint);
make_udf_function!(none_match_array_1_function_1_boolean::Func, NONE_MATCH_ARRAY_1_FUNCTION_1_BOOLEAN, none_match_array_1_function_1_boolean);
make_udf_function!(normal_cdf_double_double_double::Func, NORMAL_CDF_DOUBLE_DOUBLE_DOUBLE, normal_cdf_double_double_double);
make_udf_function!(normalize_varchar_varchar::Func, NORMALIZE_VARCHAR_VARCHAR, normalize_varchar_varchar);
make_udf_function!(now::Func, NOW, now);
make_udf_function!(nullif_1_1::Func, NULLIF_1_1, nullif_1_1);
make_udf_function!(objectid::Func, OBJECTID, objectid);
make_udf_function!(objectid_varchar::Func, OBJECTID_VARCHAR, objectid_varchar);
make_udf_function!(objectid_timestamp_objectid::Func, OBJECTID_TIMESTAMP_OBJECTID, objectid_timestamp_objectid);
make_udf_function!(parse_data_size_varchar::Func, PARSE_DATA_SIZE_VARCHAR, parse_data_size_varchar);
make_udf_function!(parse_datetime_varchar_varchar::Func, PARSE_DATETIME_VARCHAR_VARCHAR, parse_datetime_varchar_varchar);
make_udf_function!(parse_duration_varchar::Func, PARSE_DURATION_VARCHAR, parse_duration_varchar);
make_udf_function!(parse_presto_data_size_varchar::Func, PARSE_PRESTO_DATA_SIZE_VARCHAR, parse_presto_data_size_varchar);
make_udf_function!(pi::Func, PI, pi);
make_udf_function!(pow_double_double::Func, POW_DOUBLE_DOUBLE, pow_double_double);
make_udf_function!(power_double_double::Func, POWER_DOUBLE_DOUBLE, power_double_double);
make_udf_function!(quantile_at_value_qdigest_bigint::Func, QUANTILE_AT_VALUE_QDIGEST_BIGINT, quantile_at_value_qdigest_bigint);
make_udf_function!(quantile_at_value_qdigest_double::Func, QUANTILE_AT_VALUE_QDIGEST_DOUBLE, quantile_at_value_qdigest_double);
make_udf_function!(quantile_at_value_qdigest_real::Func, QUANTILE_AT_VALUE_QDIGEST_REAL, quantile_at_value_qdigest_real);
make_udf_function!(quarter_date::Func, QUARTER_DATE, quarter_date);
make_udf_function!(quarter_timestamp_p::Func, QUARTER_TIMESTAMP_P, quarter_timestamp_p);
make_udf_function!(radians_double::Func, RADIANS_DOUBLE, radians_double);
make_udf_function!(rand_bigint::Func, RAND_BIGINT, rand_bigint);
make_udf_function!(rand_bigint_bigint::Func, RAND_BIGINT_BIGINT, rand_bigint_bigint);
make_udf_function!(rand::Func, RAND, rand);
make_udf_function!(rand_integer::Func, RAND_INTEGER, rand_integer);
make_udf_function!(rand_integer_integer::Func, RAND_INTEGER_INTEGER, rand_integer_integer);
make_udf_function!(rand_smallint::Func, RAND_SMALLINT, rand_smallint);
make_udf_function!(rand_smallint_smallint::Func, RAND_SMALLINT_SMALLINT, rand_smallint_smallint);
make_udf_function!(rand_tinyint::Func, RAND_TINYINT, rand_tinyint);
make_udf_function!(rand_tinyint_tinyint::Func, RAND_TINYINT_TINYINT, rand_tinyint_tinyint);
make_udf_function!(random_bigint::Func, RANDOM_BIGINT, random_bigint);
make_udf_function!(random_bigint_bigint::Func, RANDOM_BIGINT_BIGINT, random_bigint_bigint);
make_udf_function!(random::Func, RANDOM, random);
make_udf_function!(random_integer::Func, RANDOM_INTEGER, random_integer);
make_udf_function!(random_integer_integer::Func, RANDOM_INTEGER_INTEGER, random_integer_integer);
make_udf_function!(random_smallint::Func, RANDOM_SMALLINT, random_smallint);
make_udf_function!(random_smallint_smallint::Func, RANDOM_SMALLINT_SMALLINT, random_smallint_smallint);
make_udf_function!(random_tinyint::Func, RANDOM_TINYINT, random_tinyint);
make_udf_function!(random_tinyint_tinyint::Func, RANDOM_TINYINT_TINYINT, random_tinyint_tinyint);
make_udf_function!(reduce_array_1_10_function_10_1_10_function_10_9::Func, REDUCE_ARRAY_1_10_FUNCTION_10_1_10_FUNCTION_10_9, reduce_array_1_10_function_10_1_10_function_10_9);
make_udf_function!(regexp_count_varchar_joniregexp::Func, REGEXP_COUNT_VARCHAR_JONIREGEXP, regexp_count_varchar_joniregexp);
make_udf_function!(regexp_extract_varchar_joniregexp::Func, REGEXP_EXTRACT_VARCHAR_JONIREGEXP, regexp_extract_varchar_joniregexp);
make_udf_function!(regexp_extract_varchar_joniregexp_bigint::Func, REGEXP_EXTRACT_VARCHAR_JONIREGEXP_BIGINT, regexp_extract_varchar_joniregexp_bigint);
make_udf_function!(regexp_extract_all_varchar_joniregexp::Func, REGEXP_EXTRACT_ALL_VARCHAR_JONIREGEXP, regexp_extract_all_varchar_joniregexp);
make_udf_function!(regexp_extract_all_varchar_joniregexp_bigint::Func, REGEXP_EXTRACT_ALL_VARCHAR_JONIREGEXP_BIGINT, regexp_extract_all_varchar_joniregexp_bigint);
make_udf_function!(regexp_like_varchar_joniregexp::Func, REGEXP_LIKE_VARCHAR_JONIREGEXP, regexp_like_varchar_joniregexp);
make_udf_function!(regexp_position_varchar_joniregexp::Func, REGEXP_POSITION_VARCHAR_JONIREGEXP, regexp_position_varchar_joniregexp);
make_udf_function!(regexp_position_varchar_joniregexp_bigint::Func, REGEXP_POSITION_VARCHAR_JONIREGEXP_BIGINT, regexp_position_varchar_joniregexp_bigint);
make_udf_function!(regexp_position_varchar_joniregexp_bigint_bigint::Func, REGEXP_POSITION_VARCHAR_JONIREGEXP_BIGINT_BIGINT, regexp_position_varchar_joniregexp_bigint_bigint);
make_udf_function!(regexp_replace_varchar_joniregexp_function_array_varchar_varchar::Func, REGEXP_REPLACE_VARCHAR_JONIREGEXP_FUNCTION_ARRAY_VARCHAR_VARCHAR, regexp_replace_varchar_joniregexp_function_array_varchar_varchar);
make_udf_function!(regexp_replace_varchar_joniregexp::Func, REGEXP_REPLACE_VARCHAR_JONIREGEXP, regexp_replace_varchar_joniregexp);
make_udf_function!(regexp_replace_varchar_joniregexp_varchar::Func, REGEXP_REPLACE_VARCHAR_JONIREGEXP_VARCHAR, regexp_replace_varchar_joniregexp_varchar);
make_udf_function!(regexp_split_varchar_joniregexp::Func, REGEXP_SPLIT_VARCHAR_JONIREGEXP, regexp_split_varchar_joniregexp);
make_udf_function!(regress_map_bigint_double_regressor::Func, REGRESS_MAP_BIGINT_DOUBLE_REGRESSOR, regress_map_bigint_double_regressor);
make_udf_function!(render_boolean::Func, RENDER_BOOLEAN, render_boolean);
make_udf_function!(render_bigint_color::Func, RENDER_BIGINT_COLOR, render_bigint_color);
make_udf_function!(render_double_color::Func, RENDER_DOUBLE_COLOR, render_double_color);
make_udf_function!(render_varchar_color::Func, RENDER_VARCHAR_COLOR, render_varchar_color);
make_udf_function!(repeat_1_bigint::Func, REPEAT_1_BIGINT, repeat_1_bigint);
make_udf_function!(replace_varchar_varchar_varchar::Func, REPLACE_VARCHAR_VARCHAR_VARCHAR, replace_varchar_varchar_varchar);
make_udf_function!(replace_varchar_varchar::Func, REPLACE_VARCHAR_VARCHAR, replace_varchar_varchar);
make_udf_function!(reverse_array_3::Func, REVERSE_ARRAY_3, reverse_array_3);
make_udf_function!(reverse_varbinary::Func, REVERSE_VARBINARY, reverse_varbinary);
make_udf_function!(reverse_varchar::Func, REVERSE_VARCHAR, reverse_varchar);
make_udf_function!(rgb_bigint_bigint_bigint::Func, RGB_BIGINT_BIGINT_BIGINT, rgb_bigint_bigint_bigint);
make_udf_function!(round_double::Func, ROUND_DOUBLE, round_double);
make_udf_function!(round_double_bigint::Func, ROUND_DOUBLE_BIGINT, round_double_bigint);
make_udf_function!(round_real::Func, ROUND_REAL, round_real);
make_udf_function!(round_real_bigint::Func, ROUND_REAL_BIGINT, round_real_bigint);
make_udf_function!(round_integer::Func, ROUND_INTEGER, round_integer);
make_udf_function!(round_integer_integer::Func, ROUND_INTEGER_INTEGER, round_integer_integer);
make_udf_function!(round_decimal_p_s::Func, ROUND_DECIMAL_P_S, round_decimal_p_s);
make_udf_function!(round_decimal_p_s_bigint::Func, ROUND_DECIMAL_P_S_BIGINT, round_decimal_p_s_bigint);
make_udf_function!(round_bigint::Func, ROUND_BIGINT, round_bigint);
make_udf_function!(round_bigint_bigint::Func, ROUND_BIGINT_BIGINT, round_bigint_bigint);
make_udf_function!(round_smallint::Func, ROUND_SMALLINT, round_smallint);
make_udf_function!(round_smallint_bigint::Func, ROUND_SMALLINT_BIGINT, round_smallint_bigint);
make_udf_function!(round_tinyint::Func, ROUND_TINYINT, round_tinyint);
make_udf_function!(round_tinyint_bigint::Func, ROUND_TINYINT_BIGINT, round_tinyint_bigint);
make_udf_function!(rpad_varbinary_bigint_varbinary::Func, RPAD_VARBINARY_BIGINT_VARBINARY, rpad_varbinary_bigint_varbinary);
make_udf_function!(rpad_varchar_bigint_varchar::Func, RPAD_VARCHAR_BIGINT_VARCHAR, rpad_varchar_bigint_varchar);
make_udf_function!(rtrim_varchar::Func, RTRIM_VARCHAR, rtrim_varchar);
make_udf_function!(rtrim_varchar_codepoints::Func, RTRIM_VARCHAR_CODEPOINTS, rtrim_varchar_codepoints);
make_udf_function!(second_intervaldaytosecond::Func, SECOND_INTERVALDAYTOSECOND, second_intervaldaytosecond);
make_udf_function!(second_time_p::Func, SECOND_TIME_P, second_time_p);
make_udf_function!(second_timestamp_p::Func, SECOND_TIMESTAMP_P, second_timestamp_p);
make_udf_function!(sequence_bigint_bigint::Func, SEQUENCE_BIGINT_BIGINT, sequence_bigint_bigint);
make_udf_function!(sequence_bigint_bigint_bigint::Func, SEQUENCE_BIGINT_BIGINT_BIGINT, sequence_bigint_bigint_bigint);
make_udf_function!(sequence_date_date::Func, SEQUENCE_DATE_DATE, sequence_date_date);
make_udf_function!(sequence_date_date_intervaldaytosecond::Func, SEQUENCE_DATE_DATE_INTERVALDAYTOSECOND, sequence_date_date_intervaldaytosecond);
make_udf_function!(sequence_date_date_intervalyeartomonth::Func, SEQUENCE_DATE_DATE_INTERVALYEARTOMONTH, sequence_date_date_intervalyeartomonth);
make_udf_function!(sequence_timestamp_p_timestamp_p_intervaldaytosecond::Func, SEQUENCE_TIMESTAMP_P_TIMESTAMP_P_INTERVALDAYTOSECOND, sequence_timestamp_p_timestamp_p_intervaldaytosecond);
make_udf_function!(sha1_varbinary::Func, SHA1_VARBINARY, sha1_varbinary);
make_udf_function!(sha256_varbinary::Func, SHA256_VARBINARY, sha256_varbinary);
make_udf_function!(sha512_varbinary::Func, SHA512_VARBINARY, sha512_varbinary);
make_udf_function!(shuffle_array_3::Func, SHUFFLE_ARRAY_3, shuffle_array_3);
make_udf_function!(sign_bigint::Func, SIGN_BIGINT, sign_bigint);
make_udf_function!(sign_decimal_p_s::Func, SIGN_DECIMAL_P_S, sign_decimal_p_s);
make_udf_function!(sign_double::Func, SIGN_DOUBLE, sign_double);
make_udf_function!(sign_integer::Func, SIGN_INTEGER, sign_integer);
make_udf_function!(sign_real::Func, SIGN_REAL, sign_real);
make_udf_function!(sign_smallint::Func, SIGN_SMALLINT, sign_smallint);
make_udf_function!(sign_tinyint::Func, SIGN_TINYINT, sign_tinyint);
make_udf_function!(simplify_geometry_geometry_double::Func, SIMPLIFY_GEOMETRY_GEOMETRY_DOUBLE, simplify_geometry_geometry_double);
make_udf_function!(sin_double::Func, SIN_DOUBLE, sin_double);
make_udf_function!(sinh_double::Func, SINH_DOUBLE, sinh_double);
make_udf_function!(slice_array_3_bigint_bigint::Func, SLICE_ARRAY_3_BIGINT_BIGINT, slice_array_3_bigint_bigint);
make_udf_function!(soundex_varchar::Func, SOUNDEX_VARCHAR, soundex_varchar);
make_udf_function!(spatial_partitions_kdbtree_geometry::Func, SPATIAL_PARTITIONS_KDBTREE_GEOMETRY, spatial_partitions_kdbtree_geometry);
make_udf_function!(spatial_partitions_kdbtree_geometry_double::Func, SPATIAL_PARTITIONS_KDBTREE_GEOMETRY_DOUBLE, spatial_partitions_kdbtree_geometry_double);
make_udf_function!(split_varchar_varchar::Func, SPLIT_VARCHAR_VARCHAR, split_varchar_varchar);
make_udf_function!(split_varchar_varchar_bigint::Func, SPLIT_VARCHAR_VARCHAR_BIGINT, split_varchar_varchar_bigint);
make_udf_function!(split_part_varchar_varchar_bigint::Func, SPLIT_PART_VARCHAR_VARCHAR_BIGINT, split_part_varchar_varchar_bigint);
make_udf_function!(split_to_map_varchar_varchar_varchar::Func, SPLIT_TO_MAP_VARCHAR_VARCHAR_VARCHAR, split_to_map_varchar_varchar_varchar);
make_udf_function!(split_to_multimap_varchar_varchar_varchar::Func, SPLIT_TO_MULTIMAP_VARCHAR_VARCHAR_VARCHAR, split_to_multimap_varchar_varchar_varchar);
make_udf_function!(spooky_hash_v2_32_varbinary::Func, SPOOKY_HASH_V2_32_VARBINARY, spooky_hash_v2_32_varbinary);
make_udf_function!(spooky_hash_v2_64_varbinary::Func, SPOOKY_HASH_V2_64_VARBINARY, spooky_hash_v2_64_varbinary);
make_udf_function!(sqrt_double::Func, SQRT_DOUBLE, sqrt_double);
make_udf_function!(st_area_geometry::Func, ST_AREA_GEOMETRY, st_area_geometry);
make_udf_function!(st_area_sphericalgeography::Func, ST_AREA_SPHERICALGEOGRAPHY, st_area_sphericalgeography);
make_udf_function!(st_asbinary_geometry::Func, ST_ASBINARY_GEOMETRY, st_asbinary_geometry);
make_udf_function!(st_astext_geometry::Func, ST_ASTEXT_GEOMETRY, st_astext_geometry);
make_udf_function!(st_boundary_geometry::Func, ST_BOUNDARY_GEOMETRY, st_boundary_geometry);
make_udf_function!(st_buffer_geometry_double::Func, ST_BUFFER_GEOMETRY_DOUBLE, st_buffer_geometry_double);
make_udf_function!(st_centroid_geometry::Func, ST_CENTROID_GEOMETRY, st_centroid_geometry);
make_udf_function!(st_contains_geometry_geometry::Func, ST_CONTAINS_GEOMETRY_GEOMETRY, st_contains_geometry_geometry);
make_udf_function!(st_convexhull_geometry::Func, ST_CONVEXHULL_GEOMETRY, st_convexhull_geometry);
make_udf_function!(st_coorddim_geometry::Func, ST_COORDDIM_GEOMETRY, st_coorddim_geometry);
make_udf_function!(st_crosses_geometry_geometry::Func, ST_CROSSES_GEOMETRY_GEOMETRY, st_crosses_geometry_geometry);
make_udf_function!(st_difference_geometry_geometry::Func, ST_DIFFERENCE_GEOMETRY_GEOMETRY, st_difference_geometry_geometry);
make_udf_function!(st_dimension_geometry::Func, ST_DIMENSION_GEOMETRY, st_dimension_geometry);
make_udf_function!(st_disjoint_geometry_geometry::Func, ST_DISJOINT_GEOMETRY_GEOMETRY, st_disjoint_geometry_geometry);
make_udf_function!(st_distance_geometry_geometry::Func, ST_DISTANCE_GEOMETRY_GEOMETRY, st_distance_geometry_geometry);
make_udf_function!(st_distance_sphericalgeography_sphericalgeography::Func, ST_DISTANCE_SPHERICALGEOGRAPHY_SPHERICALGEOGRAPHY, st_distance_sphericalgeography_sphericalgeography);
make_udf_function!(st_endpoint_geometry::Func, ST_ENDPOINT_GEOMETRY, st_endpoint_geometry);
make_udf_function!(st_envelope_geometry::Func, ST_ENVELOPE_GEOMETRY, st_envelope_geometry);
make_udf_function!(st_envelopeaspts_geometry::Func, ST_ENVELOPEASPTS_GEOMETRY, st_envelopeaspts_geometry);
make_udf_function!(st_equals_geometry_geometry::Func, ST_EQUALS_GEOMETRY_GEOMETRY, st_equals_geometry_geometry);
make_udf_function!(st_exteriorring_geometry::Func, ST_EXTERIORRING_GEOMETRY, st_exteriorring_geometry);
make_udf_function!(st_geometries_geometry::Func, ST_GEOMETRIES_GEOMETRY, st_geometries_geometry);
make_udf_function!(st_geometryfromtext_varchar::Func, ST_GEOMETRYFROMTEXT_VARCHAR, st_geometryfromtext_varchar);
make_udf_function!(st_geometryn_geometry_bigint::Func, ST_GEOMETRYN_GEOMETRY_BIGINT, st_geometryn_geometry_bigint);
make_udf_function!(st_geometrytype_geometry::Func, ST_GEOMETRYTYPE_GEOMETRY, st_geometrytype_geometry);
make_udf_function!(st_geomfrombinary_varbinary::Func, ST_GEOMFROMBINARY_VARBINARY, st_geomfrombinary_varbinary);
make_udf_function!(st_interiorringn_geometry_bigint::Func, ST_INTERIORRINGN_GEOMETRY_BIGINT, st_interiorringn_geometry_bigint);
make_udf_function!(st_interiorrings_geometry::Func, ST_INTERIORRINGS_GEOMETRY, st_interiorrings_geometry);
make_udf_function!(st_intersection_geometry_geometry::Func, ST_INTERSECTION_GEOMETRY_GEOMETRY, st_intersection_geometry_geometry);
make_udf_function!(st_intersects_geometry_geometry::Func, ST_INTERSECTS_GEOMETRY_GEOMETRY, st_intersects_geometry_geometry);
make_udf_function!(st_isclosed_geometry::Func, ST_ISCLOSED_GEOMETRY, st_isclosed_geometry);
make_udf_function!(st_isempty_geometry::Func, ST_ISEMPTY_GEOMETRY, st_isempty_geometry);
make_udf_function!(st_isring_geometry::Func, ST_ISRING_GEOMETRY, st_isring_geometry);
make_udf_function!(st_issimple_geometry::Func, ST_ISSIMPLE_GEOMETRY, st_issimple_geometry);
make_udf_function!(st_isvalid_geometry::Func, ST_ISVALID_GEOMETRY, st_isvalid_geometry);
make_udf_function!(st_length_geometry::Func, ST_LENGTH_GEOMETRY, st_length_geometry);
make_udf_function!(st_length_sphericalgeography::Func, ST_LENGTH_SPHERICALGEOGRAPHY, st_length_sphericalgeography);
make_udf_function!(st_linefromtext_varchar::Func, ST_LINEFROMTEXT_VARCHAR, st_linefromtext_varchar);
make_udf_function!(st_linestring_array_geometry::Func, ST_LINESTRING_ARRAY_GEOMETRY, st_linestring_array_geometry);
make_udf_function!(st_multipoint_array_geometry::Func, ST_MULTIPOINT_ARRAY_GEOMETRY, st_multipoint_array_geometry);
make_udf_function!(st_numgeometries_geometry::Func, ST_NUMGEOMETRIES_GEOMETRY, st_numgeometries_geometry);
make_udf_function!(st_numinteriorring_geometry::Func, ST_NUMINTERIORRING_GEOMETRY, st_numinteriorring_geometry);
make_udf_function!(st_numpoints_geometry::Func, ST_NUMPOINTS_GEOMETRY, st_numpoints_geometry);
make_udf_function!(st_overlaps_geometry_geometry::Func, ST_OVERLAPS_GEOMETRY_GEOMETRY, st_overlaps_geometry_geometry);
make_udf_function!(st_point_double_double::Func, ST_POINT_DOUBLE_DOUBLE, st_point_double_double);
make_udf_function!(st_pointn_geometry_bigint::Func, ST_POINTN_GEOMETRY_BIGINT, st_pointn_geometry_bigint);
make_udf_function!(st_points_geometry::Func, ST_POINTS_GEOMETRY, st_points_geometry);
make_udf_function!(st_polygon_varchar::Func, ST_POLYGON_VARCHAR, st_polygon_varchar);
make_udf_function!(st_relate_geometry_geometry_varchar::Func, ST_RELATE_GEOMETRY_GEOMETRY_VARCHAR, st_relate_geometry_geometry_varchar);
make_udf_function!(st_startpoint_geometry::Func, ST_STARTPOINT_GEOMETRY, st_startpoint_geometry);
make_udf_function!(st_symdifference_geometry_geometry::Func, ST_SYMDIFFERENCE_GEOMETRY_GEOMETRY, st_symdifference_geometry_geometry);
make_udf_function!(st_touches_geometry_geometry::Func, ST_TOUCHES_GEOMETRY_GEOMETRY, st_touches_geometry_geometry);
make_udf_function!(st_union_geometry_geometry::Func, ST_UNION_GEOMETRY_GEOMETRY, st_union_geometry_geometry);
make_udf_function!(st_within_geometry_geometry::Func, ST_WITHIN_GEOMETRY_GEOMETRY, st_within_geometry_geometry);
make_udf_function!(st_x_geometry::Func, ST_X_GEOMETRY, st_x_geometry);
make_udf_function!(st_xmax_geometry::Func, ST_XMAX_GEOMETRY, st_xmax_geometry);
make_udf_function!(st_xmin_geometry::Func, ST_XMIN_GEOMETRY, st_xmin_geometry);
make_udf_function!(st_y_geometry::Func, ST_Y_GEOMETRY, st_y_geometry);
make_udf_function!(st_ymax_geometry::Func, ST_YMAX_GEOMETRY, st_ymax_geometry);
make_udf_function!(st_ymin_geometry::Func, ST_YMIN_GEOMETRY, st_ymin_geometry);
make_udf_function!(starts_with_varchar_varchar::Func, STARTS_WITH_VARCHAR_VARCHAR, starts_with_varchar_varchar);
make_udf_function!(strpos_varchar_varchar::Func, STRPOS_VARCHAR_VARCHAR, strpos_varchar_varchar);
make_udf_function!(strpos_varchar_varchar_bigint::Func, STRPOS_VARCHAR_VARCHAR_BIGINT, strpos_varchar_varchar_bigint);
make_udf_function!(substr_varchar_bigint::Func, SUBSTR_VARCHAR_BIGINT, substr_varchar_bigint);
make_udf_function!(substr_varchar_bigint_bigint::Func, SUBSTR_VARCHAR_BIGINT_BIGINT, substr_varchar_bigint_bigint);
make_udf_function!(substr_varbinary_bigint::Func, SUBSTR_VARBINARY_BIGINT, substr_varbinary_bigint);
make_udf_function!(substr_varbinary_bigint_bigint::Func, SUBSTR_VARBINARY_BIGINT_BIGINT, substr_varbinary_bigint_bigint);
make_udf_function!(substring_varchar_bigint::Func, SUBSTRING_VARCHAR_BIGINT, substring_varchar_bigint);
make_udf_function!(substring_varchar_bigint_bigint::Func, SUBSTRING_VARCHAR_BIGINT_BIGINT, substring_varchar_bigint_bigint);
make_udf_function!(tan_double::Func, TAN_DOUBLE, tan_double);
make_udf_function!(tanh_double::Func, TANH_DOUBLE, tanh_double);
make_udf_function!(timestamp_objectid_timestamp_0::Func, TIMESTAMP_OBJECTID_TIMESTAMP_0, timestamp_objectid_timestamp_0);
make_udf_function!(timezone_hour_time_p::Func, TIMEZONE_HOUR_TIME_P, timezone_hour_time_p);
make_udf_function!(timezone_hour_timestamp_p::Func, TIMEZONE_HOUR_TIMESTAMP_P, timezone_hour_timestamp_p);
make_udf_function!(timezone_minute_time_p::Func, TIMEZONE_MINUTE_TIME_P, timezone_minute_time_p);
make_udf_function!(timezone_minute_timestamp_p::Func, TIMEZONE_MINUTE_TIMESTAMP_P, timezone_minute_timestamp_p);
make_udf_function!(to_base_bigint_bigint::Func, TO_BASE_BIGINT_BIGINT, to_base_bigint_bigint);
make_udf_function!(to_base32_varbinary::Func, TO_BASE32_VARBINARY, to_base32_varbinary);
make_udf_function!(to_base64_varbinary::Func, TO_BASE64_VARBINARY, to_base64_varbinary);
make_udf_function!(to_base64url_varbinary::Func, TO_BASE64URL_VARBINARY, to_base64url_varbinary);
make_udf_function!(to_big_endian_32_bigint::Func, TO_BIG_ENDIAN_32_BIGINT, to_big_endian_32_bigint);
make_udf_function!(to_big_endian_64_bigint::Func, TO_BIG_ENDIAN_64_BIGINT, to_big_endian_64_bigint);
make_udf_function!(to_char_timestamp_p_varchar::Func, TO_CHAR_TIMESTAMP_P_VARCHAR, to_char_timestamp_p_varchar);
make_udf_function!(to_date_varchar_varchar::Func, TO_DATE_VARCHAR_VARCHAR, to_date_varchar_varchar);
make_udf_function!(to_encoded_polyline_geometry::Func, TO_ENCODED_POLYLINE_GEOMETRY, to_encoded_polyline_geometry);
make_udf_function!(to_geojson_geometry_sphericalgeography::Func, TO_GEOJSON_GEOMETRY_SPHERICALGEOGRAPHY, to_geojson_geometry_sphericalgeography);
make_udf_function!(to_geometry_sphericalgeography::Func, TO_GEOMETRY_SPHERICALGEOGRAPHY, to_geometry_sphericalgeography);
make_udf_function!(to_hex_varbinary::Func, TO_HEX_VARBINARY, to_hex_varbinary);
make_udf_function!(to_ieee754_32_real::Func, TO_IEEE754_32_REAL, to_ieee754_32_real);
make_udf_function!(to_ieee754_64_double::Func, TO_IEEE754_64_DOUBLE, to_ieee754_64_double);
make_udf_function!(to_iso8601_date::Func, TO_ISO8601_DATE, to_iso8601_date);
make_udf_function!(to_iso8601_timestamp_p::Func, TO_ISO8601_TIMESTAMP_P, to_iso8601_timestamp_p);
make_udf_function!(to_milliseconds_intervaldaytosecond::Func, TO_MILLISECONDS_INTERVALDAYTOSECOND, to_milliseconds_intervaldaytosecond);
make_udf_function!(to_spherical_geography_geometry::Func, TO_SPHERICAL_GEOGRAPHY_GEOMETRY, to_spherical_geography_geometry);
make_udf_function!(to_timestamp_varchar_varchar::Func, TO_TIMESTAMP_VARCHAR_VARCHAR, to_timestamp_varchar_varchar);
make_udf_function!(to_unixtime_timestamp_p::Func, TO_UNIXTIME_TIMESTAMP_P, to_unixtime_timestamp_p);
make_udf_function!(to_utf8_varchar::Func, TO_UTF8_VARCHAR, to_utf8_varchar);
make_udf_function!(transform_array_1_function_1_11::Func, TRANSFORM_ARRAY_1_FUNCTION_1_11, transform_array_1_function_1_11);
make_udf_function!(transform_keys_map_13_5_function_13_5_12::Func, TRANSFORM_KEYS_MAP_13_5_FUNCTION_13_5_12, transform_keys_map_13_5_function_13_5_12);
make_udf_function!(transform_values_map_4_8_function_4_8_7::Func, TRANSFORM_VALUES_MAP_4_8_FUNCTION_4_8_7, transform_values_map_4_8_function_4_8_7);
make_udf_function!(translate_varchar_varchar_varchar::Func, TRANSLATE_VARCHAR_VARCHAR_VARCHAR, translate_varchar_varchar_varchar);
make_udf_function!(trim_varchar::Func, TRIM_VARCHAR, trim_varchar);
make_udf_function!(trim_varchar_codepoints::Func, TRIM_VARCHAR_CODEPOINTS, trim_varchar_codepoints);
make_udf_function!(trim_array_array_3_bigint::Func, TRIM_ARRAY_ARRAY_3_BIGINT, trim_array_array_3_bigint);
make_udf_function!(truncate_decimal_p_s_bigint::Func, TRUNCATE_DECIMAL_P_S_BIGINT, truncate_decimal_p_s_bigint);
make_udf_function!(truncate_decimal_p_s::Func, TRUNCATE_DECIMAL_P_S, truncate_decimal_p_s);
make_udf_function!(truncate_double::Func, TRUNCATE_DOUBLE, truncate_double);
make_udf_function!(truncate_real::Func, TRUNCATE_REAL, truncate_real);
make_udf_function!(typeof_1::Func, TYPEOF_1, typeof_1);
make_udf_function!(upper_varchar::Func, UPPER_VARCHAR, upper_varchar);
make_udf_function!(url_decode_varchar::Func, URL_DECODE_VARCHAR, url_decode_varchar);
make_udf_function!(url_encode_varchar::Func, URL_ENCODE_VARCHAR, url_encode_varchar);
make_udf_function!(url_extract_fragment_varchar::Func, URL_EXTRACT_FRAGMENT_VARCHAR, url_extract_fragment_varchar);
make_udf_function!(url_extract_host_varchar::Func, URL_EXTRACT_HOST_VARCHAR, url_extract_host_varchar);
make_udf_function!(url_extract_parameter_varchar_varchar::Func, URL_EXTRACT_PARAMETER_VARCHAR_VARCHAR, url_extract_parameter_varchar_varchar);
make_udf_function!(url_extract_path_varchar::Func, URL_EXTRACT_PATH_VARCHAR, url_extract_path_varchar);
make_udf_function!(url_extract_port_varchar::Func, URL_EXTRACT_PORT_VARCHAR, url_extract_port_varchar);
make_udf_function!(url_extract_protocol_varchar::Func, URL_EXTRACT_PROTOCOL_VARCHAR, url_extract_protocol_varchar);
make_udf_function!(url_extract_query_varchar::Func, URL_EXTRACT_QUERY_VARCHAR, url_extract_query_varchar);
make_udf_function!(uuid::Func, UUID, uuid);
make_udf_function!(value_at_quantile_qdigest_double::Func, VALUE_AT_QUANTILE_QDIGEST_DOUBLE, value_at_quantile_qdigest_double);
make_udf_function!(value_at_quantile_tdigest_double::Func, VALUE_AT_QUANTILE_TDIGEST_DOUBLE, value_at_quantile_tdigest_double);
make_udf_function!(values_at_quantiles_qdigest_array_double::Func, VALUES_AT_QUANTILES_QDIGEST_ARRAY_DOUBLE, values_at_quantiles_qdigest_array_double);
make_udf_function!(values_at_quantiles_tdigest_array_double::Func, VALUES_AT_QUANTILES_TDIGEST_ARRAY_DOUBLE, values_at_quantiles_tdigest_array_double);
make_udf_function!(week_date::Func, WEEK_DATE, week_date);
make_udf_function!(week_timestamp_p::Func, WEEK_TIMESTAMP_P, week_timestamp_p);
make_udf_function!(week_of_year_date::Func, WEEK_OF_YEAR_DATE, week_of_year_date);
make_udf_function!(week_of_year_timestamp_p::Func, WEEK_OF_YEAR_TIMESTAMP_P, week_of_year_timestamp_p);
make_udf_function!(width_bucket_double_array_double::Func, WIDTH_BUCKET_DOUBLE_ARRAY_DOUBLE, width_bucket_double_array_double);
make_udf_function!(width_bucket_double_double_double_bigint::Func, WIDTH_BUCKET_DOUBLE_DOUBLE_DOUBLE_BIGINT, width_bucket_double_double_double_bigint);
make_udf_function!(wilson_interval_lower_bigint_bigint_double::Func, WILSON_INTERVAL_LOWER_BIGINT_BIGINT_DOUBLE, wilson_interval_lower_bigint_bigint_double);
make_udf_function!(wilson_interval_upper_bigint_bigint_double::Func, WILSON_INTERVAL_UPPER_BIGINT_BIGINT_DOUBLE, wilson_interval_upper_bigint_bigint_double);
make_udf_function!(with_timezone_timestamp_p_varchar::Func, WITH_TIMEZONE_TIMESTAMP_P_VARCHAR, with_timezone_timestamp_p_varchar);
make_udf_function!(word_stem_varchar::Func, WORD_STEM_VARCHAR, word_stem_varchar);
make_udf_function!(word_stem_varchar_varchar::Func, WORD_STEM_VARCHAR_VARCHAR, word_stem_varchar_varchar);
make_udf_function!(xxhash64_varbinary::Func, XXHASH64_VARBINARY, xxhash64_varbinary);
make_udf_function!(year_date::Func, YEAR_DATE, year_date);
make_udf_function!(year_intervalyeartomonth::Func, YEAR_INTERVALYEARTOMONTH, year_intervalyeartomonth);
make_udf_function!(year_timestamp_p::Func, YEAR_TIMESTAMP_P, year_timestamp_p);
make_udf_function!(year_of_week_date::Func, YEAR_OF_WEEK_DATE, year_of_week_date);
make_udf_function!(year_of_week_timestamp_p::Func, YEAR_OF_WEEK_TIMESTAMP_P, year_of_week_timestamp_p);
make_udf_function!(yow_date::Func, YOW_DATE, yow_date);
make_udf_function!(yow_timestamp_p::Func, YOW_TIMESTAMP_P, yow_timestamp_p);
make_udf_function!(zip_array_14_array_15::Func, ZIP_ARRAY_14_ARRAY_15, zip_array_14_array_15);
make_udf_function!(zip_array_14_array_15_array_16::Func, ZIP_ARRAY_14_ARRAY_15_ARRAY_16, zip_array_14_array_15_array_16);
make_udf_function!(zip_array_14_array_15_array_16_array_17::Func, ZIP_ARRAY_14_ARRAY_15_ARRAY_16_ARRAY_17, zip_array_14_array_15_array_16_array_17);
make_udf_function!(zip_array_14_array_15_array_16_array_17_array_18::Func, ZIP_ARRAY_14_ARRAY_15_ARRAY_16_ARRAY_17_ARRAY_18, zip_array_14_array_15_array_16_array_17_array_18);
make_udf_function!(zip_with_array_1_array_11_function_1_11_9::Func, ZIP_WITH_ARRAY_1_ARRAY_11_FUNCTION_1_11_9, zip_with_array_1_array_11_function_1_11_9);
make_udf_function!(if_boolean_1_1::Func, IF_BOOLEAN_1_1, if_boolean_1_1);
make_udf_function!(try_1::Func, TRY_1, try_1);
make_udf_function!(current_time::Func, CURRENT_TIME, current_time);
make_udf_function!(current_timestamp::Func, CURRENT_TIMESTAMP, current_timestamp);
make_udf_function!(current_timestamp_bigint_0::Func, CURRENT_TIMESTAMP_BIGINT_0, current_timestamp_bigint_0);
make_udf_function!(current_timestamp_bigint_3::Func, CURRENT_TIMESTAMP_BIGINT_3, current_timestamp_bigint_3);
make_udf_function!(current_timestamp_bigint_6::Func, CURRENT_TIMESTAMP_BIGINT_6, current_timestamp_bigint_6);
make_udf_function!(current_timestamp_bigint_9::Func, CURRENT_TIMESTAMP_BIGINT_9, current_timestamp_bigint_9);
make_udf_function!(date_part_varchar_time_p::Func, DATE_PART_VARCHAR_TIME_P, date_part_varchar_time_p);
make_udf_function!(date_part_varchar_timestamp_p::Func, DATE_PART_VARCHAR_TIMESTAMP_P, date_part_varchar_timestamp_p);
make_udf_function!(date_part_varchar_date::Func, DATE_PART_VARCHAR_DATE, date_part_varchar_date);
make_udf_function!(localtime::Func, LOCALTIME, localtime);
make_udf_function!(localtimestamp::Func, LOCALTIMESTAMP, localtimestamp);
make_udf_function!(localtimestamp_bigint_0::Func, LOCALTIMESTAMP_BIGINT_0, localtimestamp_bigint_0);
make_udf_function!(localtimestamp_bigint_3::Func, LOCALTIMESTAMP_BIGINT_3, localtimestamp_bigint_3);
make_udf_function!(localtimestamp_bigint_6::Func, LOCALTIMESTAMP_BIGINT_6, localtimestamp_bigint_6);
make_udf_function!(localtimestamp_bigint_9::Func, LOCALTIMESTAMP_BIGINT_9, localtimestamp_bigint_9);
make_udf_function!(current_user::Func, CURRENT_USER, current_user);
make_udf_function!(current_catalog::Func, CURRENT_CATALOG, current_catalog);
make_udf_function!(current_schema::Func, CURRENT_SCHEMA, current_schema);
make_udf_function!(format_varchar_1::Func, FORMAT_VARCHAR_1, format_varchar_1);
make_udf_function!(format_varchar_1_2::Func, FORMAT_VARCHAR_1_2, format_varchar_1_2);
make_udf_function!(format_varchar_1_2_3::Func, FORMAT_VARCHAR_1_2_3, format_varchar_1_2_3);
make_udf_function!(format_varchar_1_2_3_4::Func, FORMAT_VARCHAR_1_2_3_4, format_varchar_1_2_3_4);
make_udf_function!(format_varchar_1_2_3_4_5::Func, FORMAT_VARCHAR_1_2_3_4_5, format_varchar_1_2_3_4_5);
make_udf_function!(reclassify_T_varchar_varchar::Func, RECLASSIFY_T_VARCHAR_VARCHAR, reclassify_T_varchar_varchar);
make_udf_function!(reclassify_T_varchar::Func, RECLASSIFY_T_VARCHAR, reclassify_T_varchar);
make_udf_function!(btrim_varchar::Func, BTRIM_VARCHAR, btrim_varchar);
make_udf_function!(btrim_varchar_varchar::Func, BTRIM_VARCHAR_VARCHAR, btrim_varchar_varchar);
make_udf_function!(to_timestamp_seconds_1::Func, TO_TIMESTAMP_SECONDS_1, to_timestamp_seconds_1);
make_udf_function!(make_array_1::Func, MAKE_ARRAY_1, make_array_1);
make_udf_function!(length_array_array_1::Func, LENGTH_ARRAY_ARRAY_1, length_array_array_1);


// Export the functions out of this package, both as expr_fn as well as a list of functions
export_functions!(
    (presto, abs_tinyint, arg1, "function doc"),
    (presto, abs_smallint, arg1, "function doc"),
    (presto, abs_bigint, arg1, "function doc"),
    (presto, abs_double, arg1, "function doc"),
    (presto, abs_decimal_p_s, arg1, "function doc"),
    (presto, abs_real, arg1, "function doc"),
    (presto, acos_double, arg1, "function doc"),
    (presto, all_match_array_1_function_1_boolean, arg1 arg2, "function doc"),
    (presto, any_match_array_1_function_1_boolean, arg1 arg2, "function doc"),
    (presto, array_distinct_array_3, arg1, "function doc"),
    (presto, array_except_array_3_array_3, arg1 arg2, "function doc"),
    (presto, array_intersect_array_3_array_3, arg1 arg2, "function doc"),
    (presto, array_join_array_1_varchar, arg1 arg2, "function doc"),
    (presto, array_join_array_1_varchar_varchar, arg1 arg2 arg3, "function doc"),
    (presto, array_max_array_1, arg1, "function doc"),
    (presto, array_min_array_1, arg1, "function doc"),
    (presto, array_position_array_1_1, arg1 arg2, "function doc"),
    (presto, array_remove_array_3_3, arg1 arg2, "function doc"),
    (presto, array_sort_array_3, arg1, "function doc"),
    (presto, array_sort_array_1_function_1_1_bigint, arg1 arg2, "function doc"),
    (presto, array_union_array_3_array_3, arg1 arg2, "function doc"),
    (presto, arrays_overlap_array_3_array_3, arg1 arg2, "function doc"),
    (presto, asin_double, arg1, "function doc"),
    (presto, at_timezone_timestamp_p_varchar, arg1 arg2, "function doc"),
    (presto, atan_double, arg1, "function doc"),
    (presto, atan2_double_double, arg1 arg2, "function doc"),
    (presto, bar_double_bigint, arg1 arg2, "function doc"),
    (presto, bar_double_bigint_color_color, arg1 arg2 arg3 arg4, "function doc"),
    (presto, beta_cdf_double_double_double, arg1 arg2 arg3, "function doc"),
    (presto, bing_tile_bigint_bigint_bigint, arg1 arg2 arg3, "function doc"),
    (presto, bing_tile_varchar, arg1, "function doc"),
    (presto, bing_tile_at_double_double_bigint, arg1 arg2 arg3, "function doc"),
    (presto, bing_tile_coordinates_bingtile, arg1, "function doc"),
    (presto, bing_tile_polygon_bingtile, arg1, "function doc"),
    (presto, bing_tile_quadkey_bingtile, arg1, "function doc"),
    (presto, bing_tile_zoom_level_bingtile, arg1, "function doc"),
    (presto, bing_tiles_around_double_double_bigint, arg1 arg2 arg3, "function doc"),
    (presto, bing_tiles_around_double_double_bigint_double, arg1 arg2 arg3 arg4, "function doc"),
    (presto, bit_count_bigint_bigint, arg1 arg2, "function doc"),
    (presto, bitwise_and_bigint_bigint, arg1 arg2, "function doc"),
    (presto, bitwise_left_shift_bigint_bigint, arg1 arg2, "function doc"),
    (presto, bitwise_left_shift_integer_bigint, arg1 arg2, "function doc"),
    (presto, bitwise_left_shift_smallint_bigint, arg1 arg2, "function doc"),
    (presto, bitwise_left_shift_tinyint_bigint, arg1 arg2, "function doc"),
    (presto, bitwise_not_bigint, arg1, "function doc"),
    (presto, bitwise_or_bigint_bigint, arg1 arg2, "function doc"),
    (presto, bitwise_right_shift_bigint_bigint, arg1 arg2, "function doc"),
    (presto, bitwise_right_shift_integer_bigint, arg1 arg2, "function doc"),
    (presto, bitwise_right_shift_smallint_bigint, arg1 arg2, "function doc"),
    (presto, bitwise_right_shift_tinyint_bigint, arg1 arg2, "function doc"),
    (presto, bitwise_right_shift_arithmetic_bigint_bigint, arg1 arg2, "function doc"),
    (presto, bitwise_right_shift_arithmetic_integer_bigint, arg1 arg2, "function doc"),
    (presto, bitwise_right_shift_arithmetic_smallint_bigint, arg1 arg2, "function doc"),
    (presto, bitwise_right_shift_arithmetic_tinyint_bigint, arg1 arg2, "function doc"),
    (presto, bitwise_xor_bigint_bigint, arg1 arg2, "function doc"),
    (presto, cardinality_array_3, arg1, "function doc"),
    (presto, cardinality_hyperloglog, arg1, "function doc"),
    (presto, cardinality_map_4_5, arg1, "function doc"),
    (presto, cardinality_setdigest, arg1, "function doc"),
    (presto, cbrt_double, arg1, "function doc"),
    (presto, ceil_bigint, arg1, "function doc"),
    (presto, ceil_decimal_p_s, arg1, "function doc"),
    (presto, ceil_double, arg1, "function doc"),
    (presto, ceil_integer, arg1, "function doc"),
    (presto, ceil_real, arg1, "function doc"),
    (presto, ceil_smallint, arg1, "function doc"),
    (presto, ceil_tinyint, arg1, "function doc"),
    (presto, ceiling_bigint, arg1, "function doc"),
    (presto, ceiling_decimal_p_s, arg1, "function doc"),
    (presto, ceiling_double, arg1, "function doc"),
    (presto, ceiling_integer, arg1, "function doc"),
    (presto, ceiling_real, arg1, "function doc"),
    (presto, ceiling_smallint, arg1, "function doc"),
    (presto, ceiling_tinyint, arg1, "function doc"),
    (presto, char2hexint_varchar, arg1, "function doc"),
    (presto, chr_bigint, arg1, "function doc"),
    (presto, classify_map_bigint_double_classifier, arg1 arg2, "function doc"),
    (presto, coalesce_1, arg1, "function doc"),
    (presto, codepoint_varchar, arg1, "function doc"),
    (presto, color_double_color_color, arg1 arg2 arg3, "function doc"),
    (presto, color_double_double_double_color_color, arg1 arg2 arg3 arg4 arg5, "function doc"),
    (presto, color_varchar, arg1, "function doc"),
    (presto, combinations_array_1_bigint, arg1 arg2, "function doc"),
    (presto, concat_3_array_3, arg1 arg2, "function doc"),
    (presto, concat_array_3, arg1, "function doc"),
    (presto, concat_array_3_3, arg1 arg2, "function doc"),
    (presto, concat_varchar_varchar, arg1 arg2, "function doc"),
    (presto, concat_varchar, arg1, "function doc"),
    (presto, concat_varbinary, arg1, "function doc"),
    (presto, concat_ws_varchar_array_varchar, arg1 arg2, "function doc"),
    (presto, concat_ws_varchar, arg1, "function doc"),
    (presto, contains_array_1_1, arg1 arg2, "function doc"),
    (presto, contains_varchar_ipaddress, arg1 arg2, "function doc"),
    (presto, contains_sequence_array_1_array_1, arg1 arg2, "function doc"),
    (presto, cos_double, arg1, "function doc"),
    (presto, cosh_double, arg1, "function doc"),
    (presto, cosine_similarity_map_varchar_double_map_varchar_double, arg1 arg2, "function doc"),
    (presto, crc32_varbinary, arg1, "function doc"),
    (presto, current_date, , "function doc"),
    (presto, current_groups, , "function doc"),
    (presto, current_timezone, , "function doc"),
    (presto, date_timestamp_p, arg1, "function doc"),
    (presto, date_varchar, arg1, "function doc"),
    (presto, date_add_varchar_bigint_date, arg1 arg2 arg3, "function doc"),
    (presto, date_add_varchar_bigint_time_p, arg1 arg2 arg3, "function doc"),
    (presto, date_add_varchar_bigint_timestamp_p, arg1 arg2 arg3, "function doc"),
    (presto, date_diff_varchar_date_date, arg1 arg2 arg3, "function doc"),
    (presto, date_diff_varchar_time_p_time_p, arg1 arg2 arg3, "function doc"),
    (presto, date_diff_varchar_timestamp_p_timestamp_p, arg1 arg2 arg3, "function doc"),
    (presto, date_format_timestamp_p_varchar, arg1 arg2, "function doc"),
    (presto, date_parse_varchar_varchar, arg1 arg2, "function doc"),
    (presto, date_trunc_varchar_time_p, arg1 arg2, "function doc"),
    (presto, date_trunc_varchar_timestamp_p, arg1 arg2, "function doc"),
    (presto, date_trunc_varchar_date, arg1 arg2, "function doc"),
    (presto, day_date, arg1, "function doc"),
    (presto, day_intervaldaytosecond, arg1, "function doc"),
    (presto, day_timestamp_p, arg1, "function doc"),
    (presto, day_of_month_date, arg1, "function doc"),
    (presto, day_of_month_intervaldaytosecond, arg1, "function doc"),
    (presto, day_of_month_timestamp_p, arg1, "function doc"),
    (presto, day_of_week_date, arg1, "function doc"),
    (presto, day_of_week_timestamp_p, arg1, "function doc"),
    (presto, day_of_year_date, arg1, "function doc"),
    (presto, day_of_year_timestamp_p, arg1, "function doc"),
    (presto, degrees_double, arg1, "function doc"),
    (presto, dow_date, arg1, "function doc"),
    (presto, dow_timestamp_p, arg1, "function doc"),
    (presto, doy_date, arg1, "function doc"),
    (presto, doy_timestamp_p, arg1, "function doc"),
    (presto, e, , "function doc"),
    (presto, element_at_map_4_5_4, arg1 arg2, "function doc"),
    (presto, element_at_array_3_bigint, arg1 arg2, "function doc"),
    (presto, empty_approx_set, , "function doc"),
    (presto, exp_double, arg1, "function doc"),
    (presto, features_double, arg1, "function doc"),
    (presto, features_double_double, arg1 arg2, "function doc"),
    (presto, features_double_double_double, arg1 arg2 arg3, "function doc"),
    (presto, features_double_double_double_double, arg1 arg2 arg3 arg4, "function doc"),
    (presto, features_double_double_double_double_double, arg1 arg2 arg3 arg4 arg5, "function doc"),
    (presto, features_double_double_double_double_double_double, arg1 arg2 arg3 arg4 arg5 arg6, "function doc"),
    (presto, features_double_double_double_double_double_double_double, arg1 arg2 arg3 arg4 arg5 arg6 arg7, "function doc"),
    (presto, features_double_double_double_double_double_double_double_double, arg1 arg2 arg3 arg4 arg5 arg6 arg7 arg8, "function doc"),
    (presto, features_double_double_double_double_double_double_double_double_double, arg1 arg2 arg3 arg4 arg5 arg6 arg7 arg8 arg9, "function doc"),
    (presto, features_double_double_double_double_double_double_double_double_double_double, arg1 arg2 arg3 arg4 arg5 arg6 arg7 arg8 arg9 arg10, "function doc"),
    (presto, filter_array_1_function_1_boolean, arg1 arg2, "function doc"),
    (presto, flatten_array_array_3, arg1, "function doc"),
    (presto, floor_bigint, arg1, "function doc"),
    (presto, floor_decimal_p_s, arg1, "function doc"),
    (presto, floor_double, arg1, "function doc"),
    (presto, floor_integer, arg1, "function doc"),
    (presto, floor_real, arg1, "function doc"),
    (presto, floor_smallint, arg1, "function doc"),
    (presto, floor_tinyint, arg1, "function doc"),
    (presto, format_datetime_timestamp_p_varchar, arg1 arg2, "function doc"),
    (presto, format_number_bigint, arg1, "function doc"),
    (presto, format_number_double, arg1, "function doc"),
    (presto, from_base_varchar_bigint, arg1 arg2, "function doc"),
    (presto, from_base32_varbinary, arg1, "function doc"),
    (presto, from_base32_varchar, arg1, "function doc"),
    (presto, from_base64_varbinary, arg1, "function doc"),
    (presto, from_base64_varchar, arg1, "function doc"),
    (presto, from_base64url_varbinary, arg1, "function doc"),
    (presto, from_base64url_varchar, arg1, "function doc"),
    (presto, from_big_endian_32_varbinary, arg1, "function doc"),
    (presto, from_big_endian_64_varbinary, arg1, "function doc"),
    (presto, from_encoded_polyline_varchar, arg1, "function doc"),
    (presto, from_geojson_geometry_varchar, arg1, "function doc"),
    (presto, from_hex_varbinary, arg1, "function doc"),
    (presto, from_hex_varchar, arg1, "function doc"),
    (presto, from_ieee754_32_varbinary, arg1, "function doc"),
    (presto, from_ieee754_64_varbinary, arg1, "function doc"),
    (presto, from_iso8601_date_varchar, arg1, "function doc"),
    (presto, from_iso8601_timestamp_varchar, arg1, "function doc"),
    (presto, from_iso8601_timestamp_nanos_varchar, arg1, "function doc"),
    (presto, from_unixtime_bigint, arg1, "function doc"),
    (presto, from_unixtime_bigint_bigint_bigint, arg1 arg2 arg3, "function doc"),
    (presto, from_unixtime_bigint_varchar, arg1 arg2, "function doc"),
    (presto, from_unixtime_nanos_bigint, arg1, "function doc"),
    (presto, from_unixtime_nanos_decimal_p_s, arg1, "function doc"),
    (presto, from_utf8_varbinary, arg1, "function doc"),
    (presto, from_utf8_varbinary_bigint, arg1 arg2, "function doc"),
    (presto, from_utf8_varbinary_varchar, arg1 arg2, "function doc"),
    (presto, geometry_from_hadoop_shape_varbinary, arg1, "function doc"),
    (presto, geometry_invalid_reason_geometry, arg1, "function doc"),
    (presto, geometry_nearest_points_geometry_geometry, arg1 arg2, "function doc"),
    (presto, geometry_to_bing_tiles_geometry_bigint, arg1 arg2, "function doc"),
    (presto, geometry_union_array_geometry, arg1, "function doc"),
    (presto, great_circle_distance_double_double_double_double, arg1 arg2 arg3 arg4, "function doc"),
    (presto, greatest_3, arg1, "function doc"),
    (presto, hamming_distance_varchar_varchar, arg1 arg2, "function doc"),
    (presto, hash_counts_setdigest, arg1, "function doc"),
    (presto, hmac_md5_varbinary_varbinary, arg1 arg2, "function doc"),
    (presto, hmac_sha1_varbinary_varbinary, arg1 arg2, "function doc"),
    (presto, hmac_sha256_varbinary_varbinary, arg1 arg2, "function doc"),
    (presto, hmac_sha512_varbinary_varbinary, arg1 arg2, "function doc"),
    (presto, hour_intervaldaytosecond, arg1, "function doc"),
    (presto, hour_time_p, arg1, "function doc"),
    (presto, hour_timestamp_p, arg1, "function doc"),
    (presto, human_readable_seconds_double, arg1, "function doc"),
    (presto, index_varchar_varchar, arg1 arg2, "function doc"),
    (presto, infinity, , "function doc"),
    (presto, intersection_cardinality_setdigest_setdigest, arg1 arg2, "function doc"),
    (presto, inverse_beta_cdf_double_double_double, arg1 arg2 arg3, "function doc"),
    (presto, inverse_normal_cdf_double_double_double, arg1 arg2 arg3, "function doc"),
    (presto, is_finite_double, arg1, "function doc"),
    (presto, is_infinite_double, arg1, "function doc"),
    (presto, is_json_scalar_json, arg1, "function doc"),
    (presto, is_json_scalar_varchar, arg1, "function doc"),
    (presto, is_nan_double, arg1, "function doc"),
    (presto, is_nan_real, arg1, "function doc"),
    (presto, jaccard_index_setdigest_setdigest, arg1 arg2, "function doc"),
    (presto, json_array_contains_json_bigint, arg1 arg2, "function doc"),
    (presto, json_array_contains_json_boolean, arg1 arg2, "function doc"),
    (presto, json_array_contains_json_double, arg1 arg2, "function doc"),
    (presto, json_array_contains_json_varchar, arg1 arg2, "function doc"),
    (presto, json_array_contains_varchar_bigint, arg1 arg2, "function doc"),
    (presto, json_array_contains_varchar_boolean, arg1 arg2, "function doc"),
    (presto, json_array_contains_varchar_double, arg1 arg2, "function doc"),
    (presto, json_array_contains_varchar_varchar, arg1 arg2, "function doc"),
    (presto, json_array_get_json_bigint, arg1 arg2, "function doc"),
    (presto, json_array_get_varchar_bigint, arg1 arg2, "function doc"),
    (presto, json_array_length_json, arg1, "function doc"),
    (presto, json_array_length_varchar, arg1, "function doc"),
    (presto, json_extract_json_jsonpath, arg1 arg2, "function doc"),
    (presto, json_extract_varchar_jsonpath, arg1 arg2, "function doc"),
    (presto, json_extract_scalar_json_jsonpath, arg1 arg2, "function doc"),
    (presto, json_extract_scalar_varchar_jsonpath, arg1 arg2, "function doc"),
    (presto, json_format_json, arg1, "function doc"),
    (presto, json_parse_varchar, arg1, "function doc"),
    (presto, json_size_json_jsonpath, arg1 arg2, "function doc"),
    (presto, json_size_varchar_jsonpath, arg1 arg2, "function doc"),
    (presto, last_day_of_month_date, arg1, "function doc"),
    (presto, last_day_of_month_timestamp_p, arg1, "function doc"),
    (presto, least_3, arg1, "function doc"),
    (presto, length_varchar, arg1, "function doc"),
    (presto, length_varbinary, arg1, "function doc"),
    (presto, length_array_1, arg1, "function doc"),
    (presto, levenshtein_distance_varchar_varchar, arg1 arg2, "function doc"),
    (presto, line_interpolate_point_geometry_double, arg1 arg2, "function doc"),
    (presto, line_interpolate_points_geometry_double, arg1 arg2, "function doc"),
    (presto, line_locate_point_geometry_geometry, arg1 arg2, "function doc"),
    (presto, ln_double, arg1, "function doc"),
    (presto, log_double_double, arg1 arg2, "function doc"),
    (presto, log10_double, arg1, "function doc"),
    (presto, log2_double, arg1, "function doc"),
    (presto, lower_varchar, arg1, "function doc"),
    (presto, lpad_varbinary_bigint_varbinary, arg1 arg2 arg3, "function doc"),
    (presto, lpad_varchar_bigint_varchar, arg1 arg2 arg3, "function doc"),
    (presto, ltrim_varchar, arg1, "function doc"),
    (presto, ltrim_varchar_codepoints, arg1 arg2, "function doc"),
    (presto, luhn_check_varchar, arg1, "function doc"),
    (presto, map_array_4_array_5, arg1 arg2, "function doc"),
    (presto, map, , "function doc"),
    (presto, map_concat_map_4_5, arg1, "function doc"),
    (presto, map_entries_map_4_5, arg1, "function doc"),
    (presto, map_filter_map_4_5_function_4_5_boolean, arg1 arg2, "function doc"),
    (presto, map_from_entries_array_row_c04_c15, arg1, "function doc"),
    (presto, map_keys_map_4_5, arg1, "function doc"),
    (presto, map_values_map_4_5, arg1, "function doc"),
    (presto, map_zip_with_map_4_8_map_4_7_function_4_8_7_6, arg1 arg2 arg3, "function doc"),
    (presto, md5_varbinary, arg1, "function doc"),
    (presto, millisecond_intervaldaytosecond, arg1, "function doc"),
    (presto, millisecond_time_p, arg1, "function doc"),
    (presto, millisecond_timestamp_p, arg1, "function doc"),
    (presto, minute_intervaldaytosecond, arg1, "function doc"),
    (presto, minute_time_p, arg1, "function doc"),
    (presto, minute_timestamp_p, arg1, "function doc"),
    (presto, mod_bigint_bigint, arg1 arg2, "function doc"),
    (presto, mod_decimal_a_precision_a_scale_decimal_b_precision_b_scale, arg1 arg2, "function doc"),
    (presto, mod_double_double, arg1 arg2, "function doc"),
    (presto, mod_integer_integer, arg1 arg2, "function doc"),
    (presto, mod_real_real, arg1 arg2, "function doc"),
    (presto, mod_smallint_smallint, arg1 arg2, "function doc"),
    (presto, mod_tinyint_tinyint, arg1 arg2, "function doc"),
    (presto, month_date, arg1, "function doc"),
    (presto, month_intervalyeartomonth, arg1, "function doc"),
    (presto, month_timestamp_p, arg1, "function doc"),
    (presto, multimap_from_entries_array_row_c04_c15, arg1, "function doc"),
    (presto, murmur3_varbinary, arg1, "function doc"),
    (presto, nan, , "function doc"),
    (presto, ngrams_array_1_bigint, arg1 arg2, "function doc"),
    (presto, none_match_array_1_function_1_boolean, arg1 arg2, "function doc"),
    (presto, normal_cdf_double_double_double, arg1 arg2 arg3, "function doc"),
    (presto, normalize_varchar_varchar, arg1 arg2, "function doc"),
    (presto, now, , "function doc"),
    (presto, nullif_1_1, arg1 arg2, "function doc"),
    (presto, objectid, , "function doc"),
    (presto, objectid_varchar, arg1, "function doc"),
    (presto, objectid_timestamp_objectid, arg1, "function doc"),
    (presto, parse_data_size_varchar, arg1, "function doc"),
    (presto, parse_datetime_varchar_varchar, arg1 arg2, "function doc"),
    (presto, parse_duration_varchar, arg1, "function doc"),
    (presto, parse_presto_data_size_varchar, arg1, "function doc"),
    (presto, pi, , "function doc"),
    (presto, pow_double_double, arg1 arg2, "function doc"),
    (presto, power_double_double, arg1 arg2, "function doc"),
    (presto, quantile_at_value_qdigest_bigint, arg1 arg2, "function doc"),
    (presto, quantile_at_value_qdigest_double, arg1 arg2, "function doc"),
    (presto, quantile_at_value_qdigest_real, arg1 arg2, "function doc"),
    (presto, quarter_date, arg1, "function doc"),
    (presto, quarter_timestamp_p, arg1, "function doc"),
    (presto, radians_double, arg1, "function doc"),
    (presto, rand_bigint, arg1, "function doc"),
    (presto, rand_bigint_bigint, arg1 arg2, "function doc"),
    (presto, rand, , "function doc"),
    (presto, rand_integer, arg1, "function doc"),
    (presto, rand_integer_integer, arg1 arg2, "function doc"),
    (presto, rand_smallint, arg1, "function doc"),
    (presto, rand_smallint_smallint, arg1 arg2, "function doc"),
    (presto, rand_tinyint, arg1, "function doc"),
    (presto, rand_tinyint_tinyint, arg1 arg2, "function doc"),
    (presto, random_bigint, arg1, "function doc"),
    (presto, random_bigint_bigint, arg1 arg2, "function doc"),
    (presto, random, , "function doc"),
    (presto, random_integer, arg1, "function doc"),
    (presto, random_integer_integer, arg1 arg2, "function doc"),
    (presto, random_smallint, arg1, "function doc"),
    (presto, random_smallint_smallint, arg1 arg2, "function doc"),
    (presto, random_tinyint, arg1, "function doc"),
    (presto, random_tinyint_tinyint, arg1 arg2, "function doc"),
    (presto, reduce_array_1_10_function_10_1_10_function_10_9, arg1 arg2 arg3 arg4, "function doc"),
    (presto, regexp_count_varchar_joniregexp, arg1 arg2, "function doc"),
    (presto, regexp_extract_varchar_joniregexp, arg1 arg2, "function doc"),
    (presto, regexp_extract_varchar_joniregexp_bigint, arg1 arg2 arg3, "function doc"),
    (presto, regexp_extract_all_varchar_joniregexp, arg1 arg2, "function doc"),
    (presto, regexp_extract_all_varchar_joniregexp_bigint, arg1 arg2 arg3, "function doc"),
    (presto, regexp_like_varchar_joniregexp, arg1 arg2, "function doc"),
    (presto, regexp_position_varchar_joniregexp, arg1 arg2, "function doc"),
    (presto, regexp_position_varchar_joniregexp_bigint, arg1 arg2 arg3, "function doc"),
    (presto, regexp_position_varchar_joniregexp_bigint_bigint, arg1 arg2 arg3 arg4, "function doc"),
    (presto, regexp_replace_varchar_joniregexp_function_array_varchar_varchar, arg1 arg2 arg3, "function doc"),
    (presto, regexp_replace_varchar_joniregexp, arg1 arg2, "function doc"),
    (presto, regexp_replace_varchar_joniregexp_varchar, arg1 arg2 arg3, "function doc"),
    (presto, regexp_split_varchar_joniregexp, arg1 arg2, "function doc"),
    (presto, regress_map_bigint_double_regressor, arg1 arg2, "function doc"),
    (presto, render_boolean, arg1, "function doc"),
    (presto, render_bigint_color, arg1 arg2, "function doc"),
    (presto, render_double_color, arg1 arg2, "function doc"),
    (presto, render_varchar_color, arg1 arg2, "function doc"),
    (presto, repeat_1_bigint, arg1 arg2, "function doc"),
    (presto, replace_varchar_varchar_varchar, arg1 arg2 arg3, "function doc"),
    (presto, replace_varchar_varchar, arg1 arg2, "function doc"),
    (presto, reverse_array_3, arg1, "function doc"),
    (presto, reverse_varbinary, arg1, "function doc"),
    (presto, reverse_varchar, arg1, "function doc"),
    (presto, rgb_bigint_bigint_bigint, arg1 arg2 arg3, "function doc"),
    (presto, round_double, arg1, "function doc"),
    (presto, round_double_bigint, arg1 arg2, "function doc"),
    (presto, round_real, arg1, "function doc"),
    (presto, round_real_bigint, arg1 arg2, "function doc"),
    (presto, round_integer, arg1, "function doc"),
    (presto, round_integer_integer, arg1 arg2, "function doc"),
    (presto, round_decimal_p_s, arg1, "function doc"),
    (presto, round_decimal_p_s_bigint, arg1 arg2, "function doc"),
    (presto, round_bigint, arg1, "function doc"),
    (presto, round_bigint_bigint, arg1 arg2, "function doc"),
    (presto, round_smallint, arg1, "function doc"),
    (presto, round_smallint_bigint, arg1 arg2, "function doc"),
    (presto, round_tinyint, arg1, "function doc"),
    (presto, round_tinyint_bigint, arg1 arg2, "function doc"),
    (presto, rpad_varbinary_bigint_varbinary, arg1 arg2 arg3, "function doc"),
    (presto, rpad_varchar_bigint_varchar, arg1 arg2 arg3, "function doc"),
    (presto, rtrim_varchar, arg1, "function doc"),
    (presto, rtrim_varchar_codepoints, arg1 arg2, "function doc"),
    (presto, second_intervaldaytosecond, arg1, "function doc"),
    (presto, second_time_p, arg1, "function doc"),
    (presto, second_timestamp_p, arg1, "function doc"),
    (presto, sequence_bigint_bigint, arg1 arg2, "function doc"),
    (presto, sequence_bigint_bigint_bigint, arg1 arg2 arg3, "function doc"),
    (presto, sequence_date_date, arg1 arg2, "function doc"),
    (presto, sequence_date_date_intervaldaytosecond, arg1 arg2 arg3, "function doc"),
    (presto, sequence_date_date_intervalyeartomonth, arg1 arg2 arg3, "function doc"),
    (presto, sequence_timestamp_p_timestamp_p_intervaldaytosecond, arg1 arg2 arg3, "function doc"),
    (presto, sha1_varbinary, arg1, "function doc"),
    (presto, sha256_varbinary, arg1, "function doc"),
    (presto, sha512_varbinary, arg1, "function doc"),
    (presto, shuffle_array_3, arg1, "function doc"),
    (presto, sign_bigint, arg1, "function doc"),
    (presto, sign_decimal_p_s, arg1, "function doc"),
    (presto, sign_double, arg1, "function doc"),
    (presto, sign_integer, arg1, "function doc"),
    (presto, sign_real, arg1, "function doc"),
    (presto, sign_smallint, arg1, "function doc"),
    (presto, sign_tinyint, arg1, "function doc"),
    (presto, simplify_geometry_geometry_double, arg1 arg2, "function doc"),
    (presto, sin_double, arg1, "function doc"),
    (presto, sinh_double, arg1, "function doc"),
    (presto, slice_array_3_bigint_bigint, arg1 arg2 arg3, "function doc"),
    (presto, soundex_varchar, arg1, "function doc"),
    (presto, spatial_partitions_kdbtree_geometry, arg1 arg2, "function doc"),
    (presto, spatial_partitions_kdbtree_geometry_double, arg1 arg2 arg3, "function doc"),
    (presto, split_varchar_varchar, arg1 arg2, "function doc"),
    (presto, split_varchar_varchar_bigint, arg1 arg2 arg3, "function doc"),
    (presto, split_part_varchar_varchar_bigint, arg1 arg2 arg3, "function doc"),
    (presto, split_to_map_varchar_varchar_varchar, arg1 arg2 arg3, "function doc"),
    (presto, split_to_multimap_varchar_varchar_varchar, arg1 arg2 arg3, "function doc"),
    (presto, spooky_hash_v2_32_varbinary, arg1, "function doc"),
    (presto, spooky_hash_v2_64_varbinary, arg1, "function doc"),
    (presto, sqrt_double, arg1, "function doc"),
    (presto, st_area_geometry, arg1, "function doc"),
    (presto, st_area_sphericalgeography, arg1, "function doc"),
    (presto, st_asbinary_geometry, arg1, "function doc"),
    (presto, st_astext_geometry, arg1, "function doc"),
    (presto, st_boundary_geometry, arg1, "function doc"),
    (presto, st_buffer_geometry_double, arg1 arg2, "function doc"),
    (presto, st_centroid_geometry, arg1, "function doc"),
    (presto, st_contains_geometry_geometry, arg1 arg2, "function doc"),
    (presto, st_convexhull_geometry, arg1, "function doc"),
    (presto, st_coorddim_geometry, arg1, "function doc"),
    (presto, st_crosses_geometry_geometry, arg1 arg2, "function doc"),
    (presto, st_difference_geometry_geometry, arg1 arg2, "function doc"),
    (presto, st_dimension_geometry, arg1, "function doc"),
    (presto, st_disjoint_geometry_geometry, arg1 arg2, "function doc"),
    (presto, st_distance_geometry_geometry, arg1 arg2, "function doc"),
    (presto, st_distance_sphericalgeography_sphericalgeography, arg1 arg2, "function doc"),
    (presto, st_endpoint_geometry, arg1, "function doc"),
    (presto, st_envelope_geometry, arg1, "function doc"),
    (presto, st_envelopeaspts_geometry, arg1, "function doc"),
    (presto, st_equals_geometry_geometry, arg1 arg2, "function doc"),
    (presto, st_exteriorring_geometry, arg1, "function doc"),
    (presto, st_geometries_geometry, arg1, "function doc"),
    (presto, st_geometryfromtext_varchar, arg1, "function doc"),
    (presto, st_geometryn_geometry_bigint, arg1 arg2, "function doc"),
    (presto, st_geometrytype_geometry, arg1, "function doc"),
    (presto, st_geomfrombinary_varbinary, arg1, "function doc"),
    (presto, st_interiorringn_geometry_bigint, arg1 arg2, "function doc"),
    (presto, st_interiorrings_geometry, arg1, "function doc"),
    (presto, st_intersection_geometry_geometry, arg1 arg2, "function doc"),
    (presto, st_intersects_geometry_geometry, arg1 arg2, "function doc"),
    (presto, st_isclosed_geometry, arg1, "function doc"),
    (presto, st_isempty_geometry, arg1, "function doc"),
    (presto, st_isring_geometry, arg1, "function doc"),
    (presto, st_issimple_geometry, arg1, "function doc"),
    (presto, st_isvalid_geometry, arg1, "function doc"),
    (presto, st_length_geometry, arg1, "function doc"),
    (presto, st_length_sphericalgeography, arg1, "function doc"),
    (presto, st_linefromtext_varchar, arg1, "function doc"),
    (presto, st_linestring_array_geometry, arg1, "function doc"),
    (presto, st_multipoint_array_geometry, arg1, "function doc"),
    (presto, st_numgeometries_geometry, arg1, "function doc"),
    (presto, st_numinteriorring_geometry, arg1, "function doc"),
    (presto, st_numpoints_geometry, arg1, "function doc"),
    (presto, st_overlaps_geometry_geometry, arg1 arg2, "function doc"),
    (presto, st_point_double_double, arg1 arg2, "function doc"),
    (presto, st_pointn_geometry_bigint, arg1 arg2, "function doc"),
    (presto, st_points_geometry, arg1, "function doc"),
    (presto, st_polygon_varchar, arg1, "function doc"),
    (presto, st_relate_geometry_geometry_varchar, arg1 arg2 arg3, "function doc"),
    (presto, st_startpoint_geometry, arg1, "function doc"),
    (presto, st_symdifference_geometry_geometry, arg1 arg2, "function doc"),
    (presto, st_touches_geometry_geometry, arg1 arg2, "function doc"),
    (presto, st_union_geometry_geometry, arg1 arg2, "function doc"),
    (presto, st_within_geometry_geometry, arg1 arg2, "function doc"),
    (presto, st_x_geometry, arg1, "function doc"),
    (presto, st_xmax_geometry, arg1, "function doc"),
    (presto, st_xmin_geometry, arg1, "function doc"),
    (presto, st_y_geometry, arg1, "function doc"),
    (presto, st_ymax_geometry, arg1, "function doc"),
    (presto, st_ymin_geometry, arg1, "function doc"),
    (presto, starts_with_varchar_varchar, arg1 arg2, "function doc"),
    (presto, strpos_varchar_varchar, arg1 arg2, "function doc"),
    (presto, strpos_varchar_varchar_bigint, arg1 arg2 arg3, "function doc"),
    (presto, substr_varchar_bigint, arg1 arg2, "function doc"),
    (presto, substr_varchar_bigint_bigint, arg1 arg2 arg3, "function doc"),
    (presto, substr_varbinary_bigint, arg1 arg2, "function doc"),
    (presto, substr_varbinary_bigint_bigint, arg1 arg2 arg3, "function doc"),
    (presto, substring_varchar_bigint, arg1 arg2, "function doc"),
    (presto, substring_varchar_bigint_bigint, arg1 arg2 arg3, "function doc"),
    (presto, tan_double, arg1, "function doc"),
    (presto, tanh_double, arg1, "function doc"),
    (presto, timestamp_objectid_timestamp_0, arg1, "function doc"),
    (presto, timezone_hour_time_p, arg1, "function doc"),
    (presto, timezone_hour_timestamp_p, arg1, "function doc"),
    (presto, timezone_minute_time_p, arg1, "function doc"),
    (presto, timezone_minute_timestamp_p, arg1, "function doc"),
    (presto, to_base_bigint_bigint, arg1 arg2, "function doc"),
    (presto, to_base32_varbinary, arg1, "function doc"),
    (presto, to_base64_varbinary, arg1, "function doc"),
    (presto, to_base64url_varbinary, arg1, "function doc"),
    (presto, to_big_endian_32_bigint, arg1, "function doc"),
    (presto, to_big_endian_64_bigint, arg1, "function doc"),
    (presto, to_char_timestamp_p_varchar, arg1 arg2, "function doc"),
    (presto, to_date_varchar_varchar, arg1 arg2, "function doc"),
    (presto, to_encoded_polyline_geometry, arg1, "function doc"),
    (presto, to_geojson_geometry_sphericalgeography, arg1, "function doc"),
    (presto, to_geometry_sphericalgeography, arg1, "function doc"),
    (presto, to_hex_varbinary, arg1, "function doc"),
    (presto, to_ieee754_32_real, arg1, "function doc"),
    (presto, to_ieee754_64_double, arg1, "function doc"),
    (presto, to_iso8601_date, arg1, "function doc"),
    (presto, to_iso8601_timestamp_p, arg1, "function doc"),
    (presto, to_milliseconds_intervaldaytosecond, arg1, "function doc"),
    (presto, to_spherical_geography_geometry, arg1, "function doc"),
    (presto, to_timestamp_varchar_varchar, arg1 arg2, "function doc"),
    (presto, to_unixtime_timestamp_p, arg1, "function doc"),
    (presto, to_utf8_varchar, arg1, "function doc"),
    (presto, transform_array_1_function_1_11, arg1 arg2, "function doc"),
    (presto, transform_keys_map_13_5_function_13_5_12, arg1 arg2, "function doc"),
    (presto, transform_values_map_4_8_function_4_8_7, arg1 arg2, "function doc"),
    (presto, translate_varchar_varchar_varchar, arg1 arg2 arg3, "function doc"),
    (presto, trim_varchar, arg1, "function doc"),
    (presto, trim_varchar_codepoints, arg1 arg2, "function doc"),
    (presto, trim_array_array_3_bigint, arg1 arg2, "function doc"),
    (presto, truncate_decimal_p_s_bigint, arg1 arg2, "function doc"),
    (presto, truncate_decimal_p_s, arg1, "function doc"),
    (presto, truncate_double, arg1, "function doc"),
    (presto, truncate_real, arg1, "function doc"),
    (presto, typeof_1, arg1, "function doc"),
    (presto, upper_varchar, arg1, "function doc"),
    (presto, url_decode_varchar, arg1, "function doc"),
    (presto, url_encode_varchar, arg1, "function doc"),
    (presto, url_extract_fragment_varchar, arg1, "function doc"),
    (presto, url_extract_host_varchar, arg1, "function doc"),
    (presto, url_extract_parameter_varchar_varchar, arg1 arg2, "function doc"),
    (presto, url_extract_path_varchar, arg1, "function doc"),
    (presto, url_extract_port_varchar, arg1, "function doc"),
    (presto, url_extract_protocol_varchar, arg1, "function doc"),
    (presto, url_extract_query_varchar, arg1, "function doc"),
    (presto, uuid, , "function doc"),
    (presto, value_at_quantile_qdigest_double, arg1 arg2, "function doc"),
    (presto, value_at_quantile_tdigest_double, arg1 arg2, "function doc"),
    (presto, values_at_quantiles_qdigest_array_double, arg1 arg2, "function doc"),
    (presto, values_at_quantiles_tdigest_array_double, arg1 arg2, "function doc"),
    (presto, week_date, arg1, "function doc"),
    (presto, week_timestamp_p, arg1, "function doc"),
    (presto, week_of_year_date, arg1, "function doc"),
    (presto, week_of_year_timestamp_p, arg1, "function doc"),
    (presto, width_bucket_double_array_double, arg1 arg2, "function doc"),
    (presto, width_bucket_double_double_double_bigint, arg1 arg2 arg3 arg4, "function doc"),
    (presto, wilson_interval_lower_bigint_bigint_double, arg1 arg2 arg3, "function doc"),
    (presto, wilson_interval_upper_bigint_bigint_double, arg1 arg2 arg3, "function doc"),
    (presto, with_timezone_timestamp_p_varchar, arg1 arg2, "function doc"),
    (presto, word_stem_varchar, arg1, "function doc"),
    (presto, word_stem_varchar_varchar, arg1 arg2, "function doc"),
    (presto, xxhash64_varbinary, arg1, "function doc"),
    (presto, year_date, arg1, "function doc"),
    (presto, year_intervalyeartomonth, arg1, "function doc"),
    (presto, year_timestamp_p, arg1, "function doc"),
    (presto, year_of_week_date, arg1, "function doc"),
    (presto, year_of_week_timestamp_p, arg1, "function doc"),
    (presto, yow_date, arg1, "function doc"),
    (presto, yow_timestamp_p, arg1, "function doc"),
    (presto, zip_array_14_array_15, arg1 arg2, "function doc"),
    (presto, zip_array_14_array_15_array_16, arg1 arg2 arg3, "function doc"),
    (presto, zip_array_14_array_15_array_16_array_17, arg1 arg2 arg3 arg4, "function doc"),
    (presto, zip_array_14_array_15_array_16_array_17_array_18, arg1 arg2 arg3 arg4 arg5, "function doc"),
    (presto, zip_with_array_1_array_11_function_1_11_9, arg1 arg2 arg3, "function doc"),
    (presto, if_boolean_1_1, arg1 arg2 arg3, "function doc"),
    (presto, try_1, arg1, "function doc"),
    (presto, current_time, , "function doc"),
    (presto, current_timestamp, , "function doc"),
    (presto, current_timestamp_bigint_0, arg1, "function doc"),
    (presto, current_timestamp_bigint_3, arg1, "function doc"),
    (presto, current_timestamp_bigint_6, arg1, "function doc"),
    (presto, current_timestamp_bigint_9, arg1, "function doc"),
    (presto, date_part_varchar_time_p, arg1 arg2, "function doc"),
    (presto, date_part_varchar_timestamp_p, arg1 arg2, "function doc"),
    (presto, date_part_varchar_date, arg1 arg2, "function doc"),
    (presto, localtime, , "function doc"),
    (presto, localtimestamp, , "function doc"),
    (presto, localtimestamp_bigint_0, arg1, "function doc"),
    (presto, localtimestamp_bigint_3, arg1, "function doc"),
    (presto, localtimestamp_bigint_6, arg1, "function doc"),
    (presto, localtimestamp_bigint_9, arg1, "function doc"),
    (presto, current_user, , "function doc"),
    (presto, current_catalog, , "function doc"),
    (presto, current_schema, , "function doc"),
    (presto, format_varchar_1, arg1 arg2, "function doc"),
    (presto, format_varchar_1_2, arg1 arg2 arg3, "function doc"),
    (presto, format_varchar_1_2_3, arg1 arg2 arg3 arg4, "function doc"),
    (presto, format_varchar_1_2_3_4, arg1 arg2 arg3 arg4 arg5, "function doc"),
    (presto, format_varchar_1_2_3_4_5, arg1 arg2 arg3 arg4 arg5 arg6, "function doc"),
    (presto, reclassify_T_varchar_varchar, arg1 arg2 arg3, "function doc"),
    (presto, reclassify_T_varchar, arg1 arg2, "function doc"),
    (presto, btrim_varchar, arg1, "function doc"),
    (presto, btrim_varchar_varchar, arg1 arg2, "function doc"),
    (presto, to_timestamp_seconds_1, arg1, "function doc"),
    (presto, make_array_1, arg1, "function doc"),
    (presto, length_array_array_1, arg1, "function doc"),
);
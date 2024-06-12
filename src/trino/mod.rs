// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

#![allow(non_snake_case)]
mod abs_impl;
mod acos_impl;
mod all_match_impl;
mod any_match_impl;
mod array_distinct_impl;
mod array_except_impl;
mod array_intersect_impl;
mod array_join_impl;
mod array_max_impl;
mod array_min_impl;
mod array_position_impl;
mod array_remove_impl;
mod array_sort_impl;
mod array_union_impl;
mod arrays_overlap_impl;
mod asin_impl;
mod at_timezone_impl;
mod atan_impl;
mod atan2_impl;
mod bar_impl;
mod beta_cdf_impl;
mod bing_tile_impl;
mod bing_tile_at_impl;
mod bing_tile_coordinates_impl;
mod bing_tile_polygon_impl;
mod bing_tile_quadkey_impl;
mod bing_tile_zoom_level_impl;
mod bing_tiles_around_impl;
mod bit_count_impl;
mod bitwise_and_impl;
mod bitwise_left_shift_impl;
mod bitwise_not_impl;
mod bitwise_or_impl;
mod bitwise_right_shift_impl;
mod bitwise_right_shift_arithmetic_impl;
mod bitwise_xor_impl;
mod cardinality_impl;
mod cbrt_impl;
mod ceil_impl;
mod ceiling_impl;
mod char2hexint_impl;
mod chr_impl;
mod classify_impl;
mod coalesce_impl;
mod codepoint_impl;
mod color_impl;
mod combinations_impl;
mod concat_impl;
mod concat_ws_impl;
mod contains_impl;
mod contains_sequence_impl;
mod cos_impl;
mod cosh_impl;
mod cosine_similarity_impl;
mod crc32_impl;
mod current_catalog_impl;
mod current_date_impl;
mod current_groups_impl;
mod current_schema_impl;
mod current_time_impl;
mod current_timestamp_impl;
mod current_timezone_impl;
mod current_user_impl;
mod date_impl;
mod date_add_impl;
mod date_diff_impl;
mod date_format_impl;
mod date_parse_impl;
mod date_trunc_impl;
mod day_impl;
mod day_of_month_impl;
mod day_of_week_impl;
mod day_of_year_impl;
mod degrees_impl;
mod dow_impl;
mod doy_impl;
mod e_impl;
mod element_at_impl;
mod empty_approx_set_impl;
mod exp_impl;
mod features_impl;
mod filter_impl;
mod flatten_impl;
mod floor_impl;
mod format_impl;
mod format_datetime_impl;
mod format_number_impl;
mod from_base_impl;
mod from_base32_impl;
mod from_base64_impl;
mod from_base64url_impl;
mod from_big_endian_32_impl;
mod from_big_endian_64_impl;
mod from_encoded_polyline_impl;
mod from_geojson_geometry_impl;
mod from_hex_impl;
mod from_ieee754_32_impl;
mod from_ieee754_64_impl;
mod from_iso8601_date_impl;
mod from_iso8601_timestamp_impl;
mod from_iso8601_timestamp_nanos_impl;
mod from_unixtime_impl;
mod from_unixtime_nanos_impl;
mod from_utf8_impl;
mod geometry_from_hadoop_shape_impl;
mod geometry_invalid_reason_impl;
mod geometry_nearest_points_impl;
mod geometry_to_bing_tiles_impl;
mod geometry_union_impl;
mod great_circle_distance_impl;
mod greatest_impl;
mod hamming_distance_impl;
mod hash_counts_impl;
mod hmac_md5_impl;
mod hmac_sha1_impl;
mod hmac_sha256_impl;
mod hmac_sha512_impl;
mod hour_impl;
mod human_readable_seconds_impl;
mod if_impl;
mod index_impl;
mod infinity_impl;
mod intersection_cardinality_impl;
mod inverse_beta_cdf_impl;
mod inverse_normal_cdf_impl;
mod is_finite_impl;
mod is_infinite_impl;
mod is_json_scalar_impl;
mod is_nan_impl;
mod jaccard_index_impl;
mod json_array_contains_impl;
mod json_array_get_impl;
mod json_array_length_impl;
mod json_extract_impl;
mod json_extract_scalar_impl;
mod json_format_impl;
mod json_parse_impl;
mod json_size_impl;
mod last_day_of_month_impl;
mod least_impl;
mod length_impl;
mod levenshtein_distance_impl;
mod line_interpolate_point_impl;
mod line_interpolate_points_impl;
mod line_locate_point_impl;
mod ln_impl;
mod localtime_impl;
mod localtimestamp_impl;
mod log_impl;
mod log10_impl;
mod log2_impl;
mod lower_impl;
mod lpad_impl;
mod ltrim_impl;
mod luhn_check_impl;
mod map_impl;
mod map_concat_impl;
mod map_entries_impl;
mod map_filter_impl;
mod map_from_entries_impl;
mod map_keys_impl;
mod map_values_impl;
mod map_zip_with_impl;
mod md5_impl;
mod millisecond_impl;
mod minute_impl;
mod mod_impl;
mod month_impl;
mod multimap_from_entries_impl;
mod murmur3_impl;
mod nan_impl;
mod ngrams_impl;
mod none_match_impl;
mod normal_cdf_impl;
mod normalize_impl;
mod now_impl;
mod nullif_impl;
mod objectid_impl;
mod objectid_timestamp_impl;
mod parse_data_size_impl;
mod parse_datetime_impl;
mod parse_duration_impl;
mod parse_presto_data_size_impl;
mod pi_impl;
mod pow_impl;
mod power_impl;
mod quantile_at_value_impl;
mod quarter_impl;
mod radians_impl;
mod rand_impl;
mod random_impl;
mod reduce_impl;
mod regexp_count_impl;
mod regexp_extract_impl;
mod regexp_extract_all_impl;
mod regexp_like_impl;
mod regexp_position_impl;
mod regexp_replace_impl;
mod regexp_split_impl;
mod regress_impl;
mod render_impl;
mod repeat_impl;
mod replace_impl;
mod reverse_impl;
mod rgb_impl;
mod round_impl;
mod rpad_impl;
mod rtrim_impl;
mod second_impl;
mod sequence_impl;
mod sha1_impl;
mod sha256_impl;
mod sha512_impl;
mod shuffle_impl;
mod sign_impl;
mod simplify_geometry_impl;
mod sin_impl;
mod sinh_impl;
mod slice_impl;
mod soundex_impl;
mod spatial_partitions_impl;
mod split_impl;
mod split_part_impl;
mod split_to_map_impl;
mod split_to_multimap_impl;
mod spooky_hash_v2_32_impl;
mod spooky_hash_v2_64_impl;
mod sqrt_impl;
mod st_area_impl;
mod st_asbinary_impl;
mod st_astext_impl;
mod st_boundary_impl;
mod st_buffer_impl;
mod st_centroid_impl;
mod st_contains_impl;
mod st_convexhull_impl;
mod st_coorddim_impl;
mod st_crosses_impl;
mod st_difference_impl;
mod st_dimension_impl;
mod st_disjoint_impl;
mod st_distance_impl;
mod st_endpoint_impl;
mod st_envelope_impl;
mod st_envelopeaspts_impl;
mod st_equals_impl;
mod st_exteriorring_impl;
mod st_geometries_impl;
mod st_geometryfromtext_impl;
mod st_geometryn_impl;
mod st_geometrytype_impl;
mod st_geomfrombinary_impl;
mod st_interiorringn_impl;
mod st_interiorrings_impl;
mod st_intersection_impl;
mod st_intersects_impl;
mod st_isclosed_impl;
mod st_isempty_impl;
mod st_isring_impl;
mod st_issimple_impl;
mod st_isvalid_impl;
mod st_length_impl;
mod st_linefromtext_impl;
mod st_linestring_impl;
mod st_multipoint_impl;
mod st_numgeometries_impl;
mod st_numinteriorring_impl;
mod st_numpoints_impl;
mod st_overlaps_impl;
mod st_point_impl;
mod st_pointn_impl;
mod st_points_impl;
mod st_polygon_impl;
mod st_relate_impl;
mod st_startpoint_impl;
mod st_symdifference_impl;
mod st_touches_impl;
mod st_union_impl;
mod st_within_impl;
mod st_x_impl;
mod st_xmax_impl;
mod st_xmin_impl;
mod st_y_impl;
mod st_ymax_impl;
mod st_ymin_impl;
mod starts_with_impl;
mod strpos_impl;
mod substr_impl;
mod substring_impl;
mod tan_impl;
mod tanh_impl;
mod timestamp_objectid_impl;
mod timezone_hour_impl;
mod timezone_minute_impl;
mod to_base_impl;
mod to_base32_impl;
mod to_base64_impl;
mod to_base64url_impl;
mod to_big_endian_32_impl;
mod to_big_endian_64_impl;
mod to_char_impl;
mod to_date_impl;
mod to_encoded_polyline_impl;
mod to_geojson_geometry_impl;
mod to_geometry_impl;
mod to_hex_impl;
mod to_ieee754_32_impl;
mod to_ieee754_64_impl;
mod to_iso8601_impl;
mod to_milliseconds_impl;
mod to_spherical_geography_impl;
mod to_timestamp_impl;
mod to_unixtime_impl;
mod to_utf8_impl;
mod transform_impl;
mod transform_keys_impl;
mod transform_values_impl;
mod translate_impl;
mod trim_impl;
mod trim_array_impl;
mod truncate_impl;
mod try_impl;
mod typeof_impl;
mod upper_impl;
mod url_decode_impl;
mod url_encode_impl;
mod url_extract_fragment_impl;
mod url_extract_host_impl;
mod url_extract_parameter_impl;
mod url_extract_path_impl;
mod url_extract_port_impl;
mod url_extract_protocol_impl;
mod url_extract_query_impl;
mod uuid_impl;
mod value_at_quantile_impl;
mod values_at_quantiles_impl;
mod week_impl;
mod week_of_year_impl;
mod width_bucket_impl;
mod wilson_interval_lower_impl;
mod wilson_interval_upper_impl;
mod with_timezone_impl;
mod word_stem_impl;
mod xxhash64_impl;
mod year_impl;
mod year_of_week_impl;
mod yow_impl;
mod zip_impl;
mod zip_with_impl;


// create  UDFs
make_udf_function!(abs_impl::abs_tinyintFunc, ABS_TINYINT, abs_tinyint);
make_udf_function!(abs_impl::abs_smallintFunc, ABS_SMALLINT, abs_smallint);
make_udf_function!(abs_impl::abs_bigintFunc, ABS_BIGINT, abs_bigint);
make_udf_function!(abs_impl::abs_doubleFunc, ABS_DOUBLE, abs_double);
make_udf_function!(abs_impl::abs_decimal_p_sFunc, ABS_DECIMAL_P_S, abs_decimal_p_s);
make_udf_function!(abs_impl::abs_realFunc, ABS_REAL, abs_real);

make_udf_function!(acos_impl::acos_doubleFunc, ACOS_DOUBLE, acos_double);

make_udf_function!(all_match_impl::all_match_array_1_function_1_booleanFunc, ALL_MATCH_ARRAY_1_FUNCTION_1_BOOLEAN, all_match_array_1_function_1_boolean);

make_udf_function!(any_match_impl::any_match_array_1_function_1_booleanFunc, ANY_MATCH_ARRAY_1_FUNCTION_1_BOOLEAN, any_match_array_1_function_1_boolean);

make_udf_function!(array_distinct_impl::array_distinct_array_3Func, ARRAY_DISTINCT_ARRAY_3, array_distinct_array_3);

make_udf_function!(array_except_impl::array_except_array_3_array_3Func, ARRAY_EXCEPT_ARRAY_3_ARRAY_3, array_except_array_3_array_3);

make_udf_function!(array_intersect_impl::array_intersect_array_3_array_3Func, ARRAY_INTERSECT_ARRAY_3_ARRAY_3, array_intersect_array_3_array_3);

make_udf_function!(array_join_impl::array_join_array_1_varcharFunc, ARRAY_JOIN_ARRAY_1_VARCHAR, array_join_array_1_varchar);
make_udf_function!(array_join_impl::array_join_array_1_varchar_varcharFunc, ARRAY_JOIN_ARRAY_1_VARCHAR_VARCHAR, array_join_array_1_varchar_varchar);

make_udf_function!(array_max_impl::array_max_array_1Func, ARRAY_MAX_ARRAY_1, array_max_array_1);

make_udf_function!(array_min_impl::array_min_array_1Func, ARRAY_MIN_ARRAY_1, array_min_array_1);

make_udf_function!(array_position_impl::array_position_array_1_1Func, ARRAY_POSITION_ARRAY_1_1, array_position_array_1_1);

make_udf_function!(array_remove_impl::array_remove_array_3_3Func, ARRAY_REMOVE_ARRAY_3_3, array_remove_array_3_3);

make_udf_function!(array_sort_impl::array_sort_array_3Func, ARRAY_SORT_ARRAY_3, array_sort_array_3);
make_udf_function!(array_sort_impl::array_sort_array_1_function_1_1_bigintFunc, ARRAY_SORT_ARRAY_1_FUNCTION_1_1_BIGINT, array_sort_array_1_function_1_1_bigint);

make_udf_function!(array_union_impl::array_union_array_3_array_3Func, ARRAY_UNION_ARRAY_3_ARRAY_3, array_union_array_3_array_3);

make_udf_function!(arrays_overlap_impl::arrays_overlap_array_3_array_3Func, ARRAYS_OVERLAP_ARRAY_3_ARRAY_3, arrays_overlap_array_3_array_3);

make_udf_function!(asin_impl::asin_doubleFunc, ASIN_DOUBLE, asin_double);

make_udf_function!(at_timezone_impl::at_timezone_timestamp_p_varcharFunc, AT_TIMEZONE_TIMESTAMP_P_VARCHAR, at_timezone_timestamp_p_varchar);

make_udf_function!(atan_impl::atan_doubleFunc, ATAN_DOUBLE, atan_double);

make_udf_function!(atan2_impl::atan2_double_doubleFunc, ATAN2_DOUBLE_DOUBLE, atan2_double_double);

make_udf_function!(bar_impl::bar_double_bigintFunc, BAR_DOUBLE_BIGINT, bar_double_bigint);
make_udf_function!(bar_impl::bar_double_bigint_color_colorFunc, BAR_DOUBLE_BIGINT_COLOR_COLOR, bar_double_bigint_color_color);

make_udf_function!(beta_cdf_impl::beta_cdf_double_double_doubleFunc, BETA_CDF_DOUBLE_DOUBLE_DOUBLE, beta_cdf_double_double_double);

make_udf_function!(bing_tile_impl::bing_tile_bigint_bigint_bigintFunc, BING_TILE_BIGINT_BIGINT_BIGINT, bing_tile_bigint_bigint_bigint);
make_udf_function!(bing_tile_impl::bing_tile_varcharFunc, BING_TILE_VARCHAR, bing_tile_varchar);

make_udf_function!(bing_tile_at_impl::bing_tile_at_double_double_bigintFunc, BING_TILE_AT_DOUBLE_DOUBLE_BIGINT, bing_tile_at_double_double_bigint);

make_udf_function!(bing_tile_coordinates_impl::bing_tile_coordinates_bingtileFunc, BING_TILE_COORDINATES_BINGTILE, bing_tile_coordinates_bingtile);

make_udf_function!(bing_tile_polygon_impl::bing_tile_polygon_bingtileFunc, BING_TILE_POLYGON_BINGTILE, bing_tile_polygon_bingtile);

make_udf_function!(bing_tile_quadkey_impl::bing_tile_quadkey_bingtileFunc, BING_TILE_QUADKEY_BINGTILE, bing_tile_quadkey_bingtile);

make_udf_function!(bing_tile_zoom_level_impl::bing_tile_zoom_level_bingtileFunc, BING_TILE_ZOOM_LEVEL_BINGTILE, bing_tile_zoom_level_bingtile);

make_udf_function!(bing_tiles_around_impl::bing_tiles_around_double_double_bigintFunc, BING_TILES_AROUND_DOUBLE_DOUBLE_BIGINT, bing_tiles_around_double_double_bigint);
make_udf_function!(bing_tiles_around_impl::bing_tiles_around_double_double_bigint_doubleFunc, BING_TILES_AROUND_DOUBLE_DOUBLE_BIGINT_DOUBLE, bing_tiles_around_double_double_bigint_double);

make_udf_function!(bit_count_impl::bit_count_bigint_bigintFunc, BIT_COUNT_BIGINT_BIGINT, bit_count_bigint_bigint);

make_udf_function!(bitwise_and_impl::bitwise_and_bigint_bigintFunc, BITWISE_AND_BIGINT_BIGINT, bitwise_and_bigint_bigint);

make_udf_function!(bitwise_left_shift_impl::bitwise_left_shift_bigint_bigintFunc, BITWISE_LEFT_SHIFT_BIGINT_BIGINT, bitwise_left_shift_bigint_bigint);
make_udf_function!(bitwise_left_shift_impl::bitwise_left_shift_integer_bigintFunc, BITWISE_LEFT_SHIFT_INTEGER_BIGINT, bitwise_left_shift_integer_bigint);
make_udf_function!(bitwise_left_shift_impl::bitwise_left_shift_smallint_bigintFunc, BITWISE_LEFT_SHIFT_SMALLINT_BIGINT, bitwise_left_shift_smallint_bigint);
make_udf_function!(bitwise_left_shift_impl::bitwise_left_shift_tinyint_bigintFunc, BITWISE_LEFT_SHIFT_TINYINT_BIGINT, bitwise_left_shift_tinyint_bigint);

make_udf_function!(bitwise_not_impl::bitwise_not_bigintFunc, BITWISE_NOT_BIGINT, bitwise_not_bigint);

make_udf_function!(bitwise_or_impl::bitwise_or_bigint_bigintFunc, BITWISE_OR_BIGINT_BIGINT, bitwise_or_bigint_bigint);

make_udf_function!(bitwise_right_shift_impl::bitwise_right_shift_bigint_bigintFunc, BITWISE_RIGHT_SHIFT_BIGINT_BIGINT, bitwise_right_shift_bigint_bigint);
make_udf_function!(bitwise_right_shift_impl::bitwise_right_shift_integer_bigintFunc, BITWISE_RIGHT_SHIFT_INTEGER_BIGINT, bitwise_right_shift_integer_bigint);
make_udf_function!(bitwise_right_shift_impl::bitwise_right_shift_smallint_bigintFunc, BITWISE_RIGHT_SHIFT_SMALLINT_BIGINT, bitwise_right_shift_smallint_bigint);
make_udf_function!(bitwise_right_shift_impl::bitwise_right_shift_tinyint_bigintFunc, BITWISE_RIGHT_SHIFT_TINYINT_BIGINT, bitwise_right_shift_tinyint_bigint);

make_udf_function!(bitwise_right_shift_arithmetic_impl::bitwise_right_shift_arithmetic_bigint_bigintFunc, BITWISE_RIGHT_SHIFT_ARITHMETIC_BIGINT_BIGINT, bitwise_right_shift_arithmetic_bigint_bigint);
make_udf_function!(bitwise_right_shift_arithmetic_impl::bitwise_right_shift_arithmetic_integer_bigintFunc, BITWISE_RIGHT_SHIFT_ARITHMETIC_INTEGER_BIGINT, bitwise_right_shift_arithmetic_integer_bigint);
make_udf_function!(bitwise_right_shift_arithmetic_impl::bitwise_right_shift_arithmetic_smallint_bigintFunc, BITWISE_RIGHT_SHIFT_ARITHMETIC_SMALLINT_BIGINT, bitwise_right_shift_arithmetic_smallint_bigint);
make_udf_function!(bitwise_right_shift_arithmetic_impl::bitwise_right_shift_arithmetic_tinyint_bigintFunc, BITWISE_RIGHT_SHIFT_ARITHMETIC_TINYINT_BIGINT, bitwise_right_shift_arithmetic_tinyint_bigint);

make_udf_function!(bitwise_xor_impl::bitwise_xor_bigint_bigintFunc, BITWISE_XOR_BIGINT_BIGINT, bitwise_xor_bigint_bigint);

make_udf_function!(cardinality_impl::cardinality_array_3Func, CARDINALITY_ARRAY_3, cardinality_array_3);
make_udf_function!(cardinality_impl::cardinality_hyperloglogFunc, CARDINALITY_HYPERLOGLOG, cardinality_hyperloglog);
make_udf_function!(cardinality_impl::cardinality_map_4_5Func, CARDINALITY_MAP_4_5, cardinality_map_4_5);
make_udf_function!(cardinality_impl::cardinality_setdigestFunc, CARDINALITY_SETDIGEST, cardinality_setdigest);

make_udf_function!(cbrt_impl::cbrt_doubleFunc, CBRT_DOUBLE, cbrt_double);

make_udf_function!(ceil_impl::ceil_bigintFunc, CEIL_BIGINT, ceil_bigint);
make_udf_function!(ceil_impl::ceil_decimal_p_sFunc, CEIL_DECIMAL_P_S, ceil_decimal_p_s);
make_udf_function!(ceil_impl::ceil_doubleFunc, CEIL_DOUBLE, ceil_double);
make_udf_function!(ceil_impl::ceil_integerFunc, CEIL_INTEGER, ceil_integer);
make_udf_function!(ceil_impl::ceil_realFunc, CEIL_REAL, ceil_real);
make_udf_function!(ceil_impl::ceil_smallintFunc, CEIL_SMALLINT, ceil_smallint);
make_udf_function!(ceil_impl::ceil_tinyintFunc, CEIL_TINYINT, ceil_tinyint);

make_udf_function!(ceiling_impl::ceiling_bigintFunc, CEILING_BIGINT, ceiling_bigint);
make_udf_function!(ceiling_impl::ceiling_decimal_p_sFunc, CEILING_DECIMAL_P_S, ceiling_decimal_p_s);
make_udf_function!(ceiling_impl::ceiling_doubleFunc, CEILING_DOUBLE, ceiling_double);
make_udf_function!(ceiling_impl::ceiling_integerFunc, CEILING_INTEGER, ceiling_integer);
make_udf_function!(ceiling_impl::ceiling_realFunc, CEILING_REAL, ceiling_real);
make_udf_function!(ceiling_impl::ceiling_smallintFunc, CEILING_SMALLINT, ceiling_smallint);
make_udf_function!(ceiling_impl::ceiling_tinyintFunc, CEILING_TINYINT, ceiling_tinyint);

make_udf_function!(char2hexint_impl::char2hexint_varcharFunc, CHAR2HEXINT_VARCHAR, char2hexint_varchar);

make_udf_function!(chr_impl::chr_bigintFunc, CHR_BIGINT, chr_bigint);

make_udf_function!(classify_impl::classify_map_bigint_double_classifierFunc, CLASSIFY_MAP_BIGINT_DOUBLE_CLASSIFIER, classify_map_bigint_double_classifier);

make_udf_function!(coalesce_impl::coalesce_1Func, COALESCE_1, coalesce_1);

make_udf_function!(codepoint_impl::codepoint_varcharFunc, CODEPOINT_VARCHAR, codepoint_varchar);

make_udf_function!(color_impl::color_double_color_colorFunc, COLOR_DOUBLE_COLOR_COLOR, color_double_color_color);
make_udf_function!(color_impl::color_double_double_double_color_colorFunc, COLOR_DOUBLE_DOUBLE_DOUBLE_COLOR_COLOR, color_double_double_double_color_color);
make_udf_function!(color_impl::color_varcharFunc, COLOR_VARCHAR, color_varchar);

make_udf_function!(combinations_impl::combinations_array_1_bigintFunc, COMBINATIONS_ARRAY_1_BIGINT, combinations_array_1_bigint);

make_udf_function!(concat_impl::concat_3_array_3Func, CONCAT_3_ARRAY_3, concat_3_array_3);
make_udf_function!(concat_impl::concat_array_3Func, CONCAT_ARRAY_3, concat_array_3);
make_udf_function!(concat_impl::concat_array_3_3Func, CONCAT_ARRAY_3_3, concat_array_3_3);
make_udf_function!(concat_impl::concat_varchar_varcharFunc, CONCAT_VARCHAR_VARCHAR, concat_varchar_varchar);
make_udf_function!(concat_impl::concat_varcharFunc, CONCAT_VARCHAR, concat_varchar);
make_udf_function!(concat_impl::concat_varbinaryFunc, CONCAT_VARBINARY, concat_varbinary);

make_udf_function!(concat_ws_impl::concat_ws_varchar_array_varcharFunc, CONCAT_WS_VARCHAR_ARRAY_VARCHAR, concat_ws_varchar_array_varchar);
make_udf_function!(concat_ws_impl::concat_ws_varcharFunc, CONCAT_WS_VARCHAR, concat_ws_varchar);

make_udf_function!(contains_impl::contains_array_1_1Func, CONTAINS_ARRAY_1_1, contains_array_1_1);
make_udf_function!(contains_impl::contains_varchar_ipaddressFunc, CONTAINS_VARCHAR_IPADDRESS, contains_varchar_ipaddress);

make_udf_function!(contains_sequence_impl::contains_sequence_array_1_array_1Func, CONTAINS_SEQUENCE_ARRAY_1_ARRAY_1, contains_sequence_array_1_array_1);

make_udf_function!(cos_impl::cos_doubleFunc, COS_DOUBLE, cos_double);

make_udf_function!(cosh_impl::cosh_doubleFunc, COSH_DOUBLE, cosh_double);

make_udf_function!(cosine_similarity_impl::cosine_similarity_map_varchar_double_map_varchar_doubleFunc, COSINE_SIMILARITY_MAP_VARCHAR_DOUBLE_MAP_VARCHAR_DOUBLE, cosine_similarity_map_varchar_double_map_varchar_double);

make_udf_function!(crc32_impl::crc32_varbinaryFunc, CRC32_VARBINARY, crc32_varbinary);

make_udf_function!(current_catalog_impl::current_catalogFunc, CURRENT_CATALOG, current_catalog);

make_udf_function!(current_date_impl::current_dateFunc, CURRENT_DATE, current_date);

make_udf_function!(current_groups_impl::current_groupsFunc, CURRENT_GROUPS, current_groups);

make_udf_function!(current_schema_impl::current_schemaFunc, CURRENT_SCHEMA, current_schema);

make_udf_function!(current_time_impl::current_timeFunc, CURRENT_TIME, current_time);

make_udf_function!(current_timestamp_impl::current_timestampFunc, CURRENT_TIMESTAMP, current_timestamp);
make_udf_function!(current_timestamp_impl::current_timestamp_bigint_0Func, CURRENT_TIMESTAMP_BIGINT_0, current_timestamp_bigint_0);
make_udf_function!(current_timestamp_impl::current_timestamp_bigint_3Func, CURRENT_TIMESTAMP_BIGINT_3, current_timestamp_bigint_3);
make_udf_function!(current_timestamp_impl::current_timestamp_bigint_6Func, CURRENT_TIMESTAMP_BIGINT_6, current_timestamp_bigint_6);
make_udf_function!(current_timestamp_impl::current_timestamp_bigint_9Func, CURRENT_TIMESTAMP_BIGINT_9, current_timestamp_bigint_9);

make_udf_function!(current_timezone_impl::current_timezoneFunc, CURRENT_TIMEZONE, current_timezone);

make_udf_function!(current_user_impl::current_userFunc, CURRENT_USER, current_user);

make_udf_function!(date_impl::date_timestamp_pFunc, DATE_TIMESTAMP_P, date_timestamp_p);
make_udf_function!(date_impl::date_varcharFunc, DATE_VARCHAR, date_varchar);

make_udf_function!(date_add_impl::date_add_varchar_bigint_dateFunc, DATE_ADD_VARCHAR_BIGINT_DATE, date_add_varchar_bigint_date);
make_udf_function!(date_add_impl::date_add_varchar_bigint_time_pFunc, DATE_ADD_VARCHAR_BIGINT_TIME_P, date_add_varchar_bigint_time_p);
make_udf_function!(date_add_impl::date_add_varchar_bigint_timestamp_pFunc, DATE_ADD_VARCHAR_BIGINT_TIMESTAMP_P, date_add_varchar_bigint_timestamp_p);

make_udf_function!(date_diff_impl::date_diff_varchar_date_dateFunc, DATE_DIFF_VARCHAR_DATE_DATE, date_diff_varchar_date_date);
make_udf_function!(date_diff_impl::date_diff_varchar_time_p_time_pFunc, DATE_DIFF_VARCHAR_TIME_P_TIME_P, date_diff_varchar_time_p_time_p);
make_udf_function!(date_diff_impl::date_diff_varchar_timestamp_p_timestamp_pFunc, DATE_DIFF_VARCHAR_TIMESTAMP_P_TIMESTAMP_P, date_diff_varchar_timestamp_p_timestamp_p);

make_udf_function!(date_format_impl::date_format_timestamp_p_varcharFunc, DATE_FORMAT_TIMESTAMP_P_VARCHAR, date_format_timestamp_p_varchar);

make_udf_function!(date_parse_impl::date_parse_varchar_varcharFunc, DATE_PARSE_VARCHAR_VARCHAR, date_parse_varchar_varchar);

make_udf_function!(date_trunc_impl::date_trunc_varchar_time_pFunc, DATE_TRUNC_VARCHAR_TIME_P, date_trunc_varchar_time_p);
make_udf_function!(date_trunc_impl::date_trunc_varchar_timestamp_pFunc, DATE_TRUNC_VARCHAR_TIMESTAMP_P, date_trunc_varchar_timestamp_p);
make_udf_function!(date_trunc_impl::date_trunc_varchar_dateFunc, DATE_TRUNC_VARCHAR_DATE, date_trunc_varchar_date);

make_udf_function!(day_impl::day_dateFunc, DAY_DATE, day_date);
make_udf_function!(day_impl::day_intervaldaytosecondFunc, DAY_INTERVALDAYTOSECOND, day_intervaldaytosecond);
make_udf_function!(day_impl::day_timestamp_pFunc, DAY_TIMESTAMP_P, day_timestamp_p);

make_udf_function!(day_of_month_impl::day_of_month_dateFunc, DAY_OF_MONTH_DATE, day_of_month_date);
make_udf_function!(day_of_month_impl::day_of_month_intervaldaytosecondFunc, DAY_OF_MONTH_INTERVALDAYTOSECOND, day_of_month_intervaldaytosecond);
make_udf_function!(day_of_month_impl::day_of_month_timestamp_pFunc, DAY_OF_MONTH_TIMESTAMP_P, day_of_month_timestamp_p);

make_udf_function!(day_of_week_impl::day_of_week_dateFunc, DAY_OF_WEEK_DATE, day_of_week_date);
make_udf_function!(day_of_week_impl::day_of_week_timestamp_pFunc, DAY_OF_WEEK_TIMESTAMP_P, day_of_week_timestamp_p);

make_udf_function!(day_of_year_impl::day_of_year_dateFunc, DAY_OF_YEAR_DATE, day_of_year_date);
make_udf_function!(day_of_year_impl::day_of_year_timestamp_pFunc, DAY_OF_YEAR_TIMESTAMP_P, day_of_year_timestamp_p);

make_udf_function!(degrees_impl::degrees_doubleFunc, DEGREES_DOUBLE, degrees_double);

make_udf_function!(dow_impl::dow_dateFunc, DOW_DATE, dow_date);
make_udf_function!(dow_impl::dow_timestamp_pFunc, DOW_TIMESTAMP_P, dow_timestamp_p);

make_udf_function!(doy_impl::doy_dateFunc, DOY_DATE, doy_date);
make_udf_function!(doy_impl::doy_timestamp_pFunc, DOY_TIMESTAMP_P, doy_timestamp_p);

make_udf_function!(e_impl::eFunc, E, e);

make_udf_function!(element_at_impl::element_at_map_4_5_4Func, ELEMENT_AT_MAP_4_5_4, element_at_map_4_5_4);
make_udf_function!(element_at_impl::element_at_array_3_bigintFunc, ELEMENT_AT_ARRAY_3_BIGINT, element_at_array_3_bigint);

make_udf_function!(empty_approx_set_impl::empty_approx_setFunc, EMPTY_APPROX_SET, empty_approx_set);

make_udf_function!(exp_impl::exp_doubleFunc, EXP_DOUBLE, exp_double);

make_udf_function!(features_impl::features_doubleFunc, FEATURES_DOUBLE, features_double);
make_udf_function!(features_impl::features_double_doubleFunc, FEATURES_DOUBLE_DOUBLE, features_double_double);
make_udf_function!(features_impl::features_double_double_doubleFunc, FEATURES_DOUBLE_DOUBLE_DOUBLE, features_double_double_double);
make_udf_function!(features_impl::features_double_double_double_doubleFunc, FEATURES_DOUBLE_DOUBLE_DOUBLE_DOUBLE, features_double_double_double_double);
make_udf_function!(features_impl::features_double_double_double_double_doubleFunc, FEATURES_DOUBLE_DOUBLE_DOUBLE_DOUBLE_DOUBLE, features_double_double_double_double_double);
make_udf_function!(features_impl::features_double_double_double_double_double_doubleFunc, FEATURES_DOUBLE_DOUBLE_DOUBLE_DOUBLE_DOUBLE_DOUBLE, features_double_double_double_double_double_double);
make_udf_function!(features_impl::features_double_double_double_double_double_double_doubleFunc, FEATURES_DOUBLE_DOUBLE_DOUBLE_DOUBLE_DOUBLE_DOUBLE_DOUBLE, features_double_double_double_double_double_double_double);
make_udf_function!(features_impl::features_double_double_double_double_double_double_double_doubleFunc, FEATURES_DOUBLE_DOUBLE_DOUBLE_DOUBLE_DOUBLE_DOUBLE_DOUBLE_DOUBLE, features_double_double_double_double_double_double_double_double);
make_udf_function!(features_impl::features_double_double_double_double_double_double_double_double_doubleFunc, FEATURES_DOUBLE_DOUBLE_DOUBLE_DOUBLE_DOUBLE_DOUBLE_DOUBLE_DOUBLE_DOUBLE, features_double_double_double_double_double_double_double_double_double);
make_udf_function!(features_impl::features_double_double_double_double_double_double_double_double_double_doubleFunc, FEATURES_DOUBLE_DOUBLE_DOUBLE_DOUBLE_DOUBLE_DOUBLE_DOUBLE_DOUBLE_DOUBLE_DOUBLE, features_double_double_double_double_double_double_double_double_double_double);

make_udf_function!(filter_impl::filter_array_1_function_1_booleanFunc, FILTER_ARRAY_1_FUNCTION_1_BOOLEAN, filter_array_1_function_1_boolean);

make_udf_function!(flatten_impl::flatten_array_array_3Func, FLATTEN_ARRAY_ARRAY_3, flatten_array_array_3);

make_udf_function!(floor_impl::floor_bigintFunc, FLOOR_BIGINT, floor_bigint);
make_udf_function!(floor_impl::floor_decimal_p_sFunc, FLOOR_DECIMAL_P_S, floor_decimal_p_s);
make_udf_function!(floor_impl::floor_doubleFunc, FLOOR_DOUBLE, floor_double);
make_udf_function!(floor_impl::floor_integerFunc, FLOOR_INTEGER, floor_integer);
make_udf_function!(floor_impl::floor_realFunc, FLOOR_REAL, floor_real);
make_udf_function!(floor_impl::floor_smallintFunc, FLOOR_SMALLINT, floor_smallint);
make_udf_function!(floor_impl::floor_tinyintFunc, FLOOR_TINYINT, floor_tinyint);

make_udf_function!(format_impl::format_varchar_1Func, FORMAT_VARCHAR_1, format_varchar_1);
make_udf_function!(format_impl::format_varchar_1_2Func, FORMAT_VARCHAR_1_2, format_varchar_1_2);
make_udf_function!(format_impl::format_varchar_1_2_3Func, FORMAT_VARCHAR_1_2_3, format_varchar_1_2_3);
make_udf_function!(format_impl::format_varchar_1_2_3_4Func, FORMAT_VARCHAR_1_2_3_4, format_varchar_1_2_3_4);
make_udf_function!(format_impl::format_varchar_1_2_3_4_5Func, FORMAT_VARCHAR_1_2_3_4_5, format_varchar_1_2_3_4_5);

make_udf_function!(format_datetime_impl::format_datetime_timestamp_p_varcharFunc, FORMAT_DATETIME_TIMESTAMP_P_VARCHAR, format_datetime_timestamp_p_varchar);

make_udf_function!(format_number_impl::format_number_bigintFunc, FORMAT_NUMBER_BIGINT, format_number_bigint);
make_udf_function!(format_number_impl::format_number_doubleFunc, FORMAT_NUMBER_DOUBLE, format_number_double);

make_udf_function!(from_base_impl::from_base_varchar_bigintFunc, FROM_BASE_VARCHAR_BIGINT, from_base_varchar_bigint);

make_udf_function!(from_base32_impl::from_base32_varbinaryFunc, FROM_BASE32_VARBINARY, from_base32_varbinary);
make_udf_function!(from_base32_impl::from_base32_varcharFunc, FROM_BASE32_VARCHAR, from_base32_varchar);

make_udf_function!(from_base64_impl::from_base64_varbinaryFunc, FROM_BASE64_VARBINARY, from_base64_varbinary);
make_udf_function!(from_base64_impl::from_base64_varcharFunc, FROM_BASE64_VARCHAR, from_base64_varchar);

make_udf_function!(from_base64url_impl::from_base64url_varbinaryFunc, FROM_BASE64URL_VARBINARY, from_base64url_varbinary);
make_udf_function!(from_base64url_impl::from_base64url_varcharFunc, FROM_BASE64URL_VARCHAR, from_base64url_varchar);

make_udf_function!(from_big_endian_32_impl::from_big_endian_32_varbinaryFunc, FROM_BIG_ENDIAN_32_VARBINARY, from_big_endian_32_varbinary);

make_udf_function!(from_big_endian_64_impl::from_big_endian_64_varbinaryFunc, FROM_BIG_ENDIAN_64_VARBINARY, from_big_endian_64_varbinary);

make_udf_function!(from_encoded_polyline_impl::from_encoded_polyline_varcharFunc, FROM_ENCODED_POLYLINE_VARCHAR, from_encoded_polyline_varchar);

make_udf_function!(from_geojson_geometry_impl::from_geojson_geometry_varcharFunc, FROM_GEOJSON_GEOMETRY_VARCHAR, from_geojson_geometry_varchar);

make_udf_function!(from_hex_impl::from_hex_varbinaryFunc, FROM_HEX_VARBINARY, from_hex_varbinary);
make_udf_function!(from_hex_impl::from_hex_varcharFunc, FROM_HEX_VARCHAR, from_hex_varchar);

make_udf_function!(from_ieee754_32_impl::from_ieee754_32_varbinaryFunc, FROM_IEEE754_32_VARBINARY, from_ieee754_32_varbinary);

make_udf_function!(from_ieee754_64_impl::from_ieee754_64_varbinaryFunc, FROM_IEEE754_64_VARBINARY, from_ieee754_64_varbinary);

make_udf_function!(from_iso8601_date_impl::from_iso8601_date_varcharFunc, FROM_ISO8601_DATE_VARCHAR, from_iso8601_date_varchar);

make_udf_function!(from_iso8601_timestamp_impl::from_iso8601_timestamp_varcharFunc, FROM_ISO8601_TIMESTAMP_VARCHAR, from_iso8601_timestamp_varchar);

make_udf_function!(from_iso8601_timestamp_nanos_impl::from_iso8601_timestamp_nanos_varcharFunc, FROM_ISO8601_TIMESTAMP_NANOS_VARCHAR, from_iso8601_timestamp_nanos_varchar);

make_udf_function!(from_unixtime_impl::from_unixtime_bigintFunc, FROM_UNIXTIME_BIGINT, from_unixtime_bigint);
make_udf_function!(from_unixtime_impl::from_unixtime_bigint_bigint_bigintFunc, FROM_UNIXTIME_BIGINT_BIGINT_BIGINT, from_unixtime_bigint_bigint_bigint);
make_udf_function!(from_unixtime_impl::from_unixtime_bigint_varcharFunc, FROM_UNIXTIME_BIGINT_VARCHAR, from_unixtime_bigint_varchar);

make_udf_function!(from_unixtime_nanos_impl::from_unixtime_nanos_bigintFunc, FROM_UNIXTIME_NANOS_BIGINT, from_unixtime_nanos_bigint);
make_udf_function!(from_unixtime_nanos_impl::from_unixtime_nanos_decimal_p_sFunc, FROM_UNIXTIME_NANOS_DECIMAL_P_S, from_unixtime_nanos_decimal_p_s);

make_udf_function!(from_utf8_impl::from_utf8_varbinaryFunc, FROM_UTF8_VARBINARY, from_utf8_varbinary);
make_udf_function!(from_utf8_impl::from_utf8_varbinary_bigintFunc, FROM_UTF8_VARBINARY_BIGINT, from_utf8_varbinary_bigint);
make_udf_function!(from_utf8_impl::from_utf8_varbinary_varcharFunc, FROM_UTF8_VARBINARY_VARCHAR, from_utf8_varbinary_varchar);

make_udf_function!(geometry_from_hadoop_shape_impl::geometry_from_hadoop_shape_varbinaryFunc, GEOMETRY_FROM_HADOOP_SHAPE_VARBINARY, geometry_from_hadoop_shape_varbinary);

make_udf_function!(geometry_invalid_reason_impl::geometry_invalid_reason_geometryFunc, GEOMETRY_INVALID_REASON_GEOMETRY, geometry_invalid_reason_geometry);

make_udf_function!(geometry_nearest_points_impl::geometry_nearest_points_geometry_geometryFunc, GEOMETRY_NEAREST_POINTS_GEOMETRY_GEOMETRY, geometry_nearest_points_geometry_geometry);

make_udf_function!(geometry_to_bing_tiles_impl::geometry_to_bing_tiles_geometry_bigintFunc, GEOMETRY_TO_BING_TILES_GEOMETRY_BIGINT, geometry_to_bing_tiles_geometry_bigint);

make_udf_function!(geometry_union_impl::geometry_union_array_geometryFunc, GEOMETRY_UNION_ARRAY_GEOMETRY, geometry_union_array_geometry);

make_udf_function!(great_circle_distance_impl::great_circle_distance_double_double_double_doubleFunc, GREAT_CIRCLE_DISTANCE_DOUBLE_DOUBLE_DOUBLE_DOUBLE, great_circle_distance_double_double_double_double);

make_udf_function!(greatest_impl::greatest_3Func, GREATEST_3, greatest_3);

make_udf_function!(hamming_distance_impl::hamming_distance_varchar_varcharFunc, HAMMING_DISTANCE_VARCHAR_VARCHAR, hamming_distance_varchar_varchar);

make_udf_function!(hash_counts_impl::hash_counts_setdigestFunc, HASH_COUNTS_SETDIGEST, hash_counts_setdigest);

make_udf_function!(hmac_md5_impl::hmac_md5_varbinary_varbinaryFunc, HMAC_MD5_VARBINARY_VARBINARY, hmac_md5_varbinary_varbinary);

make_udf_function!(hmac_sha1_impl::hmac_sha1_varbinary_varbinaryFunc, HMAC_SHA1_VARBINARY_VARBINARY, hmac_sha1_varbinary_varbinary);

make_udf_function!(hmac_sha256_impl::hmac_sha256_varbinary_varbinaryFunc, HMAC_SHA256_VARBINARY_VARBINARY, hmac_sha256_varbinary_varbinary);

make_udf_function!(hmac_sha512_impl::hmac_sha512_varbinary_varbinaryFunc, HMAC_SHA512_VARBINARY_VARBINARY, hmac_sha512_varbinary_varbinary);

make_udf_function!(hour_impl::hour_intervaldaytosecondFunc, HOUR_INTERVALDAYTOSECOND, hour_intervaldaytosecond);
make_udf_function!(hour_impl::hour_time_pFunc, HOUR_TIME_P, hour_time_p);
make_udf_function!(hour_impl::hour_timestamp_pFunc, HOUR_TIMESTAMP_P, hour_timestamp_p);

make_udf_function!(human_readable_seconds_impl::human_readable_seconds_doubleFunc, HUMAN_READABLE_SECONDS_DOUBLE, human_readable_seconds_double);

make_udf_function!(if_impl::if_boolean_1_1Func, IF_BOOLEAN_1_1, if_boolean_1_1);

make_udf_function!(index_impl::index_varchar_varcharFunc, INDEX_VARCHAR_VARCHAR, index_varchar_varchar);

make_udf_function!(infinity_impl::infinityFunc, INFINITY, infinity);

make_udf_function!(intersection_cardinality_impl::intersection_cardinality_setdigest_setdigestFunc, INTERSECTION_CARDINALITY_SETDIGEST_SETDIGEST, intersection_cardinality_setdigest_setdigest);

make_udf_function!(inverse_beta_cdf_impl::inverse_beta_cdf_double_double_doubleFunc, INVERSE_BETA_CDF_DOUBLE_DOUBLE_DOUBLE, inverse_beta_cdf_double_double_double);

make_udf_function!(inverse_normal_cdf_impl::inverse_normal_cdf_double_double_doubleFunc, INVERSE_NORMAL_CDF_DOUBLE_DOUBLE_DOUBLE, inverse_normal_cdf_double_double_double);

make_udf_function!(is_finite_impl::is_finite_doubleFunc, IS_FINITE_DOUBLE, is_finite_double);

make_udf_function!(is_infinite_impl::is_infinite_doubleFunc, IS_INFINITE_DOUBLE, is_infinite_double);

make_udf_function!(is_json_scalar_impl::is_json_scalar_jsonFunc, IS_JSON_SCALAR_JSON, is_json_scalar_json);
make_udf_function!(is_json_scalar_impl::is_json_scalar_varcharFunc, IS_JSON_SCALAR_VARCHAR, is_json_scalar_varchar);

make_udf_function!(is_nan_impl::is_nan_doubleFunc, IS_NAN_DOUBLE, is_nan_double);
make_udf_function!(is_nan_impl::is_nan_realFunc, IS_NAN_REAL, is_nan_real);

make_udf_function!(jaccard_index_impl::jaccard_index_setdigest_setdigestFunc, JACCARD_INDEX_SETDIGEST_SETDIGEST, jaccard_index_setdigest_setdigest);

make_udf_function!(json_array_contains_impl::json_array_contains_json_bigintFunc, JSON_ARRAY_CONTAINS_JSON_BIGINT, json_array_contains_json_bigint);
make_udf_function!(json_array_contains_impl::json_array_contains_json_booleanFunc, JSON_ARRAY_CONTAINS_JSON_BOOLEAN, json_array_contains_json_boolean);
make_udf_function!(json_array_contains_impl::json_array_contains_json_doubleFunc, JSON_ARRAY_CONTAINS_JSON_DOUBLE, json_array_contains_json_double);
make_udf_function!(json_array_contains_impl::json_array_contains_json_varcharFunc, JSON_ARRAY_CONTAINS_JSON_VARCHAR, json_array_contains_json_varchar);
make_udf_function!(json_array_contains_impl::json_array_contains_varchar_bigintFunc, JSON_ARRAY_CONTAINS_VARCHAR_BIGINT, json_array_contains_varchar_bigint);
make_udf_function!(json_array_contains_impl::json_array_contains_varchar_booleanFunc, JSON_ARRAY_CONTAINS_VARCHAR_BOOLEAN, json_array_contains_varchar_boolean);
make_udf_function!(json_array_contains_impl::json_array_contains_varchar_doubleFunc, JSON_ARRAY_CONTAINS_VARCHAR_DOUBLE, json_array_contains_varchar_double);
make_udf_function!(json_array_contains_impl::json_array_contains_varchar_varcharFunc, JSON_ARRAY_CONTAINS_VARCHAR_VARCHAR, json_array_contains_varchar_varchar);

make_udf_function!(json_array_get_impl::json_array_get_json_bigintFunc, JSON_ARRAY_GET_JSON_BIGINT, json_array_get_json_bigint);
make_udf_function!(json_array_get_impl::json_array_get_varchar_bigintFunc, JSON_ARRAY_GET_VARCHAR_BIGINT, json_array_get_varchar_bigint);

make_udf_function!(json_array_length_impl::json_array_length_jsonFunc, JSON_ARRAY_LENGTH_JSON, json_array_length_json);
make_udf_function!(json_array_length_impl::json_array_length_varcharFunc, JSON_ARRAY_LENGTH_VARCHAR, json_array_length_varchar);

make_udf_function!(json_extract_impl::json_extract_json_jsonpathFunc, JSON_EXTRACT_JSON_JSONPATH, json_extract_json_jsonpath);
make_udf_function!(json_extract_impl::json_extract_varchar_jsonpathFunc, JSON_EXTRACT_VARCHAR_JSONPATH, json_extract_varchar_jsonpath);

make_udf_function!(json_extract_scalar_impl::json_extract_scalar_json_jsonpathFunc, JSON_EXTRACT_SCALAR_JSON_JSONPATH, json_extract_scalar_json_jsonpath);
make_udf_function!(json_extract_scalar_impl::json_extract_scalar_varchar_jsonpathFunc, JSON_EXTRACT_SCALAR_VARCHAR_JSONPATH, json_extract_scalar_varchar_jsonpath);

make_udf_function!(json_format_impl::json_format_jsonFunc, JSON_FORMAT_JSON, json_format_json);

make_udf_function!(json_parse_impl::json_parse_varcharFunc, JSON_PARSE_VARCHAR, json_parse_varchar);

make_udf_function!(json_size_impl::json_size_json_jsonpathFunc, JSON_SIZE_JSON_JSONPATH, json_size_json_jsonpath);
make_udf_function!(json_size_impl::json_size_varchar_jsonpathFunc, JSON_SIZE_VARCHAR_JSONPATH, json_size_varchar_jsonpath);

make_udf_function!(last_day_of_month_impl::last_day_of_month_dateFunc, LAST_DAY_OF_MONTH_DATE, last_day_of_month_date);
make_udf_function!(last_day_of_month_impl::last_day_of_month_timestamp_pFunc, LAST_DAY_OF_MONTH_TIMESTAMP_P, last_day_of_month_timestamp_p);

make_udf_function!(least_impl::least_3Func, LEAST_3, least_3);

make_udf_function!(length_impl::length_varcharFunc, LENGTH_VARCHAR, length_varchar);
make_udf_function!(length_impl::length_varbinaryFunc, LENGTH_VARBINARY, length_varbinary);
make_udf_function!(length_impl::length_array_1Func, LENGTH_ARRAY_1, length_array_1);

make_udf_function!(levenshtein_distance_impl::levenshtein_distance_varchar_varcharFunc, LEVENSHTEIN_DISTANCE_VARCHAR_VARCHAR, levenshtein_distance_varchar_varchar);

make_udf_function!(line_interpolate_point_impl::line_interpolate_point_geometry_doubleFunc, LINE_INTERPOLATE_POINT_GEOMETRY_DOUBLE, line_interpolate_point_geometry_double);

make_udf_function!(line_interpolate_points_impl::line_interpolate_points_geometry_doubleFunc, LINE_INTERPOLATE_POINTS_GEOMETRY_DOUBLE, line_interpolate_points_geometry_double);

make_udf_function!(line_locate_point_impl::line_locate_point_geometry_geometryFunc, LINE_LOCATE_POINT_GEOMETRY_GEOMETRY, line_locate_point_geometry_geometry);

make_udf_function!(ln_impl::ln_doubleFunc, LN_DOUBLE, ln_double);

make_udf_function!(localtime_impl::localtimeFunc, LOCALTIME, localtime);

make_udf_function!(localtimestamp_impl::localtimestampFunc, LOCALTIMESTAMP, localtimestamp);
make_udf_function!(localtimestamp_impl::localtimestamp_bigint_0Func, LOCALTIMESTAMP_BIGINT_0, localtimestamp_bigint_0);
make_udf_function!(localtimestamp_impl::localtimestamp_bigint_3Func, LOCALTIMESTAMP_BIGINT_3, localtimestamp_bigint_3);
make_udf_function!(localtimestamp_impl::localtimestamp_bigint_6Func, LOCALTIMESTAMP_BIGINT_6, localtimestamp_bigint_6);
make_udf_function!(localtimestamp_impl::localtimestamp_bigint_9Func, LOCALTIMESTAMP_BIGINT_9, localtimestamp_bigint_9);

make_udf_function!(log_impl::log_double_doubleFunc, LOG_DOUBLE_DOUBLE, log_double_double);

make_udf_function!(log10_impl::log10_doubleFunc, LOG10_DOUBLE, log10_double);

make_udf_function!(log2_impl::log2_doubleFunc, LOG2_DOUBLE, log2_double);

make_udf_function!(lower_impl::lower_varcharFunc, LOWER_VARCHAR, lower_varchar);

make_udf_function!(lpad_impl::lpad_varbinary_bigint_varbinaryFunc, LPAD_VARBINARY_BIGINT_VARBINARY, lpad_varbinary_bigint_varbinary);
make_udf_function!(lpad_impl::lpad_varchar_bigint_varcharFunc, LPAD_VARCHAR_BIGINT_VARCHAR, lpad_varchar_bigint_varchar);

make_udf_function!(ltrim_impl::ltrim_varcharFunc, LTRIM_VARCHAR, ltrim_varchar);
make_udf_function!(ltrim_impl::ltrim_varchar_codepointsFunc, LTRIM_VARCHAR_CODEPOINTS, ltrim_varchar_codepoints);

make_udf_function!(luhn_check_impl::luhn_check_varcharFunc, LUHN_CHECK_VARCHAR, luhn_check_varchar);

make_udf_function!(map_impl::map_array_4_array_5Func, MAP_ARRAY_4_ARRAY_5, map_array_4_array_5);
make_udf_function!(map_impl::mapFunc, MAP, map);

make_udf_function!(map_concat_impl::map_concat_map_4_5Func, MAP_CONCAT_MAP_4_5, map_concat_map_4_5);

make_udf_function!(map_entries_impl::map_entries_map_4_5Func, MAP_ENTRIES_MAP_4_5, map_entries_map_4_5);

make_udf_function!(map_filter_impl::map_filter_map_4_5_function_4_5_booleanFunc, MAP_FILTER_MAP_4_5_FUNCTION_4_5_BOOLEAN, map_filter_map_4_5_function_4_5_boolean);

make_udf_function!(map_from_entries_impl::map_from_entries_array_row_c04_c15Func, MAP_FROM_ENTRIES_ARRAY_ROW_C04_C15, map_from_entries_array_row_c04_c15);

make_udf_function!(map_keys_impl::map_keys_map_4_5Func, MAP_KEYS_MAP_4_5, map_keys_map_4_5);

make_udf_function!(map_values_impl::map_values_map_4_5Func, MAP_VALUES_MAP_4_5, map_values_map_4_5);

make_udf_function!(map_zip_with_impl::map_zip_with_map_4_8_map_4_7_function_4_8_7_6Func, MAP_ZIP_WITH_MAP_4_8_MAP_4_7_FUNCTION_4_8_7_6, map_zip_with_map_4_8_map_4_7_function_4_8_7_6);

make_udf_function!(md5_impl::md5_varbinaryFunc, MD5_VARBINARY, md5_varbinary);

make_udf_function!(millisecond_impl::millisecond_intervaldaytosecondFunc, MILLISECOND_INTERVALDAYTOSECOND, millisecond_intervaldaytosecond);
make_udf_function!(millisecond_impl::millisecond_time_pFunc, MILLISECOND_TIME_P, millisecond_time_p);
make_udf_function!(millisecond_impl::millisecond_timestamp_pFunc, MILLISECOND_TIMESTAMP_P, millisecond_timestamp_p);

make_udf_function!(minute_impl::minute_intervaldaytosecondFunc, MINUTE_INTERVALDAYTOSECOND, minute_intervaldaytosecond);
make_udf_function!(minute_impl::minute_time_pFunc, MINUTE_TIME_P, minute_time_p);
make_udf_function!(minute_impl::minute_timestamp_pFunc, MINUTE_TIMESTAMP_P, minute_timestamp_p);

make_udf_function!(mod_impl::mod_bigint_bigintFunc, MOD_BIGINT_BIGINT, mod_bigint_bigint);
make_udf_function!(mod_impl::mod_decimal_a_precision_a_scale_decimal_b_precision_b_scaleFunc, MOD_DECIMAL_A_PRECISION_A_SCALE_DECIMAL_B_PRECISION_B_SCALE, mod_decimal_a_precision_a_scale_decimal_b_precision_b_scale);
make_udf_function!(mod_impl::mod_double_doubleFunc, MOD_DOUBLE_DOUBLE, mod_double_double);
make_udf_function!(mod_impl::mod_integer_integerFunc, MOD_INTEGER_INTEGER, mod_integer_integer);
make_udf_function!(mod_impl::mod_real_realFunc, MOD_REAL_REAL, mod_real_real);
make_udf_function!(mod_impl::mod_smallint_smallintFunc, MOD_SMALLINT_SMALLINT, mod_smallint_smallint);
make_udf_function!(mod_impl::mod_tinyint_tinyintFunc, MOD_TINYINT_TINYINT, mod_tinyint_tinyint);

make_udf_function!(month_impl::month_dateFunc, MONTH_DATE, month_date);
make_udf_function!(month_impl::month_intervalyeartomonthFunc, MONTH_INTERVALYEARTOMONTH, month_intervalyeartomonth);
make_udf_function!(month_impl::month_timestamp_pFunc, MONTH_TIMESTAMP_P, month_timestamp_p);

make_udf_function!(multimap_from_entries_impl::multimap_from_entries_array_row_c04_c15Func, MULTIMAP_FROM_ENTRIES_ARRAY_ROW_C04_C15, multimap_from_entries_array_row_c04_c15);

make_udf_function!(murmur3_impl::murmur3_varbinaryFunc, MURMUR3_VARBINARY, murmur3_varbinary);

make_udf_function!(nan_impl::nanFunc, NAN, nan);

make_udf_function!(ngrams_impl::ngrams_array_1_bigintFunc, NGRAMS_ARRAY_1_BIGINT, ngrams_array_1_bigint);

make_udf_function!(none_match_impl::none_match_array_1_function_1_booleanFunc, NONE_MATCH_ARRAY_1_FUNCTION_1_BOOLEAN, none_match_array_1_function_1_boolean);

make_udf_function!(normal_cdf_impl::normal_cdf_double_double_doubleFunc, NORMAL_CDF_DOUBLE_DOUBLE_DOUBLE, normal_cdf_double_double_double);

make_udf_function!(normalize_impl::normalize_varchar_varcharFunc, NORMALIZE_VARCHAR_VARCHAR, normalize_varchar_varchar);

make_udf_function!(now_impl::nowFunc, NOW, now);

make_udf_function!(nullif_impl::nullif_1_1Func, NULLIF_1_1, nullif_1_1);

make_udf_function!(objectid_impl::objectidFunc, OBJECTID, objectid);
make_udf_function!(objectid_impl::objectid_varcharFunc, OBJECTID_VARCHAR, objectid_varchar);

make_udf_function!(objectid_timestamp_impl::objectid_timestamp_objectidFunc, OBJECTID_TIMESTAMP_OBJECTID, objectid_timestamp_objectid);

make_udf_function!(parse_data_size_impl::parse_data_size_varcharFunc, PARSE_DATA_SIZE_VARCHAR, parse_data_size_varchar);

make_udf_function!(parse_datetime_impl::parse_datetime_varchar_varcharFunc, PARSE_DATETIME_VARCHAR_VARCHAR, parse_datetime_varchar_varchar);

make_udf_function!(parse_duration_impl::parse_duration_varcharFunc, PARSE_DURATION_VARCHAR, parse_duration_varchar);

make_udf_function!(parse_presto_data_size_impl::parse_presto_data_size_varcharFunc, PARSE_PRETO_DATA_SIZE_VARCHAR, parse_presto_data_size_varchar);

make_udf_function!(pi_impl::piFunc, PI, pi);

make_udf_function!(pow_impl::pow_double_doubleFunc, POW_DOUBLE_DOUBLE, pow_double_double);

make_udf_function!(power_impl::power_double_doubleFunc, POWER_DOUBLE_DOUBLE, power_double_double);

make_udf_function!(quantile_at_value_impl::quantile_at_value_qdigest_bigintFunc, QUANTILE_AT_VALUE_QDIGEST_BIGINT, quantile_at_value_qdigest_bigint);
make_udf_function!(quantile_at_value_impl::quantile_at_value_qdigest_doubleFunc, QUANTILE_AT_VALUE_QDIGEST_DOUBLE, quantile_at_value_qdigest_double);
make_udf_function!(quantile_at_value_impl::quantile_at_value_qdigest_realFunc, QUANTILE_AT_VALUE_QDIGEST_REAL, quantile_at_value_qdigest_real);

make_udf_function!(quarter_impl::quarter_dateFunc, QUARTER_DATE, quarter_date);
make_udf_function!(quarter_impl::quarter_timestamp_pFunc, QUARTER_TIMESTAMP_P, quarter_timestamp_p);

make_udf_function!(radians_impl::radians_doubleFunc, RADIANS_DOUBLE, radians_double);

make_udf_function!(rand_impl::rand_bigintFunc, RAND_BIGINT, rand_bigint);
make_udf_function!(rand_impl::rand_bigint_bigintFunc, RAND_BIGINT_BIGINT, rand_bigint_bigint);
make_udf_function!(rand_impl::randFunc, RAND, rand);
make_udf_function!(rand_impl::rand_integerFunc, RAND_INTEGER, rand_integer);
make_udf_function!(rand_impl::rand_integer_integerFunc, RAND_INTEGER_INTEGER, rand_integer_integer);
make_udf_function!(rand_impl::rand_smallintFunc, RAND_SMALLINT, rand_smallint);
make_udf_function!(rand_impl::rand_smallint_smallintFunc, RAND_SMALLINT_SMALLINT, rand_smallint_smallint);
make_udf_function!(rand_impl::rand_tinyintFunc, RAND_TINYINT, rand_tinyint);
make_udf_function!(rand_impl::rand_tinyint_tinyintFunc, RAND_TINYINT_TINYINT, rand_tinyint_tinyint);

make_udf_function!(random_impl::random_bigintFunc, RANDOM_BIGINT, random_bigint);
make_udf_function!(random_impl::random_bigint_bigintFunc, RANDOM_BIGINT_BIGINT, random_bigint_bigint);
make_udf_function!(random_impl::randomFunc, RANDOM, random);
make_udf_function!(random_impl::random_integerFunc, RANDOM_INTEGER, random_integer);
make_udf_function!(random_impl::random_integer_integerFunc, RANDOM_INTEGER_INTEGER, random_integer_integer);
make_udf_function!(random_impl::random_smallintFunc, RANDOM_SMALLINT, random_smallint);
make_udf_function!(random_impl::random_smallint_smallintFunc, RANDOM_SMALLINT_SMALLINT, random_smallint_smallint);
make_udf_function!(random_impl::random_tinyintFunc, RANDOM_TINYINT, random_tinyint);
make_udf_function!(random_impl::random_tinyint_tinyintFunc, RANDOM_TINYINT_TINYINT, random_tinyint_tinyint);

make_udf_function!(reduce_impl::reduce_array_1_10_function_10_1_10_function_10_9Func, REDUCE_ARRAY_1_10_FUNCTION_10_1_10_FUNCTION_10_9, reduce_array_1_10_function_10_1_10_function_10_9);

make_udf_function!(regexp_count_impl::regexp_count_varchar_joniregexpFunc, REGEXP_COUNT_VARCHAR_JONIREGEXP, regexp_count_varchar_joniregexp);

make_udf_function!(regexp_extract_impl::regexp_extract_varchar_joniregexpFunc, REGEXP_EXTRACT_VARCHAR_JONIREGEXP, regexp_extract_varchar_joniregexp);
make_udf_function!(regexp_extract_impl::regexp_extract_varchar_joniregexp_bigintFunc, REGEXP_EXTRACT_VARCHAR_JONIREGEXP_BIGINT, regexp_extract_varchar_joniregexp_bigint);

make_udf_function!(regexp_extract_all_impl::regexp_extract_all_varchar_joniregexpFunc, REGEXP_EXTRACT_ALL_VARCHAR_JONIREGEXP, regexp_extract_all_varchar_joniregexp);
make_udf_function!(regexp_extract_all_impl::regexp_extract_all_varchar_joniregexp_bigintFunc, REGEXP_EXTRACT_ALL_VARCHAR_JONIREGEXP_BIGINT, regexp_extract_all_varchar_joniregexp_bigint);

make_udf_function!(regexp_like_impl::regexp_like_varchar_joniregexpFunc, REGEXP_LIKE_VARCHAR_JONIREGEXP, regexp_like_varchar_joniregexp);

make_udf_function!(regexp_position_impl::regexp_position_varchar_joniregexpFunc, REGEXP_POSITION_VARCHAR_JONIREGEXP, regexp_position_varchar_joniregexp);
make_udf_function!(regexp_position_impl::regexp_position_varchar_joniregexp_bigintFunc, REGEXP_POSITION_VARCHAR_JONIREGEXP_BIGINT, regexp_position_varchar_joniregexp_bigint);
make_udf_function!(regexp_position_impl::regexp_position_varchar_joniregexp_bigint_bigintFunc, REGEXP_POSITION_VARCHAR_JONIREGEXP_BIGINT_BIGINT, regexp_position_varchar_joniregexp_bigint_bigint);

make_udf_function!(regexp_replace_impl::regexp_replace_varchar_joniregexp_function_array_varchar_varcharFunc, REGEXP_REPLACE_VARCHAR_JONIREGEXP_FUNCTION_ARRAY_VARCHAR_VARCHAR, regexp_replace_varchar_joniregexp_function_array_varchar_varchar);
make_udf_function!(regexp_replace_impl::regexp_replace_varchar_joniregexpFunc, REGEXP_REPLACE_VARCHAR_JONIREGEXP, regexp_replace_varchar_joniregexp);
make_udf_function!(regexp_replace_impl::regexp_replace_varchar_joniregexp_varcharFunc, REGEXP_REPLACE_VARCHAR_JONIREGEXP_VARCHAR, regexp_replace_varchar_joniregexp_varchar);

make_udf_function!(regexp_split_impl::regexp_split_varchar_joniregexpFunc, REGEXP_SPLIT_VARCHAR_JONIREGEXP, regexp_split_varchar_joniregexp);

make_udf_function!(regress_impl::regress_map_bigint_double_regressorFunc, REGRESS_MAP_BIGINT_DOUBLE_REGRESSOR, regress_map_bigint_double_regressor);

make_udf_function!(render_impl::render_booleanFunc, RENDER_BOOLEAN, render_boolean);
make_udf_function!(render_impl::render_bigint_colorFunc, RENDER_BIGINT_COLOR, render_bigint_color);
make_udf_function!(render_impl::render_double_colorFunc, RENDER_DOUBLE_COLOR, render_double_color);
make_udf_function!(render_impl::render_varchar_colorFunc, RENDER_VARCHAR_COLOR, render_varchar_color);

make_udf_function!(repeat_impl::repeat_1_bigintFunc, REPEAT_1_BIGINT, repeat_1_bigint);

make_udf_function!(replace_impl::replace_varchar_varchar_varcharFunc, REPLACE_VARCHAR_VARCHAR_VARCHAR, replace_varchar_varchar_varchar);
make_udf_function!(replace_impl::replace_varchar_varcharFunc, REPLACE_VARCHAR_VARCHAR, replace_varchar_varchar);

make_udf_function!(reverse_impl::reverse_array_3Func, REVERSE_ARRAY_3, reverse_array_3);
make_udf_function!(reverse_impl::reverse_varbinaryFunc, REVERSE_VARBINARY, reverse_varbinary);
make_udf_function!(reverse_impl::reverse_varcharFunc, REVERSE_VARCHAR, reverse_varchar);

make_udf_function!(rgb_impl::rgb_bigint_bigint_bigintFunc, RGB_BIGINT_BIGINT_BIGINT, rgb_bigint_bigint_bigint);

make_udf_function!(round_impl::round_doubleFunc, ROUND_DOUBLE, round_double);
make_udf_function!(round_impl::round_double_bigintFunc, ROUND_DOUBLE_BIGINT, round_double_bigint);
make_udf_function!(round_impl::round_realFunc, ROUND_REAL, round_real);
make_udf_function!(round_impl::round_real_bigintFunc, ROUND_REAL_BIGINT, round_real_bigint);
make_udf_function!(round_impl::round_integerFunc, ROUND_INTEGER, round_integer);
make_udf_function!(round_impl::round_integer_integerFunc, ROUND_INTEGER_INTEGER, round_integer_integer);
make_udf_function!(round_impl::round_decimal_p_sFunc, ROUND_DECIMAL_P_S, round_decimal_p_s);
make_udf_function!(round_impl::round_decimal_p_s_bigintFunc, ROUND_DECIMAL_P_S_BIGINT, round_decimal_p_s_bigint);
make_udf_function!(round_impl::round_bigintFunc, ROUND_BIGINT, round_bigint);
make_udf_function!(round_impl::round_bigint_bigintFunc, ROUND_BIGINT_BIGINT, round_bigint_bigint);
make_udf_function!(round_impl::round_smallintFunc, ROUND_SMALLINT, round_smallint);
make_udf_function!(round_impl::round_smallint_bigintFunc, ROUND_SMALLINT_BIGINT, round_smallint_bigint);
make_udf_function!(round_impl::round_tinyintFunc, ROUND_TINYINT, round_tinyint);
make_udf_function!(round_impl::round_tinyint_bigintFunc, ROUND_TINYINT_BIGINT, round_tinyint_bigint);

make_udf_function!(rpad_impl::rpad_varbinary_bigint_varbinaryFunc, RPAD_VARBINARY_BIGINT_VARBINARY, rpad_varbinary_bigint_varbinary);
make_udf_function!(rpad_impl::rpad_varchar_bigint_varcharFunc, RPAD_VARCHAR_BIGINT_VARCHAR, rpad_varchar_bigint_varchar);

make_udf_function!(rtrim_impl::rtrim_varcharFunc, RTRIM_VARCHAR, rtrim_varchar);
make_udf_function!(rtrim_impl::rtrim_varchar_codepointsFunc, RTRIM_VARCHAR_CODEPOINTS, rtrim_varchar_codepoints);

make_udf_function!(second_impl::second_intervaldaytosecondFunc, SECOND_INTERVALDAYTOSECOND, second_intervaldaytosecond);
make_udf_function!(second_impl::second_time_pFunc, SECOND_TIME_P, second_time_p);
make_udf_function!(second_impl::second_timestamp_pFunc, SECOND_TIMESTAMP_P, second_timestamp_p);

make_udf_function!(sequence_impl::sequence_bigint_bigintFunc, SEQUENCE_BIGINT_BIGINT, sequence_bigint_bigint);
make_udf_function!(sequence_impl::sequence_bigint_bigint_bigintFunc, SEQUENCE_BIGINT_BIGINT_BIGINT, sequence_bigint_bigint_bigint);
make_udf_function!(sequence_impl::sequence_date_dateFunc, SEQUENCE_DATE_DATE, sequence_date_date);
make_udf_function!(sequence_impl::sequence_date_date_intervaldaytosecondFunc, SEQUENCE_DATE_DATE_INTERVALDAYTOSECOND, sequence_date_date_intervaldaytosecond);
make_udf_function!(sequence_impl::sequence_date_date_intervalyeartomonthFunc, SEQUENCE_DATE_DATE_INTERVALYEARTOMONTH, sequence_date_date_intervalyeartomonth);
make_udf_function!(sequence_impl::sequence_timestamp_p_timestamp_p_intervaldaytosecondFunc, SEQUENCE_TIMESTAMP_P_TIMESTAMP_P_INTERVALDAYTOSECOND, sequence_timestamp_p_timestamp_p_intervaldaytosecond);

make_udf_function!(sha1_impl::sha1_varbinaryFunc, SHA1_VARBINARY, sha1_varbinary);

make_udf_function!(sha256_impl::sha256_varbinaryFunc, SHA256_VARBINARY, sha256_varbinary);

make_udf_function!(sha512_impl::sha512_varbinaryFunc, SHA512_VARBINARY, sha512_varbinary);

make_udf_function!(shuffle_impl::shuffle_array_3Func, SHUFFLE_ARRAY_3, shuffle_array_3);

make_udf_function!(sign_impl::sign_bigintFunc, SIGN_BIGINT, sign_bigint);
make_udf_function!(sign_impl::sign_decimal_p_sFunc, SIGN_DECIMAL_P_S, sign_decimal_p_s);
make_udf_function!(sign_impl::sign_doubleFunc, SIGN_DOUBLE, sign_double);
make_udf_function!(sign_impl::sign_integerFunc, SIGN_INTEGER, sign_integer);
make_udf_function!(sign_impl::sign_realFunc, SIGN_REAL, sign_real);
make_udf_function!(sign_impl::sign_smallintFunc, SIGN_SMALLINT, sign_smallint);
make_udf_function!(sign_impl::sign_tinyintFunc, SIGN_TINYINT, sign_tinyint);

make_udf_function!(simplify_geometry_impl::simplify_geometry_geometry_doubleFunc, SIMPLIFY_GEOMETRY_GEOMETRY_DOUBLE, simplify_geometry_geometry_double);

make_udf_function!(sin_impl::sin_doubleFunc, SIN_DOUBLE, sin_double);

make_udf_function!(sinh_impl::sinh_doubleFunc, SINH_DOUBLE, sinh_double);

make_udf_function!(slice_impl::slice_array_3_bigint_bigintFunc, SLICE_ARRAY_3_BIGINT_BIGINT, slice_array_3_bigint_bigint);

make_udf_function!(soundex_impl::soundex_varcharFunc, SOUNDEX_VARCHAR, soundex_varchar);

make_udf_function!(spatial_partitions_impl::spatial_partitions_kdbtree_geometryFunc, SPATIAL_PARTITIONS_KDBTREE_GEOMETRY, spatial_partitions_kdbtree_geometry);
make_udf_function!(spatial_partitions_impl::spatial_partitions_kdbtree_geometry_doubleFunc, SPATIAL_PARTITIONS_KDBTREE_GEOMETRY_DOUBLE, spatial_partitions_kdbtree_geometry_double);

make_udf_function!(split_impl::split_varchar_varcharFunc, SPLIT_VARCHAR_VARCHAR, split_varchar_varchar);
make_udf_function!(split_impl::split_varchar_varchar_bigintFunc, SPLIT_VARCHAR_VARCHAR_BIGINT, split_varchar_varchar_bigint);

make_udf_function!(split_part_impl::split_part_varchar_varchar_bigintFunc, SPLIT_PART_VARCHAR_VARCHAR_BIGINT, split_part_varchar_varchar_bigint);

make_udf_function!(split_to_map_impl::split_to_map_varchar_varchar_varcharFunc, SPLIT_TO_MAP_VARCHAR_VARCHAR_VARCHAR, split_to_map_varchar_varchar_varchar);

make_udf_function!(split_to_multimap_impl::split_to_multimap_varchar_varchar_varcharFunc, SPLIT_TO_MULTIMAP_VARCHAR_VARCHAR_VARCHAR, split_to_multimap_varchar_varchar_varchar);

make_udf_function!(spooky_hash_v2_32_impl::spooky_hash_v2_32_varbinaryFunc, SPOOKY_HASH_V2_32_VARBINARY, spooky_hash_v2_32_varbinary);

make_udf_function!(spooky_hash_v2_64_impl::spooky_hash_v2_64_varbinaryFunc, SPOOKY_HASH_V2_64_VARBINARY, spooky_hash_v2_64_varbinary);

make_udf_function!(sqrt_impl::sqrt_doubleFunc, SQRT_DOUBLE, sqrt_double);

make_udf_function!(st_area_impl::st_area_geometryFunc, ST_AREA_GEOMETRY, st_area_geometry);
make_udf_function!(st_area_impl::st_area_sphericalgeographyFunc, ST_AREA_SPHERICALGEOGRAPHY, st_area_sphericalgeography);

make_udf_function!(st_asbinary_impl::st_asbinary_geometryFunc, ST_ASBINARY_GEOMETRY, st_asbinary_geometry);

make_udf_function!(st_astext_impl::st_astext_geometryFunc, ST_ASTEXT_GEOMETRY, st_astext_geometry);

make_udf_function!(st_boundary_impl::st_boundary_geometryFunc, ST_BOUNDARY_GEOMETRY, st_boundary_geometry);

make_udf_function!(st_buffer_impl::st_buffer_geometry_doubleFunc, ST_BUFFER_GEOMETRY_DOUBLE, st_buffer_geometry_double);

make_udf_function!(st_centroid_impl::st_centroid_geometryFunc, ST_CENTROID_GEOMETRY, st_centroid_geometry);

make_udf_function!(st_contains_impl::st_contains_geometry_geometryFunc, ST_CONTAINS_GEOMETRY_GEOMETRY, st_contains_geometry_geometry);

make_udf_function!(st_convexhull_impl::st_convexhull_geometryFunc, ST_CONVEXHULL_GEOMETRY, st_convexhull_geometry);

make_udf_function!(st_coorddim_impl::st_coorddim_geometryFunc, ST_COORDDIM_GEOMETRY, st_coorddim_geometry);

make_udf_function!(st_crosses_impl::st_crosses_geometry_geometryFunc, ST_CROSSES_GEOMETRY_GEOMETRY, st_crosses_geometry_geometry);

make_udf_function!(st_difference_impl::st_difference_geometry_geometryFunc, ST_DIFFERENCE_GEOMETRY_GEOMETRY, st_difference_geometry_geometry);

make_udf_function!(st_dimension_impl::st_dimension_geometryFunc, ST_DIMENSION_GEOMETRY, st_dimension_geometry);

make_udf_function!(st_disjoint_impl::st_disjoint_geometry_geometryFunc, ST_DISJOINT_GEOMETRY_GEOMETRY, st_disjoint_geometry_geometry);

make_udf_function!(st_distance_impl::st_distance_geometry_geometryFunc, ST_DISTANCE_GEOMETRY_GEOMETRY, st_distance_geometry_geometry);
make_udf_function!(st_distance_impl::st_distance_sphericalgeography_sphericalgeographyFunc, ST_DISTANCE_SPHERICALGEOGRAPHY_SPHERICALGEOGRAPHY, st_distance_sphericalgeography_sphericalgeography);

make_udf_function!(st_endpoint_impl::st_endpoint_geometryFunc, ST_ENDPOINT_GEOMETRY, st_endpoint_geometry);

make_udf_function!(st_envelope_impl::st_envelope_geometryFunc, ST_ENVELOPE_GEOMETRY, st_envelope_geometry);

make_udf_function!(st_envelopeaspts_impl::st_envelopeaspts_geometryFunc, ST_ENVELOPEASPTS_GEOMETRY, st_envelopeaspts_geometry);

make_udf_function!(st_equals_impl::st_equals_geometry_geometryFunc, ST_EQUALS_GEOMETRY_GEOMETRY, st_equals_geometry_geometry);

make_udf_function!(st_exteriorring_impl::st_exteriorring_geometryFunc, ST_EXTERIORRING_GEOMETRY, st_exteriorring_geometry);

make_udf_function!(st_geometries_impl::st_geometries_geometryFunc, ST_GEOMETRIES_GEOMETRY, st_geometries_geometry);

make_udf_function!(st_geometryfromtext_impl::st_geometryfromtext_varcharFunc, ST_GEOMETRYFROMTEXT_VARCHAR, st_geometryfromtext_varchar);

make_udf_function!(st_geometryn_impl::st_geometryn_geometry_bigintFunc, ST_GEOMETRYN_GEOMETRY_BIGINT, st_geometryn_geometry_bigint);

make_udf_function!(st_geometrytype_impl::st_geometrytype_geometryFunc, ST_GEOMETRYTYPE_GEOMETRY, st_geometrytype_geometry);

make_udf_function!(st_geomfrombinary_impl::st_geomfrombinary_varbinaryFunc, ST_GEOMFROMBINARY_VARBINARY, st_geomfrombinary_varbinary);

make_udf_function!(st_interiorringn_impl::st_interiorringn_geometry_bigintFunc, ST_INTERIORRINGN_GEOMETRY_BIGINT, st_interiorringn_geometry_bigint);

make_udf_function!(st_interiorrings_impl::st_interiorrings_geometryFunc, ST_INTERIORRINGS_GEOMETRY, st_interiorrings_geometry);

make_udf_function!(st_intersection_impl::st_intersection_geometry_geometryFunc, ST_INTERSECTION_GEOMETRY_GEOMETRY, st_intersection_geometry_geometry);

make_udf_function!(st_intersects_impl::st_intersects_geometry_geometryFunc, ST_INTERSECTS_GEOMETRY_GEOMETRY, st_intersects_geometry_geometry);

make_udf_function!(st_isclosed_impl::st_isclosed_geometryFunc, ST_ISCLOSED_GEOMETRY, st_isclosed_geometry);

make_udf_function!(st_isempty_impl::st_isempty_geometryFunc, ST_ISEMPTY_GEOMETRY, st_isempty_geometry);

make_udf_function!(st_isring_impl::st_isring_geometryFunc, ST_ISRING_GEOMETRY, st_isring_geometry);

make_udf_function!(st_issimple_impl::st_issimple_geometryFunc, ST_ISSIMPLE_GEOMETRY, st_issimple_geometry);

make_udf_function!(st_isvalid_impl::st_isvalid_geometryFunc, ST_ISVALID_GEOMETRY, st_isvalid_geometry);

make_udf_function!(st_length_impl::st_length_geometryFunc, ST_LENGTH_GEOMETRY, st_length_geometry);
make_udf_function!(st_length_impl::st_length_sphericalgeographyFunc, ST_LENGTH_SPHERICALGEOGRAPHY, st_length_sphericalgeography);

make_udf_function!(st_linefromtext_impl::st_linefromtext_varcharFunc, ST_LINEFROMTEXT_VARCHAR, st_linefromtext_varchar);

make_udf_function!(st_linestring_impl::st_linestring_array_geometryFunc, ST_LINESTRING_ARRAY_GEOMETRY, st_linestring_array_geometry);

make_udf_function!(st_multipoint_impl::st_multipoint_array_geometryFunc, ST_MULTIPOINT_ARRAY_GEOMETRY, st_multipoint_array_geometry);

make_udf_function!(st_numgeometries_impl::st_numgeometries_geometryFunc, ST_NUMGEOMETRIES_GEOMETRY, st_numgeometries_geometry);

make_udf_function!(st_numinteriorring_impl::st_numinteriorring_geometryFunc, ST_NUMINTERIORRING_GEOMETRY, st_numinteriorring_geometry);

make_udf_function!(st_numpoints_impl::st_numpoints_geometryFunc, ST_NUMPOINTS_GEOMETRY, st_numpoints_geometry);

make_udf_function!(st_overlaps_impl::st_overlaps_geometry_geometryFunc, ST_OVERLAPS_GEOMETRY_GEOMETRY, st_overlaps_geometry_geometry);

make_udf_function!(st_point_impl::st_point_double_doubleFunc, ST_POINT_DOUBLE_DOUBLE, st_point_double_double);

make_udf_function!(st_pointn_impl::st_pointn_geometry_bigintFunc, ST_POINTN_GEOMETRY_BIGINT, st_pointn_geometry_bigint);

make_udf_function!(st_points_impl::st_points_geometryFunc, ST_POINTS_GEOMETRY, st_points_geometry);

make_udf_function!(st_polygon_impl::st_polygon_varcharFunc, ST_POLYGON_VARCHAR, st_polygon_varchar);

make_udf_function!(st_relate_impl::st_relate_geometry_geometry_varcharFunc, ST_RELATE_GEOMETRY_GEOMETRY_VARCHAR, st_relate_geometry_geometry_varchar);

make_udf_function!(st_startpoint_impl::st_startpoint_geometryFunc, ST_STARTPOINT_GEOMETRY, st_startpoint_geometry);

make_udf_function!(st_symdifference_impl::st_symdifference_geometry_geometryFunc, ST_SYMDIFFERENCE_GEOMETRY_GEOMETRY, st_symdifference_geometry_geometry);

make_udf_function!(st_touches_impl::st_touches_geometry_geometryFunc, ST_TOUCHES_GEOMETRY_GEOMETRY, st_touches_geometry_geometry);

make_udf_function!(st_union_impl::st_union_geometry_geometryFunc, ST_UNION_GEOMETRY_GEOMETRY, st_union_geometry_geometry);

make_udf_function!(st_within_impl::st_within_geometry_geometryFunc, ST_WITHIN_GEOMETRY_GEOMETRY, st_within_geometry_geometry);

make_udf_function!(st_x_impl::st_x_geometryFunc, ST_X_GEOMETRY, st_x_geometry);

make_udf_function!(st_xmax_impl::st_xmax_geometryFunc, ST_XMAX_GEOMETRY, st_xmax_geometry);

make_udf_function!(st_xmin_impl::st_xmin_geometryFunc, ST_XMIN_GEOMETRY, st_xmin_geometry);

make_udf_function!(st_y_impl::st_y_geometryFunc, ST_Y_GEOMETRY, st_y_geometry);

make_udf_function!(st_ymax_impl::st_ymax_geometryFunc, ST_YMAX_GEOMETRY, st_ymax_geometry);

make_udf_function!(st_ymin_impl::st_ymin_geometryFunc, ST_YMIN_GEOMETRY, st_ymin_geometry);

make_udf_function!(starts_with_impl::starts_with_varchar_varcharFunc, STARTS_WITH_VARCHAR_VARCHAR, starts_with_varchar_varchar);

make_udf_function!(strpos_impl::strpos_varchar_varcharFunc, STRPOS_VARCHAR_VARCHAR, strpos_varchar_varchar);
make_udf_function!(strpos_impl::strpos_varchar_varchar_bigintFunc, STRPOS_VARCHAR_VARCHAR_BIGINT, strpos_varchar_varchar_bigint);

make_udf_function!(substr_impl::substr_varchar_bigintFunc, SUBSTR_VARCHAR_BIGINT, substr_varchar_bigint);
make_udf_function!(substr_impl::substr_varchar_bigint_bigintFunc, SUBSTR_VARCHAR_BIGINT_BIGINT, substr_varchar_bigint_bigint);
make_udf_function!(substr_impl::substr_varbinary_bigintFunc, SUBSTR_VARBINARY_BIGINT, substr_varbinary_bigint);
make_udf_function!(substr_impl::substr_varbinary_bigint_bigintFunc, SUBSTR_VARBINARY_BIGINT_BIGINT, substr_varbinary_bigint_bigint);

make_udf_function!(substring_impl::substring_varchar_bigintFunc, SUBSTRING_VARCHAR_BIGINT, substring_varchar_bigint);
make_udf_function!(substring_impl::substring_varchar_bigint_bigintFunc, SUBSTRING_VARCHAR_BIGINT_BIGINT, substring_varchar_bigint_bigint);

make_udf_function!(tan_impl::tan_doubleFunc, TAN_DOUBLE, tan_double);

make_udf_function!(tanh_impl::tanh_doubleFunc, TANH_DOUBLE, tanh_double);

make_udf_function!(timestamp_objectid_impl::timestamp_objectid_timestamp_0Func, TIMESTAMP_OBJECTID_TIMESTAMP_0, timestamp_objectid_timestamp_0);

make_udf_function!(timezone_hour_impl::timezone_hour_time_pFunc, TIMEZONE_HOUR_TIME_P, timezone_hour_time_p);
make_udf_function!(timezone_hour_impl::timezone_hour_timestamp_pFunc, TIMEZONE_HOUR_TIMESTAMP_P, timezone_hour_timestamp_p);

make_udf_function!(timezone_minute_impl::timezone_minute_time_pFunc, TIMEZONE_MINUTE_TIME_P, timezone_minute_time_p);
make_udf_function!(timezone_minute_impl::timezone_minute_timestamp_pFunc, TIMEZONE_MINUTE_TIMESTAMP_P, timezone_minute_timestamp_p);

make_udf_function!(to_base_impl::to_base_bigint_bigintFunc, TO_BASE_BIGINT_BIGINT, to_base_bigint_bigint);

make_udf_function!(to_base32_impl::to_base32_varbinaryFunc, TO_BASE32_VARBINARY, to_base32_varbinary);

make_udf_function!(to_base64_impl::to_base64_varbinaryFunc, TO_BASE64_VARBINARY, to_base64_varbinary);

make_udf_function!(to_base64url_impl::to_base64url_varbinaryFunc, TO_BASE64URL_VARBINARY, to_base64url_varbinary);

make_udf_function!(to_big_endian_32_impl::to_big_endian_32_bigintFunc, TO_BIG_ENDIAN_32_BIGINT, to_big_endian_32_bigint);

make_udf_function!(to_big_endian_64_impl::to_big_endian_64_bigintFunc, TO_BIG_ENDIAN_64_BIGINT, to_big_endian_64_bigint);

make_udf_function!(to_char_impl::to_char_timestamp_p_varcharFunc, TO_CHAR_TIMESTAMP_P_VARCHAR, to_char_timestamp_p_varchar);

make_udf_function!(to_date_impl::to_date_varchar_varcharFunc, TO_DATE_VARCHAR_VARCHAR, to_date_varchar_varchar);

make_udf_function!(to_encoded_polyline_impl::to_encoded_polyline_geometryFunc, TO_ENCODED_POLYLINE_GEOMETRY, to_encoded_polyline_geometry);

make_udf_function!(to_geojson_geometry_impl::to_geojson_geometry_sphericalgeographyFunc, TO_GEOJSON_GEOMETRY_SPHERICALGEOGRAPHY, to_geojson_geometry_sphericalgeography);

make_udf_function!(to_geometry_impl::to_geometry_sphericalgeographyFunc, TO_GEOMETRY_SPHERICALGEOGRAPHY, to_geometry_sphericalgeography);

make_udf_function!(to_hex_impl::to_hex_varbinaryFunc, TO_HEX_VARBINARY, to_hex_varbinary);

make_udf_function!(to_ieee754_32_impl::to_ieee754_32_realFunc, TO_IEEE754_32_REAL, to_ieee754_32_real);

make_udf_function!(to_ieee754_64_impl::to_ieee754_64_doubleFunc, TO_IEEE754_64_DOUBLE, to_ieee754_64_double);

make_udf_function!(to_iso8601_impl::to_iso8601_dateFunc, TO_ISO8601_DATE, to_iso8601_date);
make_udf_function!(to_iso8601_impl::to_iso8601_timestamp_pFunc, TO_ISO8601_TIMESTAMP_P, to_iso8601_timestamp_p);

make_udf_function!(to_milliseconds_impl::to_milliseconds_intervaldaytosecondFunc, TO_MILLISECONDS_INTERVALDAYTOSECOND, to_milliseconds_intervaldaytosecond);

make_udf_function!(to_spherical_geography_impl::to_spherical_geography_geometryFunc, TO_SPHERICAL_GEOGRAPHY_GEOMETRY, to_spherical_geography_geometry);

make_udf_function!(to_timestamp_impl::to_timestamp_varchar_varcharFunc, TO_TIMESTAMP_VARCHAR_VARCHAR, to_timestamp_varchar_varchar);

make_udf_function!(to_unixtime_impl::to_unixtime_timestamp_pFunc, TO_UNIXTIME_TIMESTAMP_P, to_unixtime_timestamp_p);

make_udf_function!(to_utf8_impl::to_utf8_varcharFunc, TO_UTF8_VARCHAR, to_utf8_varchar);

make_udf_function!(transform_impl::transform_array_1_function_1_11Func, TRANSFORM_ARRAY_1_FUNCTION_1_11, transform_array_1_function_1_11);

make_udf_function!(transform_keys_impl::transform_keys_map_13_5_function_13_5_12Func, TRANSFORM_KEYS_MAP_13_5_FUNCTION_13_5_12, transform_keys_map_13_5_function_13_5_12);

make_udf_function!(transform_values_impl::transform_values_map_4_8_function_4_8_7Func, TRANSFORM_VALUES_MAP_4_8_FUNCTION_4_8_7, transform_values_map_4_8_function_4_8_7);

make_udf_function!(translate_impl::translate_varchar_varchar_varcharFunc, TRANSLATE_VARCHAR_VARCHAR_VARCHAR, translate_varchar_varchar_varchar);

make_udf_function!(trim_impl::trim_varcharFunc, TRIM_VARCHAR, trim_varchar);
make_udf_function!(trim_impl::trim_varchar_codepointsFunc, TRIM_VARCHAR_CODEPOINTS, trim_varchar_codepoints);

make_udf_function!(trim_array_impl::trim_array_array_3_bigintFunc, TRIM_ARRAY_ARRAY_3_BIGINT, trim_array_array_3_bigint);

make_udf_function!(truncate_impl::truncate_decimal_p_s_bigintFunc, TRUNCATE_DECIMAL_P_S_BIGINT, truncate_decimal_p_s_bigint);
make_udf_function!(truncate_impl::truncate_decimal_p_sFunc, TRUNCATE_DECIMAL_P_S, truncate_decimal_p_s);
make_udf_function!(truncate_impl::truncate_doubleFunc, TRUNCATE_DOUBLE, truncate_double);
make_udf_function!(truncate_impl::truncate_realFunc, TRUNCATE_REAL, truncate_real);

make_udf_function!(try_impl::try_1Func, TRY_1, try_1);

make_udf_function!(typeof_impl::typeof_1Func, TYPEOF_1, typeof_1);

make_udf_function!(upper_impl::upper_varcharFunc, UPPER_VARCHAR, upper_varchar);

make_udf_function!(url_decode_impl::url_decode_varcharFunc, URL_DECODE_VARCHAR, url_decode_varchar);

make_udf_function!(url_encode_impl::url_encode_varcharFunc, URL_ENCODE_VARCHAR, url_encode_varchar);

make_udf_function!(url_extract_fragment_impl::url_extract_fragment_varcharFunc, URL_EXTRACT_FRAGMENT_VARCHAR, url_extract_fragment_varchar);

make_udf_function!(url_extract_host_impl::url_extract_host_varcharFunc, URL_EXTRACT_HOST_VARCHAR, url_extract_host_varchar);

make_udf_function!(url_extract_parameter_impl::url_extract_parameter_varchar_varcharFunc, URL_EXTRACT_PARAMETER_VARCHAR_VARCHAR, url_extract_parameter_varchar_varchar);

make_udf_function!(url_extract_path_impl::url_extract_path_varcharFunc, URL_EXTRACT_PATH_VARCHAR, url_extract_path_varchar);

make_udf_function!(url_extract_port_impl::url_extract_port_varcharFunc, URL_EXTRACT_PORT_VARCHAR, url_extract_port_varchar);

make_udf_function!(url_extract_protocol_impl::url_extract_protocol_varcharFunc, URL_EXTRACT_PROTOCOL_VARCHAR, url_extract_protocol_varchar);

make_udf_function!(url_extract_query_impl::url_extract_query_varcharFunc, URL_EXTRACT_QUERY_VARCHAR, url_extract_query_varchar);

make_udf_function!(uuid_impl::uuidFunc, UUID, uuid);

make_udf_function!(value_at_quantile_impl::value_at_quantile_qdigest_doubleFunc, VALUE_AT_QUANTILE_QDIGEST_DOUBLE, value_at_quantile_qdigest_double);
make_udf_function!(value_at_quantile_impl::value_at_quantile_tdigest_doubleFunc, VALUE_AT_QUANTILE_TDIGEST_DOUBLE, value_at_quantile_tdigest_double);

make_udf_function!(values_at_quantiles_impl::values_at_quantiles_qdigest_array_doubleFunc, VALUES_AT_QUANTILES_QDIGEST_ARRAY_DOUBLE, values_at_quantiles_qdigest_array_double);
make_udf_function!(values_at_quantiles_impl::values_at_quantiles_tdigest_array_doubleFunc, VALUES_AT_QUANTILES_TDIGEST_ARRAY_DOUBLE, values_at_quantiles_tdigest_array_double);

make_udf_function!(week_impl::week_dateFunc, WEEK_DATE, week_date);
make_udf_function!(week_impl::week_timestamp_pFunc, WEEK_TIMESTAMP_P, week_timestamp_p);

make_udf_function!(week_of_year_impl::week_of_year_dateFunc, WEEK_OF_YEAR_DATE, week_of_year_date);
make_udf_function!(week_of_year_impl::week_of_year_timestamp_pFunc, WEEK_OF_YEAR_TIMESTAMP_P, week_of_year_timestamp_p);

make_udf_function!(width_bucket_impl::width_bucket_double_array_doubleFunc, WIDTH_BUCKET_DOUBLE_ARRAY_DOUBLE, width_bucket_double_array_double);
make_udf_function!(width_bucket_impl::width_bucket_double_double_double_bigintFunc, WIDTH_BUCKET_DOUBLE_DOUBLE_DOUBLE_BIGINT, width_bucket_double_double_double_bigint);

make_udf_function!(wilson_interval_lower_impl::wilson_interval_lower_bigint_bigint_doubleFunc, WILSON_INTERVAL_LOWER_BIGINT_BIGINT_DOUBLE, wilson_interval_lower_bigint_bigint_double);

make_udf_function!(wilson_interval_upper_impl::wilson_interval_upper_bigint_bigint_doubleFunc, WILSON_INTERVAL_UPPER_BIGINT_BIGINT_DOUBLE, wilson_interval_upper_bigint_bigint_double);

make_udf_function!(with_timezone_impl::with_timezone_timestamp_p_varcharFunc, WITH_TIMEZONE_TIMESTAMP_P_VARCHAR, with_timezone_timestamp_p_varchar);

make_udf_function!(word_stem_impl::word_stem_varcharFunc, WORD_STEM_VARCHAR, word_stem_varchar);
make_udf_function!(word_stem_impl::word_stem_varchar_varcharFunc, WORD_STEM_VARCHAR_VARCHAR, word_stem_varchar_varchar);

make_udf_function!(xxhash64_impl::xxhash64_varbinaryFunc, XXHASH64_VARBINARY, xxhash64_varbinary);

make_udf_function!(year_impl::year_dateFunc, YEAR_DATE, year_date);
make_udf_function!(year_impl::year_intervalyeartomonthFunc, YEAR_INTERVALYEARTOMONTH, year_intervalyeartomonth);
make_udf_function!(year_impl::year_timestamp_pFunc, YEAR_TIMESTAMP_P, year_timestamp_p);

make_udf_function!(year_of_week_impl::year_of_week_dateFunc, YEAR_OF_WEEK_DATE, year_of_week_date);
make_udf_function!(year_of_week_impl::year_of_week_timestamp_pFunc, YEAR_OF_WEEK_TIMESTAMP_P, year_of_week_timestamp_p);

make_udf_function!(yow_impl::yow_dateFunc, YOW_DATE, yow_date);
make_udf_function!(yow_impl::yow_timestamp_pFunc, YOW_TIMESTAMP_P, yow_timestamp_p);

make_udf_function!(zip_impl::zip_array_14_array_15Func, ZIP_ARRAY_14_ARRAY_15, zip_array_14_array_15);
make_udf_function!(zip_impl::zip_array_14_array_15_array_16Func, ZIP_ARRAY_14_ARRAY_15_ARRAY_16, zip_array_14_array_15_array_16);
make_udf_function!(zip_impl::zip_array_14_array_15_array_16_array_17Func, ZIP_ARRAY_14_ARRAY_15_ARRAY_16_ARRAY_17, zip_array_14_array_15_array_16_array_17);
make_udf_function!(zip_impl::zip_array_14_array_15_array_16_array_17_array_18Func, ZIP_ARRAY_14_ARRAY_15_ARRAY_16_ARRAY_17_ARRAY_18, zip_array_14_array_15_array_16_array_17_array_18);

make_udf_function!(zip_with_impl::zip_with_array_1_array_11_function_1_11_9Func, ZIP_WITH_ARRAY_1_ARRAY_11_FUNCTION_1_11_9, zip_with_array_1_array_11_function_1_11_9);




// Export the functions out of this package, both as expr_fn as well as a list of functions
export_functions!(
    (trino, abs_tinyint, arg1, "function doc"),
    (trino, abs_smallint, arg1, "function doc"),
    (trino, abs_bigint, arg1, "function doc"),
    (trino, abs_double, arg1, "function doc"),
    (trino, abs_decimal_p_s, arg1, "function doc"),
    (trino, abs_real, arg1, "function doc"),

    (trino, acos_double, arg1, "function doc"),

    (trino, all_match_array_1_function_1_boolean, arg1 arg2, "function doc"),

    (trino, any_match_array_1_function_1_boolean, arg1 arg2, "function doc"),

    (trino, array_distinct_array_3, arg1, "function doc"),

    (trino, array_except_array_3_array_3, arg1 arg2, "function doc"),

    (trino, array_intersect_array_3_array_3, arg1 arg2, "function doc"),

    (trino, array_join_array_1_varchar, arg1 arg2, "function doc"),
    (trino, array_join_array_1_varchar_varchar, arg1 arg2 arg3, "function doc"),

    (trino, array_max_array_1, arg1, "function doc"),

    (trino, array_min_array_1, arg1, "function doc"),

    (trino, array_position_array_1_1, arg1 arg2, "function doc"),

    (trino, array_remove_array_3_3, arg1 arg2, "function doc"),

    (trino, array_sort_array_3, arg1, "function doc"),
    (trino, array_sort_array_1_function_1_1_bigint, arg1 arg2, "function doc"),

    (trino, array_union_array_3_array_3, arg1 arg2, "function doc"),

    (trino, arrays_overlap_array_3_array_3, arg1 arg2, "function doc"),

    (trino, asin_double, arg1, "function doc"),

    (trino, at_timezone_timestamp_p_varchar, arg1 arg2, "function doc"),

    (trino, atan_double, arg1, "function doc"),

    (trino, atan2_double_double, arg1 arg2, "function doc"),

    (trino, bar_double_bigint, arg1 arg2, "function doc"),
    (trino, bar_double_bigint_color_color, arg1 arg2 arg3 arg4, "function doc"),

    (trino, beta_cdf_double_double_double, arg1 arg2 arg3, "function doc"),

    (trino, bing_tile_bigint_bigint_bigint, arg1 arg2 arg3, "function doc"),
    (trino, bing_tile_varchar, arg1, "function doc"),

    (trino, bing_tile_at_double_double_bigint, arg1 arg2 arg3, "function doc"),

    (trino, bing_tile_coordinates_bingtile, arg1, "function doc"),

    (trino, bing_tile_polygon_bingtile, arg1, "function doc"),

    (trino, bing_tile_quadkey_bingtile, arg1, "function doc"),

    (trino, bing_tile_zoom_level_bingtile, arg1, "function doc"),

    (trino, bing_tiles_around_double_double_bigint, arg1 arg2 arg3, "function doc"),
    (trino, bing_tiles_around_double_double_bigint_double, arg1 arg2 arg3 arg4, "function doc"),

    (trino, bit_count_bigint_bigint, arg1 arg2, "function doc"),

    (trino, bitwise_and_bigint_bigint, arg1 arg2, "function doc"),

    (trino, bitwise_left_shift_bigint_bigint, arg1 arg2, "function doc"),
    (trino, bitwise_left_shift_integer_bigint, arg1 arg2, "function doc"),
    (trino, bitwise_left_shift_smallint_bigint, arg1 arg2, "function doc"),
    (trino, bitwise_left_shift_tinyint_bigint, arg1 arg2, "function doc"),

    (trino, bitwise_not_bigint, arg1, "function doc"),

    (trino, bitwise_or_bigint_bigint, arg1 arg2, "function doc"),

    (trino, bitwise_right_shift_bigint_bigint, arg1 arg2, "function doc"),
    (trino, bitwise_right_shift_integer_bigint, arg1 arg2, "function doc"),
    (trino, bitwise_right_shift_smallint_bigint, arg1 arg2, "function doc"),
    (trino, bitwise_right_shift_tinyint_bigint, arg1 arg2, "function doc"),

    (trino, bitwise_right_shift_arithmetic_bigint_bigint, arg1 arg2, "function doc"),
    (trino, bitwise_right_shift_arithmetic_integer_bigint, arg1 arg2, "function doc"),
    (trino, bitwise_right_shift_arithmetic_smallint_bigint, arg1 arg2, "function doc"),
    (trino, bitwise_right_shift_arithmetic_tinyint_bigint, arg1 arg2, "function doc"),

    (trino, bitwise_xor_bigint_bigint, arg1 arg2, "function doc"),

    (trino, cardinality_array_3, arg1, "function doc"),
    (trino, cardinality_hyperloglog, arg1, "function doc"),
    (trino, cardinality_map_4_5, arg1, "function doc"),
    (trino, cardinality_setdigest, arg1, "function doc"),

    (trino, cbrt_double, arg1, "function doc"),

    (trino, ceil_bigint, arg1, "function doc"),
    (trino, ceil_decimal_p_s, arg1, "function doc"),
    (trino, ceil_double, arg1, "function doc"),
    (trino, ceil_integer, arg1, "function doc"),
    (trino, ceil_real, arg1, "function doc"),
    (trino, ceil_smallint, arg1, "function doc"),
    (trino, ceil_tinyint, arg1, "function doc"),

    (trino, ceiling_bigint, arg1, "function doc"),
    (trino, ceiling_decimal_p_s, arg1, "function doc"),
    (trino, ceiling_double, arg1, "function doc"),
    (trino, ceiling_integer, arg1, "function doc"),
    (trino, ceiling_real, arg1, "function doc"),
    (trino, ceiling_smallint, arg1, "function doc"),
    (trino, ceiling_tinyint, arg1, "function doc"),

    (trino, char2hexint_varchar, arg1, "function doc"),

    (trino, chr_bigint, arg1, "function doc"),

    (trino, classify_map_bigint_double_classifier, arg1 arg2, "function doc"),

    (trino, coalesce_1, arg1, "function doc"),

    (trino, codepoint_varchar, arg1, "function doc"),

    (trino, color_double_color_color, arg1 arg2 arg3, "function doc"),
    (trino, color_double_double_double_color_color, arg1 arg2 arg3 arg4 arg5, "function doc"),
    (trino, color_varchar, arg1, "function doc"),

    (trino, combinations_array_1_bigint, arg1 arg2, "function doc"),

    (trino, concat_3_array_3, arg1 arg2, "function doc"),
    (trino, concat_array_3, arg1, "function doc"),
    (trino, concat_array_3_3, arg1 arg2, "function doc"),
    (trino, concat_varchar_varchar, arg1 arg2, "function doc"),
    (trino, concat_varchar, arg1, "function doc"),
    (trino, concat_varbinary, arg1, "function doc"),

    (trino, concat_ws_varchar_array_varchar, arg1 arg2, "function doc"),
    (trino, concat_ws_varchar, arg1, "function doc"),

    (trino, contains_array_1_1, arg1 arg2, "function doc"),
    (trino, contains_varchar_ipaddress, arg1 arg2, "function doc"),

    (trino, contains_sequence_array_1_array_1, arg1 arg2, "function doc"),

    (trino, cos_double, arg1, "function doc"),

    (trino, cosh_double, arg1, "function doc"),

    (trino, cosine_similarity_map_varchar_double_map_varchar_double, arg1 arg2, "function doc"),

    (trino, crc32_varbinary, arg1, "function doc"),

    (trino, current_catalog, , "function doc"),

    (trino, current_date, , "function doc"),

    (trino, current_groups, , "function doc"),

    (trino, current_schema, , "function doc"),

    (trino, current_time, , "function doc"),

    (trino, current_timestamp, , "function doc"),
    (trino, current_timestamp_bigint_0, arg1, "function doc"),
    (trino, current_timestamp_bigint_3, arg1, "function doc"),
    (trino, current_timestamp_bigint_6, arg1, "function doc"),
    (trino, current_timestamp_bigint_9, arg1, "function doc"),

    (trino, current_timezone, , "function doc"),

    (trino, current_user, , "function doc"),

    (trino, date_timestamp_p, arg1, "function doc"),
    (trino, date_varchar, arg1, "function doc"),

    (trino, date_add_varchar_bigint_date, arg1 arg2 arg3, "function doc"),
    (trino, date_add_varchar_bigint_time_p, arg1 arg2 arg3, "function doc"),
    (trino, date_add_varchar_bigint_timestamp_p, arg1 arg2 arg3, "function doc"),

    (trino, date_diff_varchar_date_date, arg1 arg2 arg3, "function doc"),
    (trino, date_diff_varchar_time_p_time_p, arg1 arg2 arg3, "function doc"),
    (trino, date_diff_varchar_timestamp_p_timestamp_p, arg1 arg2 arg3, "function doc"),

    (trino, date_format_timestamp_p_varchar, arg1 arg2, "function doc"),

    (trino, date_parse_varchar_varchar, arg1 arg2, "function doc"),

    (trino, date_trunc_varchar_time_p, arg1 arg2, "function doc"),
    (trino, date_trunc_varchar_timestamp_p, arg1 arg2, "function doc"),
    (trino, date_trunc_varchar_date, arg1 arg2, "function doc"),

    (trino, day_date, arg1, "function doc"),
    (trino, day_intervaldaytosecond, arg1, "function doc"),
    (trino, day_timestamp_p, arg1, "function doc"),

    (trino, day_of_month_date, arg1, "function doc"),
    (trino, day_of_month_intervaldaytosecond, arg1, "function doc"),
    (trino, day_of_month_timestamp_p, arg1, "function doc"),

    (trino, day_of_week_date, arg1, "function doc"),
    (trino, day_of_week_timestamp_p, arg1, "function doc"),

    (trino, day_of_year_date, arg1, "function doc"),
    (trino, day_of_year_timestamp_p, arg1, "function doc"),

    (trino, degrees_double, arg1, "function doc"),

    (trino, dow_date, arg1, "function doc"),
    (trino, dow_timestamp_p, arg1, "function doc"),

    (trino, doy_date, arg1, "function doc"),
    (trino, doy_timestamp_p, arg1, "function doc"),

    (trino, e, , "function doc"),

    (trino, element_at_map_4_5_4, arg1 arg2, "function doc"),
    (trino, element_at_array_3_bigint, arg1 arg2, "function doc"),

    (trino, empty_approx_set, , "function doc"),

    (trino, exp_double, arg1, "function doc"),

    (trino, features_double, arg1, "function doc"),
    (trino, features_double_double, arg1 arg2, "function doc"),
    (trino, features_double_double_double, arg1 arg2 arg3, "function doc"),
    (trino, features_double_double_double_double, arg1 arg2 arg3 arg4, "function doc"),
    (trino, features_double_double_double_double_double, arg1 arg2 arg3 arg4 arg5, "function doc"),
    (trino, features_double_double_double_double_double_double, arg1 arg2 arg3 arg4 arg5 arg6, "function doc"),
    (trino, features_double_double_double_double_double_double_double, arg1 arg2 arg3 arg4 arg5 arg6 arg7, "function doc"),
    (trino, features_double_double_double_double_double_double_double_double, arg1 arg2 arg3 arg4 arg5 arg6 arg7 arg8, "function doc"),
    (trino, features_double_double_double_double_double_double_double_double_double, arg1 arg2 arg3 arg4 arg5 arg6 arg7 arg8 arg9, "function doc"),
    (trino, features_double_double_double_double_double_double_double_double_double_double, arg1 arg2 arg3 arg4 arg5 arg6 arg7 arg8 arg9 arg10, "function doc"),

    (trino, filter_array_1_function_1_boolean, arg1 arg2, "function doc"),

    (trino, flatten_array_array_3, arg1, "function doc"),

    (trino, floor_bigint, arg1, "function doc"),
    (trino, floor_decimal_p_s, arg1, "function doc"),
    (trino, floor_double, arg1, "function doc"),
    (trino, floor_integer, arg1, "function doc"),
    (trino, floor_real, arg1, "function doc"),
    (trino, floor_smallint, arg1, "function doc"),
    (trino, floor_tinyint, arg1, "function doc"),

    (trino, format_varchar_1, arg1 arg2, "function doc"),
    (trino, format_varchar_1_2, arg1 arg2 arg3, "function doc"),
    (trino, format_varchar_1_2_3, arg1 arg2 arg3 arg4, "function doc"),
    (trino, format_varchar_1_2_3_4, arg1 arg2 arg3 arg4 arg5, "function doc"),
    (trino, format_varchar_1_2_3_4_5, arg1 arg2 arg3 arg4 arg5 arg6, "function doc"),

    (trino, format_datetime_timestamp_p_varchar, arg1 arg2, "function doc"),

    (trino, format_number_bigint, arg1, "function doc"),
    (trino, format_number_double, arg1, "function doc"),

    (trino, from_base_varchar_bigint, arg1 arg2, "function doc"),

    (trino, from_base32_varbinary, arg1, "function doc"),
    (trino, from_base32_varchar, arg1, "function doc"),

    (trino, from_base64_varbinary, arg1, "function doc"),
    (trino, from_base64_varchar, arg1, "function doc"),

    (trino, from_base64url_varbinary, arg1, "function doc"),
    (trino, from_base64url_varchar, arg1, "function doc"),

    (trino, from_big_endian_32_varbinary, arg1, "function doc"),

    (trino, from_big_endian_64_varbinary, arg1, "function doc"),

    (trino, from_encoded_polyline_varchar, arg1, "function doc"),

    (trino, from_geojson_geometry_varchar, arg1, "function doc"),

    (trino, from_hex_varbinary, arg1, "function doc"),
    (trino, from_hex_varchar, arg1, "function doc"),

    (trino, from_ieee754_32_varbinary, arg1, "function doc"),

    (trino, from_ieee754_64_varbinary, arg1, "function doc"),

    (trino, from_iso8601_date_varchar, arg1, "function doc"),

    (trino, from_iso8601_timestamp_varchar, arg1, "function doc"),

    (trino, from_iso8601_timestamp_nanos_varchar, arg1, "function doc"),

    (trino, from_unixtime_bigint, arg1, "function doc"),
    (trino, from_unixtime_bigint_bigint_bigint, arg1 arg2 arg3, "function doc"),
    (trino, from_unixtime_bigint_varchar, arg1 arg2, "function doc"),

    (trino, from_unixtime_nanos_bigint, arg1, "function doc"),
    (trino, from_unixtime_nanos_decimal_p_s, arg1, "function doc"),

    (trino, from_utf8_varbinary, arg1, "function doc"),
    (trino, from_utf8_varbinary_bigint, arg1 arg2, "function doc"),
    (trino, from_utf8_varbinary_varchar, arg1 arg2, "function doc"),

    (trino, geometry_from_hadoop_shape_varbinary, arg1, "function doc"),

    (trino, geometry_invalid_reason_geometry, arg1, "function doc"),

    (trino, geometry_nearest_points_geometry_geometry, arg1 arg2, "function doc"),

    (trino, geometry_to_bing_tiles_geometry_bigint, arg1 arg2, "function doc"),

    (trino, geometry_union_array_geometry, arg1, "function doc"),

    (trino, great_circle_distance_double_double_double_double, arg1 arg2 arg3 arg4, "function doc"),

    (trino, greatest_3, arg1, "function doc"),

    (trino, hamming_distance_varchar_varchar, arg1 arg2, "function doc"),

    (trino, hash_counts_setdigest, arg1, "function doc"),

    (trino, hmac_md5_varbinary_varbinary, arg1 arg2, "function doc"),

    (trino, hmac_sha1_varbinary_varbinary, arg1 arg2, "function doc"),

    (trino, hmac_sha256_varbinary_varbinary, arg1 arg2, "function doc"),

    (trino, hmac_sha512_varbinary_varbinary, arg1 arg2, "function doc"),

    (trino, hour_intervaldaytosecond, arg1, "function doc"),
    (trino, hour_time_p, arg1, "function doc"),
    (trino, hour_timestamp_p, arg1, "function doc"),

    (trino, human_readable_seconds_double, arg1, "function doc"),

    (trino, if_boolean_1_1, arg1 arg2 arg3, "function doc"),

    (trino, index_varchar_varchar, arg1 arg2, "function doc"),

    (trino, infinity, , "function doc"),

    (trino, intersection_cardinality_setdigest_setdigest, arg1 arg2, "function doc"),

    (trino, inverse_beta_cdf_double_double_double, arg1 arg2 arg3, "function doc"),

    (trino, inverse_normal_cdf_double_double_double, arg1 arg2 arg3, "function doc"),

    (trino, is_finite_double, arg1, "function doc"),

    (trino, is_infinite_double, arg1, "function doc"),

    (trino, is_json_scalar_json, arg1, "function doc"),
    (trino, is_json_scalar_varchar, arg1, "function doc"),

    (trino, is_nan_double, arg1, "function doc"),
    (trino, is_nan_real, arg1, "function doc"),

    (trino, jaccard_index_setdigest_setdigest, arg1 arg2, "function doc"),

    (trino, json_array_contains_json_bigint, arg1 arg2, "function doc"),
    (trino, json_array_contains_json_boolean, arg1 arg2, "function doc"),
    (trino, json_array_contains_json_double, arg1 arg2, "function doc"),
    (trino, json_array_contains_json_varchar, arg1 arg2, "function doc"),
    (trino, json_array_contains_varchar_bigint, arg1 arg2, "function doc"),
    (trino, json_array_contains_varchar_boolean, arg1 arg2, "function doc"),
    (trino, json_array_contains_varchar_double, arg1 arg2, "function doc"),
    (trino, json_array_contains_varchar_varchar, arg1 arg2, "function doc"),

    (trino, json_array_get_json_bigint, arg1 arg2, "function doc"),
    (trino, json_array_get_varchar_bigint, arg1 arg2, "function doc"),

    (trino, json_array_length_json, arg1, "function doc"),
    (trino, json_array_length_varchar, arg1, "function doc"),

    (trino, json_extract_json_jsonpath, arg1 arg2, "function doc"),
    (trino, json_extract_varchar_jsonpath, arg1 arg2, "function doc"),

    (trino, json_extract_scalar_json_jsonpath, arg1 arg2, "function doc"),
    (trino, json_extract_scalar_varchar_jsonpath, arg1 arg2, "function doc"),

    (trino, json_format_json, arg1, "function doc"),

    (trino, json_parse_varchar, arg1, "function doc"),

    (trino, json_size_json_jsonpath, arg1 arg2, "function doc"),
    (trino, json_size_varchar_jsonpath, arg1 arg2, "function doc"),

    (trino, last_day_of_month_date, arg1, "function doc"),
    (trino, last_day_of_month_timestamp_p, arg1, "function doc"),

    (trino, least_3, arg1, "function doc"),

    (trino, length_varchar, arg1, "function doc"),
    (trino, length_varbinary, arg1, "function doc"),
    (trino, length_array_1, arg1, "function doc"),

    (trino, levenshtein_distance_varchar_varchar, arg1 arg2, "function doc"),

    (trino, line_interpolate_point_geometry_double, arg1 arg2, "function doc"),

    (trino, line_interpolate_points_geometry_double, arg1 arg2, "function doc"),

    (trino, line_locate_point_geometry_geometry, arg1 arg2, "function doc"),

    (trino, ln_double, arg1, "function doc"),

    (trino, localtime, , "function doc"),

    (trino, localtimestamp, , "function doc"),
    (trino, localtimestamp_bigint_0, arg1, "function doc"),
    (trino, localtimestamp_bigint_3, arg1, "function doc"),
    (trino, localtimestamp_bigint_6, arg1, "function doc"),
    (trino, localtimestamp_bigint_9, arg1, "function doc"),

    (trino, log_double_double, arg1 arg2, "function doc"),

    (trino, log10_double, arg1, "function doc"),

    (trino, log2_double, arg1, "function doc"),

    (trino, lower_varchar, arg1, "function doc"),

    (trino, lpad_varbinary_bigint_varbinary, arg1 arg2 arg3, "function doc"),
    (trino, lpad_varchar_bigint_varchar, arg1 arg2 arg3, "function doc"),

    (trino, ltrim_varchar, arg1, "function doc"),
    (trino, ltrim_varchar_codepoints, arg1 arg2, "function doc"),

    (trino, luhn_check_varchar, arg1, "function doc"),

    (trino, map_array_4_array_5, arg1 arg2, "function doc"),
    (trino, map, , "function doc"),

    (trino, map_concat_map_4_5, arg1, "function doc"),

    (trino, map_entries_map_4_5, arg1, "function doc"),

    (trino, map_filter_map_4_5_function_4_5_boolean, arg1 arg2, "function doc"),

    (trino, map_from_entries_array_row_c04_c15, arg1, "function doc"),

    (trino, map_keys_map_4_5, arg1, "function doc"),

    (trino, map_values_map_4_5, arg1, "function doc"),

    (trino, map_zip_with_map_4_8_map_4_7_function_4_8_7_6, arg1 arg2 arg3, "function doc"),

    (trino, md5_varbinary, arg1, "function doc"),

    (trino, millisecond_intervaldaytosecond, arg1, "function doc"),
    (trino, millisecond_time_p, arg1, "function doc"),
    (trino, millisecond_timestamp_p, arg1, "function doc"),

    (trino, minute_intervaldaytosecond, arg1, "function doc"),
    (trino, minute_time_p, arg1, "function doc"),
    (trino, minute_timestamp_p, arg1, "function doc"),

    (trino, mod_bigint_bigint, arg1 arg2, "function doc"),
    (trino, mod_decimal_a_precision_a_scale_decimal_b_precision_b_scale, arg1 arg2, "function doc"),
    (trino, mod_double_double, arg1 arg2, "function doc"),
    (trino, mod_integer_integer, arg1 arg2, "function doc"),
    (trino, mod_real_real, arg1 arg2, "function doc"),
    (trino, mod_smallint_smallint, arg1 arg2, "function doc"),
    (trino, mod_tinyint_tinyint, arg1 arg2, "function doc"),

    (trino, month_date, arg1, "function doc"),
    (trino, month_intervalyeartomonth, arg1, "function doc"),
    (trino, month_timestamp_p, arg1, "function doc"),

    (trino, multimap_from_entries_array_row_c04_c15, arg1, "function doc"),

    (trino, murmur3_varbinary, arg1, "function doc"),

    (trino, nan, , "function doc"),

    (trino, ngrams_array_1_bigint, arg1 arg2, "function doc"),

    (trino, none_match_array_1_function_1_boolean, arg1 arg2, "function doc"),

    (trino, normal_cdf_double_double_double, arg1 arg2 arg3, "function doc"),

    (trino, normalize_varchar_varchar, arg1 arg2, "function doc"),

    (trino, now, , "function doc"),

    (trino, nullif_1_1, arg1 arg2, "function doc"),

    (trino, objectid, , "function doc"),
    (trino, objectid_varchar, arg1, "function doc"),

    (trino, objectid_timestamp_objectid, arg1, "function doc"),

    (trino, parse_data_size_varchar, arg1, "function doc"),

    (trino, parse_datetime_varchar_varchar, arg1 arg2, "function doc"),

    (trino, parse_duration_varchar, arg1, "function doc"),

    (trino, parse_presto_data_size_varchar, arg1, "function doc"),

    (trino, pi, , "function doc"),

    (trino, pow_double_double, arg1 arg2, "function doc"),

    (trino, power_double_double, arg1 arg2, "function doc"),

    (trino, quantile_at_value_qdigest_bigint, arg1 arg2, "function doc"),
    (trino, quantile_at_value_qdigest_double, arg1 arg2, "function doc"),
    (trino, quantile_at_value_qdigest_real, arg1 arg2, "function doc"),

    (trino, quarter_date, arg1, "function doc"),
    (trino, quarter_timestamp_p, arg1, "function doc"),

    (trino, radians_double, arg1, "function doc"),

    (trino, rand_bigint, arg1, "function doc"),
    (trino, rand_bigint_bigint, arg1 arg2, "function doc"),
    (trino, rand, , "function doc"),
    (trino, rand_integer, arg1, "function doc"),
    (trino, rand_integer_integer, arg1 arg2, "function doc"),
    (trino, rand_smallint, arg1, "function doc"),
    (trino, rand_smallint_smallint, arg1 arg2, "function doc"),
    (trino, rand_tinyint, arg1, "function doc"),
    (trino, rand_tinyint_tinyint, arg1 arg2, "function doc"),

    (trino, random_bigint, arg1, "function doc"),
    (trino, random_bigint_bigint, arg1 arg2, "function doc"),
    (trino, random, , "function doc"),
    (trino, random_integer, arg1, "function doc"),
    (trino, random_integer_integer, arg1 arg2, "function doc"),
    (trino, random_smallint, arg1, "function doc"),
    (trino, random_smallint_smallint, arg1 arg2, "function doc"),
    (trino, random_tinyint, arg1, "function doc"),
    (trino, random_tinyint_tinyint, arg1 arg2, "function doc"),

    (trino, reduce_array_1_10_function_10_1_10_function_10_9, arg1 arg2 arg3 arg4, "function doc"),

    (trino, regexp_count_varchar_joniregexp, arg1 arg2, "function doc"),

    (trino, regexp_extract_varchar_joniregexp, arg1 arg2, "function doc"),
    (trino, regexp_extract_varchar_joniregexp_bigint, arg1 arg2 arg3, "function doc"),

    (trino, regexp_extract_all_varchar_joniregexp, arg1 arg2, "function doc"),
    (trino, regexp_extract_all_varchar_joniregexp_bigint, arg1 arg2 arg3, "function doc"),

    (trino, regexp_like_varchar_joniregexp, arg1 arg2, "function doc"),

    (trino, regexp_position_varchar_joniregexp, arg1 arg2, "function doc"),
    (trino, regexp_position_varchar_joniregexp_bigint, arg1 arg2 arg3, "function doc"),
    (trino, regexp_position_varchar_joniregexp_bigint_bigint, arg1 arg2 arg3 arg4, "function doc"),

    (trino, regexp_replace_varchar_joniregexp_function_array_varchar_varchar, arg1 arg2 arg3, "function doc"),
    (trino, regexp_replace_varchar_joniregexp, arg1 arg2, "function doc"),
    (trino, regexp_replace_varchar_joniregexp_varchar, arg1 arg2 arg3, "function doc"),

    (trino, regexp_split_varchar_joniregexp, arg1 arg2, "function doc"),

    (trino, regress_map_bigint_double_regressor, arg1 arg2, "function doc"),

    (trino, render_boolean, arg1, "function doc"),
    (trino, render_bigint_color, arg1 arg2, "function doc"),
    (trino, render_double_color, arg1 arg2, "function doc"),
    (trino, render_varchar_color, arg1 arg2, "function doc"),

    (trino, repeat_1_bigint, arg1 arg2, "function doc"),

    (trino, replace_varchar_varchar_varchar, arg1 arg2 arg3, "function doc"),
    (trino, replace_varchar_varchar, arg1 arg2, "function doc"),

    (trino, reverse_array_3, arg1, "function doc"),
    (trino, reverse_varbinary, arg1, "function doc"),
    (trino, reverse_varchar, arg1, "function doc"),

    (trino, rgb_bigint_bigint_bigint, arg1 arg2 arg3, "function doc"),

    (trino, round_double, arg1, "function doc"),
    (trino, round_double_bigint, arg1 arg2, "function doc"),
    (trino, round_real, arg1, "function doc"),
    (trino, round_real_bigint, arg1 arg2, "function doc"),
    (trino, round_integer, arg1, "function doc"),
    (trino, round_integer_integer, arg1 arg2, "function doc"),
    (trino, round_decimal_p_s, arg1, "function doc"),
    (trino, round_decimal_p_s_bigint, arg1 arg2, "function doc"),
    (trino, round_bigint, arg1, "function doc"),
    (trino, round_bigint_bigint, arg1 arg2, "function doc"),
    (trino, round_smallint, arg1, "function doc"),
    (trino, round_smallint_bigint, arg1 arg2, "function doc"),
    (trino, round_tinyint, arg1, "function doc"),
    (trino, round_tinyint_bigint, arg1 arg2, "function doc"),

    (trino, rpad_varbinary_bigint_varbinary, arg1 arg2 arg3, "function doc"),
    (trino, rpad_varchar_bigint_varchar, arg1 arg2 arg3, "function doc"),

    (trino, rtrim_varchar, arg1, "function doc"),
    (trino, rtrim_varchar_codepoints, arg1 arg2, "function doc"),

    (trino, second_intervaldaytosecond, arg1, "function doc"),
    (trino, second_time_p, arg1, "function doc"),
    (trino, second_timestamp_p, arg1, "function doc"),

    (trino, sequence_bigint_bigint, arg1 arg2, "function doc"),
    (trino, sequence_bigint_bigint_bigint, arg1 arg2 arg3, "function doc"),
    (trino, sequence_date_date, arg1 arg2, "function doc"),
    (trino, sequence_date_date_intervaldaytosecond, arg1 arg2 arg3, "function doc"),
    (trino, sequence_date_date_intervalyeartomonth, arg1 arg2 arg3, "function doc"),
    (trino, sequence_timestamp_p_timestamp_p_intervaldaytosecond, arg1 arg2 arg3, "function doc"),

    (trino, sha1_varbinary, arg1, "function doc"),

    (trino, sha256_varbinary, arg1, "function doc"),

    (trino, sha512_varbinary, arg1, "function doc"),

    (trino, shuffle_array_3, arg1, "function doc"),

    (trino, sign_bigint, arg1, "function doc"),
    (trino, sign_decimal_p_s, arg1, "function doc"),
    (trino, sign_double, arg1, "function doc"),
    (trino, sign_integer, arg1, "function doc"),
    (trino, sign_real, arg1, "function doc"),
    (trino, sign_smallint, arg1, "function doc"),
    (trino, sign_tinyint, arg1, "function doc"),

    (trino, simplify_geometry_geometry_double, arg1 arg2, "function doc"),

    (trino, sin_double, arg1, "function doc"),

    (trino, sinh_double, arg1, "function doc"),

    (trino, slice_array_3_bigint_bigint, arg1 arg2 arg3, "function doc"),

    (trino, soundex_varchar, arg1, "function doc"),

    (trino, spatial_partitions_kdbtree_geometry, arg1 arg2, "function doc"),
    (trino, spatial_partitions_kdbtree_geometry_double, arg1 arg2 arg3, "function doc"),

    (trino, split_varchar_varchar, arg1 arg2, "function doc"),
    (trino, split_varchar_varchar_bigint, arg1 arg2 arg3, "function doc"),

    (trino, split_part_varchar_varchar_bigint, arg1 arg2 arg3, "function doc"),

    (trino, split_to_map_varchar_varchar_varchar, arg1 arg2 arg3, "function doc"),

    (trino, split_to_multimap_varchar_varchar_varchar, arg1 arg2 arg3, "function doc"),

    (trino, spooky_hash_v2_32_varbinary, arg1, "function doc"),

    (trino, spooky_hash_v2_64_varbinary, arg1, "function doc"),

    (trino, sqrt_double, arg1, "function doc"),

    (trino, st_area_geometry, arg1, "function doc"),
    (trino, st_area_sphericalgeography, arg1, "function doc"),

    (trino, st_asbinary_geometry, arg1, "function doc"),

    (trino, st_astext_geometry, arg1, "function doc"),

    (trino, st_boundary_geometry, arg1, "function doc"),

    (trino, st_buffer_geometry_double, arg1 arg2, "function doc"),

    (trino, st_centroid_geometry, arg1, "function doc"),

    (trino, st_contains_geometry_geometry, arg1 arg2, "function doc"),

    (trino, st_convexhull_geometry, arg1, "function doc"),

    (trino, st_coorddim_geometry, arg1, "function doc"),

    (trino, st_crosses_geometry_geometry, arg1 arg2, "function doc"),

    (trino, st_difference_geometry_geometry, arg1 arg2, "function doc"),

    (trino, st_dimension_geometry, arg1, "function doc"),

    (trino, st_disjoint_geometry_geometry, arg1 arg2, "function doc"),

    (trino, st_distance_geometry_geometry, arg1 arg2, "function doc"),
    (trino, st_distance_sphericalgeography_sphericalgeography, arg1 arg2, "function doc"),

    (trino, st_endpoint_geometry, arg1, "function doc"),

    (trino, st_envelope_geometry, arg1, "function doc"),

    (trino, st_envelopeaspts_geometry, arg1, "function doc"),

    (trino, st_equals_geometry_geometry, arg1 arg2, "function doc"),

    (trino, st_exteriorring_geometry, arg1, "function doc"),

    (trino, st_geometries_geometry, arg1, "function doc"),

    (trino, st_geometryfromtext_varchar, arg1, "function doc"),

    (trino, st_geometryn_geometry_bigint, arg1 arg2, "function doc"),

    (trino, st_geometrytype_geometry, arg1, "function doc"),

    (trino, st_geomfrombinary_varbinary, arg1, "function doc"),

    (trino, st_interiorringn_geometry_bigint, arg1 arg2, "function doc"),

    (trino, st_interiorrings_geometry, arg1, "function doc"),

    (trino, st_intersection_geometry_geometry, arg1 arg2, "function doc"),

    (trino, st_intersects_geometry_geometry, arg1 arg2, "function doc"),

    (trino, st_isclosed_geometry, arg1, "function doc"),

    (trino, st_isempty_geometry, arg1, "function doc"),

    (trino, st_isring_geometry, arg1, "function doc"),

    (trino, st_issimple_geometry, arg1, "function doc"),

    (trino, st_isvalid_geometry, arg1, "function doc"),

    (trino, st_length_geometry, arg1, "function doc"),
    (trino, st_length_sphericalgeography, arg1, "function doc"),

    (trino, st_linefromtext_varchar, arg1, "function doc"),

    (trino, st_linestring_array_geometry, arg1, "function doc"),

    (trino, st_multipoint_array_geometry, arg1, "function doc"),

    (trino, st_numgeometries_geometry, arg1, "function doc"),

    (trino, st_numinteriorring_geometry, arg1, "function doc"),

    (trino, st_numpoints_geometry, arg1, "function doc"),

    (trino, st_overlaps_geometry_geometry, arg1 arg2, "function doc"),

    (trino, st_point_double_double, arg1 arg2, "function doc"),

    (trino, st_pointn_geometry_bigint, arg1 arg2, "function doc"),

    (trino, st_points_geometry, arg1, "function doc"),

    (trino, st_polygon_varchar, arg1, "function doc"),

    (trino, st_relate_geometry_geometry_varchar, arg1 arg2 arg3, "function doc"),

    (trino, st_startpoint_geometry, arg1, "function doc"),

    (trino, st_symdifference_geometry_geometry, arg1 arg2, "function doc"),

    (trino, st_touches_geometry_geometry, arg1 arg2, "function doc"),

    (trino, st_union_geometry_geometry, arg1 arg2, "function doc"),

    (trino, st_within_geometry_geometry, arg1 arg2, "function doc"),

    (trino, st_x_geometry, arg1, "function doc"),

    (trino, st_xmax_geometry, arg1, "function doc"),

    (trino, st_xmin_geometry, arg1, "function doc"),

    (trino, st_y_geometry, arg1, "function doc"),

    (trino, st_ymax_geometry, arg1, "function doc"),

    (trino, st_ymin_geometry, arg1, "function doc"),

    (trino, starts_with_varchar_varchar, arg1 arg2, "function doc"),

    (trino, strpos_varchar_varchar, arg1 arg2, "function doc"),
    (trino, strpos_varchar_varchar_bigint, arg1 arg2 arg3, "function doc"),

    (trino, substr_varchar_bigint, arg1 arg2, "function doc"),
    (trino, substr_varchar_bigint_bigint, arg1 arg2 arg3, "function doc"),
    (trino, substr_varbinary_bigint, arg1 arg2, "function doc"),
    (trino, substr_varbinary_bigint_bigint, arg1 arg2 arg3, "function doc"),

    (trino, substring_varchar_bigint, arg1 arg2, "function doc"),
    (trino, substring_varchar_bigint_bigint, arg1 arg2 arg3, "function doc"),

    (trino, tan_double, arg1, "function doc"),

    (trino, tanh_double, arg1, "function doc"),

    (trino, timestamp_objectid_timestamp_0, arg1, "function doc"),

    (trino, timezone_hour_time_p, arg1, "function doc"),
    (trino, timezone_hour_timestamp_p, arg1, "function doc"),

    (trino, timezone_minute_time_p, arg1, "function doc"),
    (trino, timezone_minute_timestamp_p, arg1, "function doc"),

    (trino, to_base_bigint_bigint, arg1 arg2, "function doc"),

    (trino, to_base32_varbinary, arg1, "function doc"),

    (trino, to_base64_varbinary, arg1, "function doc"),

    (trino, to_base64url_varbinary, arg1, "function doc"),

    (trino, to_big_endian_32_bigint, arg1, "function doc"),

    (trino, to_big_endian_64_bigint, arg1, "function doc"),

    (trino, to_char_timestamp_p_varchar, arg1 arg2, "function doc"),

    (trino, to_date_varchar_varchar, arg1 arg2, "function doc"),

    (trino, to_encoded_polyline_geometry, arg1, "function doc"),

    (trino, to_geojson_geometry_sphericalgeography, arg1, "function doc"),

    (trino, to_geometry_sphericalgeography, arg1, "function doc"),

    (trino, to_hex_varbinary, arg1, "function doc"),

    (trino, to_ieee754_32_real, arg1, "function doc"),

    (trino, to_ieee754_64_double, arg1, "function doc"),

    (trino, to_iso8601_date, arg1, "function doc"),
    (trino, to_iso8601_timestamp_p, arg1, "function doc"),

    (trino, to_milliseconds_intervaldaytosecond, arg1, "function doc"),

    (trino, to_spherical_geography_geometry, arg1, "function doc"),

    (trino, to_timestamp_varchar_varchar, arg1 arg2, "function doc"),

    (trino, to_unixtime_timestamp_p, arg1, "function doc"),

    (trino, to_utf8_varchar, arg1, "function doc"),

    (trino, transform_array_1_function_1_11, arg1 arg2, "function doc"),

    (trino, transform_keys_map_13_5_function_13_5_12, arg1 arg2, "function doc"),

    (trino, transform_values_map_4_8_function_4_8_7, arg1 arg2, "function doc"),

    (trino, translate_varchar_varchar_varchar, arg1 arg2 arg3, "function doc"),

    (trino, trim_varchar, arg1, "function doc"),
    (trino, trim_varchar_codepoints, arg1 arg2, "function doc"),

    (trino, trim_array_array_3_bigint, arg1 arg2, "function doc"),

    (trino, truncate_decimal_p_s_bigint, arg1 arg2, "function doc"),
    (trino, truncate_decimal_p_s, arg1, "function doc"),
    (trino, truncate_double, arg1, "function doc"),
    (trino, truncate_real, arg1, "function doc"),

    (trino, try_1, arg1, "function doc"),

    (trino, typeof_1, arg1, "function doc"),

    (trino, upper_varchar, arg1, "function doc"),

    (trino, url_decode_varchar, arg1, "function doc"),

    (trino, url_encode_varchar, arg1, "function doc"),

    (trino, url_extract_fragment_varchar, arg1, "function doc"),

    (trino, url_extract_host_varchar, arg1, "function doc"),

    (trino, url_extract_parameter_varchar_varchar, arg1 arg2, "function doc"),

    (trino, url_extract_path_varchar, arg1, "function doc"),

    (trino, url_extract_port_varchar, arg1, "function doc"),

    (trino, url_extract_protocol_varchar, arg1, "function doc"),

    (trino, url_extract_query_varchar, arg1, "function doc"),

    (trino, uuid, , "function doc"),

    (trino, value_at_quantile_qdigest_double, arg1 arg2, "function doc"),
    (trino, value_at_quantile_tdigest_double, arg1 arg2, "function doc"),

    (trino, values_at_quantiles_qdigest_array_double, arg1 arg2, "function doc"),
    (trino, values_at_quantiles_tdigest_array_double, arg1 arg2, "function doc"),

    (trino, week_date, arg1, "function doc"),
    (trino, week_timestamp_p, arg1, "function doc"),

    (trino, week_of_year_date, arg1, "function doc"),
    (trino, week_of_year_timestamp_p, arg1, "function doc"),

    (trino, width_bucket_double_array_double, arg1 arg2, "function doc"),
    (trino, width_bucket_double_double_double_bigint, arg1 arg2 arg3 arg4, "function doc"),

    (trino, wilson_interval_lower_bigint_bigint_double, arg1 arg2 arg3, "function doc"),

    (trino, wilson_interval_upper_bigint_bigint_double, arg1 arg2 arg3, "function doc"),

    (trino, with_timezone_timestamp_p_varchar, arg1 arg2, "function doc"),

    (trino, word_stem_varchar, arg1, "function doc"),
    (trino, word_stem_varchar_varchar, arg1 arg2, "function doc"),

    (trino, xxhash64_varbinary, arg1, "function doc"),

    (trino, year_date, arg1, "function doc"),
    (trino, year_intervalyeartomonth, arg1, "function doc"),
    (trino, year_timestamp_p, arg1, "function doc"),

    (trino, year_of_week_date, arg1, "function doc"),
    (trino, year_of_week_timestamp_p, arg1, "function doc"),

    (trino, yow_date, arg1, "function doc"),
    (trino, yow_timestamp_p, arg1, "function doc"),

    (trino, zip_array_14_array_15, arg1 arg2, "function doc"),
    (trino, zip_array_14_array_15_array_16, arg1 arg2 arg3, "function doc"),
    (trino, zip_array_14_array_15_array_16_array_17, arg1 arg2 arg3 arg4, "function doc"),
    (trino, zip_array_14_array_15_array_16_array_17_array_18, arg1 arg2 arg3 arg4 arg5, "function doc"),

    (trino, zip_with_array_1_array_11_function_1_11_9, arg1 arg2 arg3, "function doc"),

);
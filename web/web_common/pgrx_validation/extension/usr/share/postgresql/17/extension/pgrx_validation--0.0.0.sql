/* <begin connected objects> */
/*
This file is auto generated by pgrx.

The ordering of items is not stable, it is driven by a dependency graph.
*/
/* </end connected objects> */

/* <begin connected objects> */
-- web/web_common/pgrx_validation/src/lib.rs:24
-- pgrx_validation::must_be_distinct
CREATE  FUNCTION "must_be_distinct"(
	"left" TEXT, /* &str */
	"right" TEXT /* &str */
) RETURNS bool /* bool */
STRICT
LANGUAGE c /* Rust */
AS 'MODULE_PATHNAME', 'must_be_distinct_wrapper';
/* </end connected objects> */

/* <begin connected objects> */
-- web/web_common/pgrx_validation/src/lib.rs:47
-- pgrx_validation::must_be_distinct_i32
CREATE  FUNCTION "must_be_distinct_i32"(
	"left" INT, /* i32 */
	"right" INT /* i32 */
) RETURNS bool /* bool */
STRICT
LANGUAGE c /* Rust */
AS 'MODULE_PATHNAME', 'must_be_distinct_i32_wrapper';
/* </end connected objects> */

/* <begin connected objects> */
-- web/web_common/pgrx_validation/src/lib.rs:70
-- pgrx_validation::must_be_distinct_uuid
CREATE  FUNCTION "must_be_distinct_uuid"(
	"left" uuid, /* rosetta_uuid::Uuid */
	"right" uuid /* rosetta_uuid::Uuid */
) RETURNS bool /* bool */
STRICT
LANGUAGE c /* Rust */
AS 'MODULE_PATHNAME', 'must_be_distinct_uuid_wrapper';
/* </end connected objects> */

/* <begin connected objects> */
-- web/web_common/pgrx_validation/src/lib.rs:93
-- pgrx_validation::must_be_mail
CREATE  FUNCTION "must_be_mail"(
	"value" TEXT /* &str */
) RETURNS bool /* bool */
STRICT
LANGUAGE c /* Rust */
AS 'MODULE_PATHNAME', 'must_be_mail_wrapper';
/* </end connected objects> */

/* <begin connected objects> */
-- web/web_common/pgrx_validation/src/lib.rs:167
-- pgrx_validation::must_be_strictly_greater_than_f32
CREATE  FUNCTION "must_be_strictly_greater_than_f32"(
	"value" real, /* f32 */
	"lower_bound" real /* f32 */
) RETURNS bool /* bool */
STRICT
LANGUAGE c /* Rust */
AS 'MODULE_PATHNAME', 'must_be_strictly_greater_than_f32_wrapper';
/* </end connected objects> */

/* <begin connected objects> */
-- web/web_common/pgrx_validation/src/lib.rs:213
-- pgrx_validation::must_be_strictly_greater_than_f64
CREATE  FUNCTION "must_be_strictly_greater_than_f64"(
	"value" double precision, /* f64 */
	"lower_bound" double precision /* f64 */
) RETURNS bool /* bool */
STRICT
LANGUAGE c /* Rust */
AS 'MODULE_PATHNAME', 'must_be_strictly_greater_than_f64_wrapper';
/* </end connected objects> */

/* <begin connected objects> */
-- web/web_common/pgrx_validation/src/lib.rs:155
-- pgrx_validation::must_be_strictly_positive_f32
CREATE  FUNCTION "must_be_strictly_positive_f32"(
	"value" real /* f32 */
) RETURNS bool /* bool */
STRICT
LANGUAGE c /* Rust */
AS 'MODULE_PATHNAME', 'must_be_strictly_positive_f32_wrapper';
/* </end connected objects> */

/* <begin connected objects> */
-- web/web_common/pgrx_validation/src/lib.rs:134
-- pgrx_validation::must_be_strictly_positive_f64
CREATE  FUNCTION "must_be_strictly_positive_f64"(
	"value" double precision /* f64 */
) RETURNS bool /* bool */
STRICT
LANGUAGE c /* Rust */
AS 'MODULE_PATHNAME', 'must_be_strictly_positive_f64_wrapper';
/* </end connected objects> */

/* <begin connected objects> */
-- web/web_common/pgrx_validation/src/lib.rs:113
-- pgrx_validation::must_be_strictly_positive_i32
CREATE  FUNCTION "must_be_strictly_positive_i32"(
	"value" INT /* i32 */
) RETURNS bool /* bool */
STRICT
LANGUAGE c /* Rust */
AS 'MODULE_PATHNAME', 'must_be_strictly_positive_i32_wrapper';
/* </end connected objects> */

/* <begin connected objects> */
-- web/web_common/pgrx_validation/src/lib.rs:190
-- pgrx_validation::must_be_strictly_smaller_than_f32
CREATE  FUNCTION "must_be_strictly_smaller_than_f32"(
	"value" real, /* f32 */
	"lower_bound" real /* f32 */
) RETURNS bool /* bool */
STRICT
LANGUAGE c /* Rust */
AS 'MODULE_PATHNAME', 'must_be_strictly_smaller_than_f32_wrapper';
/* </end connected objects> */

/* <begin connected objects> */
-- web/web_common/pgrx_validation/src/lib.rs:236
-- pgrx_validation::must_be_strictly_smaller_than_f64
CREATE  FUNCTION "must_be_strictly_smaller_than_f64"(
	"value" double precision, /* f64 */
	"lower_bound" double precision /* f64 */
) RETURNS bool /* bool */
STRICT
LANGUAGE c /* Rust */
AS 'MODULE_PATHNAME', 'must_be_strictly_smaller_than_f64_wrapper';
/* </end connected objects> */

/* <begin connected objects> */
-- web/web_common/pgrx_validation/src/lib.rs:10
-- pgrx_validation::must_not_be_empty
CREATE  FUNCTION "must_not_be_empty"(
	"value" TEXT /* &str */
) RETURNS bool /* bool */
STRICT
LANGUAGE c /* Rust */
AS 'MODULE_PATHNAME', 'must_not_be_empty_wrapper';
/* </end connected objects> */


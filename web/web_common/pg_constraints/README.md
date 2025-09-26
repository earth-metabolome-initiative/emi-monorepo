# PG Constraint

PG-diesel based higher-level constraints built on top of `pg_relations`. These are general-purpose constraints that can and most likely should be used in any SQL project, as they enforce common-sense rules about the schemas which are not enforced by the database itself.

This crate differs from `sql_constraints` in that it is built on top of `pg_relations` and thus is able to catch more sophisticated constraints that require knowledge of the relationships between tables, while the former focuses on constraints that can be enforced within a single SQL statement, so if a database schema has been populated by a mix of SQL statements and other means, `pg_constraints` will be able to catch more issues, while `sql_constraints` is strictly limited to what can be parsed from SQL statements.

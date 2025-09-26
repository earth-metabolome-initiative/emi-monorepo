# SQL Constraints

[sqlparser](https://docs.rs/sqlparser/latest/sqlparser/)-based higher-level constraints. These are general-purpose constraints that can and most likely should be used in any SQL project, as they enforce common-sense rules about the schemas which are not enforced by the database itself.

This crate differs from `pg_constraints` in that it is built on top of `sqlparser` and thus while it is not able to catch as sophisticated constraints as the former, it can be used without executing any SQL code, and focuses on constraints that can be determined solely by parsing a single SQL statement.

Therefore, this crate aims to be a very lightweight no-false-negatives solution for basic SQL constraints which can run in CI environments, while `pg_constraints` aims to be a complete solution for `PostgreSQL`-specific constraints.

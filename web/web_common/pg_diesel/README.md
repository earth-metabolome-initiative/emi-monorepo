# pg_diesel

Diesel models and schemas for PostgreSQL system catalogs (`pg_catalog`, `information_schema`).

## What it does

- Provides Diesel schemas for all PostgreSQL metadata tables
- Includes models for querying system catalogs type-safely
- Offers `PgDatabaseBuilder` for runtime database introspection
- Implements `sql_traits` for generic metadata access

## Known limitations

Some PostgreSQL types are excluded because Diesel can't map them:

- `anyarray` - polymorphic pseudo-type
- `pg_ndistinct`, `pg_dependencies`, `pg_mcv_list`, `_pg_statistic` - internal statistics types

Columns with these types are omitted from generated schemas.

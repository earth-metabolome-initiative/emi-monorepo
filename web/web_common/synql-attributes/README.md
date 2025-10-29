# SynQL Attributes

Crate to generate attribute enumerations from a SQL schema, based on `sql_traits`.

This crate provides functionality to create enumerations of attributes (columns) associated with database tables. These attribute enums are commonly used for:

- Tracking which fields are being set during insertions
- Error reporting (indicating which attribute caused an error)

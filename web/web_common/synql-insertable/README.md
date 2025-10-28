# SynQL Insertable

Crate to generate insertable structs from a SQL schema, based on `sql_traits`.

This crate provides functionality to create Diesel `Insertable` structs that can be used to insert new records into database tables. It automatically handles nullable columns and excludes auto-generated columns like serial primary keys.

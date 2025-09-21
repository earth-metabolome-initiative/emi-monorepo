# PG Diesel

Postgres metadata tables and utilities for Diesel ORM.

## Known limitations

Some of the Postgres tables have more than 32 columns, which is the default number of columns that Diesel supports. While it is possible to increase this limit, it will increase compile times, therefore for now we are dropping a subset of the columns that we don't need. We are open to increasing this limit in the future if we find that we need some of those columns.

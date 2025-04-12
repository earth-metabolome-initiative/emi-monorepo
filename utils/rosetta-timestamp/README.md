# Rosetta Timestamp

Crate providing a standardized mapping for timestamps with time zones for diesel with PostgreSQL and SQLite backends.

The library uses as the underlying data type [`chrono::DateTime<Utc>`](https://docs.rs/chrono/0.4.40/chrono/struct.DateTime.html) since SQLite has
no native timezone support but defaults to UTC (Coordinated Universal Time).

Furthermore, the library provides the trait implementations for the [`pgrx`](https://crates.io/crates/pgrx) crate to allow for using these types in the context of postgres extensions.

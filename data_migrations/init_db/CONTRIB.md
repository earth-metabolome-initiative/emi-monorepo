# CONTRIB

Notes on the extension of the database for additional procedures and data types.

## Adding new procedure template

1. Create a new migration dir under `data_migrations/init_db/migrations/` following the preexisting patterns.
Notably we increment the digits so that the table gets the highest number. Reorganization will be taken care of automatically, if needed.

2. Create the `up.sql` and `down.sql` files in the new migration dir.
The `up.sql` should contain the SQL commands to create the new procedure template, and the `down.sql` should contain the SQL commands to remove it.

3. In `data_migrations/init_db/`, run the init_db tests via `cargo test --release` which will take care of of the reorganization of the dir and checking the constraints.

(deletion of the cache should be automated)

4. At this stage we should be able to move to the codegen part, which will generate the structs associated to the tables and extends the procedure graph and enumeration automatically.

    - For this we switch to `web/core_structures/core_structures_builder`.
    - Remove the cache via `rm -rf cache/`
    - Run `cargo run --release`

## Notes

Here is a list of reserved and non-reserved words in the database that should not be used as identifiers for tables or columns.

https://www.postgresql.org/docs/current/sql-keywords-appendix.html

In our case it is OK to use the `non-reserved` words, but it is not recommended.

For reformatting

4. cd to root
4. switch to nightly `rustup default nightly`
5. format `cargo fmt`
6. switch back to default rust `rustup default stable`


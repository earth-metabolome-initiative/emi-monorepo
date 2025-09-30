## If a change is made on the Directus side

(e.g. a new field is added or an existing field is set to NOT NULL)

1. make the change on Directus (using admin login)
2. cd to `emi-monorepo/data_migrations/directus_migration/directus_migration_builder`
3. `cargo run --release`
4. cd to root
4. switch to nightly `rustup default nightly`
5. format `cargo fmt`
6. switch back to default rust `rustup default stable`




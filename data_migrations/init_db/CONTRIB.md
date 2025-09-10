# CONTRIB

Notes on the extension of the database for additional procedures and data types.

## Adding new procedure template

1. Create a new migration dir under `data_migrations/init_db/migrations/` following the preexisting patterns.
Notably we increment the digits so that the table gets the highest number. Reorganization will be taken care of automatically, if needed.

2. Create the `up.sql` and `down.sql` files in the new migration dir.
The `up.sql` should contain the SQL commands to create the new procedure template, and the `down.sql` should contain the SQL commands to remove it.

3. In `data_migrations/init_db/`, run the init_db tests via `cargo test --release` which will take care of of the reorganization of the dir and checking the constraints.

(deletion of the cahe should be automated)

4. At this stage we should be able to move to the codegen part, which will generate the structs associated to the tables and extends the procedure graph and enumeration automatically.

    - For this we switch to `web/core_structures/core_structures_builder`.
    - Run `cargo run --release`


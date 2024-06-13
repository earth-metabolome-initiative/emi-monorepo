"""Write the database schema to the frontend."""

import os
from glob import glob

from constraint_checkers.ensures_gluesql_compliance import TABLES_IGNORED_FOR_GLUESQL
from constraint_checkers.regroup_tables import (
    get_create_table_desinence,
    is_create_table_migration,
)


def write_frontend_database_schema():
    """Write the database schema to the frontend."""

    # we look in all the migrations in the migrations folder
    migrations = glob("migrations/*/up.sql")

    # we filter the migrations to keep only the ones that create tables
    create_table_migrations = sorted(
        [
            migration
            for migration in migrations
            if is_create_table_migration(migration.rsplit(os.sep, 1)[0])
        ]
    )

    # we also want to filter accoding to out exclusion list
    create_table_migrations = [
        migration
        for migration in create_table_migrations
        if not any(
            get_create_table_desinence(table_name) in migration
            for table_name in TABLES_IGNORED_FOR_GLUESQL
        )
    ]

    path = "../frontend/src/workers/database_schema.rs"

    document = open(path, "w", encoding="utf8")

    document.write(
        "//! This module contains the database schema for the frontend. \n"
        "//!\n"
        "//! This module is automatically generated. Do not write anything here.\n\n"
    )

    imports = [
        "use sql_minifier::macros::load_sql;",
        "use gluesql::prelude::Glue;",
        "use gluesql::prelude::IdbStorage;",
    ]

    document.write("\n".join(imports) + "\n\n")

    # the rust file should now have a concatenation of all the create table migrations
    # such as :
    # const DATABASE_SCHEMA: &str = concat!(
    # and all the table names

    document.write(
        "pub(super) async fn create_schema(database: &mut Glue<IdbStorage>) {\n"
    )
    files_to_load = [
        f'if let Err(error) = database.execute(load_sql!("../backend/{table_creation}")).await {{\n'
        f'    log::error!("Failed to create table {table_creation}: {{:?}}", error);\n'
        '     unreachable!("Failed to create table");\n'
        "}"
        for table_creation in create_table_migrations
    ]
    document.write("\n".join(files_to_load))
    document.write("}\n")

    document.flush()
    document.close()
    print("Database schema written to the frontend.")

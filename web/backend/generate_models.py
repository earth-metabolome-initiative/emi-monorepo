"""This module contains the functions to generate the backend & frontend models."""

import os
import shutil
from typing import List
from dotenv import load_dotenv
from retrieve_taxons import retrieve_taxons
from constraint_checkers import (
    ensure_no_dead_python_code,
    ensures_all_update_at_trigger_exists,
)
from constraint_checkers import ensure_created_at_columns, ensure_updated_at_columns
from constraint_checkers import handle_minimal_revertion
from constraint_checkers import replace_serial_indices
from constraint_checkers import TableStructMetadata, StructMetadata
from constraint_checkers import write_frontend_pages, write_frontend_router_page
from constraint_checkers import (
    enforce_migration_naming_convention,
    ensure_updatable_tables_have_roles_tables,
    ensure_tables_have_creation_notification_trigger,
)
from constraint_checkers.regroup_tables import regroup_tables
from constraint_checkers import generate_view_schema, generate_table_schema
from constraint_checkers import (
    check_parent_circularity_trigger,
    create_filter_structs,
    ensures_migrations_simmetry,
    ensures_gluesql_compliance,
)
from constraint_checkers import (
    check_for_common_typos_in_migrations,
    write_frontend_forms,
    check_schema_completion,
    generate_view_structs,
)
from constraint_checkers import (
    write_web_common_flat_variants,
    write_web_common_update_variants,
    write_web_common_new_variants,
    write_backend_new_variants,
    write_backend_update_variants,
    write_web_common_nested_structs,
    write_backend_flat_variants,
    extract_structs,
)
from constraint_checkers import derive_nested_structs, write_backend_nested_structs
from constraint_checkers import (
    write_web_common_table_names_enumeration,
    write_backend_table_names_enumeration,
    derive_frontend_builders,
    derive_webcommon_new_variants,
    derive_webcommon_update_variants,
    write_web_common_search_trait_implementations,
    write_diesel_sql_function_bindings
)
from constraint_checkers import (
    ensure_can_x_function_existance,
    ensures_no_duplicated_migrations,
    register_derived_search_indices
)


if __name__ == "__main__":
    # Load dotenv file
    load_dotenv()
    check_for_common_typos_in_migrations()
    ensure_no_dead_python_code()
    ensures_no_duplicated_migrations()
    regroup_tables()

    # If there is a "__pycache__" directory, we remove it as Diesel
    # seems to be keen to try and run it as a migration, and it will
    # fail.
    if os.path.exists("__pycache__"):
        shutil.rmtree("__pycache__")

    if os.path.exists("migrations/__pycache__"):
        shutil.rmtree("migrations/__pycache__")

    if not os.path.exists("./db_data/bio_ott_taxons.csv.gz"):
        retrieve_taxons()

    StructMetadata.init_table_metadata()

    enforce_migration_naming_convention()
    replace_serial_indices()
    ensures_migrations_simmetry()
    ensures_gluesql_compliance()
    print("Ensured migrations simmetry & GlueSQL compliance.")

    handle_minimal_revertion()
    generate_table_schema()
    print("Generated models.")

    ensures_all_update_at_trigger_exists(StructMetadata.table_metadata)
    ensure_created_at_columns(StructMetadata.table_metadata)
    ensure_updated_at_columns(StructMetadata.table_metadata)
    check_parent_circularity_trigger(StructMetadata.table_metadata)

    generate_view_schema(StructMetadata.table_metadata)
    print("Generated view schema.")
    check_schema_completion()
    print("Checked schema completion.")
    generate_view_structs()
    print("Generated view structs.")

    table_structs: List[StructMetadata] = extract_structs("src/models.rs")
    view_structs: List[StructMetadata] = extract_structs("src/views/views.rs")
    flat_variants = table_structs + view_structs

    print(
        f"Extracted {len(table_structs)} tables and {len(view_structs)} views from the backend."
    )

    StructMetadata.init_indices()
    register_derived_search_indices(flat_variants)

    filter_structs: List[StructMetadata] = create_filter_structs(
        table_structs + view_structs
    )
    print(f"Generated {len(filter_structs)} filter structs.")

    nested_structs: List[StructMetadata] = derive_nested_structs(
        table_structs + view_structs
    )
    print(f"Derived {len(nested_structs)} nested structs.")
    assert len(nested_structs) > 0, (
        "No nested structs were derived. This is likely due some error in the "
        "generation process. Please rerun the generation script."
    )

    new_model_structs = derive_webcommon_new_variants(table_structs)
    print(f"Derived {len(new_model_structs)} structs for the New versions")

    update_model_structs = derive_webcommon_update_variants(table_structs)
    print(f"Derived {len(update_model_structs)} structs for the Update versions")

    tables: List[TableStructMetadata] = write_web_common_table_names_enumeration(
        table_structs + view_structs + nested_structs,
        new_model_structs,
        update_model_structs,
    )

    write_diesel_sql_function_bindings(StructMetadata.table_metadata)
    ensure_updatable_tables_have_roles_tables(tables, StructMetadata.table_metadata)
    ensure_can_x_function_existance(tables)
    ensure_tables_have_creation_notification_trigger(tables, StructMetadata.table_metadata)
    print("Generated table names enumeration for web_common.")

    write_backend_flat_variants("src/models.rs", "tables", table_structs)
    write_backend_flat_variants("src/views/views.rs", "views", view_structs)
    print(
        f"Generated {len(table_structs)} tables and {len(view_structs)} views implementations for backend."
    )

    write_web_common_flat_variants(table_structs, "tables")
    write_web_common_flat_variants(view_structs, "views")
    print("Generated web common structs.")

    write_backend_nested_structs(nested_structs)
    print(f"Generated {len(nested_structs)} nested structs for backend.")

    write_backend_table_names_enumeration(tables)
    print("Generated table names enumeration for diesel.")

    write_web_common_nested_structs("nested_models.rs", nested_structs)
    print("Generated nested structs for web_common.")

    write_web_common_search_trait_implementations(
        nested_structs + table_structs + view_structs,
    )
    print("Generated search trait implementations for web_common.")

    builder_structs = derive_frontend_builders(new_model_structs + update_model_structs)
    print(f"Derived {len(builder_structs)} builders for the New & Update versions")

    write_web_common_new_variants(new_model_structs)
    write_web_common_update_variants(update_model_structs)
    print("Generated new & update structs for web_common.")

    write_backend_new_variants(new_model_structs)
    write_backend_update_variants(
        update_model_structs
        + [
            new_model_struct
            for new_model_struct in new_model_structs
            if new_model_struct.is_update_variant()
        ]
    )
    print("Generated new & update structs for backend.")

    write_frontend_forms(
        builder_structs,
    )
    print("Generated frontend forms.")

    write_frontend_pages(
        flat_variants,
    )
    print("Generated frontend pages.")

    write_frontend_router_page(
        flat_variants,
    )
    print("Generated frontend router page.")

    # Finally, as we are currently running in the backend, we can now
    # format the generated rust code.
    status = os.system("cargo fmt")
    if status != 0:
        print("Error running 'cargo fmt'!")
        exit(1)
    print("Formatted backend rust code.")
    # And the generated frontend rust code, for which we need to be in the
    # frontend directory.
    os.chdir("../frontend")
    status = os.system("cargo fmt")
    if status != 0:
        print("Error running 'cargo fmt'!")
        exit(1)
    print("Formatted frontend rust code.")
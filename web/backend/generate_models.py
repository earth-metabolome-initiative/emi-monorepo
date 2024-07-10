"""This module contains the functions to generate the backend & frontend models."""

import os
import shutil
from typing import List

from constraint_checkers import (
    StructMetadata,
    check_for_common_typos_in_migrations,
    check_parent_circularity_trigger,
    create_filter_variants,
    derive_frontend_builders,
    derive_nested_structs,
    derive_webcommon_new_variants,
    derive_webcommon_update_variants,
    enforce_migration_naming_convention,
    ensure_can_x_function_existance,
    ensure_created_at_columns,
    ensure_no_dead_python_code,
    ensure_tables_have_creation_notification_trigger,
    ensure_updatable_tables_have_roles_tables,
    ensure_updated_at_columns,
    ensures_all_update_at_trigger_exists,
    ensures_gluesql_compliance,
    ensures_migrations_simmetry,
    ensures_no_duplicated_migrations,
    execute_migrations,
    extract_structs,
    generate_schema,
    handle_minimal_revertion,
    register_derived_search_indices,
    replace_serial_indices,
    update_all_files_hashes,
    update_migrations_hash,
    write_backend_flat_variants,
    write_backend_nested_structs,
    write_backend_new_variants,
    write_backend_table_names_enumeration,
    write_backend_update_variants,
    write_diesel_sql_function_bindings,
    write_diesel_sql_operator_bindings,
    write_diesel_sql_types_bindings,
    write_frontend_forms,
    write_frontend_pages,
    write_frontend_router_page,
    write_web_common_flat_variants,
    write_web_common_nested_variants,
    write_web_common_new_variants,
    write_web_common_search_trait_implementations,
    write_web_common_table_names_enumeration,
    write_web_common_update_variants,
    derive_tables_enumeration,
    derive_web_common_table_methods
)
from constraint_checkers.regroup_tables import regroup_tables
from constraint_checkers.write_frontend_database_schema import (
    write_frontend_database_schema,
)
from dotenv import load_dotenv
from retrieve_taxa import retrieve_taxa

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

    if not os.path.exists("./db_data/bio_ott_taxa.csv.gz"):
        retrieve_taxa()

    StructMetadata.init_table_metadata()

    enforce_migration_naming_convention()
    replace_serial_indices()
    ensures_migrations_simmetry()
    ensures_gluesql_compliance()
    print("Ensured migrations simmetry & GlueSQL compliance.")

    handle_minimal_revertion()
    execute_migrations()

    ensures_all_update_at_trigger_exists(StructMetadata.table_metadata)
    ensure_created_at_columns(StructMetadata.table_metadata)
    ensure_updated_at_columns(StructMetadata.table_metadata)
    check_parent_circularity_trigger(StructMetadata.table_metadata)

    generate_schema(StructMetadata.table_metadata)

    flat_variants: List[StructMetadata] = extract_structs()

    assert len(flat_variants) > 0, (
        "No table structs were extracted. This is likely due some error in the "
        "generation process. Please rerun the generation script."
    )

    print(f"Extracted {len(flat_variants)} tables from the backend.")

    StructMetadata.init_indices()
    register_derived_search_indices(flat_variants)

    filter_variants: List[StructMetadata] = create_filter_variants(flat_variants)
    print(f"Generated {len(filter_variants)} filter structs.")

    nested_structs: List[StructMetadata] = derive_nested_structs(flat_variants)
    print(f"Derived {len(nested_structs)} nested structs.")
    assert len(nested_structs) > 0, (
        "No nested structs were derived. This is likely due some error in the "
        "generation process. Please rerun the generation script."
    )

    new_model_structs = derive_webcommon_new_variants(flat_variants)
    print(f"Derived {len(new_model_structs)} structs for the New versions")

    update_model_structs = derive_webcommon_update_variants(flat_variants)
    print(f"Derived {len(update_model_structs)} structs for the Update versions")

    (table_enum_struct, tables) = derive_tables_enumeration(
        flat_variants + nested_structs,
        new_model_structs,
        update_model_structs,
    )

    ensure_updatable_tables_have_roles_tables(tables, StructMetadata.table_metadata)
    ensure_can_x_function_existance(tables)
    ensure_tables_have_creation_notification_trigger(
        tables, StructMetadata.table_metadata
    )
    print("Generated table names enumeration for web_common.")

    write_diesel_sql_function_bindings(StructMetadata.table_metadata)
    write_diesel_sql_operator_bindings(StructMetadata.table_metadata)
    write_diesel_sql_types_bindings()
    
    write_backend_flat_variants(flat_variants)
    print(f"Generated {len(flat_variants)} tables implementations for backend.")

    write_backend_nested_structs(nested_structs)
    print(f"Generated {len(nested_structs)} nested structs for backend.")

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

    # We RC-ify the nested structs
    for nested_struct in nested_structs:
        rc_nested_struct = nested_struct.into_rc()

    write_web_common_flat_variants(flat_variants)
    print("Generated web common structs.")

    write_web_common_nested_variants(nested_structs)
    print("Generated nested structs for web_common.")

    write_web_common_search_trait_implementations(nested_structs + flat_variants)
    print("Generated search trait implementations for web_common.")

    derive_web_common_table_methods(
        table_enum_struct=table_enum_struct,
        tables=tables,
    )

    write_web_common_table_names_enumeration(
        tables,
        table_enum_struct,
    )
    
    write_backend_table_names_enumeration(tables)
    print("Generated table names enumeration for diesel.")

    builder_structs = derive_frontend_builders(new_model_structs + update_model_structs)
    print(f"Derived {len(builder_structs)} builders for the New & Update versions")

    write_web_common_new_variants(new_model_structs)
    write_web_common_update_variants(update_model_structs)
    print("Generated new & update structs for web_common.")

    write_frontend_forms(
        builder_structs,
    )

    write_frontend_pages(
        flat_variants,
    )
    print("Generated frontend pages.")

    write_frontend_router_page(
        flat_variants,
    )
    print("Generated frontend router page.")

    write_frontend_database_schema()
    # Since everything run appropriately since the last time we updated the
    # migrations hash in the last generation, we update it now.
    update_migrations_hash()
    # And analogously for the files hashes.
    update_all_files_hashes()

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
    # And the webcommon code.
    os.chdir("../web_common")
    status = os.system("cargo fmt")
    if status != 0:
        print("Error running 'cargo fmt'!")
        exit(1)

    print("Formatted frontend rust code.")

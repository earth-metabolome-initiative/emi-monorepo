use diesel::prelude::{allow_tables_to_appear_in_same_query, table};

table! {
    information_schema.tables (table_catalog, table_schema, table_name) {
        table_catalog -> Text,
        table_schema -> Text,
        table_name -> Text,
        table_type -> Text,
        self_referencing_column_name -> Nullable<Text>,
        reference_generation -> Nullable<Text>,
        user_defined_type_catalog -> Nullable<Text>,
        user_defined_type_schema -> Nullable<Text>,
        user_defined_type_name -> Nullable<Text>,
        is_insertable_into -> Text,
        is_typed -> Text,
        commit_action -> Nullable<Text>,
    }
}

table! {
    information_schema.columns (table_catalog, table_schema, table_name, column_name) {
        table_catalog -> Text,
        table_schema -> Text,
        table_name -> Text,
        column_name -> Text,
        ordinal_position -> Integer,
        column_default -> Nullable<Text>,
        #[sql_name = "is_nullable"]
        __is_nullable -> Text,
        data_type -> Text,
        character_maximum_length -> Nullable<Integer>,
        character_octet_length -> Nullable<Integer>,
        numeric_precision -> Nullable<Integer>,
        numeric_precision_radix -> Nullable<Integer>,
        numeric_scale -> Nullable<Integer>,
        datetime_precision -> Nullable<Integer>,
        interval_type -> Nullable<Text>,
        interval_precision -> Nullable<Integer>,
        character_set_catalog -> Nullable<Text>,
        character_set_schema -> Nullable<Text>,
        character_set_name -> Nullable<Text>,
        collation_catalog -> Nullable<Text>,
        collation_schema -> Nullable<Text>,
        collation_name -> Nullable<Text>,
        domain_catalog -> Nullable<Text>,
        domain_schema -> Nullable<Text>,
        domain_name -> Nullable<Text>,
        udt_catalog -> Nullable<Text>,
        udt_schema -> Nullable<Text>,
        udt_name -> Nullable<Text>,
        scope_catalog -> Nullable<Text>,
        scope_schema -> Nullable<Text>,
        scope_name -> Nullable<Text>,
        maximum_cardinality -> Nullable<Integer>,
        dtd_identifier -> Nullable<Text>,
        is_self_referencing -> Nullable<Text>,
        is_identity -> Nullable<Text>,
        identity_generation -> Nullable<Text>,
        identity_start -> Nullable<Text>,
        identity_increment -> Nullable<Text>,
        identity_maximum -> Nullable<Text>,
        identity_minimum -> Nullable<Text>,
        identity_cycle -> Nullable<Text>,
        is_generated -> Text,
        generation_expression -> Nullable<Text>,
        is_updatable -> Text,
    }
}

table! {
    information_schema.key_column_usage (table_catalog, table_schema, table_name, column_name) {
        constraint_catalog -> Text,
        constraint_schema -> Text,
        constraint_name -> Text,
        table_catalog -> Text,
        table_schema -> Text,
        table_name -> Text,
        column_name -> Text,
        ordinal_position -> Integer,
        position_in_unique_constraint -> Nullable<Integer>,
    }
}

table! {
    information_schema.table_constraints (table_catalog, table_schema, table_name, constraint_name) {
        constraint_catalog -> Text,
        constraint_schema -> Text,
        constraint_name -> Text,
        table_catalog -> Text,
        table_schema -> Text,
        table_name -> Text,
        constraint_type -> Text,
        is_deferrable -> Text,
        initially_deferred -> Text,
        enforced -> Text,
        nulls_distinct -> Nullable<Text>,
    }
}

table! {
    information_schema.referential_constraints (constraint_catalog, constraint_schema, constraint_name) {
        constraint_catalog -> Text,
        constraint_schema -> Text,
        constraint_name -> Text,
        unique_constraint_catalog -> Nullable<Text>,
        unique_constraint_schema -> Nullable<Text>,
        unique_constraint_name -> Nullable<Text>,
        match_option -> Text,
        update_rule -> Text,
        delete_rule -> Text,
    }
}

table! {
    information_schema.constraint_column_usage (table_catalog, table_schema, table_name, column_name) {
        constraint_catalog -> Text,
        constraint_schema -> Text,
        constraint_name -> Text,
        table_catalog -> Text,
        table_schema -> Text,
        table_name -> Text,
        column_name -> Text,
    }
}

table! {
    information_schema.check_constraints (constraint_catalog, constraint_schema, constraint_name) {
        constraint_catalog -> Text,
        constraint_schema -> Text,
        constraint_name -> Text,
        check_clause -> Text,
    }
}

table! {
    information_schema.domain_constraints (constraint_catalog, constraint_schema, constraint_name) {
        constraint_catalog -> Text,
        constraint_schema -> Text,
        constraint_name -> Text,
        domain_catalog -> Nullable<Text>,
        domain_schema -> Nullable<Text>,
        domain_name -> Nullable<Text>,
        is_deferrable -> Text,
        initially_deferred -> Text,
    }
}

table! {
    pg_indexes (schemaname, tablename, indexname) {
        schemaname -> Text,
        tablename -> Text,
        indexname -> Text,
        tablespace -> Nullable<Text>,
        indexdef -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    columns,
    key_column_usage,
    table_constraints,
    referential_constraints,
    constraint_column_usage,
    check_constraints,
    domain_constraints,
    tables,
);

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
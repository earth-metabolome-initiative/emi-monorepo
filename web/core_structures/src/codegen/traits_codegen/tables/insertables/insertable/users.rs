impl web_common_traits::database::Insertable for crate::User {
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableUserBuilder;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableUser;
}

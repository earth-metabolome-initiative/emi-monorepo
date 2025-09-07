impl web_common_traits::database::Insertable for crate::EmailProvider {
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableEmailProviderBuilder;
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableEmailProvider;
}

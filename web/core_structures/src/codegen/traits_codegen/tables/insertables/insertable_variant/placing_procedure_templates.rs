impl<
    GeolocationProcedureTemplate,
> web_common_traits::database::DispatchableInsertVariantMetadata
for crate::codegen::structs_codegen::tables::insertables::InsertablePlacingProcedureTemplateBuilder<
    GeolocationProcedureTemplate,
> {
    type Row = crate::codegen::structs_codegen::tables::placing_procedure_templates::PlacingProcedureTemplate;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::PlacingProcedureTemplateAttribute,
    >;
}
impl<GeolocationProcedureTemplate> web_common_traits::database::InsertableVariantMetadata
for crate::codegen::structs_codegen::tables::insertables::InsertablePlacingProcedureTemplateBuilder<
    GeolocationProcedureTemplate,
> {
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertablePlacingProcedureTemplate;
}
#[cfg(feature = "backend")]
impl<GeolocationProcedureTemplate> web_common_traits::database::BackendInsertableVariant
for crate::codegen::structs_codegen::tables::insertables::InsertablePlacingProcedureTemplateBuilder<
    GeolocationProcedureTemplate,
>
where
    Self: web_common_traits::database::DispatchableInsertableVariant<
        diesel::PgConnection,
    >,
{}
impl<
    C: diesel::connection::LoadConnection,
    GeolocationProcedureTemplate,
> web_common_traits::database::DispatchableInsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertablePlacingProcedureTemplateBuilder<
    GeolocationProcedureTemplate,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::placing_procedure_templates::PlacingProcedureTemplate as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertablePlacingProcedureTemplate as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::placing_procedure_templates::PlacingProcedureTemplate as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::placing_procedure_templates::PlacingProcedureTemplate,
    >,
    Self: web_common_traits::database::InsertableVariant<
        C,
        InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertablePlacingProcedureTemplate,
        Row = crate::codegen::structs_codegen::tables::placing_procedure_templates::PlacingProcedureTemplate,
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::PlacingProcedureTemplateAttribute,
        >,
    >,
    Self: web_common_traits::database::MostConcreteTable,
{
    fn insert(mut self, user_id: i32, conn: &mut C) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::InsertableVariant;
        use web_common_traits::database::MostConcreteTable;
        self.set_most_concrete_table("placing_procedure_templates");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertablePlacingProcedureTemplate = self
            .try_insert(user_id, conn)?;
        Ok(
            diesel::insert_into(Self::table())
                .values(insertable_struct)
                .get_result(conn)?,
        )
    }
}
impl<
    C: diesel::connection::LoadConnection,
    GeolocationProcedureTemplate,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertablePlacingProcedureTemplateBuilder<
    GeolocationProcedureTemplate,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::placing_procedure_templates::PlacingProcedureTemplate as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertablePlacingProcedureTemplate as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::placing_procedure_templates::PlacingProcedureTemplate as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::placing_procedure_templates::PlacingProcedureTemplate,
    >,
    Self::Error: web_common_traits::database::FromExtension<
        <GeolocationProcedureTemplate as web_common_traits::database::TryInsertGeneric<
            C,
        >>::Error,
    >,
    GeolocationProcedureTemplate: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
{
    fn try_insert(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::InsertableVariant, Self::Error> {
        use web_common_traits::database::FromExtension;
        let procedure_template = self
            .procedure_template
            .mint_primary_key(user_id, conn)
            .map_err(Self::Error::from_extension)?;
        Ok(Self::InsertableVariant {
            procedure_template,
        })
    }
}

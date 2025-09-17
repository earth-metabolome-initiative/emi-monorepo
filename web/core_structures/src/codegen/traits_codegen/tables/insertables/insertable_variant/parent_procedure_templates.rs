impl web_common_traits::database::DispatchableInsertVariantMetadata
for crate::codegen::structs_codegen::tables::insertables::InsertableParentProcedureTemplateBuilder {
    type Row = crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::ParentProcedureTemplateAttribute,
    >;
}
impl web_common_traits::database::InsertableVariantMetadata
for crate::codegen::structs_codegen::tables::insertables::InsertableParentProcedureTemplateBuilder {
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableParentProcedureTemplate;
}
#[cfg(feature = "backend")]
impl web_common_traits::database::BackendInsertableVariant
for crate::codegen::structs_codegen::tables::insertables::InsertableParentProcedureTemplateBuilder
where
    Self: web_common_traits::database::DispatchableInsertableVariant<
        diesel::PgConnection,
    >,
{}
impl<
    C: diesel::connection::LoadConnection,
> web_common_traits::database::DispatchableInsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableParentProcedureTemplateBuilder
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableParentProcedureTemplate as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate,
    >,
    Self: web_common_traits::database::InsertableVariant<
        C,
        InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableParentProcedureTemplate,
        Row = crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate,
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::ParentProcedureTemplateAttribute,
        >,
    >,
{
    fn insert(self, user_id: i32, conn: &mut C) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::InsertableVariant;
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableParentProcedureTemplate = self
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
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableParentProcedureTemplateBuilder
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableParentProcedureTemplate as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate,
    >,
{
    fn try_insert(
        self,
        _user_id: i32,
        _conn: &mut C,
    ) -> Result<Self::InsertableVariant, Self::Error> {
        let parent = self
            .parent
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::ParentProcedureTemplateAttribute::Parent,
                ),
            )?;
        let child = self
            .child
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::ParentProcedureTemplateAttribute::Child,
                ),
            )?;
        let created_by = self
            .created_by
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::ParentProcedureTemplateAttribute::CreatedBy,
                ),
            )?;
        let created_at = self
            .created_at
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::ParentProcedureTemplateAttribute::CreatedAt,
                ),
            )?;
        Ok(Self::InsertableVariant {
            parent,
            child,
            created_by,
            created_at,
        })
    }
}

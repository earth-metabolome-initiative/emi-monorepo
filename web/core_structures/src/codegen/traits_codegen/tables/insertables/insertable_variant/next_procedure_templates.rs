impl web_common_traits::database::InsertableVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableNextProcedureTemplateBuilder
{
    type Row =
        crate::codegen::structs_codegen::tables::next_procedure_templates::NextProcedureTemplate;
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableNextProcedureTemplate;
    type UserId = i32;
}
impl<
    C: diesel::connection::LoadConnection,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableNextProcedureTemplateBuilder
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::next_procedure_templates::NextProcedureTemplate as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableNextProcedureTemplate as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::next_procedure_templates::NextProcedureTemplate as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::next_procedure_templates::NextProcedureTemplate,
    >,
{
    fn insert(
        self,
        user_id: Self::UserId,
        conn: &mut C,
    ) -> Result<
        Self::Row,
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::NextProcedureTemplateAttribute,
        >,
    > {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableNextProcedureTemplate = self
            .try_insert(user_id, conn)?;
        Ok(
            diesel::insert_into(Self::Row::table())
                .values(insertable_struct)
                .get_result(conn)?,
        )
    }
    fn try_insert(
        self,
        _user_id: i32,
        _conn: &mut C,
    ) -> Result<
        Self::InsertableVariant,
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::NextProcedureTemplateAttribute,
        >,
    > {
        let parent = self
            .parent
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::NextProcedureTemplateAttribute::Parent,
                ),
            )?;
        let predecessor = self
            .predecessor
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::NextProcedureTemplateAttribute::Predecessor,
                ),
            )?;
        let successor = self
            .successor
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::NextProcedureTemplateAttribute::Successor,
                ),
            )?;
        let created_by = self
            .created_by
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::NextProcedureTemplateAttribute::CreatedBy,
                ),
            )?;
        let created_at = self
            .created_at
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::NextProcedureTemplateAttribute::CreatedAt,
                ),
            )?;
        Ok(Self::InsertableVariant {
            parent,
            predecessor,
            successor,
            created_by,
            created_at,
        })
    }
}

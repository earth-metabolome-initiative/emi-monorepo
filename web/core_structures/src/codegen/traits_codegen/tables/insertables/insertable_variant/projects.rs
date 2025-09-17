impl web_common_traits::database::DispatchableInsertVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableProjectBuilder
{
    type Row = crate::codegen::structs_codegen::tables::projects::Project;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::ProjectAttribute,
    >;
}
impl web_common_traits::database::InsertableVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableProjectBuilder
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableProject;
}
#[cfg(feature = "backend")]
impl web_common_traits::database::BackendInsertableVariant
    for crate::codegen::structs_codegen::tables::insertables::InsertableProjectBuilder
where
    Self: web_common_traits::database::DispatchableInsertableVariant<diesel::PgConnection>,
{
}
impl<
    C: diesel::connection::LoadConnection,
> web_common_traits::database::DispatchableInsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableProjectBuilder
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::projects::Project as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableProject as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::projects::Project as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::projects::Project,
    >,
    Self: web_common_traits::database::InsertableVariant<
        C,
        InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableProject,
        Row = crate::codegen::structs_codegen::tables::projects::Project,
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::ProjectAttribute,
        >,
    >,
{
    fn insert(self, user_id: i32, conn: &mut C) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::InsertableVariant;
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableProject = self
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
for crate::codegen::structs_codegen::tables::insertables::InsertableProjectBuilder
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::projects::Project as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableProject as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::projects::Project as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::projects::Project,
    >,
{
    fn try_insert(
        self,
        _user_id: i32,
        _conn: &mut C,
    ) -> Result<Self::InsertableVariant, Self::Error> {
        let id = self
            .id
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::ProjectAttribute::Id,
                ),
            )?;
        let name = self
            .name
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::ProjectAttribute::Name,
                ),
            )?;
        let description = self
            .description
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::ProjectAttribute::Description,
                ),
            )?;
        let state_id = self
            .state_id
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::ProjectAttribute::StateId,
                ),
            )?;
        let icon = self
            .icon
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::ProjectAttribute::Icon,
                ),
            )?;
        let color_id = self
            .color_id
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::ProjectAttribute::ColorId,
                ),
            )?;
        let created_by = self
            .created_by
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::ProjectAttribute::CreatedBy,
                ),
            )?;
        let created_at = self
            .created_at
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::ProjectAttribute::CreatedAt,
                ),
            )?;
        let updated_by = self
            .updated_by
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::ProjectAttribute::UpdatedBy,
                ),
            )?;
        let updated_at = self
            .updated_at
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::ProjectAttribute::UpdatedAt,
                ),
            )?;
        let expected_end_date = self
            .expected_end_date
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::ProjectAttribute::ExpectedEndDate,
                ),
            )?;
        let end_date = self
            .end_date
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::ProjectAttribute::EndDate,
                ),
            )?;
        Ok(Self::InsertableVariant {
            id,
            name,
            description,
            state_id,
            icon,
            color_id,
            parent_project_id: self.parent_project_id,
            budget: self.budget,
            expenses: self.expenses,
            created_by,
            created_at,
            updated_by,
            updated_at,
            expected_end_date,
            end_date,
        })
    }
}

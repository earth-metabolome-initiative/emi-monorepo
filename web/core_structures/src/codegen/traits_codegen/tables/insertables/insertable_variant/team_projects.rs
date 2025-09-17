impl web_common_traits::database::DispatchableInsertVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableTeamProjectBuilder
{
    type Row = crate::codegen::structs_codegen::tables::team_projects::TeamProject;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::TeamProjectAttribute,
    >;
}
impl web_common_traits::database::InsertableVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableTeamProjectBuilder
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableTeamProject;
}
#[cfg(feature = "backend")]
impl web_common_traits::database::BackendInsertableVariant
    for crate::codegen::structs_codegen::tables::insertables::InsertableTeamProjectBuilder
where
    Self: web_common_traits::database::DispatchableInsertableVariant<diesel::PgConnection>,
{
}
impl<
    C: diesel::connection::LoadConnection,
> web_common_traits::database::DispatchableInsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableTeamProjectBuilder
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::team_projects::TeamProject as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableTeamProject as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::team_projects::TeamProject as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::team_projects::TeamProject,
    >,
    Self: web_common_traits::database::InsertableVariant<
        C,
        InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableTeamProject,
        Row = crate::codegen::structs_codegen::tables::team_projects::TeamProject,
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::TeamProjectAttribute,
        >,
    >,
{
    fn insert(self, user_id: i32, conn: &mut C) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::InsertableVariant;
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableTeamProject = self
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
for crate::codegen::structs_codegen::tables::insertables::InsertableTeamProjectBuilder
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::team_projects::TeamProject as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableTeamProject as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::team_projects::TeamProject as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::team_projects::TeamProject,
    >,
{
    fn try_insert(
        self,
        _user_id: i32,
        _conn: &mut C,
    ) -> Result<Self::InsertableVariant, Self::Error> {
        let team_id = self
            .team_id
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::TeamProjectAttribute::TeamId,
                ),
            )?;
        let project_id = self
            .project_id
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::TeamProjectAttribute::ProjectId,
                ),
            )?;
        Ok(Self::InsertableVariant {
            team_id,
            project_id,
        })
    }
}

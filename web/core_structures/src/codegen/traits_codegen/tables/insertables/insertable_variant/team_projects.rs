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
    C: diesel::connection::LoadConnection,
    crate::codegen::structs_codegen::tables::projects::Project: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::projects::Project: web_common_traits::database::Updatable<
        C,
        UserId = i32,
    >,
    crate::codegen::structs_codegen::tables::teams::Team: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::teams::Team: web_common_traits::database::Updatable<
        C,
        UserId = i32,
    >,
{
    type Row = crate::codegen::structs_codegen::tables::team_projects::TeamProject;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableTeamProject;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::TeamProjectAttribute,
    >;
    type UserId = i32;
    fn insert(
        self,
        user_id: Self::UserId,
        conn: &mut C,
    ) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::Updatable;
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableTeamProject = self
            .try_insert(user_id, conn)?;
        if !insertable_struct.project(conn)?.can_update(user_id, conn)? {
            return Err(
                generic_backend_request_errors::GenericBackendRequestError::Unauthorized
                    .into(),
            );
        }
        if !insertable_struct.team(conn)?.can_update(user_id, conn)? {
            return Err(
                generic_backend_request_errors::GenericBackendRequestError::Unauthorized
                    .into(),
            );
        }
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

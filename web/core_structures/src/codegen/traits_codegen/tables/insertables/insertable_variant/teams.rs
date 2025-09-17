impl web_common_traits::database::DispatchableInsertVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableTeamBuilder
{
    type Row = crate::codegen::structs_codegen::tables::teams::Team;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::TeamAttribute,
    >;
}
impl web_common_traits::database::InsertableVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableTeamBuilder
{
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableTeam;
}
#[cfg(feature = "backend")]
impl web_common_traits::database::BackendInsertableVariant
    for crate::codegen::structs_codegen::tables::insertables::InsertableTeamBuilder
where
    Self: web_common_traits::database::DispatchableInsertableVariant<diesel::PgConnection>,
{
}
impl<
    C: diesel::connection::LoadConnection,
> web_common_traits::database::DispatchableInsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableTeamBuilder
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::teams::Team as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableTeam as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::teams::Team as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::teams::Team,
    >,
    Self: web_common_traits::database::InsertableVariant<
        C,
        InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableTeam,
        Row = crate::codegen::structs_codegen::tables::teams::Team,
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::TeamAttribute,
        >,
    >,
{
    fn insert(self, user_id: i32, conn: &mut C) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::InsertableVariant;
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableTeam = self
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
for crate::codegen::structs_codegen::tables::insertables::InsertableTeamBuilder
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::teams::Team as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableTeam as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::teams::Team as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::teams::Team,
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
                    crate::codegen::structs_codegen::tables::insertables::TeamAttribute::Id,
                ),
            )?;
        let name = self
            .name
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::TeamAttribute::Name,
                ),
            )?;
        let description = self
            .description
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::TeamAttribute::Description,
                ),
            )?;
        let icon = self
            .icon
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::TeamAttribute::Icon,
                ),
            )?;
        let color_id = self
            .color_id
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::TeamAttribute::ColorId,
                ),
            )?;
        let state_id = self
            .state_id
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::TeamAttribute::StateId,
                ),
            )?;
        let created_by = self
            .created_by
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::TeamAttribute::CreatedBy,
                ),
            )?;
        let created_at = self
            .created_at
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::TeamAttribute::CreatedAt,
                ),
            )?;
        let updated_by = self
            .updated_by
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::TeamAttribute::UpdatedBy,
                ),
            )?;
        let updated_at = self
            .updated_at
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::TeamAttribute::UpdatedAt,
                ),
            )?;
        Ok(Self::InsertableVariant {
            id,
            name,
            description,
            icon,
            color_id,
            state_id,
            parent_team_id: self.parent_team_id,
            created_by,
            created_at,
            updated_by,
            updated_at,
        })
    }
}

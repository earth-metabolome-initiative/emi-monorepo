impl web_common_traits::database::DispatchableInsertVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableTeamStateBuilder
{
    type Row = crate::codegen::structs_codegen::tables::team_states::TeamState;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::TeamStateAttribute,
    >;
}
impl web_common_traits::database::InsertableVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableTeamStateBuilder
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableTeamState;
}
#[cfg(feature = "backend")]
impl web_common_traits::database::BackendInsertableVariant
    for crate::codegen::structs_codegen::tables::insertables::InsertableTeamStateBuilder
where
    Self: web_common_traits::database::DispatchableInsertableVariant<diesel::PgConnection>,
{
}
impl<
    C: diesel::connection::LoadConnection,
> web_common_traits::database::DispatchableInsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableTeamStateBuilder
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::team_states::TeamState as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableTeamState as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::team_states::TeamState as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::team_states::TeamState,
    >,
    Self: web_common_traits::database::InsertableVariant<
        C,
        InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableTeamState,
        Row = crate::codegen::structs_codegen::tables::team_states::TeamState,
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::TeamStateAttribute,
        >,
    >,
{
    fn insert(self, user_id: i32, conn: &mut C) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::InsertableVariant;
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableTeamState = self
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
for crate::codegen::structs_codegen::tables::insertables::InsertableTeamStateBuilder
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::team_states::TeamState as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableTeamState as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::team_states::TeamState as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::team_states::TeamState,
    >,
{
    fn try_insert(
        self,
        _user_id: i32,
        _conn: &mut C,
    ) -> Result<Self::InsertableVariant, Self::Error> {
        let name = self
            .name
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::TeamStateAttribute::Name,
                ),
            )?;
        let description = self
            .description
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::TeamStateAttribute::Description,
                ),
            )?;
        let icon = self
            .icon
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::TeamStateAttribute::Icon,
                ),
            )?;
        let color_id = self
            .color_id
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::TeamStateAttribute::ColorId,
                ),
            )?;
        Ok(Self::InsertableVariant {
            name,
            description,
            icon,
            color_id,
        })
    }
}

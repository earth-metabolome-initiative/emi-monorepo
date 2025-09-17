impl web_common_traits::database::DispatchableInsertVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableTeamMemberBuilder
{
    type Row = crate::codegen::structs_codegen::tables::team_members::TeamMember;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::TeamMemberAttribute,
    >;
}
impl web_common_traits::database::InsertableVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableTeamMemberBuilder
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableTeamMember;
}
#[cfg(feature = "backend")]
impl web_common_traits::database::BackendInsertableVariant
    for crate::codegen::structs_codegen::tables::insertables::InsertableTeamMemberBuilder
where
    Self: web_common_traits::database::DispatchableInsertableVariant<diesel::PgConnection>,
{
}
impl<
    C: diesel::connection::LoadConnection,
> web_common_traits::database::DispatchableInsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableTeamMemberBuilder
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::team_members::TeamMember as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableTeamMember as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::team_members::TeamMember as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::team_members::TeamMember,
    >,
    Self: web_common_traits::database::InsertableVariant<
        C,
        InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableTeamMember,
        Row = crate::codegen::structs_codegen::tables::team_members::TeamMember,
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::TeamMemberAttribute,
        >,
    >,
{
    fn insert(self, user_id: i32, conn: &mut C) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::InsertableVariant;
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableTeamMember = self
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
for crate::codegen::structs_codegen::tables::insertables::InsertableTeamMemberBuilder
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::team_members::TeamMember as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableTeamMember as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::team_members::TeamMember as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::team_members::TeamMember,
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
                    crate::codegen::structs_codegen::tables::insertables::TeamMemberAttribute::TeamId,
                ),
            )?;
        let member_id = self
            .member_id
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::TeamMemberAttribute::MemberId,
                ),
            )?;
        Ok(Self::InsertableVariant {
            team_id,
            member_id,
        })
    }
}

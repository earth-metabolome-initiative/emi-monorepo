impl web_common_traits::database::InsertableVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableTeamMemberBuilder
{
    type Row = crate::codegen::structs_codegen::tables::team_members::TeamMember;
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableTeamMember;
    type UserId = i32;
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
    fn insert(
        self,
        user_id: Self::UserId,
        conn: &mut C,
    ) -> Result<
        Self::Row,
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::TeamMemberAttribute,
        >,
    > {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableTeamMember = self
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
            crate::codegen::structs_codegen::tables::insertables::TeamMemberAttribute,
        >,
    > {
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

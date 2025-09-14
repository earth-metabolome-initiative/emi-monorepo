impl web_common_traits::database::InsertableVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableUserOrganizationBuilder
{
    type Row = crate::codegen::structs_codegen::tables::user_organizations::UserOrganization;
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableUserOrganization;
    type UserId = i32;
}
impl<
    C: diesel::connection::LoadConnection,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableUserOrganizationBuilder
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::user_organizations::UserOrganization as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableUserOrganization as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::user_organizations::UserOrganization as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::user_organizations::UserOrganization,
    >,
{
    fn insert(
        self,
        user_id: Self::UserId,
        conn: &mut C,
    ) -> Result<
        Self::Row,
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::UserOrganizationAttribute,
        >,
    > {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableUserOrganization = self
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
            crate::codegen::structs_codegen::tables::insertables::UserOrganizationAttribute,
        >,
    > {
        let user_id = self
            .user_id
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::UserOrganizationAttribute::UserId,
                ),
            )?;
        let organization_id = self
            .organization_id
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::UserOrganizationAttribute::OrganizationId,
                ),
            )?;
        Ok(Self::InsertableVariant {
            user_id,
            organization_id,
        })
    }
}

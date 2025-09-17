impl web_common_traits::database::DispatchableInsertVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableUserOrganizationBuilder
{
    type Row = crate::codegen::structs_codegen::tables::user_organizations::UserOrganization;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::UserOrganizationAttribute,
    >;
}
impl web_common_traits::database::InsertableVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableUserOrganizationBuilder
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableUserOrganization;
}
#[cfg(feature = "backend")]
impl web_common_traits::database::BackendInsertableVariant
    for crate::codegen::structs_codegen::tables::insertables::InsertableUserOrganizationBuilder
where
    Self: web_common_traits::database::DispatchableInsertableVariant<diesel::PgConnection>,
{
}
impl<
    C: diesel::connection::LoadConnection,
> web_common_traits::database::DispatchableInsertableVariant<C>
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
    Self: web_common_traits::database::InsertableVariant<
        C,
        InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableUserOrganization,
        Row = crate::codegen::structs_codegen::tables::user_organizations::UserOrganization,
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::UserOrganizationAttribute,
        >,
    >,
{
    fn insert(self, user_id: i32, conn: &mut C) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::InsertableVariant;
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableUserOrganization = self
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
    fn try_insert(
        self,
        _user_id: i32,
        _conn: &mut C,
    ) -> Result<Self::InsertableVariant, Self::Error> {
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

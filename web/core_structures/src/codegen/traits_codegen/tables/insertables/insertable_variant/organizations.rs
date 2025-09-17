impl web_common_traits::database::DispatchableInsertVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableOrganizationBuilder
{
    type Row = crate::codegen::structs_codegen::tables::organizations::Organization;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::OrganizationAttribute,
    >;
}
impl web_common_traits::database::InsertableVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableOrganizationBuilder
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableOrganization;
}
#[cfg(feature = "backend")]
impl web_common_traits::database::BackendInsertableVariant
    for crate::codegen::structs_codegen::tables::insertables::InsertableOrganizationBuilder
where
    Self: web_common_traits::database::DispatchableInsertableVariant<diesel::PgConnection>,
{
}
impl<
    C: diesel::connection::LoadConnection,
> web_common_traits::database::DispatchableInsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableOrganizationBuilder
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::organizations::Organization as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableOrganization as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::organizations::Organization as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::organizations::Organization,
    >,
    Self: web_common_traits::database::InsertableVariant<
        C,
        InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableOrganization,
        Row = crate::codegen::structs_codegen::tables::organizations::Organization,
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::OrganizationAttribute,
        >,
    >,
{
    fn insert(self, user_id: i32, conn: &mut C) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::InsertableVariant;
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableOrganization = self
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
for crate::codegen::structs_codegen::tables::insertables::InsertableOrganizationBuilder
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::organizations::Organization as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableOrganization as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::organizations::Organization as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::organizations::Organization,
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
                    crate::codegen::structs_codegen::tables::insertables::OrganizationAttribute::Name,
                ),
            )?;
        let url = self
            .url
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::OrganizationAttribute::Url,
                ),
            )?;
        let country = self
            .country
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::OrganizationAttribute::Country,
                ),
            )?;
        let alpha_two_code = self
            .alpha_two_code
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::OrganizationAttribute::AlphaTwoCode,
                ),
            )?;
        let domain = self
            .domain
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::OrganizationAttribute::Domain,
                ),
            )?;
        Ok(Self::InsertableVariant {
            name,
            url,
            country,
            alpha_two_code,
            state_province: self.state_province,
            domain,
        })
    }
}

impl web_common_traits::database::InsertableVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableOrganizationBuilder
{
    type Row = crate::codegen::structs_codegen::tables::organizations::Organization;
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableOrganization;
    type UserId = i32;
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
    fn insert(
        self,
        user_id: Self::UserId,
        conn: &mut C,
    ) -> Result<
        Self::Row,
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::OrganizationAttribute,
        >,
    > {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableOrganization = self
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
            crate::codegen::structs_codegen::tables::insertables::OrganizationAttribute,
        >,
    > {
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

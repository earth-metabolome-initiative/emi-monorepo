impl<
    C: diesel::connection::LoadConnection,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableLoginProviderBuilder
where
    diesel::query_builder::InsertStatement<
        <crate::LoginProvider as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableLoginProvider as diesel::Insertable<
            <crate::LoginProvider as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<'query, C, crate::LoginProvider>,
    C: diesel::connection::LoadConnection,
{
    type Row = crate::LoginProvider;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableLoginProvider;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::LoginProviderAttribute,
    >;
    type UserId = i32;
    fn insert(
        self,
        user_id: Self::UserId,
        conn: &mut C,
    ) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableLoginProvider = self
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
    ) -> Result<Self::InsertableVariant, Self::Error> {
        let name = self
            .name
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::LoginProviderAttribute::Name,
                ),
            )?;
        let icon = self
            .icon
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::LoginProviderAttribute::Icon,
                ),
            )?;
        let client_id = self
            .client_id
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::LoginProviderAttribute::ClientId,
                ),
            )?;
        let redirect_uri = self
            .redirect_uri
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::LoginProviderAttribute::RedirectUri,
                ),
            )?;
        let oauth_url = self
            .oauth_url
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::LoginProviderAttribute::OauthUrl,
                ),
            )?;
        let scope = self
            .scope
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::LoginProviderAttribute::Scope,
                ),
            )?;
        Ok(Self::InsertableVariant {
            name,
            icon,
            client_id,
            redirect_uri,
            oauth_url,
            scope,
        })
    }
}

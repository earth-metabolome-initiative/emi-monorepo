impl web_common_traits::database::DispatchableInsertVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableLoginProviderBuilder
{
    type Row = crate::codegen::structs_codegen::tables::login_providers::LoginProvider;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::LoginProviderAttribute,
    >;
}
impl web_common_traits::database::InsertableVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableLoginProviderBuilder
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableLoginProvider;
}
#[cfg(feature = "backend")]
impl web_common_traits::database::BackendInsertableVariant
    for crate::codegen::structs_codegen::tables::insertables::InsertableLoginProviderBuilder
where
    Self: web_common_traits::database::DispatchableInsertableVariant<diesel::PgConnection>,
{
}
impl<
    C: diesel::connection::LoadConnection,
> web_common_traits::database::DispatchableInsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableLoginProviderBuilder
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::login_providers::LoginProvider as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableLoginProvider as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::login_providers::LoginProvider as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::login_providers::LoginProvider,
    >,
    Self: web_common_traits::database::InsertableVariant<
        C,
        InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableLoginProvider,
        Row = crate::codegen::structs_codegen::tables::login_providers::LoginProvider,
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::LoginProviderAttribute,
        >,
    >,
{
    fn insert(self, user_id: i32, conn: &mut C) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::InsertableVariant;
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableLoginProvider = self
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
for crate::codegen::structs_codegen::tables::insertables::InsertableLoginProviderBuilder
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::login_providers::LoginProvider as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableLoginProvider as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::login_providers::LoginProvider as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::login_providers::LoginProvider,
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

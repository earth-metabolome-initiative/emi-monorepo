impl web_common_traits::database::DispatchableInsertVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableEmailProviderBuilder
{
    type Row = crate::codegen::structs_codegen::tables::email_providers::EmailProvider;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::EmailProviderAttribute,
    >;
}
impl web_common_traits::database::InsertableVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableEmailProviderBuilder
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableEmailProvider;
}
#[cfg(feature = "backend")]
impl web_common_traits::database::BackendInsertableVariant
    for crate::codegen::structs_codegen::tables::insertables::InsertableEmailProviderBuilder
where
    Self: web_common_traits::database::DispatchableInsertableVariant<diesel::PgConnection>,
{
}
impl<
    C: diesel::connection::LoadConnection,
> web_common_traits::database::DispatchableInsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableEmailProviderBuilder
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::email_providers::EmailProvider as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableEmailProvider as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::email_providers::EmailProvider as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::email_providers::EmailProvider,
    >,
    Self: web_common_traits::database::InsertableVariant<
        C,
        InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableEmailProvider,
        Row = crate::codegen::structs_codegen::tables::email_providers::EmailProvider,
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::EmailProviderAttribute,
        >,
    >,
{
    fn insert(self, user_id: i32, conn: &mut C) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::InsertableVariant;
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableEmailProvider = self
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
for crate::codegen::structs_codegen::tables::insertables::InsertableEmailProviderBuilder
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::email_providers::EmailProvider as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableEmailProvider as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::email_providers::EmailProvider as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::email_providers::EmailProvider,
    >,
{
    fn try_insert(
        self,
        _user_id: i32,
        _conn: &mut C,
    ) -> Result<Self::InsertableVariant, Self::Error> {
        let email_id = self
            .email_id
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::EmailProviderAttribute::EmailId,
                ),
            )?;
        let login_provider_id = self
            .login_provider_id
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::EmailProviderAttribute::LoginProviderId,
                ),
            )?;
        Ok(Self::InsertableVariant {
            email_id,
            login_provider_id,
        })
    }
}

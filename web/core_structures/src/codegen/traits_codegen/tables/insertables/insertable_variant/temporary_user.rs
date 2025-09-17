impl web_common_traits::database::DispatchableInsertVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableTemporaryUserBuilder
{
    type Row = crate::codegen::structs_codegen::tables::temporary_user::TemporaryUser;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::TemporaryUserAttribute,
    >;
}
impl web_common_traits::database::InsertableVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableTemporaryUserBuilder
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableTemporaryUser;
}
#[cfg(feature = "backend")]
impl web_common_traits::database::BackendInsertableVariant
    for crate::codegen::structs_codegen::tables::insertables::InsertableTemporaryUserBuilder
where
    Self: web_common_traits::database::DispatchableInsertableVariant<diesel::PgConnection>,
{
}
impl<
    C: diesel::connection::LoadConnection,
> web_common_traits::database::DispatchableInsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableTemporaryUserBuilder
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::temporary_user::TemporaryUser as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableTemporaryUser as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::temporary_user::TemporaryUser as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::temporary_user::TemporaryUser,
    >,
    Self: web_common_traits::database::InsertableVariant<
        C,
        InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableTemporaryUser,
        Row = crate::codegen::structs_codegen::tables::temporary_user::TemporaryUser,
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::TemporaryUserAttribute,
        >,
    >,
{
    fn insert(self, user_id: i32, conn: &mut C) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::InsertableVariant;
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableTemporaryUser = self
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
for crate::codegen::structs_codegen::tables::insertables::InsertableTemporaryUserBuilder
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::temporary_user::TemporaryUser as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableTemporaryUser as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::temporary_user::TemporaryUser as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::temporary_user::TemporaryUser,
    >,
{
    fn try_insert(
        self,
        _user_id: i32,
        _conn: &mut C,
    ) -> Result<Self::InsertableVariant, Self::Error> {
        let email = self
            .email
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::TemporaryUserAttribute::Email,
                ),
            )?;
        let login_provider_id = self
            .login_provider_id
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::TemporaryUserAttribute::LoginProviderId,
                ),
            )?;
        Ok(Self::InsertableVariant {
            email,
            login_provider_id,
        })
    }
}

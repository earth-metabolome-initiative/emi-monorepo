impl<
    C: diesel::connection::LoadConnection,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableTemporaryUserBuilder
where
    <C as diesel::Connection>::Backend: diesel::backend::DieselReserveSpecialization,
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
    C: diesel::connection::LoadConnection,
{
    type Row = crate::codegen::structs_codegen::tables::temporary_user::TemporaryUser;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableTemporaryUser;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::TemporaryUserAttribute,
    >;
    type UserId = i32;
    fn insert(
        self,
        user_id: Self::UserId,
        conn: &mut C,
    ) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableTemporaryUser = self
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

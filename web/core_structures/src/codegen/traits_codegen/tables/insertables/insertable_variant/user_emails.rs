impl<
    C: diesel::connection::LoadConnection,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableUserEmailBuilder
where
    diesel::query_builder::InsertStatement<
        <crate::UserEmail as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableUserEmail as diesel::Insertable<
            <crate::UserEmail as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<'query, C, crate::UserEmail>,
    C: diesel::connection::LoadConnection,
{
    type Row = crate::UserEmail;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableUserEmail;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::UserEmailAttribute,
    >;
    type UserId = i32;
    fn insert(
        self,
        user_id: Self::UserId,
        conn: &mut C,
    ) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableUserEmail = self
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
                    crate::codegen::structs_codegen::tables::insertables::UserEmailAttribute::Email,
                ),
            )?;
        let created_by = self
            .created_by
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::UserEmailAttribute::CreatedBy,
                ),
            )?;
        let created_at = self
            .created_at
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::UserEmailAttribute::CreatedAt,
                ),
            )?;
        let primary_email = self
            .primary_email
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::UserEmailAttribute::PrimaryEmail,
                ),
            )?;
        Ok(Self::InsertableVariant {
            email,
            created_by,
            created_at,
            primary_email,
        })
    }
}

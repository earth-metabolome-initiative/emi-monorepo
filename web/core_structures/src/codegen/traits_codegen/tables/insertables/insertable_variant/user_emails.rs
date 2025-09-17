impl web_common_traits::database::DispatchableInsertVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableUserEmailBuilder
{
    type Row = crate::codegen::structs_codegen::tables::user_emails::UserEmail;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::UserEmailAttribute,
    >;
}
impl web_common_traits::database::InsertableVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableUserEmailBuilder
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableUserEmail;
}
#[cfg(feature = "backend")]
impl web_common_traits::database::BackendInsertableVariant
    for crate::codegen::structs_codegen::tables::insertables::InsertableUserEmailBuilder
where
    Self: web_common_traits::database::DispatchableInsertableVariant<diesel::PgConnection>,
{
}
impl<
    C: diesel::connection::LoadConnection,
> web_common_traits::database::DispatchableInsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableUserEmailBuilder
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::user_emails::UserEmail as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableUserEmail as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::user_emails::UserEmail as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::user_emails::UserEmail,
    >,
    Self: web_common_traits::database::InsertableVariant<
        C,
        InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableUserEmail,
        Row = crate::codegen::structs_codegen::tables::user_emails::UserEmail,
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::UserEmailAttribute,
        >,
    >,
{
    fn insert(self, user_id: i32, conn: &mut C) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::InsertableVariant;
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableUserEmail = self
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
for crate::codegen::structs_codegen::tables::insertables::InsertableUserEmailBuilder
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::user_emails::UserEmail as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableUserEmail as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::user_emails::UserEmail as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::user_emails::UserEmail,
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

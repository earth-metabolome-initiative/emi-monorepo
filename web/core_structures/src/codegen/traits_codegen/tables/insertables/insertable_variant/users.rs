impl web_common_traits::database::DispatchableInsertVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableUserBuilder
{
    type Row = crate::codegen::structs_codegen::tables::users::User;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::UserAttribute,
    >;
}
impl web_common_traits::database::InsertableVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableUserBuilder
{
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableUser;
}
#[cfg(feature = "backend")]
impl web_common_traits::database::BackendInsertableVariant
    for crate::codegen::structs_codegen::tables::insertables::InsertableUserBuilder
where
    Self: web_common_traits::database::DispatchableInsertableVariant<diesel::PgConnection>,
{
}
impl<
    C: diesel::connection::LoadConnection,
> web_common_traits::database::DispatchableInsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableUserBuilder
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::users::User as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableUser as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::users::User as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::users::User,
    >,
    Self: web_common_traits::database::InsertableVariant<
        C,
        InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableUser,
        Row = crate::codegen::structs_codegen::tables::users::User,
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::UserAttribute,
        >,
    >,
{
    fn insert(self, user_id: i32, conn: &mut C) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::InsertableVariant;
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableUser = self
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
for crate::codegen::structs_codegen::tables::insertables::InsertableUserBuilder
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::users::User as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableUser as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::users::User as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::users::User,
    >,
{
    fn try_insert(
        self,
        _user_id: i32,
        _conn: &mut C,
    ) -> Result<Self::InsertableVariant, Self::Error> {
        let first_name = self
            .first_name
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::UserAttribute::FirstName,
                ),
            )?;
        let last_name = self
            .last_name
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::UserAttribute::LastName,
                ),
            )?;
        let created_at = self
            .created_at
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::UserAttribute::CreatedAt,
                ),
            )?;
        let updated_at = self
            .updated_at
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::UserAttribute::UpdatedAt,
                ),
            )?;
        Ok(Self::InsertableVariant {
            first_name,
            last_name,
            created_at,
            updated_at,
        })
    }
}

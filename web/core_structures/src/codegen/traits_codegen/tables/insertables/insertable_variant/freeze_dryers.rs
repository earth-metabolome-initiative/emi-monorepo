impl<
    C: diesel::connection::LoadConnection,
    PhysicalAsset,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryerBuilder<
    PhysicalAsset,
>
where
    diesel::query_builder::InsertStatement<
        <crate::FreezeDryer as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryer as diesel::Insertable<
            <crate::FreezeDryer as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<'query, C, crate::FreezeDryer>,
    C: diesel::connection::LoadConnection,
    PhysicalAsset: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = ::rosetta_uuid::Uuid,
    >,
    crate::PhysicalAsset: web_common_traits::database::Read<C>,
    crate::PhysicalAsset: web_common_traits::database::Updatable<C, UserId = i32>,
    Self: web_common_traits::database::MostConcreteTable,
{
    type Row = crate::FreezeDryer;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryer;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::FreezeDryerAttribute,
    >;
    type UserId = i32;
    fn insert(
        mut self,
        user_id: Self::UserId,
        conn: &mut C,
    ) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::Updatable;
        use web_common_traits::database::MostConcreteTable;
        self.set_most_concrete_table("freeze_dryers");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryer = self
            .try_insert(user_id, conn)?;
        if !insertable_struct.id(conn)?.can_update(user_id, conn)? {
            return Err(
                generic_backend_request_errors::GenericBackendRequestError::Unauthorized
                    .into(),
            );
        }
        Ok(
            diesel::insert_into(Self::Row::table())
                .values(insertable_struct)
                .get_result(conn)?,
        )
    }
    fn try_insert(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::InsertableVariant, Self::Error> {
        let model = self
            .model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::FreezeDryerAttribute::Model,
                ),
            )?;
        let id = self
            .id
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::FreezeDryerAttribute::Extension(
                    crate::codegen::structs_codegen::tables::insertables::FreezeDryerExtensionAttribute::PhysicalAsset(
                        crate::codegen::structs_codegen::tables::insertables::PhysicalAssetAttribute::Id,
                    ),
                ))
            })?;
        Ok(Self::InsertableVariant {
            id,
            model,
        })
    }
}

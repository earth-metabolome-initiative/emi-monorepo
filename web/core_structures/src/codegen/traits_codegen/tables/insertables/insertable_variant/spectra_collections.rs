impl<
    C: diesel::connection::LoadConnection,
    DigitalAsset,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableSpectraCollectionBuilder<
    DigitalAsset,
>
where
    <C as diesel::Connection>::Backend: diesel::backend::DieselReserveSpecialization,
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableSpectraCollection as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection,
    >,
    C: diesel::connection::LoadConnection,
    DigitalAsset: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = ::rosetta_uuid::Uuid,
    >,
    crate::codegen::structs_codegen::tables::digital_assets::DigitalAsset: diesel::Identifiable
        + web_common_traits::database::Updatable<C, UserId = i32>,
    <crate::codegen::structs_codegen::tables::digital_assets::DigitalAsset as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::digital_assets::DigitalAsset as diesel::Identifiable>::Id,
    >,
    <<crate::codegen::structs_codegen::tables::digital_assets::DigitalAsset as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::digital_assets::DigitalAsset as diesel::Identifiable>::Id,
    >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
    <<<crate::codegen::structs_codegen::tables::digital_assets::DigitalAsset as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::digital_assets::DigitalAsset as diesel::Identifiable>::Id,
    >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
        'a,
        C,
        crate::codegen::structs_codegen::tables::digital_assets::DigitalAsset,
    >,
    Self: web_common_traits::database::MostConcreteTable,
{
    type Row = crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableSpectraCollection;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::SpectraCollectionAttribute,
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
        self.set_most_concrete_table("spectra_collections");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableSpectraCollection = self
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
        let id = self
            .id
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::SpectraCollectionAttribute::Extension(
                    crate::codegen::structs_codegen::tables::insertables::SpectraCollectionExtensionAttribute::DigitalAsset(
                        crate::codegen::structs_codegen::tables::insertables::DigitalAssetAttribute::Id,
                    ),
                ))
            })?;
        Ok(Self::InsertableVariant { id })
    }
}

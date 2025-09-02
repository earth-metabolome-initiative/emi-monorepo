impl<
    C: diesel::connection::LoadConnection,
    DigitalAsset,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableSpectrumBuilder<
    DigitalAsset,
>
where
    <C as diesel::Connection>::Backend: diesel::backend::DieselReserveSpecialization,
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::spectra::Spectrum as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableSpectrum as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::spectra::Spectrum as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::spectra::Spectrum,
    >,
    C: diesel::connection::LoadConnection,
    DigitalAsset: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = ::rosetta_uuid::Uuid,
    >,
    crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection: diesel::Identifiable
        + web_common_traits::database::Updatable<C, UserId = i32>,
    <crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection as diesel::Identifiable>::Id,
    >,
    <<crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection as diesel::Identifiable>::Id,
    >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
    <<<crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection as diesel::Identifiable>::Id,
    >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
        'a,
        C,
        crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection,
    >,
    Self: crate::codegen::structs_codegen::tables::insertables::AssetBuildable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableSpectrumAttributes,
    >,
{
    type Row = crate::codegen::structs_codegen::tables::spectra::Spectrum;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableSpectrum;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::InsertableSpectrumAttributes,
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
        self = <Self as crate::codegen::structs_codegen::tables::insertables::AssetBuildable>::most_concrete_table(
            self,
            "spectra",
        )?;
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableSpectrum = self
            .try_insert(user_id, conn)?;
        if !insertable_struct.spectra_collection(conn)?.can_update(user_id, conn)? {
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
        let spectra_collection_id = self
            .spectra_collection_id
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableSpectrumAttributes::SpectraCollectionId,
                ),
            )?;
        let id = self
            .id
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::InsertableSpectrumAttributes::Extension(
                    crate::codegen::structs_codegen::tables::insertables::InsertableSpectrumExtensionAttributes::DigitalAsset(
                        crate::codegen::structs_codegen::tables::insertables::InsertableDigitalAssetAttributes::Id,
                    ),
                ))
            })?;
        Ok(Self::InsertableVariant {
            id,
            spectra_collection_id,
        })
    }
}

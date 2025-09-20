impl<DigitalAsset> web_common_traits::database::DispatchableInsertVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableSpectraCollectionBuilder<
        DigitalAsset,
    >
{
    type Row = crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::SpectraCollectionAttribute,
    >;
}
impl<DigitalAsset> web_common_traits::database::InsertableVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableSpectraCollectionBuilder<
        DigitalAsset,
    >
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableSpectraCollection;
}
#[cfg(feature = "backend")]
impl<DigitalAsset> web_common_traits::database::BackendInsertableVariant
    for crate::codegen::structs_codegen::tables::insertables::InsertableSpectraCollectionBuilder<
        DigitalAsset,
    >
where
    Self: web_common_traits::database::DispatchableInsertableVariant<diesel::PgConnection>,
{
}
impl<
    C: diesel::connection::LoadConnection,
    DigitalAsset,
> web_common_traits::database::DispatchableInsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableSpectraCollectionBuilder<
    DigitalAsset,
>
where
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
    Self: web_common_traits::database::InsertableVariant<
        C,
        InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableSpectraCollection,
        Row = crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection,
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::SpectraCollectionAttribute,
        >,
    >,
    Self: web_common_traits::database::MostConcreteTable,
{
    fn insert(mut self, user_id: i32, conn: &mut C) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::InsertableVariant;
        use web_common_traits::database::MostConcreteTable;
        self.set_most_concrete_table("spectra_collections");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableSpectraCollection = self
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
    DigitalAsset,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableSpectraCollectionBuilder<
    DigitalAsset,
>
where
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
    Self::Error: web_common_traits::database::FromExtension<
        <DigitalAsset as web_common_traits::database::TryInsertGeneric<C>>::Error,
    >,
    DigitalAsset: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = ::rosetta_uuid::Uuid,
    >,
{
    fn try_insert(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::InsertableVariant, Self::Error> {
        use web_common_traits::database::FromExtension;
        let id = self
            .id
            .mint_primary_key(user_id, conn)
            .map_err(Self::Error::from_extension)?;
        Ok(Self::InsertableVariant { id })
    }
}

impl<PhysicalAssetModel> web_common_traits::database::DispatchableInsertVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryerModelBuilder<
        PhysicalAssetModel,
    >
{
    type Row = crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::FreezeDryerModelAttribute,
    >;
}
impl<PhysicalAssetModel> web_common_traits::database::InsertableVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryerModelBuilder<
        PhysicalAssetModel,
    >
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryerModel;
}
#[cfg(feature = "backend")]
impl<PhysicalAssetModel> web_common_traits::database::BackendInsertableVariant
    for crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryerModelBuilder<
        PhysicalAssetModel,
    >
where
    Self: web_common_traits::database::DispatchableInsertableVariant<diesel::PgConnection>,
{
}
impl<
    C: diesel::connection::LoadConnection,
    PhysicalAssetModel,
> web_common_traits::database::DispatchableInsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryerModelBuilder<
    PhysicalAssetModel,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryerModel as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel,
    >,
    Self: web_common_traits::database::InsertableVariant<
        C,
        InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryerModel,
        Row = crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel,
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::FreezeDryerModelAttribute,
        >,
    >,
    Self: web_common_traits::database::MostConcreteTable,
{
    fn insert(mut self, user_id: i32, conn: &mut C) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::InsertableVariant;
        use web_common_traits::database::MostConcreteTable;
        self.set_most_concrete_table("freeze_dryer_models");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryerModel = self
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
    PhysicalAssetModel,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryerModelBuilder<
    PhysicalAssetModel,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryerModel as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel,
    >,
    Self::Error: web_common_traits::database::FromExtension<
        <PhysicalAssetModel as web_common_traits::database::TryInsertGeneric<C>>::Error,
    >,
    PhysicalAssetModel: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
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

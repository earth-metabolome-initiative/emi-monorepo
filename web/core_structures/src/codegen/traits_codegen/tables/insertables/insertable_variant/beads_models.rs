impl<
    C: diesel::connection::LoadConnection,
    PhysicalAssetModel,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableBeadsModelBuilder<
    PhysicalAssetModel,
>
where
    <C as diesel::Connection>::Backend: diesel::backend::DieselReserveSpecialization,
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::beads_models::BeadsModel as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableBeadsModel as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::beads_models::BeadsModel as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::beads_models::BeadsModel,
    >,
    C: diesel::connection::LoadConnection,
    PhysicalAssetModel: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
{
    type Row = crate::codegen::structs_codegen::tables::beads_models::BeadsModel;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableBeadsModel;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::InsertableBeadsModelAttributes,
    >;
    type UserId = i32;
    fn insert(
        self,
        user_id: Self::UserId,
        conn: &mut C,
    ) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableBeadsModel = self
            .try_insert(user_id, conn)?;
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
        let diameter_millimeters = self
            .diameter_millimeters
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableBeadsModelAttributes::DiameterMillimeters,
                ),
            )?;
        let id = self
            .id
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::InsertableBeadsModelAttributes::Extension(
                    crate::codegen::structs_codegen::tables::insertables::InsertableBeadsModelExtensionAttributes::PhysicalAssetModel(
                        crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetModelAttributes::Id,
                    ),
                ))
            })?;
        Ok(Self::InsertableVariant {
            id,
            diameter_millimeters,
        })
    }
}

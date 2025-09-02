impl<
    C: diesel::connection::LoadConnection,
    AssetModel,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableReagentModelBuilder<
    AssetModel,
>
where
    <C as diesel::Connection>::Backend: diesel::backend::DieselReserveSpecialization,
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::reagent_models::ReagentModel as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableReagentModel as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::reagent_models::ReagentModel as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::reagent_models::ReagentModel,
    >,
    AssetModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
    C: diesel::connection::LoadConnection,
    Self: crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableReagentModelAttributes,
    >,
{
    type Row = crate::codegen::structs_codegen::tables::reagent_models::ReagentModel;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableReagentModel;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::InsertableReagentModelAttributes,
    >;
    type UserId = i32;
    fn insert(
        mut self,
        user_id: Self::UserId,
        conn: &mut C,
    ) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        self = <Self as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::most_concrete_table(
            self,
            "reagent_models",
        )?;
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableReagentModel = self
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
        let purity = self
            .purity
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableReagentModelAttributes::Purity,
                ),
            )?;
        let cas_code = self
            .cas_code
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableReagentModelAttributes::CasCode,
                ),
            )?;
        let molecular_formula = self
            .molecular_formula
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableReagentModelAttributes::MolecularFormula,
                ),
            )?;
        let id = self
            .id
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::InsertableReagentModelAttributes::Extension(
                    crate::codegen::structs_codegen::tables::insertables::InsertableReagentModelExtensionAttributes::AssetModel(
                        crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelAttributes::Id,
                    ),
                ))
            })?;
        Ok(Self::InsertableVariant {
            id,
            purity,
            cas_code,
            molecular_formula,
        })
    }
}

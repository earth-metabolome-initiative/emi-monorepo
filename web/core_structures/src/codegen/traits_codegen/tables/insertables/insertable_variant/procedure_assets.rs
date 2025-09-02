impl<
    C: diesel::connection::LoadConnection,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder
where
    <C as diesel::Connection>::Backend: diesel::backend::DieselReserveSpecialization,
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAsset as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
    >,
    C: diesel::connection::LoadConnection,
{
    type Row = crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAsset;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetAttributes,
    >;
    type UserId = i32;
    fn insert(
        self,
        user_id: Self::UserId,
        conn: &mut C,
    ) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAsset = self
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
        let procedure = self
            .procedure
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetAttributes::Procedure,
                ),
            )?;
        let procedure_template = self
            .procedure_template
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetAttributes::ProcedureTemplate,
                ),
            )?;
        let asset_model = self
            .asset_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetAttributes::AssetModel,
                ),
            )?;
        let procedure_template_asset_model = self
            .procedure_template_asset_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetAttributes::ProcedureTemplateAssetModel,
                ),
            )?;
        let ancestor_model = self
            .ancestor_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetAttributes::AncestorModel,
                ),
            )?;
        let created_by = self
            .created_by
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetAttributes::CreatedBy,
                ),
            )?;
        let created_at = self
            .created_at
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetAttributes::CreatedAt,
                ),
            )?;
        Ok(Self::InsertableVariant {
            procedure,
            procedure_template,
            asset_model,
            asset: self.asset,
            procedure_template_asset_model,
            ancestor_model,
            created_by,
            created_at,
        })
    }
}

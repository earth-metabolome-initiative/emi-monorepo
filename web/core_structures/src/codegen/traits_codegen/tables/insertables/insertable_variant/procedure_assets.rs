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
    Self: crate::codegen::structs_codegen::tables::insertables::ProcedureAssetBuildable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetAttributes,
    >,
    crate::codegen::structs_codegen::tables::assets::Asset: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::procedures::Procedure: web_common_traits::database::Read<
        C,
    >,
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
        mut self,
        _user_id: i32,
        conn: &mut C,
    ) -> Result<Self::InsertableVariant, Self::Error> {
        use web_common_traits::database::Read;
        if let Some(procedure) = self.procedure {
            if let Some(procedures) = crate::codegen::structs_codegen::tables::procedures::Procedure::read(
                procedure,
                conn,
            )? {
                self = <Self as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetBuildable>::procedure_template(
                    self,
                    procedures.procedure_template,
                )?;
            }
        }
        if let Some(asset) = self.asset {
            if let Some(assets) = crate::codegen::structs_codegen::tables::assets::Asset::read(
                asset,
                conn,
            )? {
                self = <Self as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetBuildable>::asset_model(
                    self,
                    assets.model,
                )?;
            }
        }
        if let Some(procedure_template_asset_model) = self.procedure_template_asset_model
        {
            if let Some(procedure_template_asset_models) = crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::read(
                procedure_template_asset_model,
                conn,
            )? {
                self = <Self as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetBuildable>::ancestor_model(
                    self,
                    procedure_template_asset_models.asset_model,
                )?;
                self = <Self as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetBuildable>::procedure_template(
                    self,
                    procedure_template_asset_models.procedure_template,
                )?;
            }
        }
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

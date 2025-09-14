impl web_common_traits::database::InsertableVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder
{
    type Row = crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset;
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAsset;
    type UserId = i32;
}
impl<
    C: diesel::connection::LoadConnection,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder
where
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
    Self: crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute,
    >,
    crate::codegen::structs_codegen::tables::asset_models::AssetModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::asset_models::AssetModel: web_common_traits::database::Updatable<
        C,
        UserId = i32,
    >,
    crate::codegen::structs_codegen::tables::assets::Asset: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::assets::Asset: web_common_traits::database::Updatable<
        C,
        UserId = i32,
    >,
    crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate: web_common_traits::database::Updatable<
        C,
        UserId = i32,
    >,
    crate::codegen::structs_codegen::tables::procedures::Procedure: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::procedures::Procedure: web_common_traits::database::Updatable<
        C,
        UserId = i32,
    >,
{
    fn insert(
        self,
        user_id: Self::UserId,
        conn: &mut C,
    ) -> Result<
        Self::Row,
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute,
        >,
    > {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::Updatable;
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAsset = self
            .try_insert(user_id, conn)?;
        if !insertable_struct.ancestor_model(conn)?.can_update(user_id, conn)? {
            return Err(
                generic_backend_request_errors::GenericBackendRequestError::Unauthorized
                    .into(),
            );
        }
        if let Some(parent) = insertable_struct.asset(conn)? {
            if !parent.can_update(user_id, conn)? {
                return Err(
                    generic_backend_request_errors::GenericBackendRequestError::Unauthorized
                        .into(),
                );
            }
        }
        if !insertable_struct.asset_model(conn)?.can_update(user_id, conn)? {
            return Err(
                generic_backend_request_errors::GenericBackendRequestError::Unauthorized
                    .into(),
            );
        }
        if !insertable_struct.procedure(conn)?.can_update(user_id, conn)? {
            return Err(
                generic_backend_request_errors::GenericBackendRequestError::Unauthorized
                    .into(),
            );
        }
        if !insertable_struct.procedure_template(conn)?.can_update(user_id, conn)? {
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
        mut self,
        _user_id: i32,
        conn: &mut C,
    ) -> Result<
        Self::InsertableVariant,
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute,
        >,
    > {
        use web_common_traits::database::Read;
        if let Some(procedure) = self.procedure {
            let procedures = crate::codegen::structs_codegen::tables::procedures::Procedure::read(
                procedure,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template(
                self,
                procedures.procedure_template,
            )?;
        }
        if let Some(asset) = self.asset {
            let assets = crate::codegen::structs_codegen::tables::assets::Asset::read(
                asset,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset_model(
                self,
                assets.model,
            )?;
        }
        if let Some(procedure_template_asset_model) = self.procedure_template_asset_model
        {
            let procedure_template_asset_models = crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::read(
                procedure_template_asset_model,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::ancestor_model(
                self,
                procedure_template_asset_models.asset_model,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template(
                self,
                procedure_template_asset_models.procedure_template,
            )?;
        }
        let id = self
            .id
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::Id,
                ),
            )?;
        let procedure = self
            .procedure
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::Procedure,
                ),
            )?;
        let procedure_template = self
            .procedure_template
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::ProcedureTemplate,
                ),
            )?;
        let asset_model = self
            .asset_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::AssetModel,
                ),
            )?;
        let procedure_template_asset_model = self
            .procedure_template_asset_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::ProcedureTemplateAssetModel,
                ),
            )?;
        let ancestor_model = self
            .ancestor_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::AncestorModel,
                ),
            )?;
        let created_by = self
            .created_by
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::CreatedBy,
                ),
            )?;
        let created_at = self
            .created_at
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::CreatedAt,
                ),
            )?;
        Ok(Self::InsertableVariant {
            id,
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

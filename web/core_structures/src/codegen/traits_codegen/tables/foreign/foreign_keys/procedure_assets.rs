#[derive(Debug, Clone, PartialEq, Default, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProcedureAssetForeignKeys {
    pub procedure: Option<
        crate::codegen::structs_codegen::tables::procedures::Procedure,
    >,
    pub procedure_template: Option<
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
    >,
    pub asset_model: Option<
        crate::codegen::structs_codegen::tables::asset_models::AssetModel,
    >,
    pub asset: Option<crate::codegen::structs_codegen::tables::assets::Asset>,
    pub procedure_template_asset_model: Option<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
    >,
    pub ancestor_model: Option<
        crate::codegen::structs_codegen::tables::asset_models::AssetModel,
    >,
    pub created_by: Option<crate::codegen::structs_codegen::tables::users::User>,
    pub procedure_assets_asset_model_ancestor_model_fkey: Option<
        crate::codegen::structs_codegen::tables::asset_model_ancestors::AssetModelAncestor,
    >,
}
impl web_common_traits::prelude::HasForeignKeys
    for crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset
{
    type ForeignKeys = ProcedureAssetForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Procedure(self.procedure),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplate(
                self.procedure_template,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::AssetModel(
                self.asset_model,
            ),
        ));
        if let Some(asset) = self.asset {
            connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Asset(asset),
            ));
        }
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplateAssetModel(
                        self.procedure_template_asset_model,
                    ),
                ),
            );
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::AssetModel(
                self.ancestor_model,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::User(self.created_by),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::AssetModelAncestor((
                self.asset_model,
                self.ancestor_model,
            )),
        ));
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.procedure.is_some()
            && foreign_keys.procedure_template.is_some()
            && foreign_keys.asset_model.is_some()
            && (foreign_keys.asset.is_some() || self.asset.is_some())
            && foreign_keys.procedure_template_asset_model.is_some()
            && foreign_keys.ancestor_model.is_some()
            && foreign_keys.created_by.is_some()
            && foreign_keys.procedure_assets_asset_model_ancestor_model_fkey.is_some()
    }
    fn update(
        &self,
        foreign_keys: &mut Self::ForeignKeys,
        row: Self::Row,
        crud: web_common_traits::crud::CRUD,
    ) -> bool {
        let mut updated = false;
        match (row, crud) {
            (
                crate::codegen::tables::row::Row::AssetModelAncestor(asset_model_ancestors),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.asset_model == asset_model_ancestors.descendant_model
                    && self.ancestor_model == asset_model_ancestors.ancestor_model
                {
                    foreign_keys.procedure_assets_asset_model_ancestor_model_fkey =
                        Some(asset_model_ancestors);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::AssetModelAncestor(asset_model_ancestors),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.asset_model == asset_model_ancestors.descendant_model
                    && self.ancestor_model == asset_model_ancestors.ancestor_model
                {
                    foreign_keys.procedure_assets_asset_model_ancestor_model_fkey = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::AssetModel(asset_models),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.asset_model == asset_models.id {
                    foreign_keys.asset_model = Some(asset_models.clone());
                    updated = true;
                }
                if self.ancestor_model == asset_models.id {
                    foreign_keys.ancestor_model = Some(asset_models.clone());
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::AssetModel(asset_models),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.asset_model == asset_models.id {
                    foreign_keys.asset_model = None;
                    updated = true;
                }
                if self.ancestor_model == asset_models.id {
                    foreign_keys.ancestor_model = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Asset(assets),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.asset.is_some_and(|asset| asset == assets.id) {
                    foreign_keys.asset = Some(assets);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Asset(assets),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.asset.is_some_and(|asset| asset == assets.id) {
                    foreign_keys.asset = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureTemplateAssetModel(
                    procedure_template_asset_models,
                ),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.procedure_template_asset_model == procedure_template_asset_models.id {
                    foreign_keys.procedure_template_asset_model =
                        Some(procedure_template_asset_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureTemplateAssetModel(
                    procedure_template_asset_models,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.procedure_template_asset_model == procedure_template_asset_models.id {
                    foreign_keys.procedure_template_asset_model = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureTemplate(procedure_templates),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.procedure_template == procedure_templates.procedure_template {
                    foreign_keys.procedure_template = Some(procedure_templates);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureTemplate(procedure_templates),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.procedure_template == procedure_templates.procedure_template {
                    foreign_keys.procedure_template = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Procedure(procedures),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.procedure == procedures.procedure {
                    foreign_keys.procedure = Some(procedures);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Procedure(procedures),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.procedure == procedures.procedure {
                    foreign_keys.procedure = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::User(users),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.created_by == users.id {
                    foreign_keys.created_by = Some(users);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::User(users),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.created_by == users.id {
                    foreign_keys.created_by = None;
                    updated = true;
                }
            }
            (_, crud) => {
                unreachable!("Unexpected row type with operation {crud}");
            }
        }
        updated
    }
}
impl web_common_traits::prelude::ForeignKeys for ProcedureAssetForeignKeys {}

#[derive(Debug, Clone, PartialEq, Default, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PhotographProcedureForeignKeys {
    pub photographed_asset: Option<crate::PhysicalAsset>,
    pub photographed_with: Option<crate::Camera>,
    pub procedure: Option<crate::Procedure>,
    pub procedure_photographed_asset: Option<crate::ProcedureAsset>,
    pub procedure_photographed_with: Option<crate::ProcedureAsset>,
    pub procedure_template: Option<crate::PhotographProcedureTemplate>,
    pub procedure_template_photographed_asset_model: Option<crate::ProcedureTemplateAssetModel>,
    pub procedure_template_photographed_with_model: Option<crate::ProcedureTemplateAssetModel>,
}
impl web_common_traits::prelude::HasForeignKeys for crate::PhotographProcedure {
    type ForeignKeys = PhotographProcedureForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::PhysicalAsset(
                self.photographed_asset,
            ),
        ));
        if let Some(photographed_with) = self.photographed_with {
            connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Camera(
                    photographed_with,
                ),
            ));
        }
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Procedure(self.procedure),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureAsset(
                self.procedure_photographed_asset,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureAsset(
                self.procedure_photographed_with,
            ),
        ));
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::PhotographProcedureTemplate(
                        self.procedure_template,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplateAssetModel(
                        self.procedure_template_photographed_asset_model,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplateAssetModel(
                        self.procedure_template_photographed_with_model,
                    ),
                ),
            );
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.photographed_asset.is_some()
            && (foreign_keys.photographed_with.is_some() || self.photographed_with.is_some())
            && foreign_keys.procedure.is_some()
            && foreign_keys.procedure_photographed_asset.is_some()
            && foreign_keys.procedure_photographed_with.is_some()
            && foreign_keys.procedure_template.is_some()
            && foreign_keys.procedure_template_photographed_asset_model.is_some()
            && foreign_keys.procedure_template_photographed_with_model.is_some()
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
                crate::codegen::tables::row::Row::Camera(cameras),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self
                    .photographed_with
                    .is_some_and(|photographed_with| photographed_with == cameras.id)
                {
                    foreign_keys.photographed_with = Some(cameras);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Camera(cameras),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self
                    .photographed_with
                    .is_some_and(|photographed_with| photographed_with == cameras.id)
                {
                    foreign_keys.photographed_with = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::PhotographProcedureTemplate(
                    photograph_procedure_templates,
                ),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.procedure_template == photograph_procedure_templates.procedure_template {
                    foreign_keys.procedure_template = Some(photograph_procedure_templates);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::PhotographProcedureTemplate(
                    photograph_procedure_templates,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.procedure_template == photograph_procedure_templates.procedure_template {
                    foreign_keys.procedure_template = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::PhysicalAsset(physical_assets),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.photographed_asset == physical_assets.id {
                    foreign_keys.photographed_asset = Some(physical_assets);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::PhysicalAsset(physical_assets),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.photographed_asset == physical_assets.id {
                    foreign_keys.photographed_asset = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureAsset(procedure_assets),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.procedure_photographed_asset == procedure_assets.id {
                    foreign_keys.procedure_photographed_asset = Some(procedure_assets);
                    updated = true;
                }
                if self.procedure_photographed_with == procedure_assets.id {
                    foreign_keys.procedure_photographed_with = Some(procedure_assets);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureAsset(procedure_assets),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.procedure_photographed_asset == procedure_assets.id {
                    foreign_keys.procedure_photographed_asset = None;
                    updated = true;
                }
                if self.procedure_photographed_with == procedure_assets.id {
                    foreign_keys.procedure_photographed_with = None;
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
                if self.procedure_template_photographed_asset_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_photographed_asset_model =
                        Some(procedure_template_asset_models.clone());
                    updated = true;
                }
                if self.procedure_template_photographed_with_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_photographed_with_model =
                        Some(procedure_template_asset_models.clone());
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureTemplateAssetModel(
                    procedure_template_asset_models,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.procedure_template_photographed_asset_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_photographed_asset_model = None;
                    updated = true;
                }
                if self.procedure_template_photographed_with_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_photographed_with_model = None;
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
            (_, crud) => {
                unreachable!("Unexpected row type with operation {crud}");
            }
        }
        updated
    }
}
impl web_common_traits::prelude::ForeignKeys for PhotographProcedureForeignKeys {}

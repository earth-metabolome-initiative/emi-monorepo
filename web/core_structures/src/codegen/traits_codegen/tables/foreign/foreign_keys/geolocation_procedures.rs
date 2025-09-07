#[derive(Debug, Clone, PartialEq, Default, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GeolocationProcedureForeignKeys {
    pub geolocated_asset: Option<crate::PhysicalAsset>,
    pub geolocated_with: Option<crate::PositioningDevice>,
    pub procedure: Option<crate::Procedure>,
    pub procedure_geolocated_asset: Option<crate::ProcedureAsset>,
    pub procedure_geolocated_with: Option<crate::ProcedureAsset>,
    pub procedure_template: Option<crate::GeolocationProcedureTemplate>,
    pub procedure_template_geolocated_asset_model: Option<crate::ProcedureTemplateAssetModel>,
    pub procedure_template_geolocated_with_model: Option<crate::ProcedureTemplateAssetModel>,
}
impl web_common_traits::prelude::HasForeignKeys for crate::GeolocationProcedure {
    type ForeignKeys = GeolocationProcedureForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::PhysicalAsset(
                self.geolocated_asset,
            ),
        ));
        if let Some(geolocated_with) = self.geolocated_with {
            connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::PositioningDevice(
                    geolocated_with,
                ),
            ));
        }
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Procedure(self.procedure),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureAsset(
                self.procedure_geolocated_asset,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureAsset(
                self.procedure_geolocated_with,
            ),
        ));
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::GeolocationProcedureTemplate(
                        self.procedure_template,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplateAssetModel(
                        self.procedure_template_geolocated_asset_model,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplateAssetModel(
                        self.procedure_template_geolocated_with_model,
                    ),
                ),
            );
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.geolocated_asset.is_some()
            && (foreign_keys.geolocated_with.is_some() || self.geolocated_with.is_some())
            && foreign_keys.procedure.is_some()
            && foreign_keys.procedure_geolocated_asset.is_some()
            && foreign_keys.procedure_geolocated_with.is_some()
            && foreign_keys.procedure_template.is_some()
            && foreign_keys.procedure_template_geolocated_asset_model.is_some()
            && foreign_keys.procedure_template_geolocated_with_model.is_some()
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
                crate::codegen::tables::row::Row::GeolocationProcedureTemplate(
                    geolocation_procedure_templates,
                ),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.procedure_template == geolocation_procedure_templates.procedure_template {
                    foreign_keys.procedure_template = Some(geolocation_procedure_templates);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::GeolocationProcedureTemplate(
                    geolocation_procedure_templates,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.procedure_template == geolocation_procedure_templates.procedure_template {
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
                if self.geolocated_asset == physical_assets.id {
                    foreign_keys.geolocated_asset = Some(physical_assets);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::PhysicalAsset(physical_assets),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.geolocated_asset == physical_assets.id {
                    foreign_keys.geolocated_asset = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::PositioningDevice(positioning_devices),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self
                    .geolocated_with
                    .is_some_and(|geolocated_with| geolocated_with == positioning_devices.id)
                {
                    foreign_keys.geolocated_with = Some(positioning_devices);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::PositioningDevice(positioning_devices),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self
                    .geolocated_with
                    .is_some_and(|geolocated_with| geolocated_with == positioning_devices.id)
                {
                    foreign_keys.geolocated_with = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureAsset(procedure_assets),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.procedure_geolocated_asset == procedure_assets.id {
                    foreign_keys.procedure_geolocated_asset = Some(procedure_assets);
                    updated = true;
                }
                if self.procedure_geolocated_with == procedure_assets.id {
                    foreign_keys.procedure_geolocated_with = Some(procedure_assets);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureAsset(procedure_assets),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.procedure_geolocated_asset == procedure_assets.id {
                    foreign_keys.procedure_geolocated_asset = None;
                    updated = true;
                }
                if self.procedure_geolocated_with == procedure_assets.id {
                    foreign_keys.procedure_geolocated_with = None;
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
                if self.procedure_template_geolocated_asset_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_geolocated_asset_model =
                        Some(procedure_template_asset_models.clone());
                    updated = true;
                }
                if self.procedure_template_geolocated_with_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_geolocated_with_model =
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
                if self.procedure_template_geolocated_asset_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_geolocated_asset_model = None;
                    updated = true;
                }
                if self.procedure_template_geolocated_with_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_geolocated_with_model = None;
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
impl web_common_traits::prelude::ForeignKeys for GeolocationProcedureForeignKeys {}

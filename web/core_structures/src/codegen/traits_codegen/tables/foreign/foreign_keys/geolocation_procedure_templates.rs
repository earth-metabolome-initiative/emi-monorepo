#[derive(Debug, Clone, PartialEq, Default, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GeolocationProcedureTemplateForeignKeys {
    pub procedure_template: Option<
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
    >,
    pub geolocated_with_model: Option<
        crate::codegen::structs_codegen::tables::positioning_device_models::PositioningDeviceModel,
    >,
    pub procedure_template_geolocated_with_model: Option<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
    >,
    pub geolocated_asset_model: Option<
        crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel,
    >,
    pub foreign_procedure_template: Option<
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
    >,
    pub procedure_template_geolocated_asset_model: Option<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
    >,
}
impl web_common_traits::prelude::HasForeignKeys
for crate::codegen::structs_codegen::tables::geolocation_procedure_templates::GeolocationProcedureTemplate {
    type ForeignKeys = GeolocationProcedureTemplateForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplate(
                        self.procedure_template,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::PositioningDeviceModel(
                        self.geolocated_with_model,
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
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::PhysicalAssetModel(
                        self.geolocated_asset_model,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplate(
                        self.foreign_procedure_template,
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
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.procedure_template.is_some()
            && foreign_keys.geolocated_with_model.is_some()
            && foreign_keys.procedure_template_geolocated_with_model.is_some()
            && foreign_keys.geolocated_asset_model.is_some()
            && foreign_keys.foreign_procedure_template.is_some()
            && foreign_keys.procedure_template_geolocated_asset_model.is_some()
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
                crate::codegen::tables::row::Row::PhysicalAssetModel(
                    physical_asset_models,
                ),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.geolocated_asset_model == physical_asset_models.id {
                    foreign_keys.geolocated_asset_model = Some(physical_asset_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::PhysicalAssetModel(
                    physical_asset_models,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.geolocated_asset_model == physical_asset_models.id {
                    foreign_keys.geolocated_asset_model = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::PositioningDeviceModel(
                    positioning_device_models,
                ),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.geolocated_with_model == positioning_device_models.id {
                    foreign_keys.geolocated_with_model = Some(positioning_device_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::PositioningDeviceModel(
                    positioning_device_models,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.geolocated_with_model == positioning_device_models.id {
                    foreign_keys.geolocated_with_model = None;
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
                if self.procedure_template_geolocated_with_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_geolocated_with_model = Some(
                        procedure_template_asset_models.clone(),
                    );
                    updated = true;
                }
                if self.procedure_template_geolocated_asset_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_geolocated_asset_model = Some(
                        procedure_template_asset_models.clone(),
                    );
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureTemplateAssetModel(
                    procedure_template_asset_models,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.procedure_template_geolocated_with_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_geolocated_with_model = None;
                    updated = true;
                }
                if self.procedure_template_geolocated_asset_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_geolocated_asset_model = None;
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
                    foreign_keys.procedure_template = Some(procedure_templates.clone());
                    updated = true;
                }
                if self.foreign_procedure_template
                    == procedure_templates.procedure_template
                {
                    foreign_keys.foreign_procedure_template = Some(
                        procedure_templates.clone(),
                    );
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
                if self.foreign_procedure_template
                    == procedure_templates.procedure_template
                {
                    foreign_keys.foreign_procedure_template = None;
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
impl web_common_traits::prelude::ForeignKeys for GeolocationProcedureTemplateForeignKeys {}

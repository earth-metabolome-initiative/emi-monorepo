#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PouringProcedureTemplateForeignKeys {
    pub procedure_template: Option<
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
    >,
    pub measured_with_model: Option<
        crate::codegen::structs_codegen::tables::volume_measuring_device_models::VolumeMeasuringDeviceModel,
    >,
    pub procedure_template_measured_with_model: Option<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
    >,
    pub poured_from_model: Option<
        crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel,
    >,
    pub foreign_procedure_template: Option<
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
    >,
    pub procedure_template_poured_from_model: Option<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
    >,
    pub poured_into_model: Option<
        crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel,
    >,
    pub procedure_template_poured_into_model: Option<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
    >,
}
impl web_common_traits::prelude::HasForeignKeys
for crate::codegen::structs_codegen::tables::pouring_procedure_templates::PouringProcedureTemplate {
    type ForeignKeys = PouringProcedureTemplateForeignKeys;
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
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::VolumeMeasuringDeviceModel(
                        self.measured_with_model,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplateAssetModel(
                        self.procedure_template_measured_with_model,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::VolumetricContainerModel(
                        self.poured_from_model,
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
                        self.procedure_template_poured_from_model,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::VolumetricContainerModel(
                        self.poured_into_model,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplateAssetModel(
                        self.procedure_template_poured_into_model,
                    ),
                ),
            );
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.procedure_template.is_some()
            && foreign_keys.measured_with_model.is_some()
            && foreign_keys.procedure_template_measured_with_model.is_some()
            && foreign_keys.poured_from_model.is_some()
            && foreign_keys.foreign_procedure_template.is_some()
            && foreign_keys.procedure_template_poured_from_model.is_some()
            && foreign_keys.poured_into_model.is_some()
            && foreign_keys.procedure_template_poured_into_model.is_some()
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
            (
                crate::codegen::tables::row::Row::ProcedureTemplateAssetModel(
                    procedure_template_asset_models,
                ),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.procedure_template_measured_with_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_measured_with_model = Some(
                        procedure_template_asset_models.clone(),
                    );
                    updated = true;
                }
                if self.procedure_template_poured_from_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_poured_from_model = Some(
                        procedure_template_asset_models.clone(),
                    );
                    updated = true;
                }
                if self.procedure_template_poured_into_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_poured_into_model = Some(
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
                if self.procedure_template_measured_with_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_measured_with_model = None;
                    updated = true;
                }
                if self.procedure_template_poured_from_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_poured_from_model = None;
                    updated = true;
                }
                if self.procedure_template_poured_into_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_poured_into_model = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::VolumeMeasuringDeviceModel(
                    volume_measuring_device_models,
                ),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.measured_with_model == volume_measuring_device_models.id {
                    foreign_keys.measured_with_model = Some(
                        volume_measuring_device_models,
                    );
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::VolumeMeasuringDeviceModel(
                    volume_measuring_device_models,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.measured_with_model == volume_measuring_device_models.id {
                    foreign_keys.measured_with_model = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::VolumetricContainerModel(
                    volumetric_container_models,
                ),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.poured_from_model == volumetric_container_models.id {
                    foreign_keys.poured_from_model = Some(volumetric_container_models);
                    updated = true;
                }
                if self.poured_into_model == volumetric_container_models.id {
                    foreign_keys.poured_into_model = Some(volumetric_container_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::VolumetricContainerModel(
                    volumetric_container_models,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.poured_from_model == volumetric_container_models.id {
                    foreign_keys.poured_from_model = None;
                    updated = true;
                }
                if self.poured_into_model == volumetric_container_models.id {
                    foreign_keys.poured_into_model = None;
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
impl web_common_traits::prelude::ForeignKeys for PouringProcedureTemplateForeignKeys {}

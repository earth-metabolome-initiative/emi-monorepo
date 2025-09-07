#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PouringProcedureForeignKeys {
    pub measured_with: Option<crate::VolumeMeasuringDevice>,
    pub poured_from: Option<crate::VolumetricContainer>,
    pub poured_into: Option<crate::VolumetricContainer>,
    pub procedure: Option<crate::Procedure>,
    pub procedure_measured_with: Option<crate::ProcedureAsset>,
    pub procedure_poured_from: Option<crate::ProcedureAsset>,
    pub procedure_poured_into: Option<crate::ProcedureAsset>,
    pub procedure_template: Option<crate::PouringProcedureTemplate>,
    pub procedure_template_measured_with_model: Option<crate::ProcedureTemplateAssetModel>,
    pub procedure_template_poured_from_model: Option<crate::ProcedureTemplateAssetModel>,
    pub procedure_template_poured_into_model: Option<crate::ProcedureTemplateAssetModel>,
}
impl web_common_traits::prelude::HasForeignKeys for crate::PouringProcedure {
    type ForeignKeys = PouringProcedureForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        if let Some(measured_with) = self.measured_with {
            connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::VolumeMeasuringDevice(
                    measured_with,
                ),
            ));
        }
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::VolumetricContainer(
                self.poured_from,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::VolumetricContainer(
                self.poured_into,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Procedure(self.procedure),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureAsset(
                self.procedure_measured_with,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureAsset(
                self.procedure_poured_from,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureAsset(
                self.procedure_poured_into,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::PouringProcedureTemplate(
                self.procedure_template,
            ),
        ));
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
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplateAssetModel(
                        self.procedure_template_poured_from_model,
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
        (foreign_keys.measured_with.is_some() || self.measured_with.is_some())
            && foreign_keys.poured_from.is_some()
            && foreign_keys.poured_into.is_some()
            && foreign_keys.procedure.is_some()
            && foreign_keys.procedure_measured_with.is_some()
            && foreign_keys.procedure_poured_from.is_some()
            && foreign_keys.procedure_poured_into.is_some()
            && foreign_keys.procedure_template.is_some()
            && foreign_keys.procedure_template_measured_with_model.is_some()
            && foreign_keys.procedure_template_poured_from_model.is_some()
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
                crate::codegen::tables::row::Row::PouringProcedureTemplate(
                    pouring_procedure_templates,
                ),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.procedure_template == pouring_procedure_templates.procedure_template {
                    foreign_keys.procedure_template = Some(pouring_procedure_templates);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::PouringProcedureTemplate(
                    pouring_procedure_templates,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.procedure_template == pouring_procedure_templates.procedure_template {
                    foreign_keys.procedure_template = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureAsset(procedure_assets),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.procedure_measured_with == procedure_assets.id {
                    foreign_keys.procedure_measured_with = Some(procedure_assets);
                    updated = true;
                }
                if self.procedure_poured_from == procedure_assets.id {
                    foreign_keys.procedure_poured_from = Some(procedure_assets);
                    updated = true;
                }
                if self.procedure_poured_into == procedure_assets.id {
                    foreign_keys.procedure_poured_into = Some(procedure_assets);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureAsset(procedure_assets),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.procedure_measured_with == procedure_assets.id {
                    foreign_keys.procedure_measured_with = None;
                    updated = true;
                }
                if self.procedure_poured_from == procedure_assets.id {
                    foreign_keys.procedure_poured_from = None;
                    updated = true;
                }
                if self.procedure_poured_into == procedure_assets.id {
                    foreign_keys.procedure_poured_into = None;
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
                if self.procedure_template_measured_with_model == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_measured_with_model =
                        Some(procedure_template_asset_models.clone());
                    updated = true;
                }
                if self.procedure_template_poured_from_model == procedure_template_asset_models.id {
                    foreign_keys.procedure_template_poured_from_model =
                        Some(procedure_template_asset_models.clone());
                    updated = true;
                }
                if self.procedure_template_poured_into_model == procedure_template_asset_models.id {
                    foreign_keys.procedure_template_poured_into_model =
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
                if self.procedure_template_measured_with_model == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_measured_with_model = None;
                    updated = true;
                }
                if self.procedure_template_poured_from_model == procedure_template_asset_models.id {
                    foreign_keys.procedure_template_poured_from_model = None;
                    updated = true;
                }
                if self.procedure_template_poured_into_model == procedure_template_asset_models.id {
                    foreign_keys.procedure_template_poured_into_model = None;
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
                crate::codegen::tables::row::Row::VolumeMeasuringDevice(volume_measuring_devices),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self
                    .measured_with
                    .is_some_and(|measured_with| measured_with == volume_measuring_devices.id)
                {
                    foreign_keys.measured_with = Some(volume_measuring_devices);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::VolumeMeasuringDevice(volume_measuring_devices),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self
                    .measured_with
                    .is_some_and(|measured_with| measured_with == volume_measuring_devices.id)
                {
                    foreign_keys.measured_with = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::VolumetricContainer(volumetric_containers),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.poured_from == volumetric_containers.id {
                    foreign_keys.poured_from = Some(volumetric_containers);
                    updated = true;
                }
                if self.poured_into == volumetric_containers.id {
                    foreign_keys.poured_into = Some(volumetric_containers);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::VolumetricContainer(volumetric_containers),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.poured_from == volumetric_containers.id {
                    foreign_keys.poured_from = None;
                    updated = true;
                }
                if self.poured_into == volumetric_containers.id {
                    foreign_keys.poured_into = None;
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
impl web_common_traits::prelude::ForeignKeys for PouringProcedureForeignKeys {}

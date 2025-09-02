#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FractioningProcedureForeignKeys {
    pub foreign_procedure_template: Option<
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
    >,
    pub foreign_procedure: Option<
        crate::codegen::structs_codegen::tables::procedures::Procedure,
    >,
    pub fragment_container: Option<
        crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer,
    >,
    pub procedure: Option<
        crate::codegen::structs_codegen::tables::procedures::Procedure,
    >,
    pub procedure_template: Option<
        crate::codegen::structs_codegen::tables::fractioning_procedure_templates::FractioningProcedureTemplate,
    >,
    pub fragment_placed_into: Option<
        crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer,
    >,
    pub weighed_with: Option<
        crate::codegen::structs_codegen::tables::weighing_devices::WeighingDevice,
    >,
    pub weighed_with_model: Option<
        crate::codegen::structs_codegen::tables::weighing_device_models::WeighingDeviceModel,
    >,
    pub fractioning_procedures_procedure_weighed_with_model_fkey: Option<
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
    >,
}
impl web_common_traits::prelude::HasForeignKeys
    for crate::codegen::structs_codegen::tables::fractioning_procedures::FractioningProcedure
{
    type ForeignKeys = FractioningProcedureForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplate(
                self.foreign_procedure_template,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Procedure(
                self.foreign_procedure,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::VolumetricContainer(
                self.fragment_container,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Procedure(self.procedure),
        ));
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::FractioningProcedureTemplate(
                        self.procedure_template,
                    ),
                ),
            );
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::VolumetricContainer(
                self.fragment_placed_into,
            ),
        ));
        if let Some(weighed_with) = self.weighed_with {
            connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::WeighingDevice(
                    weighed_with,
                ),
            ));
        }
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::WeighingDeviceModel(
                self.weighed_with_model,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureAsset((
                self.procedure,
                self.weighed_with_model,
            )),
        ));
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.foreign_procedure_template.is_some()
            && foreign_keys.foreign_procedure.is_some()
            && foreign_keys.fragment_container.is_some()
            && foreign_keys.procedure.is_some()
            && foreign_keys.procedure_template.is_some()
            && foreign_keys.fragment_placed_into.is_some()
            && (foreign_keys.weighed_with.is_some() || self.weighed_with.is_some())
            && foreign_keys.weighed_with_model.is_some()
            && foreign_keys.fractioning_procedures_procedure_weighed_with_model_fkey.is_some()
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
                crate::codegen::tables::row::Row::FractioningProcedureTemplate(
                    fractioning_procedure_templates,
                ),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.procedure_template == fractioning_procedure_templates.procedure_template {
                    foreign_keys.procedure_template = Some(fractioning_procedure_templates);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::FractioningProcedureTemplate(
                    fractioning_procedure_templates,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.procedure_template == fractioning_procedure_templates.procedure_template {
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
                if self.procedure == procedure_assets.procedure
                    && self.weighed_with_model == procedure_assets.asset_model
                {
                    foreign_keys.fractioning_procedures_procedure_weighed_with_model_fkey =
                        Some(procedure_assets);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureAsset(procedure_assets),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.procedure == procedure_assets.procedure
                    && self.weighed_with_model == procedure_assets.asset_model
                {
                    foreign_keys.fractioning_procedures_procedure_weighed_with_model_fkey = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureTemplate(procedure_templates),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.foreign_procedure_template == procedure_templates.procedure_template {
                    foreign_keys.foreign_procedure_template = Some(procedure_templates);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureTemplate(procedure_templates),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.foreign_procedure_template == procedure_templates.procedure_template {
                    foreign_keys.foreign_procedure_template = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Procedure(procedures),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.foreign_procedure == procedures.procedure {
                    foreign_keys.foreign_procedure = Some(procedures.clone());
                    updated = true;
                }
                if self.procedure == procedures.procedure {
                    foreign_keys.procedure = Some(procedures.clone());
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Procedure(procedures),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.foreign_procedure == procedures.procedure {
                    foreign_keys.foreign_procedure = None;
                    updated = true;
                }
                if self.procedure == procedures.procedure {
                    foreign_keys.procedure = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::VolumetricContainer(volumetric_containers),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.fragment_container == volumetric_containers.id {
                    foreign_keys.fragment_container = Some(volumetric_containers);
                    updated = true;
                }
                if self.fragment_placed_into == volumetric_containers.id {
                    foreign_keys.fragment_placed_into = Some(volumetric_containers);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::VolumetricContainer(volumetric_containers),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.fragment_container == volumetric_containers.id {
                    foreign_keys.fragment_container = None;
                    updated = true;
                }
                if self.fragment_placed_into == volumetric_containers.id {
                    foreign_keys.fragment_placed_into = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::WeighingDeviceModel(weighing_device_models),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.weighed_with_model == weighing_device_models.id {
                    foreign_keys.weighed_with_model = Some(weighing_device_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::WeighingDeviceModel(weighing_device_models),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.weighed_with_model == weighing_device_models.id {
                    foreign_keys.weighed_with_model = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::WeighingDevice(weighing_devices),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.weighed_with.is_some_and(|weighed_with| weighed_with == weighing_devices.id)
                {
                    foreign_keys.weighed_with = Some(weighing_devices);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::WeighingDevice(weighing_devices),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.weighed_with.is_some_and(|weighed_with| weighed_with == weighing_devices.id)
                {
                    foreign_keys.weighed_with = None;
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
impl web_common_traits::prelude::ForeignKeys for FractioningProcedureForeignKeys {}

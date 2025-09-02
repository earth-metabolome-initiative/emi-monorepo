#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FreezeDryingProcedureForeignKeys {
    pub procedure_template: Option<
        crate::codegen::structs_codegen::tables::freeze_drying_procedure_templates::FreezeDryingProcedureTemplate,
    >,
    pub procedure: Option<
        crate::codegen::structs_codegen::tables::procedures::Procedure,
    >,
    pub foreign_procedure_template: Option<
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
    >,
    pub foreign_procedure: Option<
        crate::codegen::structs_codegen::tables::procedures::Procedure,
    >,
    pub freeze_dryed_container: Option<
        crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer,
    >,
    pub freeze_dryed_with: Option<
        crate::codegen::structs_codegen::tables::freeze_dryers::FreezeDryer,
    >,
    pub freeze_dryed_with_model: Option<
        crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel,
    >,
    pub freeze_drying_procedures_procedure_freeze_dryed_with_model_fkey: Option<
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
    >,
}
impl web_common_traits::prelude::HasForeignKeys
    for crate::codegen::structs_codegen::tables::freeze_drying_procedures::FreezeDryingProcedure
{
    type ForeignKeys = FreezeDryingProcedureForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::FreezeDryingProcedureTemplate(
                        self.procedure_template,
                    ),
                ),
            );
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Procedure(self.procedure),
        ));
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
                self.freeze_dryed_container,
            ),
        ));
        if let Some(freeze_dryed_with) = self.freeze_dryed_with {
            connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::FreezeDryer(
                    freeze_dryed_with,
                ),
            ));
        }
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::FreezeDryerModel(
                self.freeze_dryed_with_model,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureAsset((
                self.procedure,
                self.freeze_dryed_with_model,
            )),
        ));
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.procedure_template.is_some()
            && foreign_keys.procedure.is_some()
            && foreign_keys.foreign_procedure_template.is_some()
            && foreign_keys.foreign_procedure.is_some()
            && foreign_keys.freeze_dryed_container.is_some()
            && (foreign_keys.freeze_dryed_with.is_some() || self.freeze_dryed_with.is_some())
            && foreign_keys.freeze_dryed_with_model.is_some()
            && foreign_keys
                .freeze_drying_procedures_procedure_freeze_dryed_with_model_fkey
                .is_some()
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
                crate::codegen::tables::row::Row::FreezeDryerModel(freeze_dryer_models),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.freeze_dryed_with_model == freeze_dryer_models.id {
                    foreign_keys.freeze_dryed_with_model = Some(freeze_dryer_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::FreezeDryerModel(freeze_dryer_models),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.freeze_dryed_with_model == freeze_dryer_models.id {
                    foreign_keys.freeze_dryed_with_model = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::FreezeDryer(freeze_dryers),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self
                    .freeze_dryed_with
                    .is_some_and(|freeze_dryed_with| freeze_dryed_with == freeze_dryers.id)
                {
                    foreign_keys.freeze_dryed_with = Some(freeze_dryers);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::FreezeDryer(freeze_dryers),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self
                    .freeze_dryed_with
                    .is_some_and(|freeze_dryed_with| freeze_dryed_with == freeze_dryers.id)
                {
                    foreign_keys.freeze_dryed_with = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::FreezeDryingProcedureTemplate(
                    freeze_drying_procedure_templates,
                ),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.procedure_template == freeze_drying_procedure_templates.procedure_template {
                    foreign_keys.procedure_template = Some(freeze_drying_procedure_templates);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::FreezeDryingProcedureTemplate(
                    freeze_drying_procedure_templates,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.procedure_template == freeze_drying_procedure_templates.procedure_template {
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
                    && self.freeze_dryed_with_model == procedure_assets.asset_model
                {
                    foreign_keys.freeze_drying_procedures_procedure_freeze_dryed_with_model_fkey =
                        Some(procedure_assets);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureAsset(procedure_assets),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.procedure == procedure_assets.procedure
                    && self.freeze_dryed_with_model == procedure_assets.asset_model
                {
                    foreign_keys.freeze_drying_procedures_procedure_freeze_dryed_with_model_fkey =
                        None;
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
                if self.procedure == procedures.procedure {
                    foreign_keys.procedure = Some(procedures.clone());
                    updated = true;
                }
                if self.foreign_procedure == procedures.procedure {
                    foreign_keys.foreign_procedure = Some(procedures.clone());
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
                if self.foreign_procedure == procedures.procedure {
                    foreign_keys.foreign_procedure = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::VolumetricContainer(volumetric_containers),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.freeze_dryed_container == volumetric_containers.id {
                    foreign_keys.freeze_dryed_container = Some(volumetric_containers);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::VolumetricContainer(volumetric_containers),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.freeze_dryed_container == volumetric_containers.id {
                    foreign_keys.freeze_dryed_container = None;
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
impl web_common_traits::prelude::ForeignKeys for FreezeDryingProcedureForeignKeys {}

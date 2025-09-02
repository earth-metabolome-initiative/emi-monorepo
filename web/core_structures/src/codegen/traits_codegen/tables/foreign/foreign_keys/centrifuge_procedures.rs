#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CentrifugeProcedureForeignKeys {
    pub procedure: Option<
        crate::codegen::structs_codegen::tables::procedures::Procedure,
    >,
    pub procedure_template: Option<
        crate::codegen::structs_codegen::tables::centrifuge_procedure_templates::CentrifugeProcedureTemplate,
    >,
    pub foreign_procedure_template: Option<
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
    >,
    pub foreign_procedure: Option<
        crate::codegen::structs_codegen::tables::procedures::Procedure,
    >,
    pub centrifuged_container: Option<
        crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer,
    >,
    pub centrifuged_with_model: Option<
        crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel,
    >,
    pub centrifuged_with: Option<
        crate::codegen::structs_codegen::tables::centrifuges::Centrifuge,
    >,
    pub centrifuge_procedures_procedure_centrifuged_with_model_fkey: Option<
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
    >,
}
impl web_common_traits::prelude::HasForeignKeys
    for crate::codegen::structs_codegen::tables::centrifuge_procedures::CentrifugeProcedure
{
    type ForeignKeys = CentrifugeProcedureForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Procedure(self.procedure),
        ));
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::CentrifugeProcedureTemplate(
                        self.procedure_template,
                    ),
                ),
            );
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
                self.centrifuged_container,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::CentrifugeModel(
                self.centrifuged_with_model,
            ),
        ));
        if let Some(centrifuged_with) = self.centrifuged_with {
            connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Centrifuge(
                    centrifuged_with,
                ),
            ));
        }
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureAsset((
                self.procedure,
                self.centrifuged_with_model,
            )),
        ));
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.procedure.is_some()
            && foreign_keys.procedure_template.is_some()
            && foreign_keys.foreign_procedure_template.is_some()
            && foreign_keys.foreign_procedure.is_some()
            && foreign_keys.centrifuged_container.is_some()
            && foreign_keys.centrifuged_with_model.is_some()
            && (foreign_keys.centrifuged_with.is_some() || self.centrifuged_with.is_some())
            && foreign_keys.centrifuge_procedures_procedure_centrifuged_with_model_fkey.is_some()
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
                crate::codegen::tables::row::Row::CentrifugeModel(centrifuge_models),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.centrifuged_with_model == centrifuge_models.id {
                    foreign_keys.centrifuged_with_model = Some(centrifuge_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::CentrifugeModel(centrifuge_models),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.centrifuged_with_model == centrifuge_models.id {
                    foreign_keys.centrifuged_with_model = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::CentrifugeProcedureTemplate(
                    centrifuge_procedure_templates,
                ),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.procedure_template == centrifuge_procedure_templates.procedure_template {
                    foreign_keys.procedure_template = Some(centrifuge_procedure_templates);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::CentrifugeProcedureTemplate(
                    centrifuge_procedure_templates,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.procedure_template == centrifuge_procedure_templates.procedure_template {
                    foreign_keys.procedure_template = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Centrifuge(centrifuges),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self
                    .centrifuged_with
                    .is_some_and(|centrifuged_with| centrifuged_with == centrifuges.id)
                {
                    foreign_keys.centrifuged_with = Some(centrifuges);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Centrifuge(centrifuges),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self
                    .centrifuged_with
                    .is_some_and(|centrifuged_with| centrifuged_with == centrifuges.id)
                {
                    foreign_keys.centrifuged_with = None;
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
                    && self.centrifuged_with_model == procedure_assets.asset_model
                {
                    foreign_keys.centrifuge_procedures_procedure_centrifuged_with_model_fkey =
                        Some(procedure_assets);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureAsset(procedure_assets),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.procedure == procedure_assets.procedure
                    && self.centrifuged_with_model == procedure_assets.asset_model
                {
                    foreign_keys.centrifuge_procedures_procedure_centrifuged_with_model_fkey = None;
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
                if self.centrifuged_container == volumetric_containers.id {
                    foreign_keys.centrifuged_container = Some(volumetric_containers);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::VolumetricContainer(volumetric_containers),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.centrifuged_container == volumetric_containers.id {
                    foreign_keys.centrifuged_container = None;
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
impl web_common_traits::prelude::ForeignKeys for CentrifugeProcedureForeignKeys {}

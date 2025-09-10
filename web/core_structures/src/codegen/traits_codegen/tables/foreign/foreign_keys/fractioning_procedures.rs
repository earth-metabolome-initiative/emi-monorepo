#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FractioningProcedureForeignKeys {
    pub fragment_container: Option<
        crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer,
    >,
    pub fragment_placed_into: Option<
        crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer,
    >,
    pub procedure: Option<
        crate::codegen::structs_codegen::tables::procedures::Procedure,
    >,
    pub procedure_fragment_container: Option<
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
    >,
    pub procedure_fragment_placed_into: Option<
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
    >,
    pub procedure_template: Option<
        crate::codegen::structs_codegen::tables::fractioning_procedure_templates::FractioningProcedureTemplate,
    >,
    pub procedure_template_fragment_container_model: Option<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
    >,
    pub procedure_template_fragment_placed_into_model: Option<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
    >,
    pub procedure_template_weighed_with_model: Option<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
    >,
    pub procedure_weighed_with: Option<
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
    >,
    pub weighed_with: Option<
        crate::codegen::structs_codegen::tables::weighing_devices::WeighingDevice,
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
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::VolumetricContainer(
                self.fragment_container,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::VolumetricContainer(
                self.fragment_placed_into,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Procedure(self.procedure),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureAsset(
                self.procedure_fragment_container,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureAsset(
                self.procedure_fragment_placed_into,
            ),
        ));
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::FractioningProcedureTemplate(
                        self.procedure_template,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplateAssetModel(
                        self.procedure_template_fragment_container_model,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplateAssetModel(
                        self.procedure_template_fragment_placed_into_model,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplateAssetModel(
                        self.procedure_template_weighed_with_model,
                    ),
                ),
            );
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureAsset(
                self.procedure_weighed_with,
            ),
        ));
        if let Some(weighed_with) = self.weighed_with {
            connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::WeighingDevice(
                    weighed_with,
                ),
            ));
        }
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.fragment_container.is_some()
            && foreign_keys.fragment_placed_into.is_some()
            && foreign_keys.procedure.is_some()
            && foreign_keys.procedure_fragment_container.is_some()
            && foreign_keys.procedure_fragment_placed_into.is_some()
            && foreign_keys.procedure_template.is_some()
            && foreign_keys.procedure_template_fragment_container_model.is_some()
            && foreign_keys.procedure_template_fragment_placed_into_model.is_some()
            && foreign_keys.procedure_template_weighed_with_model.is_some()
            && foreign_keys.procedure_weighed_with.is_some()
            && (foreign_keys.weighed_with.is_some() || self.weighed_with.is_some())
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
                if self.procedure_fragment_container == procedure_assets.id {
                    foreign_keys.procedure_fragment_container = Some(procedure_assets);
                    updated = true;
                }
                if self.procedure_fragment_placed_into == procedure_assets.id {
                    foreign_keys.procedure_fragment_placed_into = Some(procedure_assets);
                    updated = true;
                }
                if self.procedure_weighed_with == procedure_assets.id {
                    foreign_keys.procedure_weighed_with = Some(procedure_assets);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureAsset(procedure_assets),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.procedure_fragment_container == procedure_assets.id {
                    foreign_keys.procedure_fragment_container = None;
                    updated = true;
                }
                if self.procedure_fragment_placed_into == procedure_assets.id {
                    foreign_keys.procedure_fragment_placed_into = None;
                    updated = true;
                }
                if self.procedure_weighed_with == procedure_assets.id {
                    foreign_keys.procedure_weighed_with = None;
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
                if self.procedure_template_fragment_container_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_fragment_container_model =
                        Some(procedure_template_asset_models.clone());
                    updated = true;
                }
                if self.procedure_template_fragment_placed_into_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_fragment_placed_into_model =
                        Some(procedure_template_asset_models.clone());
                    updated = true;
                }
                if self.procedure_template_weighed_with_model == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_weighed_with_model =
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
                if self.procedure_template_fragment_container_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_fragment_container_model = None;
                    updated = true;
                }
                if self.procedure_template_fragment_placed_into_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_fragment_placed_into_model = None;
                    updated = true;
                }
                if self.procedure_template_weighed_with_model == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_weighed_with_model = None;
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

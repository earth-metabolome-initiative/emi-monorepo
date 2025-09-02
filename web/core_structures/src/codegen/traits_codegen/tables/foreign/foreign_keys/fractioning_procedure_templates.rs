#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FractioningProcedureTemplateForeignKeys {
    pub procedure_template: Option<
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
    >,
    pub weighed_with_model: Option<
        crate::codegen::structs_codegen::tables::weighing_device_models::WeighingDeviceModel,
    >,
    pub procedure_template_weighed_with_model: Option<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
    >,
    pub fragment_container_model: Option<
        crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel,
    >,
    pub foreign_procedure_template: Option<
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
    >,
    pub procedure_template_fragment_container_model: Option<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
    >,
    pub fragment_placed_into_model: Option<
        crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel,
    >,
    pub procedure_template_fragment_placed_into_model: Option<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
    >,
}
impl web_common_traits::prelude::HasForeignKeys
for crate::codegen::structs_codegen::tables::fractioning_procedure_templates::FractioningProcedureTemplate {
    type ForeignKeys = FractioningProcedureTemplateForeignKeys;
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
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::WeighingDeviceModel(
                        self.weighed_with_model,
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
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::VolumetricContainerModel(
                        self.fragment_container_model,
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
                        self.procedure_template_fragment_container_model,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::VolumetricContainerModel(
                        self.fragment_placed_into_model,
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
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.procedure_template.is_some()
            && foreign_keys.weighed_with_model.is_some()
            && foreign_keys.procedure_template_weighed_with_model.is_some()
            && foreign_keys.fragment_container_model.is_some()
            && foreign_keys.foreign_procedure_template.is_some()
            && foreign_keys.procedure_template_fragment_container_model.is_some()
            && foreign_keys.fragment_placed_into_model.is_some()
            && foreign_keys.procedure_template_fragment_placed_into_model.is_some()
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
                crate::codegen::tables::row::Row::ProcedureTemplateAssetModel(
                    procedure_template_asset_models,
                ),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.procedure_template_weighed_with_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_weighed_with_model = Some(
                        procedure_template_asset_models.clone(),
                    );
                    updated = true;
                }
                if self.procedure_template_fragment_container_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_fragment_container_model = Some(
                        procedure_template_asset_models.clone(),
                    );
                    updated = true;
                }
                if self.procedure_template_fragment_placed_into_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_fragment_placed_into_model = Some(
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
                if self.procedure_template_weighed_with_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_weighed_with_model = None;
                    updated = true;
                }
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
            (
                crate::codegen::tables::row::Row::VolumetricContainerModel(
                    volumetric_container_models,
                ),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.fragment_container_model == volumetric_container_models.id {
                    foreign_keys.fragment_container_model = Some(
                        volumetric_container_models,
                    );
                    updated = true;
                }
                if self.fragment_placed_into_model == volumetric_container_models.id {
                    foreign_keys.fragment_placed_into_model = Some(
                        volumetric_container_models,
                    );
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::VolumetricContainerModel(
                    volumetric_container_models,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.fragment_container_model == volumetric_container_models.id {
                    foreign_keys.fragment_container_model = None;
                    updated = true;
                }
                if self.fragment_placed_into_model == volumetric_container_models.id {
                    foreign_keys.fragment_placed_into_model = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::WeighingDeviceModel(
                    weighing_device_models,
                ),
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
                crate::codegen::tables::row::Row::WeighingDeviceModel(
                    weighing_device_models,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.weighed_with_model == weighing_device_models.id {
                    foreign_keys.weighed_with_model = None;
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
impl web_common_traits::prelude::ForeignKeys for FractioningProcedureTemplateForeignKeys {}

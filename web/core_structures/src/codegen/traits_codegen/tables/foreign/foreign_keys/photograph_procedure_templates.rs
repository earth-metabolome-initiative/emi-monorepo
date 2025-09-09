#[derive(Debug, Clone, PartialEq, Default, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PhotographProcedureTemplateForeignKeys {
    pub procedure_template: Option<
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
    >,
    pub photographed_with_model: Option<
        crate::codegen::structs_codegen::tables::camera_models::CameraModel,
    >,
    pub procedure_template_photographed_with_model: Option<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
    >,
    pub photographed_asset_model: Option<
        crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel,
    >,
    pub procedure_template_photographed_asset_model: Option<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
    >,
    pub photograph_model: Option<
        crate::codegen::structs_codegen::tables::digital_asset_models::DigitalAssetModel,
    >,
    pub procedure_template_photograph_model: Option<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
    >,
}
impl web_common_traits::prelude::HasForeignKeys
for crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate {
    type ForeignKeys = PhotographProcedureTemplateForeignKeys;
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
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::CameraModel(
                        self.photographed_with_model,
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
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::PhysicalAssetModel(
                        self.photographed_asset_model,
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
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::DigitalAssetModel(
                        self.photograph_model,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplateAssetModel(
                        self.procedure_template_photograph_model,
                    ),
                ),
            );
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.procedure_template.is_some()
            && foreign_keys.photographed_with_model.is_some()
            && foreign_keys.procedure_template_photographed_with_model.is_some()
            && foreign_keys.photographed_asset_model.is_some()
            && foreign_keys.procedure_template_photographed_asset_model.is_some()
            && foreign_keys.photograph_model.is_some()
            && foreign_keys.procedure_template_photograph_model.is_some()
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
                crate::codegen::tables::row::Row::CameraModel(camera_models),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.photographed_with_model == camera_models.id {
                    foreign_keys.photographed_with_model = Some(camera_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::CameraModel(camera_models),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.photographed_with_model == camera_models.id {
                    foreign_keys.photographed_with_model = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::DigitalAssetModel(
                    digital_asset_models,
                ),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.photograph_model == digital_asset_models.id {
                    foreign_keys.photograph_model = Some(digital_asset_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::DigitalAssetModel(
                    digital_asset_models,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.photograph_model == digital_asset_models.id {
                    foreign_keys.photograph_model = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::PhysicalAssetModel(
                    physical_asset_models,
                ),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.photographed_asset_model == physical_asset_models.id {
                    foreign_keys.photographed_asset_model = Some(physical_asset_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::PhysicalAssetModel(
                    physical_asset_models,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.photographed_asset_model == physical_asset_models.id {
                    foreign_keys.photographed_asset_model = None;
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
                if self.procedure_template_photographed_with_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_photographed_with_model = Some(
                        procedure_template_asset_models.clone(),
                    );
                    updated = true;
                }
                if self.procedure_template_photographed_asset_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_photographed_asset_model = Some(
                        procedure_template_asset_models.clone(),
                    );
                    updated = true;
                }
                if self.procedure_template_photograph_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_photograph_model = Some(
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
                if self.procedure_template_photographed_with_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_photographed_with_model = None;
                    updated = true;
                }
                if self.procedure_template_photographed_asset_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_photographed_asset_model = None;
                    updated = true;
                }
                if self.procedure_template_photograph_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_photograph_model = None;
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
                    foreign_keys.procedure_template = Some(procedure_templates);
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
            }
            (_, crud) => {
                unreachable!("Unexpected row type with operation {crud}");
            }
        }
        updated
    }
}
impl web_common_traits::prelude::ForeignKeys for PhotographProcedureTemplateForeignKeys {}

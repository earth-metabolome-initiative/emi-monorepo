#[derive(Debug, Clone, PartialEq, Default, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HarvestingProcedureTemplateForeignKeys {
    pub procedure_template_sample_model: Option<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
    >,
    pub procedure_template_sample_source_model: Option<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
    >,
    pub procedure_template: Option<
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
    >,
    pub sample_model: Option<
        crate::codegen::structs_codegen::tables::sample_models::SampleModel,
    >,
    pub sample_source_model: Option<
        crate::codegen::structs_codegen::tables::sample_source_models::SampleSourceModel,
    >,
}
impl web_common_traits::prelude::HasForeignKeys
for crate::codegen::structs_codegen::tables::harvesting_procedure_templates::HarvestingProcedureTemplate {
    type ForeignKeys = HarvestingProcedureTemplateForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplateAssetModel(
                        self.procedure_template_sample_model,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplateAssetModel(
                        self.procedure_template_sample_source_model,
                    ),
                ),
            );
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
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::SampleModel(
                        self.sample_model,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::SampleSourceModel(
                        self.sample_source_model,
                    ),
                ),
            );
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.procedure_template_sample_model.is_some()
            && foreign_keys.procedure_template_sample_source_model.is_some()
            && foreign_keys.procedure_template.is_some()
            && foreign_keys.sample_model.is_some()
            && foreign_keys.sample_source_model.is_some()
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
                if self.procedure_template_sample_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_sample_model = Some(
                        procedure_template_asset_models.clone(),
                    );
                    updated = true;
                }
                if self.procedure_template_sample_source_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_sample_source_model = Some(
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
                if self.procedure_template_sample_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_sample_model = None;
                    updated = true;
                }
                if self.procedure_template_sample_source_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_sample_source_model = None;
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
            (
                crate::codegen::tables::row::Row::SampleModel(sample_models),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.sample_model == sample_models.id {
                    foreign_keys.sample_model = Some(sample_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::SampleModel(sample_models),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.sample_model == sample_models.id {
                    foreign_keys.sample_model = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::SampleSourceModel(
                    sample_source_models,
                ),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.sample_source_model == sample_source_models.id {
                    foreign_keys.sample_source_model = Some(sample_source_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::SampleSourceModel(
                    sample_source_models,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.sample_source_model == sample_source_models.id {
                    foreign_keys.sample_source_model = None;
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
impl web_common_traits::prelude::ForeignKeys for HarvestingProcedureTemplateForeignKeys {}

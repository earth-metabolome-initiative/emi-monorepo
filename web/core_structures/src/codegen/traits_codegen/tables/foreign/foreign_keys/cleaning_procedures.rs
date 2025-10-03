#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CleaningProcedureForeignKeys {
    pub procedure_cleaned: Option<
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
    >,
    pub procedure_cleaned_with: Option<
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
    >,
    pub procedure: Option<
        crate::codegen::structs_codegen::tables::procedures::Procedure,
    >,
    pub procedure_template_cleaned_model: Option<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
    >,
    pub procedure_template_cleaned_with_model: Option<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
    >,
    pub procedure_template: Option<
        crate::codegen::structs_codegen::tables::cleaning_procedure_templates::CleaningProcedureTemplate,
    >,
}
impl web_common_traits::prelude::HasForeignKeys
    for crate::codegen::structs_codegen::tables::cleaning_procedures::CleaningProcedure
{
    type ForeignKeys = CleaningProcedureForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureAsset(
                self.procedure_cleaned,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureAsset(
                self.procedure_cleaned_with,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Procedure(self.procedure),
        ));
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplateAssetModel(
                        self.procedure_template_cleaned_model,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplateAssetModel(
                        self.procedure_template_cleaned_with_model,
                    ),
                ),
            );
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::CleaningProcedureTemplate(
                self.procedure_template,
            ),
        ));
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.procedure_cleaned.is_some()
            && foreign_keys.procedure_cleaned_with.is_some()
            && foreign_keys.procedure.is_some()
            && foreign_keys.procedure_template_cleaned_model.is_some()
            && foreign_keys.procedure_template_cleaned_with_model.is_some()
            && foreign_keys.procedure_template.is_some()
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
                crate::codegen::tables::row::Row::CleaningProcedureTemplate(
                    cleaning_procedure_templates,
                ),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.procedure_template == cleaning_procedure_templates.procedure_template {
                    foreign_keys.procedure_template = Some(cleaning_procedure_templates);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::CleaningProcedureTemplate(
                    cleaning_procedure_templates,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.procedure_template == cleaning_procedure_templates.procedure_template {
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
                if self.procedure_cleaned == procedure_assets.id {
                    foreign_keys.procedure_cleaned = Some(procedure_assets);
                    updated = true;
                }
                if self.procedure_cleaned_with == procedure_assets.id {
                    foreign_keys.procedure_cleaned_with = Some(procedure_assets);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureAsset(procedure_assets),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.procedure_cleaned == procedure_assets.id {
                    foreign_keys.procedure_cleaned = None;
                    updated = true;
                }
                if self.procedure_cleaned_with == procedure_assets.id {
                    foreign_keys.procedure_cleaned_with = None;
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
                if self.procedure_template_cleaned_model == procedure_template_asset_models.id {
                    foreign_keys.procedure_template_cleaned_model =
                        Some(procedure_template_asset_models.clone());
                    updated = true;
                }
                if self.procedure_template_cleaned_with_model == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_cleaned_with_model =
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
                if self.procedure_template_cleaned_model == procedure_template_asset_models.id {
                    foreign_keys.procedure_template_cleaned_model = None;
                    updated = true;
                }
                if self.procedure_template_cleaned_with_model == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_cleaned_with_model = None;
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
            (_, crud) => {
                unreachable!("Unexpected row type with operation {crud}");
            }
        }
        updated
    }
}
impl web_common_traits::prelude::ForeignKeys for CleaningProcedureForeignKeys {}

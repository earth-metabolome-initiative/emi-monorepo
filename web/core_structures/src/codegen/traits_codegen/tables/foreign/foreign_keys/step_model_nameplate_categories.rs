#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StepModelNameplateCategoryForeignKeys {
    pub step_model: Option<
        crate::codegen::structs_codegen::tables::step_models::StepModel,
    >,
    pub procedure_model_nameplate_category: Option<
        crate::codegen::structs_codegen::tables::procedure_model_nameplate_categories::ProcedureModelNameplateCategory,
    >,
    pub created_by: Option<crate::codegen::structs_codegen::tables::users::User>,
    pub updated_by: Option<crate::codegen::structs_codegen::tables::users::User>,
}
impl web_common_traits::prelude::HasForeignKeys
for crate::codegen::structs_codegen::tables::step_model_nameplate_categories::StepModelNameplateCategory {
    type ForeignKeys = StepModelNameplateCategoryForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::StepModel(
                        self.step_model_id,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureModelNameplateCategory(
                        self.procedure_model_nameplate_category_id,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::User(
                        self.created_by,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::User(
                        self.updated_by,
                    ),
                ),
            );
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.step_model.is_some()
            && foreign_keys.procedure_model_nameplate_category.is_some()
            && foreign_keys.created_by.is_some() && foreign_keys.updated_by.is_some()
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
                crate::codegen::tables::row::Row::ProcedureModelNameplateCategory(
                    procedure_model_nameplate_categories,
                ),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if procedure_model_nameplate_categories.id
                    == self.procedure_model_nameplate_category_id
                {
                    foreign_keys.procedure_model_nameplate_category = Some(
                        procedure_model_nameplate_categories,
                    );
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureModelNameplateCategory(
                    procedure_model_nameplate_categories,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if procedure_model_nameplate_categories.id
                    == self.procedure_model_nameplate_category_id
                {
                    foreign_keys.procedure_model_nameplate_category = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::StepModel(step_models),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if step_models.id == self.step_model_id {
                    foreign_keys.step_model = Some(step_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::StepModel(step_models),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if step_models.id == self.step_model_id {
                    foreign_keys.step_model = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::User(users),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if users.id == self.created_by {
                    foreign_keys.created_by = Some(users.clone());
                    updated = true;
                }
                if users.id == self.updated_by {
                    foreign_keys.updated_by = Some(users.clone());
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::User(users),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if users.id == self.created_by {
                    foreign_keys.created_by = None;
                    updated = true;
                }
                if users.id == self.updated_by {
                    foreign_keys.updated_by = None;
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
impl web_common_traits::prelude::ForeignKeys for StepModelNameplateCategoryForeignKeys {}

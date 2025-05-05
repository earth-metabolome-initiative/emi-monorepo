#[derive(Debug, Clone, PartialEq, Default, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StepToolModelForeignKeys {
    pub step: Option<crate::codegen::structs_codegen::tables::steps::Step>,
    pub tool_model: Option<crate::codegen::structs_codegen::tables::tool_models::ToolModel>,
    pub created_by: Option<crate::codegen::structs_codegen::tables::users::User>,
}
impl web_common_traits::prelude::HasForeignKeys
    for crate::codegen::structs_codegen::tables::step_tool_models::StepToolModel
{
    type ForeignKeys = StepToolModelForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Step(self.step_id),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ToolModel(
                self.tool_model_id,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::User(self.created_by),
        ));
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.step.is_some()
            && foreign_keys.tool_model.is_some()
            && foreign_keys.created_by.is_some()
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
                crate::codegen::tables::row::Row::User(users),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if users.id == self.created_by {
                    foreign_keys.created_by = Some(users);
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
            }
            (
                crate::codegen::tables::row::Row::ToolModel(tool_models),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if tool_models.id == self.tool_model_id {
                    foreign_keys.tool_model = Some(tool_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ToolModel(tool_models),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if tool_models.id == self.tool_model_id {
                    foreign_keys.tool_model = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Step(steps),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if steps.id == self.step_id {
                    foreign_keys.step = Some(steps);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Step(steps),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if steps.id == self.step_id {
                    foreign_keys.step = None;
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
impl web_common_traits::prelude::ForeignKeys for StepToolModelForeignKeys {}

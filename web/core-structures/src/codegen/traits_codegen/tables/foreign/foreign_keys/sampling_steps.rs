#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SamplingStepForeignKeys {
    pub id: Option<crate::codegen::structs_codegen::tables::steps::Step>,
    pub processable: Option<crate::codegen::structs_codegen::tables::processables::Processable>,
    pub trackable: Option<crate::codegen::structs_codegen::tables::trackables::Trackable>,
    pub created_by: Option<crate::codegen::structs_codegen::tables::users::User>,
}
impl web_common_traits::prelude::HasForeignKeys
    for crate::codegen::structs_codegen::tables::sampling_steps::SamplingStep
{
    type ForeignKeys = SamplingStepForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Step(self.id),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Processable(
                self.processable_id,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Trackable(
                self.trackable_id,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::User(self.created_by),
        ));
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.id.is_some()
            && foreign_keys.processable.is_some()
            && foreign_keys.trackable.is_some()
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
                crate::codegen::tables::row::Row::Processable(processables),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if processables.id == self.processable_id {
                    foreign_keys.processable = Some(processables);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Processable(processables),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if processables.id == self.processable_id {
                    foreign_keys.processable = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Trackable(trackables),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if trackables.id == self.trackable_id {
                    foreign_keys.trackable = Some(trackables);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Trackable(trackables),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if trackables.id == self.trackable_id {
                    foreign_keys.trackable = None;
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
                crate::codegen::tables::row::Row::Step(steps),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if steps.id == self.id {
                    foreign_keys.id = Some(steps);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Step(steps),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if steps.id == self.id {
                    foreign_keys.id = None;
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
impl web_common_traits::prelude::ForeignKeys for SamplingStepForeignKeys {}

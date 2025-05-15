#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AliquotingStepForeignKeys {
    pub id: Option<crate::codegen::structs_codegen::tables::steps::Step>,
    pub source_processable: Option<
        crate::codegen::structs_codegen::tables::volumetric_processables::VolumetricProcessable,
    >,
    pub destination_processable: Option<
        crate::codegen::structs_codegen::tables::volumetric_processables::VolumetricProcessable,
    >,
    pub instrument: Option<crate::codegen::structs_codegen::tables::instruments::Instrument>,
    pub created_by: Option<crate::codegen::structs_codegen::tables::users::User>,
}
impl web_common_traits::prelude::HasForeignKeys
    for crate::codegen::structs_codegen::tables::aliquoting_steps::AliquotingStep
{
    type ForeignKeys = AliquotingStepForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Step(self.id),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::VolumetricProcessable(
                self.source_processable_id,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::VolumetricProcessable(
                self.destination_processable_id,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Instrument(
                self.instrument_id,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::User(self.created_by),
        ));
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.id.is_some()
            && foreign_keys.source_processable.is_some()
            && foreign_keys.destination_processable.is_some()
            && foreign_keys.instrument.is_some()
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
            (
                crate::codegen::tables::row::Row::VolumetricProcessable(volumetric_processables),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if volumetric_processables.id == self.source_processable_id {
                    foreign_keys.source_processable = Some(volumetric_processables);
                    updated = true;
                }
                if volumetric_processables.id == self.destination_processable_id {
                    foreign_keys.destination_processable = Some(volumetric_processables);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::VolumetricProcessable(volumetric_processables),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if volumetric_processables.id == self.source_processable_id {
                    foreign_keys.source_processable = None;
                    updated = true;
                }
                if volumetric_processables.id == self.destination_processable_id {
                    foreign_keys.destination_processable = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Instrument(instruments),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if instruments.id == self.instrument_id {
                    foreign_keys.instrument = Some(instruments);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Instrument(instruments),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if instruments.id == self.instrument_id {
                    foreign_keys.instrument = None;
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
impl web_common_traits::prelude::ForeignKeys for AliquotingStepForeignKeys {}

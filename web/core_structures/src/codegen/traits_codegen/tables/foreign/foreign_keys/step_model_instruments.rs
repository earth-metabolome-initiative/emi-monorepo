#[derive(Debug, Clone, PartialEq, Default, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StepModelInstrumentForeignKeys {
    pub id: Option<
        crate::codegen::structs_codegen::tables::step_model_instrument_models::StepModelInstrumentModel,
    >,
    pub instrument: Option<
        crate::codegen::structs_codegen::tables::instruments::Instrument,
    >,
    pub created_by: Option<crate::codegen::structs_codegen::tables::users::User>,
    pub updated_by: Option<crate::codegen::structs_codegen::tables::users::User>,
}
impl web_common_traits::prelude::HasForeignKeys
    for crate::codegen::structs_codegen::tables::step_model_instruments::StepModelInstrument
{
    type ForeignKeys = StepModelInstrumentForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::StepModelInstrumentModel(
                self.id,
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
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::User(self.updated_by),
        ));
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.id.is_some()
            && foreign_keys.instrument.is_some()
            && foreign_keys.created_by.is_some()
            && foreign_keys.updated_by.is_some()
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
            (
                crate::codegen::tables::row::Row::StepModelInstrumentModel(
                    step_model_instrument_models,
                ),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if step_model_instrument_models.id == self.id {
                    foreign_keys.id = Some(step_model_instrument_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::StepModelInstrumentModel(
                    step_model_instrument_models,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if step_model_instrument_models.id == self.id {
                    foreign_keys.id = None;
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
impl web_common_traits::prelude::ForeignKeys for StepModelInstrumentForeignKeys {}

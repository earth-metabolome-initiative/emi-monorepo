#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InstrumentForeignKeys {
    pub user_created:
        Option<std::rc::Rc<crate::codegen::structs_codegen::tables::directus_users::DirectusUser>>,
    pub user_updated:
        Option<std::rc::Rc<crate::codegen::structs_codegen::tables::directus_users::DirectusUser>>,
    pub instrument_model: Option<
        std::rc::Rc<crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel>,
    >,
    pub instrument_location:
        Option<std::rc::Rc<crate::codegen::structs_codegen::tables::rooms::Room>>,
}
impl web_common_traits::prelude::HasForeignKeys
    for crate::codegen::structs_codegen::tables::instruments::Instrument
{
    type ForeignKeys = InstrumentForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        if let Some(user_created) = self.user_created {
            connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::DirectusUser(
                    user_created,
                ),
            ));
        }
        if let Some(user_updated) = self.user_updated {
            connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::DirectusUser(
                    user_updated,
                ),
            ));
        }
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::InstrumentModel(
                self.instrument_model,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Room(
                self.instrument_location,
            ),
        ));
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        (foreign_keys.user_created.is_some() || self.user_created.is_none())
            && (foreign_keys.user_updated.is_some() || self.user_updated.is_none())
            && foreign_keys.instrument_model.is_some()
            && foreign_keys.instrument_location.is_some()
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
                crate::codegen::tables::row::Row::DirectusUser(directus_users),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if let Some(user_created) = self.user_created {
                    if directus_users.id == user_created {
                        foreign_keys.user_created = Some(directus_users.clone());
                        updated = true;
                    }
                }
                if let Some(user_updated) = self.user_updated {
                    if directus_users.id == user_updated {
                        foreign_keys.user_updated = Some(directus_users.clone());
                        updated = true;
                    }
                }
            }
            (
                crate::codegen::tables::row::Row::DirectusUser(directus_users),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if let Some(user_created) = self.user_created {
                    if directus_users.id == user_created {
                        foreign_keys.user_created = None;
                        updated = true;
                    }
                }
                if let Some(user_updated) = self.user_updated {
                    if directus_users.id == user_updated {
                        foreign_keys.user_updated = None;
                        updated = true;
                    }
                }
            }
            (
                crate::codegen::tables::row::Row::InstrumentModel(instrument_models),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if instrument_models.id == self.instrument_model {
                    foreign_keys.instrument_model = Some(instrument_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::InstrumentModel(instrument_models),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if instrument_models.id == self.instrument_model {
                    foreign_keys.instrument_model = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Room(rooms),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if rooms.id == self.instrument_location {
                    foreign_keys.instrument_location = Some(rooms);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Room(rooms),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if rooms.id == self.instrument_location {
                    foreign_keys.instrument_location = None;
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
impl web_common_traits::prelude::ForeignKeys for InstrumentForeignKeys {}

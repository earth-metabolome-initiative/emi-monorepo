#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MsDatumForeignKeys {
    pub user_created:
        Option<std::rc::Rc<crate::codegen::structs_codegen::tables::directus_users::DirectusUser>>,
    pub user_updated:
        Option<std::rc::Rc<crate::codegen::structs_codegen::tables::directus_users::DirectusUser>>,
    pub injection_volume_unit:
        Option<std::rc::Rc<crate::codegen::structs_codegen::tables::si_units::SiUnit>>,
    pub injection_method: Option<
        std::rc::Rc<crate::codegen::structs_codegen::tables::injection_methods::InjectionMethod>,
    >,
    pub instrument_used:
        Option<std::rc::Rc<crate::codegen::structs_codegen::tables::instruments::Instrument>>,
    pub batch: Option<std::rc::Rc<crate::codegen::structs_codegen::tables::batches::Batch>>,
    pub parent_sample_container:
        Option<std::rc::Rc<crate::codegen::structs_codegen::tables::containers::Container>>,
}
impl web_common_traits::prelude::HasForeignKeys
    for crate::codegen::structs_codegen::tables::ms_data::MsDatum
{
    type ForeignKeys = MsDatumForeignKeys;
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
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::SiUnit(
                self.injection_volume_unit,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::InjectionMethod(
                self.injection_method,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Instrument(
                self.instrument_used,
            ),
        ));
        if let Some(batch) = self.batch {
            connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Batch(batch),
            ));
        }
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Container(
                self.parent_sample_container,
            ),
        ));
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        (foreign_keys.user_created.is_some() || self.user_created.is_none())
            && (foreign_keys.user_updated.is_some() || self.user_updated.is_none())
            && foreign_keys.injection_volume_unit.is_some()
            && foreign_keys.injection_method.is_some()
            && foreign_keys.instrument_used.is_some()
            && (foreign_keys.batch.is_some() || self.batch.is_none())
            && foreign_keys.parent_sample_container.is_some()
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
                if instruments.id == self.instrument_used {
                    foreign_keys.instrument_used = Some(instruments);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Instrument(instruments),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if instruments.id == self.instrument_used {
                    foreign_keys.instrument_used = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::InjectionMethod(injection_methods),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if injection_methods.id == self.injection_method {
                    foreign_keys.injection_method = Some(injection_methods);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::InjectionMethod(injection_methods),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if injection_methods.id == self.injection_method {
                    foreign_keys.injection_method = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Batch(batches),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if let Some(batch) = self.batch {
                    if batches.id == batch {
                        foreign_keys.batch = Some(batches);
                        updated = true;
                    }
                }
            }
            (
                crate::codegen::tables::row::Row::Batch(batches),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if let Some(batch) = self.batch {
                    if batches.id == batch {
                        foreign_keys.batch = None;
                        updated = true;
                    }
                }
            }
            (
                crate::codegen::tables::row::Row::Container(containers),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if containers.id == self.parent_sample_container {
                    foreign_keys.parent_sample_container = Some(containers);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Container(containers),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if containers.id == self.parent_sample_container {
                    foreign_keys.parent_sample_container = None;
                    updated = true;
                }
            }
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
                crate::codegen::tables::row::Row::SiUnit(si_units),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if si_units.id == self.injection_volume_unit {
                    foreign_keys.injection_volume_unit = Some(si_units);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::SiUnit(si_units),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if si_units.id == self.injection_volume_unit {
                    foreign_keys.injection_volume_unit = None;
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
impl web_common_traits::prelude::ForeignKeys for MsDatumForeignKeys {}

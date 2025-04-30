#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExtractionDatumForeignKeys {
    pub user_created:
        Option<std::rc::Rc<crate::codegen::structs_codegen::tables::directus_users::DirectusUser>>,
    pub user_updated:
        Option<std::rc::Rc<crate::codegen::structs_codegen::tables::directus_users::DirectusUser>>,
    pub dried_weight_unit:
        Option<std::rc::Rc<crate::codegen::structs_codegen::tables::si_units::SiUnit>>,
    pub extraction_method: Option<
        std::rc::Rc<crate::codegen::structs_codegen::tables::extraction_methods::ExtractionMethod>,
    >,
    pub batch: Option<std::rc::Rc<crate::codegen::structs_codegen::tables::batches::Batch>>,
    pub solvent_volume_unit:
        Option<std::rc::Rc<crate::codegen::structs_codegen::tables::si_units::SiUnit>>,
    pub sample_container:
        Option<std::rc::Rc<crate::codegen::structs_codegen::tables::containers::Container>>,
    pub parent_container:
        Option<std::rc::Rc<crate::codegen::structs_codegen::tables::containers::Container>>,
    pub parent_sample_container:
        Option<std::rc::Rc<crate::codegen::structs_codegen::tables::containers::Container>>,
    pub extraction_container: Option<
        std::rc::Rc<crate::codegen::structs_codegen::tables::container_models::ContainerModel>,
    >,
}
impl web_common_traits::prelude::HasForeignKeys
    for crate::codegen::structs_codegen::tables::extraction_data::ExtractionDatum
{
    type ForeignKeys = ExtractionDatumForeignKeys;
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
                self.dried_weight_unit,
            ),
        ));
        if let Some(extraction_method) = self.extraction_method {
            connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::ExtractionMethod(
                    extraction_method,
                ),
            ));
        }
        if let Some(batch) = self.batch {
            connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Batch(batch),
            ));
        }
        if let Some(solvent_volume_unit) = self.solvent_volume_unit {
            connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::SiUnit(
                    solvent_volume_unit,
                ),
            ));
        }
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Container(
                self.sample_container,
            ),
        ));
        if let Some(parent_container) = self.parent_container {
            connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Container(
                    parent_container,
                ),
            ));
        }
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Container(
                self.parent_sample_container,
            ),
        ));
        if let Some(extraction_container) = self.extraction_container {
            connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::ContainerModel(
                    extraction_container,
                ),
            ));
        }
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        (foreign_keys.user_created.is_some() || self.user_created.is_none())
            && (foreign_keys.user_updated.is_some() || self.user_updated.is_none())
            && foreign_keys.dried_weight_unit.is_some()
            && (foreign_keys.extraction_method.is_some() || self.extraction_method.is_none())
            && (foreign_keys.batch.is_some() || self.batch.is_none())
            && (foreign_keys.solvent_volume_unit.is_some() || self.solvent_volume_unit.is_none())
            && foreign_keys.sample_container.is_some()
            && (foreign_keys.parent_container.is_some() || self.parent_container.is_none())
            && foreign_keys.parent_sample_container.is_some()
            && (foreign_keys.extraction_container.is_some() || self.extraction_container.is_none())
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
                crate::codegen::tables::row::Row::Container(containers),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if containers.id == self.sample_container {
                    foreign_keys.sample_container = Some(containers.clone());
                    updated = true;
                }
                if let Some(parent_container) = self.parent_container {
                    if containers.id == parent_container {
                        foreign_keys.parent_container = Some(containers.clone());
                        updated = true;
                    }
                }
                if containers.id == self.parent_sample_container {
                    foreign_keys.parent_sample_container = Some(containers.clone());
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Container(containers),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if containers.id == self.sample_container {
                    foreign_keys.sample_container = None;
                    updated = true;
                }
                if let Some(parent_container) = self.parent_container {
                    if containers.id == parent_container {
                        foreign_keys.parent_container = None;
                        updated = true;
                    }
                }
                if containers.id == self.parent_sample_container {
                    foreign_keys.parent_sample_container = None;
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
                crate::codegen::tables::row::Row::SiUnit(si_units),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if si_units.id == self.dried_weight_unit {
                    foreign_keys.dried_weight_unit = Some(si_units.clone());
                    updated = true;
                }
                if let Some(solvent_volume_unit) = self.solvent_volume_unit {
                    if si_units.id == solvent_volume_unit {
                        foreign_keys.solvent_volume_unit = Some(si_units.clone());
                        updated = true;
                    }
                }
            }
            (
                crate::codegen::tables::row::Row::SiUnit(si_units),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if si_units.id == self.dried_weight_unit {
                    foreign_keys.dried_weight_unit = None;
                    updated = true;
                }
                if let Some(solvent_volume_unit) = self.solvent_volume_unit {
                    if si_units.id == solvent_volume_unit {
                        foreign_keys.solvent_volume_unit = None;
                        updated = true;
                    }
                }
            }
            (
                crate::codegen::tables::row::Row::ExtractionMethod(extraction_methods),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if let Some(extraction_method) = self.extraction_method {
                    if extraction_methods.id == extraction_method {
                        foreign_keys.extraction_method = Some(extraction_methods);
                        updated = true;
                    }
                }
            }
            (
                crate::codegen::tables::row::Row::ExtractionMethod(extraction_methods),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if let Some(extraction_method) = self.extraction_method {
                    if extraction_methods.id == extraction_method {
                        foreign_keys.extraction_method = None;
                        updated = true;
                    }
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
                crate::codegen::tables::row::Row::ContainerModel(container_models),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if let Some(extraction_container) = self.extraction_container {
                    if container_models.id == extraction_container {
                        foreign_keys.extraction_container = Some(container_models);
                        updated = true;
                    }
                }
            }
            (
                crate::codegen::tables::row::Row::ContainerModel(container_models),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if let Some(extraction_container) = self.extraction_container {
                    if container_models.id == extraction_container {
                        foreign_keys.extraction_container = None;
                        updated = true;
                    }
                }
            }
            (_, crud) => {
                unreachable!("Unexpected row type with operation {crud}");
            }
        }
        updated
    }
}
impl web_common_traits::prelude::ForeignKeys for ExtractionDatumForeignKeys {}

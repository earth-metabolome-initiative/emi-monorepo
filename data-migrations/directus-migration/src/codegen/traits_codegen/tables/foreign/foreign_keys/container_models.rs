#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ContainerModelForeignKeys {
    pub user_created:
        Option<std::rc::Rc<crate::codegen::structs_codegen::tables::directus_users::DirectusUser>>,
    pub user_updated:
        Option<std::rc::Rc<crate::codegen::structs_codegen::tables::directus_users::DirectusUser>>,
    pub container_type: Option<
        std::rc::Rc<crate::codegen::structs_codegen::tables::container_types::ContainerType>,
    >,
    pub volume_unit: Option<std::rc::Rc<crate::codegen::structs_codegen::tables::si_units::SiUnit>>,
    pub brand: Option<std::rc::Rc<crate::codegen::structs_codegen::tables::brands::Brand>>,
}
impl web_common_traits::prelude::HasForeignKeys
    for crate::codegen::structs_codegen::tables::container_models::ContainerModel
{
    type ForeignKeys = ContainerModelForeignKeys;
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
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ContainerType(
                self.container_type,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::SiUnit(self.volume_unit),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Brand(self.brand),
        ));
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        (foreign_keys.user_created.is_some() || self.user_created.is_none())
            && (foreign_keys.user_updated.is_some() || self.user_updated.is_none())
            && foreign_keys.container_type.is_some()
            && foreign_keys.volume_unit.is_some()
            && foreign_keys.brand.is_some()
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
                crate::codegen::tables::row::Row::Brand(brands),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if brands.id == self.brand {
                    foreign_keys.brand = Some(brands);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Brand(brands),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if brands.id == self.brand {
                    foreign_keys.brand = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ContainerType(container_types),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if container_types.id == self.container_type {
                    foreign_keys.container_type = Some(container_types);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ContainerType(container_types),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if container_types.id == self.container_type {
                    foreign_keys.container_type = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::SiUnit(si_units),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if si_units.id == self.volume_unit {
                    foreign_keys.volume_unit = Some(si_units);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::SiUnit(si_units),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if si_units.id == self.volume_unit {
                    foreign_keys.volume_unit = None;
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
            (_, crud) => {
                unreachable!("Unexpected row type with operation {crud}");
            }
        }
        updated
    }
}
impl web_common_traits::prelude::ForeignKeys for ContainerModelForeignKeys {}

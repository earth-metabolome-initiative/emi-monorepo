#[derive(Debug, Clone, PartialEq, Default, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TeamForeignKeys {
    pub color: Option<crate::codegen::structs_codegen::tables::colors::Color>,
    pub state: Option<crate::codegen::structs_codegen::tables::team_states::TeamState>,
    pub parent_team: Option<crate::codegen::structs_codegen::tables::teams::Team>,
    pub created_by: Option<crate::codegen::structs_codegen::tables::users::User>,
    pub updated_by: Option<crate::codegen::structs_codegen::tables::users::User>,
}
impl web_common_traits::prelude::HasForeignKeys
    for crate::codegen::structs_codegen::tables::teams::Team
{
    type ForeignKeys = TeamForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Color(self.color_id),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::TeamState(self.state_id),
        ));
        if let Some(parent_team_id) = self.parent_team_id {
            connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Team(parent_team_id),
            ));
        }
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::User(self.created_by),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::User(self.updated_by),
        ));
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.color.is_some()
            && foreign_keys.state.is_some()
            && (foreign_keys.parent_team.is_some() || self.parent_team_id.is_none())
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
                crate::codegen::tables::row::Row::Team(teams),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if let Some(parent_team_id) = self.parent_team_id {
                    if teams.id == parent_team_id {
                        foreign_keys.parent_team = Some(teams);
                        updated = true;
                    }
                }
            }
            (
                crate::codegen::tables::row::Row::Team(teams),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if let Some(parent_team_id) = self.parent_team_id {
                    if teams.id == parent_team_id {
                        foreign_keys.parent_team = None;
                        updated = true;
                    }
                }
            }
            (
                crate::codegen::tables::row::Row::Color(colors),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if colors.id == self.color_id {
                    foreign_keys.color = Some(colors);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Color(colors),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if colors.id == self.color_id {
                    foreign_keys.color = None;
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
            (
                crate::codegen::tables::row::Row::TeamState(team_states),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if team_states.id == self.state_id {
                    foreign_keys.state = Some(team_states);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::TeamState(team_states),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if team_states.id == self.state_id {
                    foreign_keys.state = None;
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
impl web_common_traits::prelude::ForeignKeys for TeamForeignKeys {}

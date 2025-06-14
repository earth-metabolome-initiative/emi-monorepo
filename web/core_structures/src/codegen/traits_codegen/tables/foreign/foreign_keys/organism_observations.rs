#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OrganismObservationForeignKeys {
    pub created_by: Option<crate::codegen::structs_codegen::tables::users::User>,
    pub organism: Option<crate::codegen::structs_codegen::tables::organisms::Organism>,
    pub project: Option<crate::codegen::structs_codegen::tables::projects::Project>,
    pub subject:
        Option<crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject>,
    pub updated_by: Option<crate::codegen::structs_codegen::tables::users::User>,
}
impl web_common_traits::prelude::HasForeignKeys
    for crate::codegen::structs_codegen::tables::organism_observations::OrganismObservation
{
    type ForeignKeys = OrganismObservationForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::User(self.created_by),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Organism(self.organism_id),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Project(self.project_id),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ObservationSubject(
                self.subject_id,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::User(self.updated_by),
        ));
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.created_by.is_some()
            && foreign_keys.organism.is_some()
            && foreign_keys.project.is_some()
            && foreign_keys.subject.is_some()
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
                crate::codegen::tables::row::Row::ObservationSubject(observation_subjects),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.subject_id == observation_subjects.id {
                    foreign_keys.subject = Some(observation_subjects);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ObservationSubject(observation_subjects),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.subject_id == observation_subjects.id {
                    foreign_keys.subject = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Organism(organisms),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.organism_id == organisms.id {
                    foreign_keys.organism = Some(organisms);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Organism(organisms),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.organism_id == organisms.id {
                    foreign_keys.organism = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Project(projects),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.project_id == projects.id {
                    foreign_keys.project = Some(projects);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Project(projects),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.project_id == projects.id {
                    foreign_keys.project = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::User(users),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.created_by == users.id {
                    foreign_keys.created_by = Some(users.clone());
                    updated = true;
                }
                if self.updated_by == users.id {
                    foreign_keys.updated_by = Some(users.clone());
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::User(users),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.created_by == users.id {
                    foreign_keys.created_by = None;
                    updated = true;
                }
                if self.updated_by == users.id {
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
impl web_common_traits::prelude::ForeignKeys for OrganismObservationForeignKeys {}

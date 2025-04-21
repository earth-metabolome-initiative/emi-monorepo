#[derive(Clone, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableOrganismObservationAttributes {
    Id,
    Wild,
    ProjectId,
    OrganismId,
    SubjectId,
    Picture,
    CreatedBy,
    CreatedAt,
    UpdatedBy,
    UpdatedAt,
}
impl core::fmt::Display for InsertableOrganismObservationAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableOrganismObservationAttributes::Id => write!(f, "id"),
            InsertableOrganismObservationAttributes::Wild => write!(f, "wild"),
            InsertableOrganismObservationAttributes::ProjectId => write!(f, "project_id"),
            InsertableOrganismObservationAttributes::OrganismId => {
                write!(f, "organism_id")
            }
            InsertableOrganismObservationAttributes::SubjectId => write!(f, "subject_id"),
            InsertableOrganismObservationAttributes::Picture => write!(f, "picture"),
            InsertableOrganismObservationAttributes::CreatedBy => write!(f, "created_by"),
            InsertableOrganismObservationAttributes::CreatedAt => write!(f, "created_at"),
            InsertableOrganismObservationAttributes::UpdatedBy => write!(f, "updated_by"),
            InsertableOrganismObservationAttributes::UpdatedAt => write!(f, "updated_at"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::organism_observations::organism_observations
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableOrganismObservation {
    id: rosetta_uuid::Uuid,
    wild: bool,
    project_id: i32,
    organism_id: rosetta_uuid::Uuid,
    subject_id: i16,
    picture: Vec<u8>,
    created_by: i32,
    created_at: rosetta_timestamp::TimestampUTC,
    updated_by: i32,
    updated_at: rosetta_timestamp::TimestampUTC,
}
impl InsertableOrganismObservation {
    #[cfg(feature = "postgres")]
    pub async fn project(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::projects::Project, diesel::result::Error>
    {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::projects::Project::table()
            .filter(
                crate::codegen::diesel_codegen::tables::projects::projects::dsl::id
                    .eq(&self.project_id),
            )
            .first::<crate::codegen::structs_codegen::tables::projects::Project>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn organism(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::organisms::Organism, diesel::result::Error>
    {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::organisms::Organism::table()
            .filter(
                crate::codegen::diesel_codegen::tables::organisms::organisms::dsl::id
                    .eq(&self.organism_id),
            )
            .first::<crate::codegen::structs_codegen::tables::organisms::Organism>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn subject(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject::table()
            .filter(
                crate::codegen::diesel_codegen::tables::observation_subjects::observation_subjects::dsl::id
                    .eq(&self.subject_id),
            )
            .first::<
                crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject,
            >(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn created_by(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::users::User, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::users::User::table()
            .filter(
                crate::codegen::diesel_codegen::tables::users::users::dsl::id.eq(&self.created_by),
            )
            .first::<crate::codegen::structs_codegen::tables::users::User>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn updated_by(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::users::User, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::users::User::table()
            .filter(
                crate::codegen::diesel_codegen::tables::users::users::dsl::id.eq(&self.updated_by),
            )
            .first::<crate::codegen::structs_codegen::tables::users::User>(conn)
            .await
    }
}
#[derive(Default)]
pub struct InsertableOrganismObservationBuilder {
    id: Option<rosetta_uuid::Uuid>,
    wild: Option<bool>,
    project_id: Option<i32>,
    organism_id: Option<rosetta_uuid::Uuid>,
    subject_id: Option<i16>,
    picture: Option<Vec<u8>>,
    created_by: Option<i32>,
    created_at: Option<rosetta_timestamp::TimestampUTC>,
    updated_by: Option<i32>,
    updated_at: Option<rosetta_timestamp::TimestampUTC>,
}
impl InsertableOrganismObservationBuilder {
    pub fn id(
        mut self,
        id: rosetta_uuid::Uuid,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.id = Some(id);
        Ok(self)
    }
    pub fn wild(
        mut self,
        wild: bool,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.wild = Some(wild);
        Ok(self)
    }
    pub fn project_id(
        mut self,
        project_id: i32,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.project_id = Some(project_id);
        Ok(self)
    }
    pub fn organism_id(
        mut self,
        organism_id: rosetta_uuid::Uuid,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.organism_id = Some(organism_id);
        Ok(self)
    }
    pub fn subject_id(
        mut self,
        subject_id: i16,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.subject_id = Some(subject_id);
        Ok(self)
    }
    pub fn picture(
        mut self,
        picture: Vec<u8>,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.picture = Some(picture);
        Ok(self)
    }
    pub fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.created_by = Some(created_by);
        Ok(self)
    }
    pub fn created_at(
        mut self,
        created_at: rosetta_timestamp::TimestampUTC,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.created_at = Some(created_at);
        Ok(self)
    }
    pub fn updated_by(
        mut self,
        updated_by: i32,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.updated_by = Some(updated_by);
        Ok(self)
    }
    pub fn updated_at(
        mut self,
        updated_at: rosetta_timestamp::TimestampUTC,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.updated_at = Some(updated_at);
        Ok(self)
    }
}
impl common_traits::prelude::Builder for InsertableOrganismObservationBuilder {
    type Error = web_common_traits::database::InsertError<InsertableOrganismObservationAttributes>;
    type Object = InsertableOrganismObservation;
    type Attribute = InsertableOrganismObservationAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            id: self.id.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableOrganismObservationAttributes::Id,
            ))?,
            wild: self.wild.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableOrganismObservationAttributes::Wild,
            ))?,
            project_id: self.project_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableOrganismObservationAttributes::ProjectId,
                ),
            )?,
            organism_id: self.organism_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableOrganismObservationAttributes::OrganismId,
                ),
            )?,
            subject_id: self.subject_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableOrganismObservationAttributes::SubjectId,
                ),
            )?,
            picture: self.picture.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableOrganismObservationAttributes::Picture,
            ))?,
            created_by: self.created_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableOrganismObservationAttributes::CreatedBy,
                ),
            )?,
            created_at: self.created_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableOrganismObservationAttributes::CreatedAt,
                ),
            )?,
            updated_by: self.updated_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableOrganismObservationAttributes::UpdatedBy,
                ),
            )?,
            updated_at: self.updated_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableOrganismObservationAttributes::UpdatedAt,
                ),
            )?,
        })
    }
}
impl TryFrom<InsertableOrganismObservation> for InsertableOrganismObservationBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableOrganismObservation) -> Result<Self, Self::Error> {
        Self::default()
            .id(insertable_variant.id)?
            .wild(insertable_variant.wild)?
            .project_id(insertable_variant.project_id)?
            .organism_id(insertable_variant.organism_id)?
            .subject_id(insertable_variant.subject_id)?
            .picture(insertable_variant.picture)?
            .created_by(insertable_variant.created_by)?
            .created_at(insertable_variant.created_at)?
            .updated_by(insertable_variant.updated_by)?
            .updated_at(insertable_variant.updated_at)?
    }
}

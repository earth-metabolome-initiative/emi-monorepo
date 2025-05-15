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
impl Default for InsertableOrganismObservationBuilder {
    fn default() -> Self {
        Self {
            id: None,
            wild: Some(true),
            project_id: None,
            organism_id: None,
            subject_id: None,
            picture: None,
            created_by: None,
            created_at: Some(rosetta_timestamp::TimestampUTC::default()),
            updated_by: None,
            updated_at: Some(rosetta_timestamp::TimestampUTC::default()),
        }
    }
}
impl InsertableOrganismObservationBuilder {
    pub fn id<P>(mut self, id: P) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<rosetta_uuid::Uuid>,
        <P as TryInto<rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let id = id.try_into().map_err(|err: <P as TryInto<rosetta_uuid::Uuid>>::Error| {
            Into::into(err).rename_field(InsertableOrganismObservationAttributes::Id)
        })?;
        self.id = Some(id);
        Ok(self)
    }
    pub fn wild<P>(
        mut self,
        wild: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<bool>,
        <P as TryInto<bool>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let wild = wild.try_into().map_err(|err: <P as TryInto<bool>>::Error| {
            Into::into(err).rename_field(InsertableOrganismObservationAttributes::Wild)
        })?;
        self.wild = Some(wild);
        Ok(self)
    }
    pub fn project_id<P>(
        mut self,
        project_id: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let project_id = project_id.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
            Into::into(err).rename_field(InsertableOrganismObservationAttributes::ProjectId)
        })?;
        self.project_id = Some(project_id);
        Ok(self)
    }
    pub fn organism_id<P>(
        mut self,
        organism_id: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<rosetta_uuid::Uuid>,
        <P as TryInto<rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let organism_id =
            organism_id.try_into().map_err(|err: <P as TryInto<rosetta_uuid::Uuid>>::Error| {
                Into::into(err).rename_field(InsertableOrganismObservationAttributes::OrganismId)
            })?;
        self.organism_id = Some(organism_id);
        Ok(self)
    }
    pub fn subject_id<P>(
        mut self,
        subject_id: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<i16>,
        <P as TryInto<i16>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let subject_id = subject_id.try_into().map_err(|err: <P as TryInto<i16>>::Error| {
            Into::into(err).rename_field(InsertableOrganismObservationAttributes::SubjectId)
        })?;
        self.subject_id = Some(subject_id);
        Ok(self)
    }
    pub fn picture<P>(
        mut self,
        picture: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<Vec<u8>>,
        <P as TryInto<Vec<u8>>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let picture = picture.try_into().map_err(|err: <P as TryInto<Vec<u8>>>::Error| {
            Into::into(err).rename_field(InsertableOrganismObservationAttributes::Picture)
        })?;
        self.picture = Some(picture);
        Ok(self)
    }
    pub fn created_by<P>(
        mut self,
        created_by: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let created_by = created_by.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
            Into::into(err).rename_field(InsertableOrganismObservationAttributes::CreatedBy)
        })?;
        self.created_by = Some(created_by);
        self = self.updated_by(created_by)?;
        Ok(self)
    }
    pub fn created_at<P>(
        mut self,
        created_at: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<rosetta_timestamp::TimestampUTC>,
        <P as TryInto<rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let created_at = created_at.try_into().map_err(
            |err: <P as TryInto<rosetta_timestamp::TimestampUTC>>::Error| {
                Into::into(err).rename_field(InsertableOrganismObservationAttributes::CreatedAt)
            },
        )?;
        self.created_at = Some(created_at);
        Ok(self)
    }
    pub fn updated_by<P>(
        mut self,
        updated_by: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let updated_by = updated_by.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
            Into::into(err).rename_field(InsertableOrganismObservationAttributes::UpdatedBy)
        })?;
        self.updated_by = Some(updated_by);
        Ok(self)
    }
    pub fn updated_at<P>(
        mut self,
        updated_at: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<rosetta_timestamp::TimestampUTC>,
        <P as TryInto<rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let updated_at = updated_at.try_into().map_err(
            |err: <P as TryInto<rosetta_timestamp::TimestampUTC>>::Error| {
                Into::into(err).rename_field(InsertableOrganismObservationAttributes::UpdatedAt)
            },
        )?;
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
            .updated_at(insertable_variant.updated_at)
    }
}

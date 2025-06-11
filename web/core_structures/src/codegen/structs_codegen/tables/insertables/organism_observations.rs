#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
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
    id: ::rosetta_uuid::Uuid,
    wild: bool,
    project_id: i32,
    organism_id: ::rosetta_uuid::Uuid,
    subject_id: i16,
    picture: Vec<u8>,
    created_by: i32,
    created_at: ::rosetta_timestamp::TimestampUTC,
    updated_by: i32,
    updated_at: ::rosetta_timestamp::TimestampUTC,
}
impl InsertableOrganismObservation {
    pub fn created_by<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::users::User,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::users::User: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::users::User as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::users::User as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::users::User as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::users::User as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::users::User as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::users::User as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::users::User,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::users::User::table(),
                self.created_by,
            ),
            conn,
        )
    }
    pub fn organism<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::organisms::Organism,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::organisms::Organism: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::organisms::Organism as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::organisms::Organism as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::organisms::Organism as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::organisms::Organism as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::organisms::Organism as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::organisms::Organism as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::organisms::Organism,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::organisms::Organism::table(),
                self.organism_id,
            ),
            conn,
        )
    }
    pub fn project<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::projects::Project,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::projects::Project: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::projects::Project as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::projects::Project as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::projects::Project as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::projects::Project as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::projects::Project as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::projects::Project as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::projects::Project,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::projects::Project::table(),
                self.project_id,
            ),
            conn,
        )
    }
    pub fn subject<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject::table(),
                self.subject_id,
            ),
            conn,
        )
    }
    pub fn updated_by<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::users::User,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::users::User: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::users::User as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::users::User as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::users::User as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::users::User as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::users::User as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::users::User as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::users::User,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::users::User::table(),
                self.updated_by,
            ),
            conn,
        )
    }
}
pub struct InsertableOrganismObservationBuilder {
    pub(crate) id: Option<::rosetta_uuid::Uuid>,
    pub(crate) wild: Option<bool>,
    pub(crate) project_id: Option<i32>,
    pub(crate) organism_id: Option<::rosetta_uuid::Uuid>,
    pub(crate) subject_id: Option<i16>,
    pub(crate) picture: Option<Vec<u8>>,
    pub(crate) created_by: Option<i32>,
    pub(crate) created_at: Option<::rosetta_timestamp::TimestampUTC>,
    pub(crate) updated_by: Option<i32>,
    pub(crate) updated_at: Option<::rosetta_timestamp::TimestampUTC>,
}
impl Default for InsertableOrganismObservationBuilder {
    fn default() -> Self {
        Self {
            id: Default::default(),
            wild: Some(true),
            project_id: Default::default(),
            organism_id: Default::default(),
            subject_id: Default::default(),
            picture: Default::default(),
            created_by: Default::default(),
            created_at: Some(rosetta_timestamp::TimestampUTC::default()),
            updated_by: Default::default(),
            updated_at: Some(rosetta_timestamp::TimestampUTC::default()),
        }
    }
}
impl InsertableOrganismObservationBuilder {
    pub fn id<P>(
        mut self,
        id: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableOrganismObservationAttributes>,
    >
    where
        P: TryInto<::rosetta_uuid::Uuid>,
        <P as TryInto<::rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let id = id.try_into().map_err(|err: <P as TryInto<::rosetta_uuid::Uuid>>::Error| {
            Into::into(err).rename_field(InsertableOrganismObservationAttributes::Id)
        })?;
        self.id = Some(id);
        Ok(self)
    }
    pub fn wild<P>(
        mut self,
        wild: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableOrganismObservationAttributes>,
    >
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
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableOrganismObservationAttributes>,
    >
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
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableOrganismObservationAttributes>,
    >
    where
        P: TryInto<::rosetta_uuid::Uuid>,
        <P as TryInto<::rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let organism_id = organism_id.try_into().map_err(
            |err: <P as TryInto<::rosetta_uuid::Uuid>>::Error| {
                Into::into(err).rename_field(InsertableOrganismObservationAttributes::OrganismId)
            },
        )?;
        self.organism_id = Some(organism_id);
        Ok(self)
    }
    pub fn subject_id<P>(
        mut self,
        subject_id: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableOrganismObservationAttributes>,
    >
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
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableOrganismObservationAttributes>,
    >
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
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableOrganismObservationAttributes>,
    >
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
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableOrganismObservationAttributes>,
    >
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let created_at = created_at.try_into().map_err(
            |err: <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error| {
                Into::into(err).rename_field(InsertableOrganismObservationAttributes::CreatedAt)
            },
        )?;
        if let Some(updated_at) = self.updated_at {
            pgrx_validation::must_be_smaller_than_utc(created_at, updated_at).map_err(|e| {
                e.rename_fields(
                    InsertableOrganismObservationAttributes::CreatedAt,
                    InsertableOrganismObservationAttributes::UpdatedAt,
                )
            })?;
        }
        self.created_at = Some(created_at);
        Ok(self)
    }
    pub fn updated_by<P>(
        mut self,
        updated_by: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableOrganismObservationAttributes>,
    >
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
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableOrganismObservationAttributes>,
    >
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let updated_at = updated_at.try_into().map_err(
            |err: <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error| {
                Into::into(err).rename_field(InsertableOrganismObservationAttributes::UpdatedAt)
            },
        )?;
        if let Some(created_at) = self.created_at {
            pgrx_validation::must_be_smaller_than_utc(created_at, updated_at).map_err(|e| {
                e.rename_fields(
                    InsertableOrganismObservationAttributes::CreatedAt,
                    InsertableOrganismObservationAttributes::UpdatedAt,
                )
            })?;
        }
        self.updated_at = Some(updated_at);
        Ok(self)
    }
}
impl TryFrom<InsertableOrganismObservationBuilder> for InsertableOrganismObservation {
    type Error = common_traits::prelude::BuilderError<InsertableOrganismObservationAttributes>;
    fn try_from(
        builder: InsertableOrganismObservationBuilder,
    ) -> Result<InsertableOrganismObservation, Self::Error> {
        Ok(Self {
            id: builder.id.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableOrganismObservationAttributes::Id,
            ))?,
            wild: builder.wild.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableOrganismObservationAttributes::Wild,
            ))?,
            project_id: builder.project_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableOrganismObservationAttributes::ProjectId,
                ),
            )?,
            organism_id: builder.organism_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableOrganismObservationAttributes::OrganismId,
                ),
            )?,
            subject_id: builder.subject_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableOrganismObservationAttributes::SubjectId,
                ),
            )?,
            picture: builder.picture.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableOrganismObservationAttributes::Picture,
                ),
            )?,
            created_by: builder.created_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableOrganismObservationAttributes::CreatedBy,
                ),
            )?,
            created_at: builder.created_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableOrganismObservationAttributes::CreatedAt,
                ),
            )?,
            updated_by: builder.updated_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableOrganismObservationAttributes::UpdatedBy,
                ),
            )?,
            updated_at: builder.updated_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableOrganismObservationAttributes::UpdatedAt,
                ),
            )?,
        })
    }
}

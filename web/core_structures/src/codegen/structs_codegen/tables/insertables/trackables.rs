#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableTrackableAttributes {
    Id,
    Name,
    Description,
    PhotographId,
    ParentId,
    CreatedBy,
    CreatedAt,
    UpdatedBy,
    UpdatedAt,
}
impl core::fmt::Display for InsertableTrackableAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableTrackableAttributes::Id => write!(f, "id"),
            InsertableTrackableAttributes::Name => write!(f, "name"),
            InsertableTrackableAttributes::Description => write!(f, "description"),
            InsertableTrackableAttributes::PhotographId => write!(f, "photograph_id"),
            InsertableTrackableAttributes::ParentId => write!(f, "parent_id"),
            InsertableTrackableAttributes::CreatedBy => write!(f, "created_by"),
            InsertableTrackableAttributes::CreatedAt => write!(f, "created_at"),
            InsertableTrackableAttributes::UpdatedBy => write!(f, "updated_by"),
            InsertableTrackableAttributes::UpdatedAt => write!(f, "updated_at"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(table_name = crate::codegen::diesel_codegen::tables::trackables::trackables)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableTrackable {
    id: ::rosetta_uuid::Uuid,
    name: Option<String>,
    description: Option<String>,
    photograph_id: Option<::rosetta_uuid::Uuid>,
    parent_id: Option<::rosetta_uuid::Uuid>,
    created_by: i32,
    created_at: ::rosetta_timestamp::TimestampUTC,
    updated_by: i32,
    updated_at: ::rosetta_timestamp::TimestampUTC,
}
impl InsertableTrackable {
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
    pub fn parent<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::trackables::Trackable>,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::trackables::Trackable: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::trackables::Trackable as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::trackables::Trackable as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::trackables::Trackable as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::trackables::Trackable as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::trackables::Trackable as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::trackables::Trackable as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::trackables::Trackable,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        let Some(parent_id) = self.parent_id else {
            return Ok(None);
        };
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::trackables::Trackable::table(),
                parent_id,
            ),
            conn,
        )
        .map(Some)
    }
    pub fn photograph<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::documents::Document>,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::documents::Document: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::documents::Document as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::documents::Document as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::documents::Document as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::documents::Document as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::documents::Document as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::documents::Document as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::documents::Document,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        let Some(photograph_id) = self.photograph_id else {
            return Ok(None);
        };
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::documents::Document::table(),
                photograph_id,
            ),
            conn,
        )
        .map(Some)
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
pub struct InsertableTrackableBuilder {
    pub(crate) id: Option<::rosetta_uuid::Uuid>,
    pub(crate) name: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) photograph_id: Option<::rosetta_uuid::Uuid>,
    pub(crate) parent_id: Option<::rosetta_uuid::Uuid>,
    pub(crate) created_by: Option<i32>,
    pub(crate) created_at: Option<::rosetta_timestamp::TimestampUTC>,
    pub(crate) updated_by: Option<i32>,
    pub(crate) updated_at: Option<::rosetta_timestamp::TimestampUTC>,
}
impl Default for InsertableTrackableBuilder {
    fn default() -> Self {
        Self {
            id: Some(rosetta_uuid::Uuid::new_v4()),
            name: Default::default(),
            description: Default::default(),
            photograph_id: Default::default(),
            parent_id: Default::default(),
            created_by: Default::default(),
            created_at: Some(rosetta_timestamp::TimestampUTC::default()),
            updated_by: Default::default(),
            updated_at: Some(rosetta_timestamp::TimestampUTC::default()),
        }
    }
}
impl InsertableTrackableBuilder {
    pub fn id<P>(
        mut self,
        id: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableTrackableAttributes>>
    where
        P: TryInto<::rosetta_uuid::Uuid>,
        <P as TryInto<::rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let id = id.try_into().map_err(|err: <P as TryInto<::rosetta_uuid::Uuid>>::Error| {
            Into::into(err).rename_field(InsertableTrackableAttributes::Id)
        })?;
        if let Some(parent_id) = self.parent_id {
            pgrx_validation::must_be_distinct_uuid(id, parent_id).map_err(|e| {
                e.rename_fields(
                    InsertableTrackableAttributes::Id,
                    InsertableTrackableAttributes::ParentId,
                )
            })?;
        }
        self.id = Some(id);
        Ok(self)
    }
    pub fn name<P>(
        mut self,
        name: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableTrackableAttributes>>
    where
        P: TryInto<Option<String>>,
        <P as TryInto<Option<String>>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let name = name.try_into().map_err(|err: <P as TryInto<Option<String>>>::Error| {
            Into::into(err).rename_field(InsertableTrackableAttributes::Name)
        })?;
        if let (Some(name), Some(description)) = (name.as_ref(), self.description.as_ref()) {
            pgrx_validation::must_be_distinct(name, description).map_err(|e| {
                e.rename_fields(
                    InsertableTrackableAttributes::Name,
                    InsertableTrackableAttributes::Description,
                )
            })?;
        }
        self.name = name;
        Ok(self)
    }
    pub fn description<P>(
        mut self,
        description: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableTrackableAttributes>>
    where
        P: TryInto<Option<String>>,
        <P as TryInto<Option<String>>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let description =
            description.try_into().map_err(|err: <P as TryInto<Option<String>>>::Error| {
                Into::into(err).rename_field(InsertableTrackableAttributes::Description)
            })?;
        if let (Some(name), Some(description)) = (self.name.as_ref(), description.as_ref()) {
            pgrx_validation::must_be_distinct(name, description).map_err(|e| {
                e.rename_fields(
                    InsertableTrackableAttributes::Name,
                    InsertableTrackableAttributes::Description,
                )
            })?;
        }
        if let Some(description) = description.as_ref() {
            pgrx_validation::must_be_paragraph(description)
                .map_err(|e| e.rename_field(InsertableTrackableAttributes::Description))?;
        }
        if let Some(description) = description.as_ref() {
            pgrx_validation::must_be_paragraph(description)
                .map_err(|e| e.rename_field(InsertableTrackableAttributes::Description))?;
        }
        self.description = description;
        Ok(self)
    }
    pub fn photograph_id<P>(
        mut self,
        photograph_id: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableTrackableAttributes>>
    where
        P: TryInto<Option<::rosetta_uuid::Uuid>>,
        <P as TryInto<Option<::rosetta_uuid::Uuid>>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let photograph_id = photograph_id.try_into().map_err(
            |err: <P as TryInto<Option<::rosetta_uuid::Uuid>>>::Error| {
                Into::into(err).rename_field(InsertableTrackableAttributes::PhotographId)
            },
        )?;
        self.photograph_id = photograph_id;
        Ok(self)
    }
    pub fn parent_id<P>(
        mut self,
        parent_id: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableTrackableAttributes>>
    where
        P: TryInto<Option<::rosetta_uuid::Uuid>>,
        <P as TryInto<Option<::rosetta_uuid::Uuid>>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let parent_id = parent_id.try_into().map_err(
            |err: <P as TryInto<Option<::rosetta_uuid::Uuid>>>::Error| {
                Into::into(err).rename_field(InsertableTrackableAttributes::ParentId)
            },
        )?;
        if let (Some(parent_id), Some(id)) = (parent_id, self.id) {
            pgrx_validation::must_be_distinct_uuid(id, parent_id).map_err(|e| {
                e.rename_fields(
                    InsertableTrackableAttributes::Id,
                    InsertableTrackableAttributes::ParentId,
                )
            })?;
        }
        self.parent_id = parent_id;
        Ok(self)
    }
    pub fn created_by<P>(
        mut self,
        created_by: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableTrackableAttributes>>
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let created_by = created_by.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
            Into::into(err).rename_field(InsertableTrackableAttributes::CreatedBy)
        })?;
        self.created_by = Some(created_by);
        self = self.updated_by(created_by)?;
        Ok(self)
    }
    pub fn created_at<P>(
        mut self,
        created_at: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableTrackableAttributes>>
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let created_at = created_at.try_into().map_err(
            |err: <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error| {
                Into::into(err).rename_field(InsertableTrackableAttributes::CreatedAt)
            },
        )?;
        if let Some(updated_at) = self.updated_at {
            pgrx_validation::must_be_smaller_than_utc(created_at, updated_at).map_err(|e| {
                e.rename_fields(
                    InsertableTrackableAttributes::CreatedAt,
                    InsertableTrackableAttributes::UpdatedAt,
                )
            })?;
        }
        self.created_at = Some(created_at);
        Ok(self)
    }
    pub fn updated_by<P>(
        mut self,
        updated_by: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableTrackableAttributes>>
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let updated_by = updated_by.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
            Into::into(err).rename_field(InsertableTrackableAttributes::UpdatedBy)
        })?;
        self.updated_by = Some(updated_by);
        Ok(self)
    }
    pub fn updated_at<P>(
        mut self,
        updated_at: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableTrackableAttributes>>
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let updated_at = updated_at.try_into().map_err(
            |err: <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error| {
                Into::into(err).rename_field(InsertableTrackableAttributes::UpdatedAt)
            },
        )?;
        if let Some(created_at) = self.created_at {
            pgrx_validation::must_be_smaller_than_utc(created_at, updated_at).map_err(|e| {
                e.rename_fields(
                    InsertableTrackableAttributes::CreatedAt,
                    InsertableTrackableAttributes::UpdatedAt,
                )
            })?;
        }
        self.updated_at = Some(updated_at);
        Ok(self)
    }
}
impl TryFrom<InsertableTrackableBuilder> for InsertableTrackable {
    type Error = common_traits::prelude::BuilderError<InsertableTrackableAttributes>;
    fn try_from(builder: InsertableTrackableBuilder) -> Result<InsertableTrackable, Self::Error> {
        Ok(Self {
            id: builder.id.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableTrackableAttributes::Id,
            ))?,
            name: builder.name,
            description: builder.description,
            photograph_id: builder.photograph_id,
            parent_id: builder.parent_id,
            created_by: builder.created_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableTrackableAttributes::CreatedBy,
                ),
            )?,
            created_at: builder.created_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableTrackableAttributes::CreatedAt,
                ),
            )?,
            updated_by: builder.updated_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableTrackableAttributes::UpdatedBy,
                ),
            )?,
            updated_at: builder.updated_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableTrackableAttributes::UpdatedAt,
                ),
            )?,
        })
    }
}

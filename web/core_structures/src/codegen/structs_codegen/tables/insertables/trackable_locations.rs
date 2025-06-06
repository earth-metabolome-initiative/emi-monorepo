#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableTrackableLocationAttributes {
    Id,
    TrackableId,
    ContainerId,
    Geolocation,
    Inferred,
    CreatedAt,
    CreatedBy,
}
impl core::fmt::Display for InsertableTrackableLocationAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableTrackableLocationAttributes::Id => write!(f, "id"),
            InsertableTrackableLocationAttributes::TrackableId => {
                write!(f, "trackable_id")
            }
            InsertableTrackableLocationAttributes::ContainerId => {
                write!(f, "container_id")
            }
            InsertableTrackableLocationAttributes::Geolocation => {
                write!(f, "geolocation")
            }
            InsertableTrackableLocationAttributes::Inferred => write!(f, "inferred"),
            InsertableTrackableLocationAttributes::CreatedAt => write!(f, "created_at"),
            InsertableTrackableLocationAttributes::CreatedBy => write!(f, "created_by"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::trackable_locations::trackable_locations
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableTrackableLocation {
    id: ::rosetta_uuid::Uuid,
    trackable_id: ::rosetta_uuid::Uuid,
    container_id: Option<::rosetta_uuid::Uuid>,
    geolocation: postgis_diesel::types::Point,
    inferred: bool,
    created_at: ::rosetta_timestamp::TimestampUTC,
    created_by: i32,
}
impl InsertableTrackableLocation {
    pub fn container<C: diesel::connection::LoadConnection>(
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
        let Some(container_id) = self.container_id else {
            return Ok(None);
        };
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::trackables::Trackable::table(),
                container_id,
            ),
            conn,
        )
        .map(Some)
    }
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
    pub fn trackable<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::trackables::Trackable,
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
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::trackables::Trackable::table(),
                self.trackable_id,
            ),
            conn,
        )
    }
}
pub struct InsertableTrackableLocationBuilder {
    id: Option<::rosetta_uuid::Uuid>,
    trackable_id: Option<::rosetta_uuid::Uuid>,
    container_id: Option<::rosetta_uuid::Uuid>,
    geolocation: Option<postgis_diesel::types::Point>,
    inferred: Option<bool>,
    created_at: Option<::rosetta_timestamp::TimestampUTC>,
    created_by: Option<i32>,
}
impl Default for InsertableTrackableLocationBuilder {
    fn default() -> Self {
        Self {
            id: None,
            trackable_id: None,
            container_id: None,
            geolocation: None,
            inferred: Some(false),
            created_at: Some(rosetta_timestamp::TimestampUTC::default()),
            created_by: None,
        }
    }
}
impl InsertableTrackableLocationBuilder {
    pub fn id<P>(
        mut self,
        id: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableTrackableLocationAttributes>>
    where
        P: TryInto<::rosetta_uuid::Uuid>,
        <P as TryInto<::rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let id = id.try_into().map_err(|err: <P as TryInto<::rosetta_uuid::Uuid>>::Error| {
            Into::into(err).rename_field(InsertableTrackableLocationAttributes::Id)
        })?;
        self.id = Some(id);
        Ok(self)
    }
    pub fn trackable_id<P>(
        mut self,
        trackable_id: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableTrackableLocationAttributes>>
    where
        P: TryInto<::rosetta_uuid::Uuid>,
        <P as TryInto<::rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let trackable_id = trackable_id.try_into().map_err(
            |err: <P as TryInto<::rosetta_uuid::Uuid>>::Error| {
                Into::into(err).rename_field(InsertableTrackableLocationAttributes::TrackableId)
            },
        )?;
        self.trackable_id = Some(trackable_id);
        Ok(self)
    }
    pub fn container_id<P>(
        mut self,
        container_id: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableTrackableLocationAttributes>>
    where
        P: TryInto<Option<::rosetta_uuid::Uuid>>,
        <P as TryInto<Option<::rosetta_uuid::Uuid>>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let container_id = container_id.try_into().map_err(
            |err: <P as TryInto<Option<::rosetta_uuid::Uuid>>>::Error| {
                Into::into(err).rename_field(InsertableTrackableLocationAttributes::ContainerId)
            },
        )?;
        self.container_id = container_id;
        Ok(self)
    }
    pub fn geolocation<P>(
        mut self,
        geolocation: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableTrackableLocationAttributes>>
    where
        P: TryInto<postgis_diesel::types::Point>,
        <P as TryInto<postgis_diesel::types::Point>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let geolocation = geolocation.try_into().map_err(
            |err: <P as TryInto<postgis_diesel::types::Point>>::Error| {
                Into::into(err).rename_field(InsertableTrackableLocationAttributes::Geolocation)
            },
        )?;
        self.geolocation = Some(geolocation);
        Ok(self)
    }
    pub fn inferred<P>(
        mut self,
        inferred: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableTrackableLocationAttributes>>
    where
        P: TryInto<bool>,
        <P as TryInto<bool>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let inferred = inferred.try_into().map_err(|err: <P as TryInto<bool>>::Error| {
            Into::into(err).rename_field(InsertableTrackableLocationAttributes::Inferred)
        })?;
        self.inferred = Some(inferred);
        Ok(self)
    }
    pub fn created_at<P>(
        mut self,
        created_at: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableTrackableLocationAttributes>>
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let created_at = created_at.try_into().map_err(
            |err: <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error| {
                Into::into(err).rename_field(InsertableTrackableLocationAttributes::CreatedAt)
            },
        )?;
        self.created_at = Some(created_at);
        Ok(self)
    }
    pub fn created_by<P>(
        mut self,
        created_by: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableTrackableLocationAttributes>>
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let created_by = created_by.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
            Into::into(err).rename_field(InsertableTrackableLocationAttributes::CreatedBy)
        })?;
        self.created_by = Some(created_by);
        Ok(self)
    }
}
impl TryFrom<InsertableTrackableLocationBuilder> for InsertableTrackableLocation {
    type Error = common_traits::prelude::BuilderError<InsertableTrackableLocationAttributes>;
    fn try_from(
        builder: InsertableTrackableLocationBuilder,
    ) -> Result<InsertableTrackableLocation, Self::Error> {
        Ok(Self {
            id: builder.id.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableTrackableLocationAttributes::Id,
            ))?,
            trackable_id: builder.trackable_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableTrackableLocationAttributes::TrackableId,
                ),
            )?,
            container_id: builder.container_id,
            geolocation: builder.geolocation.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableTrackableLocationAttributes::Geolocation,
                ),
            )?,
            inferred: builder.inferred.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableTrackableLocationAttributes::Inferred,
                ),
            )?,
            created_at: builder.created_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableTrackableLocationAttributes::CreatedAt,
                ),
            )?,
            created_by: builder.created_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableTrackableLocationAttributes::CreatedBy,
                ),
            )?,
        })
    }
}

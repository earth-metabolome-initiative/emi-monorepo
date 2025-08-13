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
impl core::str::FromStr for InsertableTrackableLocationAttributes {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Id" => Ok(Self::Id),
            "TrackableId" => Ok(Self::TrackableId),
            "ContainerId" => Ok(Self::ContainerId),
            "Geolocation" => Ok(Self::Geolocation),
            "Inferred" => Ok(Self::Inferred),
            "CreatedAt" => Ok(Self::CreatedAt),
            "CreatedBy" => Ok(Self::CreatedBy),
            "id" => Ok(Self::Id),
            "trackable_id" => Ok(Self::TrackableId),
            "container_id" => Ok(Self::ContainerId),
            "geolocation" => Ok(Self::Geolocation),
            "inferred" => Ok(Self::Inferred),
            "created_at" => Ok(Self::CreatedAt),
            "created_by" => Ok(Self::CreatedBy),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl core::fmt::Display for InsertableTrackableLocationAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Id => write!(f, "id"),
            Self::TrackableId => write!(f, "trackable_id"),
            Self::ContainerId => write!(f, "container_id"),
            Self::Geolocation => write!(f, "geolocation"),
            Self::Inferred => write!(f, "inferred"),
            Self::CreatedAt => write!(f, "created_at"),
            Self::CreatedBy => write!(f, "created_by"),
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
    pub(crate) id: ::rosetta_uuid::Uuid,
    pub(crate) trackable_id: ::rosetta_uuid::Uuid,
    pub(crate) container_id: Option<::rosetta_uuid::Uuid>,
    pub(crate) geolocation: postgis_diesel::types::Point,
    pub(crate) inferred: bool,
    pub(crate) created_at: ::rosetta_timestamp::TimestampUTC,
    pub(crate) created_by: i32,
}
impl InsertableTrackableLocation {
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
}
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableTrackableLocationBuilder {
    pub(crate) id: Option<::rosetta_uuid::Uuid>,
    pub(crate) trackable_id: Option<::rosetta_uuid::Uuid>,
    pub(crate) container_id: Option<::rosetta_uuid::Uuid>,
    pub(crate) geolocation: Option<postgis_diesel::types::Point>,
    pub(crate) inferred: Option<bool>,
    pub(crate) created_at: Option<::rosetta_timestamp::TimestampUTC>,
    pub(crate) created_by: Option<i32>,
}
impl Default for InsertableTrackableLocationBuilder {
    fn default() -> Self {
        Self {
            id: Default::default(),
            trackable_id: Default::default(),
            container_id: Default::default(),
            geolocation: Default::default(),
            inferred: Some(false),
            created_at: Some(rosetta_timestamp::TimestampUTC::default()),
            created_by: Default::default(),
        }
    }
}
impl web_common_traits::database::ExtendableBuilder for InsertableTrackableLocationBuilder {
    type Attributes = InsertableTrackableLocationAttributes;
    fn extend_builder(
        mut self,
        other: Self,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let Some(id) = other.id {
            self = self.id(id)?;
        }
        if let Some(trackable_id) = other.trackable_id {
            self = self.trackable(trackable_id)?;
        }
        if let Some(container_id) = other.container_id {
            self = self.container(Some(container_id))?;
        }
        if let Some(geolocation) = other.geolocation {
            self = self.geolocation(geolocation)?;
        }
        if let Some(inferred) = other.inferred {
            self = self.inferred(inferred)?;
        }
        if let Some(created_at) = other.created_at {
            self = self.created_at(created_at)?;
        }
        if let Some(created_by) = other.created_by {
            self = self.created_by(created_by)?;
        }
        Ok(self)
    }
}
impl web_common_traits::prelude::SetPrimaryKey for InsertableTrackableLocationBuilder {
    type PrimaryKey = ::rosetta_uuid::Uuid;
    fn set_primary_key(self, _primary_key: Self::PrimaryKey) -> Self {
        self
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableTrackableLocationBuilder {
    /// Sets the value of the `trackable_locations.container_id` column from
    /// table `trackable_locations`.
    pub fn container(
        mut self,
        container_id: Option<::rosetta_uuid::Uuid>,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableTrackableLocationAttributes>>
    {
        self.container_id = container_id;
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableTrackableLocationBuilder {
    /// Sets the value of the `trackable_locations.created_at` column from table
    /// `trackable_locations`.
    pub fn created_at<CreatedAt>(
        mut self,
        created_at: CreatedAt,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableTrackableLocationAttributes>>
    where
        CreatedAt: TryInto<::rosetta_timestamp::TimestampUTC>,
        <CreatedAt as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let created_at = created_at.try_into().map_err(
            |err: <CreatedAt as TryInto<::rosetta_timestamp::TimestampUTC>>::Error| {
                Into::into(err).rename_field(InsertableTrackableLocationAttributes::CreatedAt)
            },
        )?;
        self.created_at = Some(created_at);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableTrackableLocationBuilder {
    /// Sets the value of the `trackable_locations.created_by` column from table
    /// `trackable_locations`.
    pub fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableTrackableLocationAttributes>>
    {
        self.created_by = Some(created_by);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableTrackableLocationBuilder {
    /// Sets the value of the `trackable_locations.geolocation` column from
    /// table `trackable_locations`.
    pub fn geolocation<Geolocation>(
        mut self,
        geolocation: Geolocation,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableTrackableLocationAttributes>>
    where
        Geolocation: TryInto<postgis_diesel::types::Point>,
        <Geolocation as TryInto<postgis_diesel::types::Point>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let geolocation = geolocation.try_into().map_err(
            |err: <Geolocation as TryInto<postgis_diesel::types::Point>>::Error| {
                Into::into(err).rename_field(InsertableTrackableLocationAttributes::Geolocation)
            },
        )?;
        self.geolocation = Some(geolocation);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableTrackableLocationBuilder {
    /// Sets the value of the `trackable_locations.id` column from table
    /// `trackable_locations`.
    pub fn id<Id>(
        mut self,
        id: Id,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableTrackableLocationAttributes>>
    where
        Id: TryInto<::rosetta_uuid::Uuid>,
        <Id as TryInto<::rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let id = id.try_into().map_err(|err: <Id as TryInto<::rosetta_uuid::Uuid>>::Error| {
            Into::into(err).rename_field(InsertableTrackableLocationAttributes::Id)
        })?;
        self.id = Some(id);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableTrackableLocationBuilder {
    /// Sets the value of the `trackable_locations.inferred` column from table
    /// `trackable_locations`.
    pub fn inferred<Inferred>(
        mut self,
        inferred: Inferred,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableTrackableLocationAttributes>>
    where
        Inferred: TryInto<bool>,
        <Inferred as TryInto<bool>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let inferred = inferred.try_into().map_err(|err: <Inferred as TryInto<bool>>::Error| {
            Into::into(err).rename_field(InsertableTrackableLocationAttributes::Inferred)
        })?;
        self.inferred = Some(inferred);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableTrackableLocationBuilder {
    /// Sets the value of the `trackable_locations.trackable_id` column from
    /// table `trackable_locations`.
    pub fn trackable(
        mut self,
        trackable_id: ::rosetta_uuid::Uuid,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableTrackableLocationAttributes>>
    {
        self.trackable_id = Some(trackable_id);
        Ok(self)
    }
}
impl<C> web_common_traits::database::TryInsertGeneric<C> for InsertableTrackableLocationBuilder
where
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::trackable_locations::TrackableLocation,
            Error = web_common_traits::database::InsertError<InsertableTrackableLocationAttributes>,
        >,
{
    type Attributes = InsertableTrackableLocationAttributes;
    fn is_complete(&self) -> bool {
        self.id.is_some()
            && self.trackable_id.is_some()
            && self.geolocation.is_some()
            && self.inferred.is_some()
            && self.created_at.is_some()
            && self.created_by.is_some()
    }
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<Self::Attributes>> {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::trackable_locations::TrackableLocation = self
            .insert(user_id, conn)?;
        Ok(insertable.id())
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableTrackableLocationAttributes {
    Id,
    TrackableId,
    StorageContainerId,
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
            InsertableTrackableLocationAttributes::StorageContainerId => {
                write!(f, "storage_container_id")
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
    id: rosetta_uuid::Uuid,
    trackable_id: rosetta_uuid::Uuid,
    storage_container_id: Option<rosetta_uuid::Uuid>,
    geolocation: postgis_diesel::types::Point,
    inferred: bool,
    created_at: rosetta_timestamp::TimestampUTC,
    created_by: i32,
}
impl InsertableTrackableLocation {
    #[cfg(feature = "postgres")]
    pub async fn trackable(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::trackables::Trackable, diesel::result::Error>
    {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::trackables::Trackable::table()
            .filter(
                crate::codegen::diesel_codegen::tables::trackables::trackables::dsl::id
                    .eq(&self.trackable_id),
            )
            .first::<crate::codegen::structs_codegen::tables::trackables::Trackable>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn storage_container(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::storage_containers::StorageContainer>,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        let Some(storage_container_id) = self.storage_container_id.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::storage_containers::StorageContainer::table()
            .filter(
                crate::codegen::diesel_codegen::tables::storage_containers::storage_containers::dsl::id
                    .eq(storage_container_id),
            )
            .first::<
                crate::codegen::structs_codegen::tables::storage_containers::StorageContainer,
            >(conn)
            .await
            .map(Some)
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
}
pub struct InsertableTrackableLocationBuilder {
    id: Option<rosetta_uuid::Uuid>,
    trackable_id: Option<rosetta_uuid::Uuid>,
    storage_container_id: Option<rosetta_uuid::Uuid>,
    geolocation: Option<postgis_diesel::types::Point>,
    inferred: Option<bool>,
    created_at: Option<rosetta_timestamp::TimestampUTC>,
    created_by: Option<i32>,
}
impl Default for InsertableTrackableLocationBuilder {
    fn default() -> Self {
        Self {
            id: None,
            trackable_id: None,
            storage_container_id: None,
            geolocation: None,
            inferred: Some(false),
            created_at: Some(rosetta_timestamp::TimestampUTC::default()),
            created_by: None,
        }
    }
}
impl InsertableTrackableLocationBuilder {
    pub fn id<P>(mut self, id: P) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<rosetta_uuid::Uuid>,
        <P as TryInto<rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let id = id.try_into().map_err(|err: <P as TryInto<rosetta_uuid::Uuid>>::Error| {
            Into::into(err).rename_field(InsertableTrackableLocationAttributes::Id)
        })?;
        self.id = Some(id);
        Ok(self)
    }
    pub fn trackable_id<P>(
        mut self,
        trackable_id: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<rosetta_uuid::Uuid>,
        <P as TryInto<rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let trackable_id =
            trackable_id.try_into().map_err(|err: <P as TryInto<rosetta_uuid::Uuid>>::Error| {
                Into::into(err).rename_field(InsertableTrackableLocationAttributes::TrackableId)
            })?;
        self.trackable_id = Some(trackable_id);
        Ok(self)
    }
    pub fn storage_container_id<P>(
        mut self,
        storage_container_id: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<Option<rosetta_uuid::Uuid>>,
        <P as TryInto<Option<rosetta_uuid::Uuid>>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let storage_container_id = storage_container_id.try_into().map_err(
            |err: <P as TryInto<Option<rosetta_uuid::Uuid>>>::Error| {
                Into::into(err)
                    .rename_field(InsertableTrackableLocationAttributes::StorageContainerId)
            },
        )?;
        self.storage_container_id = storage_container_id;
        Ok(self)
    }
    pub fn geolocation<P>(
        mut self,
        geolocation: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
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
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
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
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<rosetta_timestamp::TimestampUTC>,
        <P as TryInto<rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let created_at = created_at.try_into().map_err(
            |err: <P as TryInto<rosetta_timestamp::TimestampUTC>>::Error| {
                Into::into(err).rename_field(InsertableTrackableLocationAttributes::CreatedAt)
            },
        )?;
        self.created_at = Some(created_at);
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
            Into::into(err).rename_field(InsertableTrackableLocationAttributes::CreatedBy)
        })?;
        self.created_by = Some(created_by);
        Ok(self)
    }
}
impl common_traits::prelude::Builder for InsertableTrackableLocationBuilder {
    type Error = web_common_traits::database::InsertError<InsertableTrackableLocationAttributes>;
    type Object = InsertableTrackableLocation;
    type Attribute = InsertableTrackableLocationAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            id: self.id.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableTrackableLocationAttributes::Id,
            ))?,
            trackable_id: self.trackable_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableTrackableLocationAttributes::TrackableId,
                ),
            )?,
            storage_container_id: self.storage_container_id,
            geolocation: self.geolocation.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableTrackableLocationAttributes::Geolocation,
                ),
            )?,
            inferred: self.inferred.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableTrackableLocationAttributes::Inferred,
                ),
            )?,
            created_at: self.created_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableTrackableLocationAttributes::CreatedAt,
                ),
            )?,
            created_by: self.created_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableTrackableLocationAttributes::CreatedBy,
                ),
            )?,
        })
    }
}
impl TryFrom<InsertableTrackableLocation> for InsertableTrackableLocationBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableTrackableLocation) -> Result<Self, Self::Error> {
        Self::default()
            .id(insertable_variant.id)?
            .trackable_id(insertable_variant.trackable_id)?
            .storage_container_id(insertable_variant.storage_container_id)?
            .geolocation(insertable_variant.geolocation)?
            .inferred(insertable_variant.inferred)?
            .created_at(insertable_variant.created_at)?
            .created_by(insertable_variant.created_by)
    }
}

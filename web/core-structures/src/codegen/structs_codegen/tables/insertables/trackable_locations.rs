#[derive(Clone, core::fmt::Debug)]
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
#[derive(Default)]
pub struct InsertableTrackableLocationBuilder {
    id: Option<rosetta_uuid::Uuid>,
    trackable_id: Option<rosetta_uuid::Uuid>,
    storage_container_id: Option<rosetta_uuid::Uuid>,
    geolocation: Option<postgis_diesel::types::Point>,
    inferred: Option<bool>,
    created_at: Option<rosetta_timestamp::TimestampUTC>,
    created_by: Option<i32>,
}
impl InsertableTrackableLocationBuilder {
    pub fn id(
        mut self,
        id: rosetta_uuid::Uuid,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.id = Some(id);
        Ok(self)
    }
    pub fn trackable_id(
        mut self,
        trackable_id: rosetta_uuid::Uuid,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.trackable_id = Some(trackable_id);
        Ok(self)
    }
    pub fn storage_container_id(
        mut self,
        storage_container_id: Option<rosetta_uuid::Uuid>,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.storage_container_id = storage_container_id;
        Ok(self)
    }
    pub fn geolocation(
        mut self,
        geolocation: postgis_diesel::types::Point,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.geolocation = Some(geolocation);
        Ok(self)
    }
    pub fn inferred(
        mut self,
        inferred: bool,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.inferred = Some(inferred);
        Ok(self)
    }
    pub fn created_at(
        mut self,
        created_at: rosetta_timestamp::TimestampUTC,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.created_at = Some(created_at);
        Ok(self)
    }
    pub fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
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
            id: self.id.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableTrackableLocationAttributes::Id,
                )
            })?,
            trackable_id: self.trackable_id.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableTrackableLocationAttributes::TrackableId,
                )
            })?,
            storage_container_id: self.storage_container_id,
            geolocation: self.geolocation.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableTrackableLocationAttributes::Geolocation,
                )
            })?,
            inferred: self.inferred.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableTrackableLocationAttributes::Inferred,
                )
            })?,
            created_at: self.created_at.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableTrackableLocationAttributes::CreatedAt,
                )
            })?,
            created_by: self.created_by.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableTrackableLocationAttributes::CreatedBy,
                )
            })?,
        })
    }
}
impl TryFrom<InsertableTrackableLocation> for InsertableTrackableLocationBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableTrackableLocation) -> Result<Self, Self::Error> {
        Ok(Self::default()
            .id(insertable_variant.id)?
            .trackable_id(insertable_variant.trackable_id)?
            .storage_container_id(insertable_variant.storage_container_id)?
            .geolocation(insertable_variant.geolocation)?
            .inferred(insertable_variant.inferred)?
            .created_at(insertable_variant.created_at)?
            .created_by(insertable_variant.created_by)?)
    }
}

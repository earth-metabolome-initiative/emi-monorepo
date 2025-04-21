#[derive(Clone, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableSpectraCollectionAttributes {
    Id,
    Notes,
    TrackableId,
    CreatedBy,
    CreatedAt,
    UpdatedBy,
    UpdatedAt,
}
impl core::fmt::Display for InsertableSpectraCollectionAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableSpectraCollectionAttributes::Id => write!(f, "id"),
            InsertableSpectraCollectionAttributes::Notes => write!(f, "notes"),
            InsertableSpectraCollectionAttributes::TrackableId => {
                write!(f, "trackable_id")
            }
            InsertableSpectraCollectionAttributes::CreatedBy => write!(f, "created_by"),
            InsertableSpectraCollectionAttributes::CreatedAt => write!(f, "created_at"),
            InsertableSpectraCollectionAttributes::UpdatedBy => write!(f, "updated_by"),
            InsertableSpectraCollectionAttributes::UpdatedAt => write!(f, "updated_at"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::spectra_collections::spectra_collections
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableSpectraCollection {
    id: i32,
    notes: Option<String>,
    trackable_id: rosetta_uuid::Uuid,
    created_by: i32,
    created_at: rosetta_timestamp::TimestampUTC,
    updated_by: i32,
    updated_at: rosetta_timestamp::TimestampUTC,
}
impl InsertableSpectraCollection {
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
pub struct InsertableSpectraCollectionBuilder {
    id: Option<i32>,
    notes: Option<String>,
    trackable_id: Option<rosetta_uuid::Uuid>,
    created_by: Option<i32>,
    created_at: Option<rosetta_timestamp::TimestampUTC>,
    updated_by: Option<i32>,
    updated_at: Option<rosetta_timestamp::TimestampUTC>,
}
impl InsertableSpectraCollectionBuilder {
    pub fn id(mut self, id: i32) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.id = Some(id);
        Ok(self)
    }
    pub fn notes(
        mut self,
        notes: Option<String>,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.notes = notes;
        Ok(self)
    }
    pub fn trackable_id(
        mut self,
        trackable_id: rosetta_uuid::Uuid,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.trackable_id = Some(trackable_id);
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
impl common_traits::prelude::Builder for InsertableSpectraCollectionBuilder {
    type Error = web_common_traits::database::InsertError<InsertableSpectraCollectionAttributes>;
    type Object = InsertableSpectraCollection;
    type Attribute = InsertableSpectraCollectionAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            id: self.id.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableSpectraCollectionAttributes::Id,
            ))?,
            notes: self.notes,
            trackable_id: self.trackable_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableSpectraCollectionAttributes::TrackableId,
                ),
            )?,
            created_by: self.created_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableSpectraCollectionAttributes::CreatedBy,
                ),
            )?,
            created_at: self.created_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableSpectraCollectionAttributes::CreatedAt,
                ),
            )?,
            updated_by: self.updated_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableSpectraCollectionAttributes::UpdatedBy,
                ),
            )?,
            updated_at: self.updated_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableSpectraCollectionAttributes::UpdatedAt,
                ),
            )?,
        })
    }
}
impl TryFrom<InsertableSpectraCollection> for InsertableSpectraCollectionBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableSpectraCollection) -> Result<Self, Self::Error> {
        Self::default()
            .id(insertable_variant.id)?
            .notes(insertable_variant.notes)?
            .trackable_id(insertable_variant.trackable_id)?
            .created_by(insertable_variant.created_by)?
            .created_at(insertable_variant.created_at)?
            .updated_by(insertable_variant.updated_by)?
            .updated_at(insertable_variant.updated_at)?
    }
}

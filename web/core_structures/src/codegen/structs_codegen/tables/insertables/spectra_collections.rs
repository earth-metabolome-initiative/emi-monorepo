#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
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
            Self::Id => write!(f, "id"),
            Self::Notes => write!(f, "notes"),
            Self::TrackableId => write!(f, "trackable_id"),
            Self::CreatedBy => write!(f, "created_by"),
            Self::CreatedAt => write!(f, "created_at"),
            Self::UpdatedBy => write!(f, "updated_by"),
            Self::UpdatedAt => write!(f, "updated_at"),
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
    pub(crate) id: i32,
    pub(crate) notes: Option<String>,
    pub(crate) trackable_id: ::rosetta_uuid::Uuid,
    pub(crate) created_by: i32,
    pub(crate) created_at: ::rosetta_timestamp::TimestampUTC,
    pub(crate) updated_by: i32,
    pub(crate) updated_at: ::rosetta_timestamp::TimestampUTC,
}
impl InsertableSpectraCollection {
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
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableSpectraCollectionBuilder {
    pub(crate) id: Option<i32>,
    pub(crate) notes: Option<String>,
    pub(crate) trackable_id: Option<::rosetta_uuid::Uuid>,
    pub(crate) created_by: Option<i32>,
    pub(crate) created_at: Option<::rosetta_timestamp::TimestampUTC>,
    pub(crate) updated_by: Option<i32>,
    pub(crate) updated_at: Option<::rosetta_timestamp::TimestampUTC>,
}
impl Default for InsertableSpectraCollectionBuilder {
    fn default() -> Self {
        Self {
            id: Default::default(),
            notes: Default::default(),
            trackable_id: Default::default(),
            created_by: Default::default(),
            created_at: Some(rosetta_timestamp::TimestampUTC::default()),
            updated_by: Default::default(),
            updated_at: Some(rosetta_timestamp::TimestampUTC::default()),
        }
    }
}
impl web_common_traits::database::ExtendableBuilder for InsertableSpectraCollectionBuilder {
    type Attributes = InsertableSpectraCollectionAttributes;
    fn extend_builder(
        mut self,
        other: Self,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let Some(id) = other.id {
            self = self.id(id)?;
        }
        if let Some(notes) = other.notes {
            self = self.notes(Some(notes))?;
        }
        if let Some(trackable_id) = other.trackable_id {
            self = self.trackable(trackable_id)?;
        }
        if let Some(created_by) = other.created_by {
            self = self.created_by(created_by)?;
        }
        if let Some(created_at) = other.created_at {
            self = self.created_at(created_at)?;
        }
        if let Some(updated_by) = other.updated_by {
            self = self.updated_by(updated_by)?;
        }
        if let Some(updated_at) = other.updated_at {
            self = self.updated_at(updated_at)?;
        }
        Ok(self)
    }
}
impl web_common_traits::prelude::SetPrimaryKey for InsertableSpectraCollectionBuilder {
    type PrimaryKey = i32;
    fn set_primary_key(self, _primary_key: Self::PrimaryKey) -> Self {
        self
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableSpectraCollectionBuilder {
    /// Sets the value of the `spectra_collections.created_at` column from table
    /// `spectra_collections`.
    pub fn created_at<P>(
        mut self,
        created_at: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableSpectraCollectionAttributes>>
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let created_at = created_at.try_into().map_err(
            |err: <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error| {
                Into::into(err).rename_field(InsertableSpectraCollectionAttributes::CreatedAt)
            },
        )?;
        if let Some(updated_at) = self.updated_at {
            pgrx_validation::must_be_smaller_than_utc(created_at, updated_at).map_err(|e| {
                e.rename_fields(
                    InsertableSpectraCollectionAttributes::CreatedAt,
                    InsertableSpectraCollectionAttributes::UpdatedAt,
                )
            })?;
        }
        self.created_at = Some(created_at);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableSpectraCollectionBuilder {
    /// Sets the value of the `spectra_collections.created_by` column from table
    /// `spectra_collections`.
    pub fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableSpectraCollectionAttributes>>
    {
        self.created_by = Some(created_by);
        self = self.updated_by(created_by)?;
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableSpectraCollectionBuilder {
    /// Sets the value of the `spectra_collections.id` column from table
    /// `spectra_collections`.
    pub fn id<P>(
        mut self,
        id: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableSpectraCollectionAttributes>>
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let id = id.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
            Into::into(err).rename_field(InsertableSpectraCollectionAttributes::Id)
        })?;
        self.id = Some(id);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableSpectraCollectionBuilder {
    /// Sets the value of the `spectra_collections.notes` column from table
    /// `spectra_collections`.
    pub fn notes<P>(
        mut self,
        notes: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableSpectraCollectionAttributes>>
    where
        P: TryInto<Option<String>>,
        <P as TryInto<Option<String>>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let notes = notes.try_into().map_err(|err: <P as TryInto<Option<String>>>::Error| {
            Into::into(err).rename_field(InsertableSpectraCollectionAttributes::Notes)
        })?;
        self.notes = notes;
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableSpectraCollectionBuilder {
    /// Sets the value of the `spectra_collections.trackable_id` column from
    /// table `spectra_collections`.
    pub fn trackable(
        mut self,
        trackable_id: ::rosetta_uuid::Uuid,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableSpectraCollectionAttributes>>
    {
        self.trackable_id = Some(trackable_id);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableSpectraCollectionBuilder {
    /// Sets the value of the `spectra_collections.updated_at` column from table
    /// `spectra_collections`.
    pub fn updated_at<P>(
        mut self,
        updated_at: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableSpectraCollectionAttributes>>
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let updated_at = updated_at.try_into().map_err(
            |err: <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error| {
                Into::into(err).rename_field(InsertableSpectraCollectionAttributes::UpdatedAt)
            },
        )?;
        if let Some(created_at) = self.created_at {
            pgrx_validation::must_be_smaller_than_utc(created_at, updated_at).map_err(|e| {
                e.rename_fields(
                    InsertableSpectraCollectionAttributes::CreatedAt,
                    InsertableSpectraCollectionAttributes::UpdatedAt,
                )
            })?;
        }
        self.updated_at = Some(updated_at);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableSpectraCollectionBuilder {
    /// Sets the value of the `spectra_collections.updated_by` column from table
    /// `spectra_collections`.
    pub fn updated_by(
        mut self,
        updated_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableSpectraCollectionAttributes>>
    {
        self.updated_by = Some(updated_by);
        Ok(self)
    }
}
impl<C> web_common_traits::database::TryInsertGeneric<C> for InsertableSpectraCollectionBuilder
where
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection,
            Error = web_common_traits::database::InsertError<InsertableSpectraCollectionAttributes>,
        >,
{
    type Attributes = InsertableSpectraCollectionAttributes;
    fn is_complete(&self) -> bool {
        self.id.is_some()
            && self.trackable_id.is_some()
            && self.created_by.is_some()
            && self.created_at.is_some()
            && self.updated_by.is_some()
            && self.updated_at.is_some()
    }
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<Self::Attributes>> {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection = self
            .insert(user_id, conn)?;
        Ok(insertable.id())
    }
}

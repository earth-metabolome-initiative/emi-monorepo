#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableRoomAttributes {
    Id,
    Name,
    Description,
    Qrcode,
    AddressesId,
    Geolocation,
    CreatedBy,
    CreatedAt,
    UpdatedBy,
    UpdatedAt,
}
impl core::fmt::Display for InsertableRoomAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Id => write!(f, "id"),
            Self::Name => write!(f, "name"),
            Self::Description => write!(f, "description"),
            Self::Qrcode => write!(f, "qrcode"),
            Self::AddressesId => write!(f, "addresses_id"),
            Self::Geolocation => write!(f, "geolocation"),
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
    diesel(table_name = crate::codegen::diesel_codegen::tables::rooms::rooms)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableRoom {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) qrcode: ::rosetta_uuid::Uuid,
    pub(crate) addresses_id: i32,
    pub(crate) geolocation: postgis_diesel::types::Point,
    pub(crate) created_by: i32,
    pub(crate) created_at: ::rosetta_timestamp::TimestampUTC,
    pub(crate) updated_by: i32,
    pub(crate) updated_at: ::rosetta_timestamp::TimestampUTC,
}
impl InsertableRoom {
    pub fn addresses<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::addresses::Address,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::addresses::Address: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::addresses::Address as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::addresses::Address as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::addresses::Address as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::addresses::Address as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::addresses::Address as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::addresses::Address as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::addresses::Address,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::addresses::Address::table(),
                self.addresses_id,
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
pub struct InsertableRoomBuilder {
    pub(crate) name: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) qrcode: Option<::rosetta_uuid::Uuid>,
    pub(crate) addresses_id: Option<i32>,
    pub(crate) geolocation: Option<postgis_diesel::types::Point>,
    pub(crate) created_by: Option<i32>,
    pub(crate) created_at: Option<::rosetta_timestamp::TimestampUTC>,
    pub(crate) updated_by: Option<i32>,
    pub(crate) updated_at: Option<::rosetta_timestamp::TimestampUTC>,
}
impl Default for InsertableRoomBuilder {
    fn default() -> Self {
        Self {
            name: Default::default(),
            description: Default::default(),
            qrcode: Default::default(),
            addresses_id: Default::default(),
            geolocation: Default::default(),
            created_by: Default::default(),
            created_at: Some(rosetta_timestamp::TimestampUTC::default()),
            updated_by: Default::default(),
            updated_at: Some(rosetta_timestamp::TimestampUTC::default()),
        }
    }
}
impl web_common_traits::database::ExtendableBuilder for InsertableRoomBuilder {
    type Attributes = InsertableRoomAttributes;
    fn extend_builder(
        mut self,
        other: Self,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let Some(name) = other.name {
            self = self.name(name)?;
        }
        if let Some(description) = other.description {
            self = self.description(description)?;
        }
        if let Some(qrcode) = other.qrcode {
            self = self.qrcode(qrcode)?;
        }
        if let Some(addresses_id) = other.addresses_id {
            self = self.addresses(addresses_id)?;
        }
        if let Some(geolocation) = other.geolocation {
            self = self.geolocation(geolocation)?;
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
impl web_common_traits::prelude::SetPrimaryKey for InsertableRoomBuilder {
    type PrimaryKey = i32;
    fn set_primary_key(self, _primary_key: Self::PrimaryKey) -> Self {
        self
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableRoomBuilder {
    /// Sets the value of the `rooms.addresses_id` column from table `rooms`.
    pub fn addresses(
        mut self,
        addresses_id: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableRoomAttributes>> {
        self.addresses_id = Some(addresses_id);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableRoomBuilder {
    /// Sets the value of the `rooms.created_at` column from table `rooms`.
    pub fn created_at<P>(
        mut self,
        created_at: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableRoomAttributes>>
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let created_at = created_at.try_into().map_err(
            |err: <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error| {
                Into::into(err).rename_field(InsertableRoomAttributes::CreatedAt)
            },
        )?;
        if let Some(updated_at) = self.updated_at {
            pgrx_validation::must_be_smaller_than_utc(created_at, updated_at).map_err(|e| {
                e.rename_fields(
                    InsertableRoomAttributes::CreatedAt,
                    InsertableRoomAttributes::UpdatedAt,
                )
            })?;
        }
        self.created_at = Some(created_at);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableRoomBuilder {
    /// Sets the value of the `rooms.created_by` column from table `rooms`.
    pub fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableRoomAttributes>> {
        self.created_by = Some(created_by);
        self = self.updated_by(created_by)?;
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableRoomBuilder {
    /// Sets the value of the `rooms.description` column from table `rooms`.
    pub fn description<P>(
        mut self,
        description: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableRoomAttributes>>
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let description =
            description.try_into().map_err(|err: <P as TryInto<String>>::Error| {
                Into::into(err).rename_field(InsertableRoomAttributes::Description)
            })?;
        pgrx_validation::must_be_paragraph(description.as_ref())
            .map_err(|e| e.rename_field(InsertableRoomAttributes::Description))?;
        self.description = Some(description);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableRoomBuilder {
    /// Sets the value of the `rooms.geolocation` column from table `rooms`.
    pub fn geolocation<P>(
        mut self,
        geolocation: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableRoomAttributes>>
    where
        P: TryInto<postgis_diesel::types::Point>,
        <P as TryInto<postgis_diesel::types::Point>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let geolocation = geolocation.try_into().map_err(
            |err: <P as TryInto<postgis_diesel::types::Point>>::Error| {
                Into::into(err).rename_field(InsertableRoomAttributes::Geolocation)
            },
        )?;
        self.geolocation = Some(geolocation);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableRoomBuilder {
    /// Sets the value of the `rooms.name` column from table `rooms`.
    pub fn name<P>(
        mut self,
        name: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableRoomAttributes>>
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let name = name.try_into().map_err(|err: <P as TryInto<String>>::Error| {
            Into::into(err).rename_field(InsertableRoomAttributes::Name)
        })?;
        pgrx_validation::must_be_paragraph(name.as_ref())
            .map_err(|e| e.rename_field(InsertableRoomAttributes::Name))?;
        self.name = Some(name);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableRoomBuilder {
    /// Sets the value of the `rooms.qrcode` column from table `rooms`.
    pub fn qrcode<P>(
        mut self,
        qrcode: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableRoomAttributes>>
    where
        P: TryInto<::rosetta_uuid::Uuid>,
        <P as TryInto<::rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let qrcode =
            qrcode.try_into().map_err(|err: <P as TryInto<::rosetta_uuid::Uuid>>::Error| {
                Into::into(err).rename_field(InsertableRoomAttributes::Qrcode)
            })?;
        self.qrcode = Some(qrcode);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableRoomBuilder {
    /// Sets the value of the `rooms.updated_at` column from table `rooms`.
    pub fn updated_at<P>(
        mut self,
        updated_at: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableRoomAttributes>>
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let updated_at = updated_at.try_into().map_err(
            |err: <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error| {
                Into::into(err).rename_field(InsertableRoomAttributes::UpdatedAt)
            },
        )?;
        if let Some(created_at) = self.created_at {
            pgrx_validation::must_be_smaller_than_utc(created_at, updated_at).map_err(|e| {
                e.rename_fields(
                    InsertableRoomAttributes::CreatedAt,
                    InsertableRoomAttributes::UpdatedAt,
                )
            })?;
        }
        self.updated_at = Some(updated_at);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableRoomBuilder {
    /// Sets the value of the `rooms.updated_by` column from table `rooms`.
    pub fn updated_by(
        mut self,
        updated_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableRoomAttributes>> {
        self.updated_by = Some(updated_by);
        Ok(self)
    }
}
impl<C> web_common_traits::database::TryInsertGeneric<C> for InsertableRoomBuilder
where
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::rooms::Room,
            Error = web_common_traits::database::InsertError<InsertableRoomAttributes>,
        >,
{
    type Attributes = InsertableRoomAttributes;
    fn is_complete(&self) -> bool {
        self.name.is_some()
            && self.description.is_some()
            && self.qrcode.is_some()
            && self.addresses_id.is_some()
            && self.geolocation.is_some()
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
        let insertable: crate::codegen::structs_codegen::tables::rooms::Room =
            self.insert(user_id, conn)?;
        Ok(insertable.id())
    }
}

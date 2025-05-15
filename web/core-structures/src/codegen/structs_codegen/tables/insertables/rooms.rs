#[derive(Clone, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableRoomAttributes {
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
            InsertableRoomAttributes::Name => write!(f, "name"),
            InsertableRoomAttributes::Description => write!(f, "description"),
            InsertableRoomAttributes::Qrcode => write!(f, "qrcode"),
            InsertableRoomAttributes::AddressesId => write!(f, "addresses_id"),
            InsertableRoomAttributes::Geolocation => write!(f, "geolocation"),
            InsertableRoomAttributes::CreatedBy => write!(f, "created_by"),
            InsertableRoomAttributes::CreatedAt => write!(f, "created_at"),
            InsertableRoomAttributes::UpdatedBy => write!(f, "updated_by"),
            InsertableRoomAttributes::UpdatedAt => write!(f, "updated_at"),
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
    name: String,
    description: String,
    qrcode: rosetta_uuid::Uuid,
    addresses_id: i32,
    geolocation: postgis_diesel::types::Point,
    created_by: i32,
    created_at: rosetta_timestamp::TimestampUTC,
    updated_by: i32,
    updated_at: rosetta_timestamp::TimestampUTC,
}
impl InsertableRoom {
    #[cfg(feature = "postgres")]
    pub async fn addresses(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::addresses::Address, diesel::result::Error>
    {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::addresses::Address::table()
            .filter(
                crate::codegen::diesel_codegen::tables::addresses::addresses::dsl::id
                    .eq(&self.addresses_id),
            )
            .first::<crate::codegen::structs_codegen::tables::addresses::Address>(conn)
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
pub struct InsertableRoomBuilder {
    name: Option<String>,
    description: Option<String>,
    qrcode: Option<rosetta_uuid::Uuid>,
    addresses_id: Option<i32>,
    geolocation: Option<postgis_diesel::types::Point>,
    created_by: Option<i32>,
    created_at: Option<rosetta_timestamp::TimestampUTC>,
    updated_by: Option<i32>,
    updated_at: Option<rosetta_timestamp::TimestampUTC>,
}
impl Default for InsertableRoomBuilder {
    fn default() -> Self {
        Self {
            name: None,
            description: None,
            qrcode: None,
            addresses_id: None,
            geolocation: None,
            created_by: None,
            created_at: Some(rosetta_timestamp::TimestampUTC::default()),
            updated_by: None,
            updated_at: Some(rosetta_timestamp::TimestampUTC::default()),
        }
    }
}
impl InsertableRoomBuilder {
    pub fn name<P>(
        mut self,
        name: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
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
    pub fn description<P>(
        mut self,
        description: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
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
    pub fn qrcode<P>(
        mut self,
        qrcode: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<rosetta_uuid::Uuid>,
        <P as TryInto<rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let qrcode =
            qrcode.try_into().map_err(|err: <P as TryInto<rosetta_uuid::Uuid>>::Error| {
                Into::into(err).rename_field(InsertableRoomAttributes::Qrcode)
            })?;
        self.qrcode = Some(qrcode);
        Ok(self)
    }
    pub fn addresses_id<P>(
        mut self,
        addresses_id: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let addresses_id = addresses_id.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
            Into::into(err).rename_field(InsertableRoomAttributes::AddressesId)
        })?;
        self.addresses_id = Some(addresses_id);
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
                Into::into(err).rename_field(InsertableRoomAttributes::Geolocation)
            },
        )?;
        self.geolocation = Some(geolocation);
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
            Into::into(err).rename_field(InsertableRoomAttributes::CreatedBy)
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
                Into::into(err).rename_field(InsertableRoomAttributes::CreatedAt)
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
            Into::into(err).rename_field(InsertableRoomAttributes::UpdatedBy)
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
                Into::into(err).rename_field(InsertableRoomAttributes::UpdatedAt)
            },
        )?;
        self.updated_at = Some(updated_at);
        Ok(self)
    }
}
impl common_traits::prelude::Builder for InsertableRoomBuilder {
    type Error = web_common_traits::database::InsertError<InsertableRoomAttributes>;
    type Object = InsertableRoom;
    type Attribute = InsertableRoomAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            name: self.name.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableRoomAttributes::Name,
            ))?,
            description: self.description.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableRoomAttributes::Description,
                ),
            )?,
            qrcode: self.qrcode.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableRoomAttributes::Qrcode,
            ))?,
            addresses_id: self.addresses_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableRoomAttributes::AddressesId,
                ),
            )?,
            geolocation: self.geolocation.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableRoomAttributes::Geolocation,
                ),
            )?,
            created_by: self.created_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableRoomAttributes::CreatedBy,
                ),
            )?,
            created_at: self.created_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableRoomAttributes::CreatedAt,
                ),
            )?,
            updated_by: self.updated_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableRoomAttributes::UpdatedBy,
                ),
            )?,
            updated_at: self.updated_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableRoomAttributes::UpdatedAt,
                ),
            )?,
        })
    }
}
impl TryFrom<InsertableRoom> for InsertableRoomBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableRoom) -> Result<Self, Self::Error> {
        Self::default()
            .name(insertable_variant.name)?
            .description(insertable_variant.description)?
            .qrcode(insertable_variant.qrcode)?
            .addresses_id(insertable_variant.addresses_id)?
            .geolocation(insertable_variant.geolocation)?
            .created_by(insertable_variant.created_by)?
            .created_at(insertable_variant.created_at)?
            .updated_by(insertable_variant.updated_by)?
            .updated_at(insertable_variant.updated_at)
    }
}

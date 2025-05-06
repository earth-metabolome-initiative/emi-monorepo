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
    pub fn name<P: Into<String>>(
        mut self,
        name: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let name = name.into();
        self.name = Some(name);
        Ok(self)
    }
    pub fn description<P: Into<String>>(
        mut self,
        description: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let description = description.into();
        self.description = Some(description);
        Ok(self)
    }
    pub fn qrcode<P: Into<rosetta_uuid::Uuid>>(
        mut self,
        qrcode: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let qrcode = qrcode.into();
        self.qrcode = Some(qrcode);
        Ok(self)
    }
    pub fn addresses_id<P: Into<i32>>(
        mut self,
        addresses_id: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let addresses_id = addresses_id.into();
        self.addresses_id = Some(addresses_id);
        Ok(self)
    }
    pub fn geolocation<P: Into<postgis_diesel::types::Point>>(
        mut self,
        geolocation: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let geolocation = geolocation.into();
        self.geolocation = Some(geolocation);
        Ok(self)
    }
    pub fn created_by<P: Into<i32>>(
        mut self,
        created_by: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let created_by = created_by.into();
        self.created_by = Some(created_by);
        self = self.updated_by(created_by)?;
        Ok(self)
    }
    pub fn created_at<P: Into<rosetta_timestamp::TimestampUTC>>(
        mut self,
        created_at: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let created_at = created_at.into();
        self.created_at = Some(created_at);
        Ok(self)
    }
    pub fn updated_by<P: Into<i32>>(
        mut self,
        updated_by: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let updated_by = updated_by.into();
        self.updated_by = Some(updated_by);
        Ok(self)
    }
    pub fn updated_at<P: Into<rosetta_timestamp::TimestampUTC>>(
        mut self,
        updated_at: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let updated_at = updated_at.into();
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

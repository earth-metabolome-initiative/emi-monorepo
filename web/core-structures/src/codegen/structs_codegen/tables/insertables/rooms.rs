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
#[derive(Default)]
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
impl InsertableRoomBuilder {
    pub fn name(
        mut self,
        name: String,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.name = Some(name);
        Ok(self)
    }
    pub fn description(
        mut self,
        description: String,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.description = Some(description);
        Ok(self)
    }
    pub fn qrcode(
        mut self,
        qrcode: rosetta_uuid::Uuid,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.qrcode = Some(qrcode);
        Ok(self)
    }
    pub fn addresses_id(
        mut self,
        addresses_id: i32,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.addresses_id = Some(addresses_id);
        Ok(self)
    }
    pub fn geolocation(
        mut self,
        geolocation: postgis_diesel::types::Point,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.geolocation = Some(geolocation);
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
            .updated_at(insertable_variant.updated_at)?
    }
}

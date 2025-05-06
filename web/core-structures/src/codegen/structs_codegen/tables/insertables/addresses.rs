#[derive(Clone, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableAddressAttributes {
    CityId,
    StreetName,
    StreetNumber,
    PostalCode,
    Geolocation,
}
impl core::fmt::Display for InsertableAddressAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableAddressAttributes::CityId => write!(f, "city_id"),
            InsertableAddressAttributes::StreetName => write!(f, "street_name"),
            InsertableAddressAttributes::StreetNumber => write!(f, "street_number"),
            InsertableAddressAttributes::PostalCode => write!(f, "postal_code"),
            InsertableAddressAttributes::Geolocation => write!(f, "geolocation"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(table_name = crate::codegen::diesel_codegen::tables::addresses::addresses)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableAddress {
    city_id: i32,
    street_name: String,
    street_number: String,
    postal_code: String,
    geolocation: postgis_diesel::types::Point,
}
impl InsertableAddress {
    #[cfg(feature = "postgres")]
    pub async fn city(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::cities::City, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::cities::City::table()
            .filter(
                crate::codegen::diesel_codegen::tables::cities::cities::dsl::id.eq(&self.city_id),
            )
            .first::<crate::codegen::structs_codegen::tables::cities::City>(conn)
            .await
    }
}
#[derive(Default)]
pub struct InsertableAddressBuilder {
    city_id: Option<i32>,
    street_name: Option<String>,
    street_number: Option<String>,
    postal_code: Option<String>,
    geolocation: Option<postgis_diesel::types::Point>,
}
impl InsertableAddressBuilder {
    pub fn city_id<P: Into<i32>>(
        mut self,
        city_id: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let city_id = city_id.into();
        self.city_id = Some(city_id);
        Ok(self)
    }
    pub fn street_name<P: Into<String>>(
        mut self,
        street_name: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let street_name = street_name.into();
        self.street_name = Some(street_name);
        Ok(self)
    }
    pub fn street_number<P: Into<String>>(
        mut self,
        street_number: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let street_number = street_number.into();
        self.street_number = Some(street_number);
        Ok(self)
    }
    pub fn postal_code<P: Into<String>>(
        mut self,
        postal_code: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let postal_code = postal_code.into();
        self.postal_code = Some(postal_code);
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
}
impl common_traits::prelude::Builder for InsertableAddressBuilder {
    type Error = web_common_traits::database::InsertError<InsertableAddressAttributes>;
    type Object = InsertableAddress;
    type Attribute = InsertableAddressAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            city_id: self.city_id.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableAddressAttributes::CityId,
            ))?,
            street_name: self.street_name.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableAddressAttributes::StreetName,
                ),
            )?,
            street_number: self.street_number.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableAddressAttributes::StreetNumber,
                ),
            )?,
            postal_code: self.postal_code.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableAddressAttributes::PostalCode,
                ),
            )?,
            geolocation: self.geolocation.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableAddressAttributes::Geolocation,
                ),
            )?,
        })
    }
}
impl TryFrom<InsertableAddress> for InsertableAddressBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableAddress) -> Result<Self, Self::Error> {
        Self::default()
            .city_id(insertable_variant.city_id)?
            .street_name(insertable_variant.street_name)?
            .street_number(insertable_variant.street_number)?
            .postal_code(insertable_variant.postal_code)?
            .geolocation(insertable_variant.geolocation)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableSpatialRefSyAttributes {
    Srid,
    AuthName,
    AuthSrid,
    Srtext,
    Proj4text,
}
impl core::fmt::Display for InsertableSpatialRefSyAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableSpatialRefSyAttributes::Srid => write!(f, "srid"),
            InsertableSpatialRefSyAttributes::AuthName => write!(f, "auth_name"),
            InsertableSpatialRefSyAttributes::AuthSrid => write!(f, "auth_srid"),
            InsertableSpatialRefSyAttributes::Srtext => write!(f, "srtext"),
            InsertableSpatialRefSyAttributes::Proj4text => write!(f, "proj4text"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::spatial_ref_sys::spatial_ref_sys
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableSpatialRefSy {
    srid: i32,
    auth_name: Option<String>,
    auth_srid: Option<i32>,
    srtext: Option<String>,
    proj4text: Option<String>,
}
impl InsertableSpatialRefSy {}
#[derive(Default)]
pub struct InsertableSpatialRefSyBuilder {
    srid: Option<i32>,
    auth_name: Option<String>,
    auth_srid: Option<i32>,
    srtext: Option<String>,
    proj4text: Option<String>,
}
impl InsertableSpatialRefSyBuilder {
    pub fn srid<P>(
        mut self,
        srid: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let srid = srid.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
            Into::into(err).rename_field(InsertableSpatialRefSyAttributes::Srid)
        })?;
        self.srid = Some(srid);
        Ok(self)
    }
    pub fn auth_name<P>(
        mut self,
        auth_name: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<Option<String>>,
        <P as TryInto<Option<String>>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let auth_name =
            auth_name.try_into().map_err(|err: <P as TryInto<Option<String>>>::Error| {
                Into::into(err).rename_field(InsertableSpatialRefSyAttributes::AuthName)
            })?;
        self.auth_name = auth_name;
        Ok(self)
    }
    pub fn auth_srid<P>(
        mut self,
        auth_srid: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<Option<i32>>,
        <P as TryInto<Option<i32>>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let auth_srid =
            auth_srid.try_into().map_err(|err: <P as TryInto<Option<i32>>>::Error| {
                Into::into(err).rename_field(InsertableSpatialRefSyAttributes::AuthSrid)
            })?;
        self.auth_srid = auth_srid;
        Ok(self)
    }
    pub fn srtext<P>(
        mut self,
        srtext: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<Option<String>>,
        <P as TryInto<Option<String>>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let srtext = srtext.try_into().map_err(|err: <P as TryInto<Option<String>>>::Error| {
            Into::into(err).rename_field(InsertableSpatialRefSyAttributes::Srtext)
        })?;
        self.srtext = srtext;
        Ok(self)
    }
    pub fn proj4text<P>(
        mut self,
        proj4text: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<Option<String>>,
        <P as TryInto<Option<String>>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let proj4text =
            proj4text.try_into().map_err(|err: <P as TryInto<Option<String>>>::Error| {
                Into::into(err).rename_field(InsertableSpatialRefSyAttributes::Proj4text)
            })?;
        self.proj4text = proj4text;
        Ok(self)
    }
}
impl common_traits::prelude::Builder for InsertableSpatialRefSyBuilder {
    type Error = web_common_traits::database::InsertError<InsertableSpatialRefSyAttributes>;
    type Object = InsertableSpatialRefSy;
    type Attribute = InsertableSpatialRefSyAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            srid: self.srid.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableSpatialRefSyAttributes::Srid,
            ))?,
            auth_name: self.auth_name,
            auth_srid: self.auth_srid,
            srtext: self.srtext,
            proj4text: self.proj4text,
        })
    }
}
impl TryFrom<InsertableSpatialRefSy> for InsertableSpatialRefSyBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableSpatialRefSy) -> Result<Self, Self::Error> {
        Self::default()
            .srid(insertable_variant.srid)?
            .auth_name(insertable_variant.auth_name)?
            .auth_srid(insertable_variant.auth_srid)?
            .srtext(insertable_variant.srtext)?
            .proj4text(insertable_variant.proj4text)
    }
}

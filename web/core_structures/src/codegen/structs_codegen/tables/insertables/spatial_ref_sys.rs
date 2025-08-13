#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableSpatialRefSyAttributes {
    Srid,
    AuthName,
    AuthSrid,
    Srtext,
    Proj4text,
}
impl core::str::FromStr for InsertableSpatialRefSyAttributes {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Srid" => Ok(Self::Srid),
            "AuthName" => Ok(Self::AuthName),
            "AuthSrid" => Ok(Self::AuthSrid),
            "Srtext" => Ok(Self::Srtext),
            "Proj4text" => Ok(Self::Proj4text),
            "srid" => Ok(Self::Srid),
            "auth_name" => Ok(Self::AuthName),
            "auth_srid" => Ok(Self::AuthSrid),
            "srtext" => Ok(Self::Srtext),
            "proj4text" => Ok(Self::Proj4text),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl core::fmt::Display for InsertableSpatialRefSyAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Srid => write!(f, "srid"),
            Self::AuthName => write!(f, "auth_name"),
            Self::AuthSrid => write!(f, "auth_srid"),
            Self::Srtext => write!(f, "srtext"),
            Self::Proj4text => write!(f, "proj4text"),
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
    pub(crate) srid: i32,
    pub(crate) auth_name: Option<String>,
    pub(crate) auth_srid: Option<i32>,
    pub(crate) srtext: Option<String>,
    pub(crate) proj4text: Option<String>,
}
impl InsertableSpatialRefSy {}
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableSpatialRefSyBuilder {
    pub(crate) srid: Option<i32>,
    pub(crate) auth_name: Option<String>,
    pub(crate) auth_srid: Option<i32>,
    pub(crate) srtext: Option<String>,
    pub(crate) proj4text: Option<String>,
}
impl web_common_traits::database::ExtendableBuilder for InsertableSpatialRefSyBuilder {
    type Attributes = InsertableSpatialRefSyAttributes;
    fn extend_builder(
        mut self,
        other: Self,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let Some(srid) = other.srid {
            self = self.srid(srid)?;
        }
        if let Some(auth_name) = other.auth_name {
            self = self.auth_name(Some(auth_name))?;
        }
        if let Some(auth_srid) = other.auth_srid {
            self = self.auth_srid(Some(auth_srid))?;
        }
        if let Some(srtext) = other.srtext {
            self = self.srtext(Some(srtext))?;
        }
        if let Some(proj4text) = other.proj4text {
            self = self.proj4text(Some(proj4text))?;
        }
        Ok(self)
    }
}
impl web_common_traits::prelude::SetPrimaryKey for InsertableSpatialRefSyBuilder {
    type PrimaryKey = i32;
    fn set_primary_key(self, _primary_key: Self::PrimaryKey) -> Self {
        self
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableSpatialRefSyBuilder {
    /// Sets the value of the `spatial_ref_sys.auth_name` column from table
    /// `spatial_ref_sys`.
    pub fn auth_name<AuthName>(
        mut self,
        auth_name: AuthName,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableSpatialRefSyAttributes>>
    where
        AuthName: TryInto<Option<String>>,
        <AuthName as TryInto<Option<String>>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let auth_name =
            auth_name.try_into().map_err(|err: <AuthName as TryInto<Option<String>>>::Error| {
                Into::into(err).rename_field(InsertableSpatialRefSyAttributes::AuthName)
            })?;
        self.auth_name = auth_name;
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableSpatialRefSyBuilder {
    /// Sets the value of the `spatial_ref_sys.auth_srid` column from table
    /// `spatial_ref_sys`.
    pub fn auth_srid<AuthSrid>(
        mut self,
        auth_srid: AuthSrid,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableSpatialRefSyAttributes>>
    where
        AuthSrid: TryInto<Option<i32>>,
        <AuthSrid as TryInto<Option<i32>>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let auth_srid =
            auth_srid.try_into().map_err(|err: <AuthSrid as TryInto<Option<i32>>>::Error| {
                Into::into(err).rename_field(InsertableSpatialRefSyAttributes::AuthSrid)
            })?;
        self.auth_srid = auth_srid;
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableSpatialRefSyBuilder {
    /// Sets the value of the `spatial_ref_sys.proj4text` column from table
    /// `spatial_ref_sys`.
    pub fn proj4text<Proj4text>(
        mut self,
        proj4text: Proj4text,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableSpatialRefSyAttributes>>
    where
        Proj4text: TryInto<Option<String>>,
        <Proj4text as TryInto<Option<String>>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let proj4text = proj4text.try_into().map_err(
            |err: <Proj4text as TryInto<Option<String>>>::Error| {
                Into::into(err).rename_field(InsertableSpatialRefSyAttributes::Proj4text)
            },
        )?;
        self.proj4text = proj4text;
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableSpatialRefSyBuilder {
    /// Sets the value of the `spatial_ref_sys.srid` column from table
    /// `spatial_ref_sys`.
    pub fn srid<Srid>(
        mut self,
        srid: Srid,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableSpatialRefSyAttributes>>
    where
        Srid: TryInto<i32>,
        <Srid as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let srid = srid.try_into().map_err(|err: <Srid as TryInto<i32>>::Error| {
            Into::into(err).rename_field(InsertableSpatialRefSyAttributes::Srid)
        })?;
        self.srid = Some(srid);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableSpatialRefSyBuilder {
    /// Sets the value of the `spatial_ref_sys.srtext` column from table
    /// `spatial_ref_sys`.
    pub fn srtext<Srtext>(
        mut self,
        srtext: Srtext,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableSpatialRefSyAttributes>>
    where
        Srtext: TryInto<Option<String>>,
        <Srtext as TryInto<Option<String>>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let srtext =
            srtext.try_into().map_err(|err: <Srtext as TryInto<Option<String>>>::Error| {
                Into::into(err).rename_field(InsertableSpatialRefSyAttributes::Srtext)
            })?;
        self.srtext = srtext;
        Ok(self)
    }
}
impl<C> web_common_traits::database::TryInsertGeneric<C> for InsertableSpatialRefSyBuilder
where
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::spatial_ref_sys::SpatialRefSy,
            Error = web_common_traits::database::InsertError<InsertableSpatialRefSyAttributes>,
        >,
{
    type Attributes = InsertableSpatialRefSyAttributes;
    fn is_complete(&self) -> bool {
        self.srid.is_some()
    }
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<Self::Attributes>> {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::spatial_ref_sys::SpatialRefSy =
            self.insert(user_id, conn)?;
        Ok(insertable.id())
    }
}

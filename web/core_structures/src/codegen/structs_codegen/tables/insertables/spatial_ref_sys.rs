#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SpatialRefSyAttribute {
    Srid,
    AuthName,
    AuthSrid,
    Srtext,
    Proj4text,
}
impl core::str::FromStr for SpatialRefSyAttribute {
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
impl core::fmt::Display for SpatialRefSyAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Srid => write!(f, "spatial_ref_sys.srid"),
            Self::AuthName => write!(f, "spatial_ref_sys.auth_name"),
            Self::AuthSrid => write!(f, "spatial_ref_sys.auth_srid"),
            Self::Srtext => write!(f, "spatial_ref_sys.srtext"),
            Self::Proj4text => write!(f, "spatial_ref_sys.proj4text"),
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
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Hash, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Builder for creating and inserting a new
/// [`SpatialRefSy`](crate::codegen::structs_codegen::tables::spatial_ref_sys::SpatialRefSy).
///
///
/// # Implementation details
/// While this builder implements several methods, a reasonably complete
/// **basic** usage example (*which may not apply to your own specific use case,
/// please adapt accordingly*) is as follows:
///
/// ```rust,ignore
/// use core_structures::SpatialRefSy;
/// use core_structures::tables::insertables::SpatialRefSySettable;
/// use web_common_traits::database::Insertable;
/// use web_common_traits::database::InsertableVariant;
///
/// let spatial_ref_sy = SpatialRefSy::new()
///    // Set mandatory fields
///    .srid(srid)?
///    // Optionally set optional fields
///    .auth_name(auth_name)?
///    .auth_srid(auth_srid)?
///    .proj4text(proj4text)?
///    .srtext(srtext)?
///    // Finally, insert the new record in the database
///    .insert(user.id, conn)?;
/// ```
pub struct InsertableSpatialRefSyBuilder {
    pub(crate) srid: Option<i32>,
    pub(crate) auth_name: Option<String>,
    pub(crate) auth_srid: Option<i32>,
    pub(crate) srtext: Option<String>,
    pub(crate) proj4text: Option<String>,
}
impl From<InsertableSpatialRefSyBuilder>
    for web_common_traits::database::IdOrBuilder<i32, InsertableSpatialRefSyBuilder>
{
    fn from(builder: InsertableSpatialRefSyBuilder) -> Self {
        Self::Builder(builder)
    }
}
impl common_traits::builder::IsCompleteBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableSpatialRefSyBuilder
{
    fn is_complete(&self) -> bool {
        self.srid.is_some()
    }
}
/// Trait defining setters for attributes of an instance of `SpatialRefSy` or
/// descendant tables.
pub trait SpatialRefSySettable: Sized {
    /// Attributes required to build the insertable.
    type Attributes;
    /// Sets the value of the `public.spatial_ref_sys.srid` column.
    ///
    /// # Arguments
    /// * `srid`: The value to set for the `public.spatial_ref_sys.srid` column.
    ///
    /// # Implementation details
    /// This method accepts a reference to a generic value which can be
    /// converted to the required type for the column. This allows passing
    /// values of different types, as long as they can be converted to the
    /// required type using the `TryFrom` trait. The method, additionally,
    /// employs same-as and inferred same-as rules to ensure that the
    /// schema-defined ancestral tables and associated table values associated
    /// to the current column (if any) are also set appropriately.
    ///
    /// # Errors
    /// * If the provided value cannot be converted to the required type `i32`.
    /// * If the provided value does not pass schema-defined validation.
    fn srid<S>(
        self,
        srid: S,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        S: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the `public.spatial_ref_sys.auth_name` column.
    ///
    /// # Arguments
    /// * `auth_name`: The value to set for the
    ///   `public.spatial_ref_sys.auth_name` column.
    ///
    /// # Implementation details
    /// This method accepts a reference to a generic value which can be
    /// converted to the required type for the column. This allows passing
    /// values of different types, as long as they can be converted to the
    /// required type using the `TryFrom` trait. The method, additionally,
    /// employs same-as and inferred same-as rules to ensure that the
    /// schema-defined ancestral tables and associated table values associated
    /// to the current column (if any) are also set appropriately.
    ///
    /// # Errors
    /// * If the provided value cannot be converted to the required type
    ///   `String`.
    /// * If the provided value does not pass schema-defined validation.
    fn auth_name<AN>(
        self,
        auth_name: AN,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        AN: TryInto<Option<String>>,
        validation_errors::SingleFieldError: From<<AN as TryInto<Option<String>>>::Error>;
    /// Sets the value of the `public.spatial_ref_sys.auth_srid` column.
    ///
    /// # Arguments
    /// * `auth_srid`: The value to set for the
    ///   `public.spatial_ref_sys.auth_srid` column.
    ///
    /// # Implementation details
    /// This method accepts a reference to a generic value which can be
    /// converted to the required type for the column. This allows passing
    /// values of different types, as long as they can be converted to the
    /// required type using the `TryFrom` trait. The method, additionally,
    /// employs same-as and inferred same-as rules to ensure that the
    /// schema-defined ancestral tables and associated table values associated
    /// to the current column (if any) are also set appropriately.
    ///
    /// # Errors
    /// * If the provided value cannot be converted to the required type `i32`.
    /// * If the provided value does not pass schema-defined validation.
    fn auth_srid<AS>(
        self,
        auth_srid: AS,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        AS: TryInto<Option<i32>>,
        validation_errors::SingleFieldError: From<<AS as TryInto<Option<i32>>>::Error>;
    /// Sets the value of the `public.spatial_ref_sys.srtext` column.
    ///
    /// # Arguments
    /// * `srtext`: The value to set for the `public.spatial_ref_sys.srtext`
    ///   column.
    ///
    /// # Implementation details
    /// This method accepts a reference to a generic value which can be
    /// converted to the required type for the column. This allows passing
    /// values of different types, as long as they can be converted to the
    /// required type using the `TryFrom` trait. The method, additionally,
    /// employs same-as and inferred same-as rules to ensure that the
    /// schema-defined ancestral tables and associated table values associated
    /// to the current column (if any) are also set appropriately.
    ///
    /// # Errors
    /// * If the provided value cannot be converted to the required type
    ///   `String`.
    /// * If the provided value does not pass schema-defined validation.
    fn srtext<S>(
        self,
        srtext: S,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        S: TryInto<Option<String>>,
        validation_errors::SingleFieldError: From<<S as TryInto<Option<String>>>::Error>;
    /// Sets the value of the `public.spatial_ref_sys.proj4text` column.
    ///
    /// # Arguments
    /// * `proj4text`: The value to set for the
    ///   `public.spatial_ref_sys.proj4text` column.
    ///
    /// # Implementation details
    /// This method accepts a reference to a generic value which can be
    /// converted to the required type for the column. This allows passing
    /// values of different types, as long as they can be converted to the
    /// required type using the `TryFrom` trait. The method, additionally,
    /// employs same-as and inferred same-as rules to ensure that the
    /// schema-defined ancestral tables and associated table values associated
    /// to the current column (if any) are also set appropriately.
    ///
    /// # Errors
    /// * If the provided value cannot be converted to the required type
    ///   `String`.
    /// * If the provided value does not pass schema-defined validation.
    fn proj4text<P>(
        self,
        proj4text: P,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        P: TryInto<Option<String>>,
        validation_errors::SingleFieldError: From<<P as TryInto<Option<String>>>::Error>;
}
impl SpatialRefSySettable for InsertableSpatialRefSyBuilder {
    type Attributes = crate::codegen::structs_codegen::tables::insertables::SpatialRefSyAttribute;
    /// Sets the value of the `public.spatial_ref_sys.srid` column.
    fn srid<S>(
        mut self,
        srid: S,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        S: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let srid = <S as web_common_traits::database::PrimaryKeyLike>::primary_key(&srid);
        self.srid = Some(srid);
        Ok(self)
    }
    /// Sets the value of the `public.spatial_ref_sys.auth_name` column.
    fn auth_name<AN>(
        mut self,
        auth_name: AN,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        AN: TryInto<Option<String>>,
        validation_errors::SingleFieldError: From<<AN as TryInto<Option<String>>>::Error>,
    {
        let auth_name = auth_name.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(SpatialRefSyAttribute::AuthName)
        })?;
        self.auth_name = auth_name;
        Ok(self)
    }
    /// Sets the value of the `public.spatial_ref_sys.auth_srid` column.
    fn auth_srid<AS>(
        mut self,
        auth_srid: AS,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        AS: TryInto<Option<i32>>,
        validation_errors::SingleFieldError: From<<AS as TryInto<Option<i32>>>::Error>,
    {
        let auth_srid = auth_srid.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(SpatialRefSyAttribute::AuthSrid)
        })?;
        self.auth_srid = auth_srid;
        Ok(self)
    }
    /// Sets the value of the `public.spatial_ref_sys.srtext` column.
    fn srtext<S>(
        mut self,
        srtext: S,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        S: TryInto<Option<String>>,
        validation_errors::SingleFieldError: From<<S as TryInto<Option<String>>>::Error>,
    {
        let srtext = srtext.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(SpatialRefSyAttribute::Srtext)
        })?;
        self.srtext = srtext;
        Ok(self)
    }
    /// Sets the value of the `public.spatial_ref_sys.proj4text` column.
    fn proj4text<P>(
        mut self,
        proj4text: P,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        P: TryInto<Option<String>>,
        validation_errors::SingleFieldError: From<<P as TryInto<Option<String>>>::Error>,
    {
        let proj4text = proj4text.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(SpatialRefSyAttribute::Proj4text)
        })?;
        self.proj4text = proj4text;
        Ok(self)
    }
}
impl web_common_traits::prelude::SetPrimaryKey for InsertableSpatialRefSyBuilder {
    type PrimaryKey = i32;
    fn set_primary_key(self, _primary_key: Self::PrimaryKey) -> Self {
        self
    }
}
impl<C> web_common_traits::database::TryInsertGeneric<C> for InsertableSpatialRefSyBuilder
where
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::spatial_ref_sys::SpatialRefSy,
            Error = web_common_traits::database::InsertError<SpatialRefSyAttribute>,
        >,
{
    type Attribute = SpatialRefSyAttribute;
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<Self::Attribute>> {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::spatial_ref_sys::SpatialRefSy =
            self.insert(user_id, conn)?;
        Ok(insertable.id())
    }
}

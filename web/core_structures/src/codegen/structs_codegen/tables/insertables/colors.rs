#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ColorAttribute {
    Name,
    HexadecimalValue,
    Description,
    Id,
}
impl core::str::FromStr for ColorAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Name" => Ok(Self::Name),
            "HexadecimalValue" => Ok(Self::HexadecimalValue),
            "Description" => Ok(Self::Description),
            "name" => Ok(Self::Name),
            "hexadecimal_value" => Ok(Self::HexadecimalValue),
            "description" => Ok(Self::Description),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl common_traits::builder::Attributed
    for crate::codegen::structs_codegen::tables::insertables::InsertableColorBuilder
{
    type Attribute = ColorAttribute;
}
impl core::fmt::Display for ColorAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Name => write!(f, "colors.name"),
            Self::HexadecimalValue => write!(f, "colors.hexadecimal_value"),
            Self::Description => write!(f, "colors.description"),
            Self::Id => write!(f, "colors.id"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(table_name = crate::codegen::diesel_codegen::tables::colors::colors)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableColor {
    pub(crate) name: String,
    pub(crate) hexadecimal_value: String,
    pub(crate) description: String,
}
impl InsertableColor {}
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Hash, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Builder for creating and inserting a new
/// [`Color`](crate::codegen::structs_codegen::tables::colors::Color).
///
/// # Implementation details
/// While this builder implements several methods, a reasonably complete
/// **basic** usage example (*which may not apply to your own specific use case,
/// please adapt accordingly*) is as follows:
///
/// ```rust,ignore
/// use core_structures::Color;
/// use core_structures::tables::insertables::ColorSettable;
/// use web_common_traits::database::Insertable;
/// use web_common_traits::database::InsertableVariant;
///
/// let color = Color::new()
///    // Set mandatory fields
///    .description(description)?
///    .hexadecimal_value(hexadecimal_value)?
///    .name(name)?
///    // Finally, insert the new record in the database
///    .insert(user.id, conn)?;
/// ```
pub struct InsertableColorBuilder {
    pub(crate) name: Option<String>,
    pub(crate) hexadecimal_value: Option<String>,
    pub(crate) description: Option<String>,
}
impl From<InsertableColorBuilder>
    for web_common_traits::database::IdOrBuilder<i16, InsertableColorBuilder>
{
    fn from(builder: InsertableColorBuilder) -> Self {
        Self::Builder(builder)
    }
}
impl common_traits::builder::IsCompleteBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableColorBuilder
{
    fn is_complete(&self) -> bool {
        self.name.is_some() && self.hexadecimal_value.is_some() && self.description.is_some()
    }
}
/// Trait defining setters for attributes of an instance of `Color` or
/// descendant tables.
pub trait ColorSettable: Sized {
    /// Attributes required to build the insertable.
    type Attributes;
    /// Sets the value of the `public.colors.name` column.
    ///
    /// # Arguments
    /// * `name`: The value to set for the `public.colors.name` column.
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
    fn name<N>(
        self,
        name: N,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        N: TryInto<String>,
        validation_errors::SingleFieldError: From<<N as TryInto<String>>::Error>;
    /// Sets the value of the `public.colors.hexadecimal_value` column.
    ///
    /// # Arguments
    /// * `hexadecimal_value`: The value to set for the
    ///   `public.colors.hexadecimal_value` column.
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
    fn hexadecimal_value<HV>(
        self,
        hexadecimal_value: HV,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        HV: TryInto<String>,
        validation_errors::SingleFieldError: From<<HV as TryInto<String>>::Error>;
    /// Sets the value of the `public.colors.description` column.
    ///
    /// # Arguments
    /// * `description`: The value to set for the `public.colors.description`
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
    fn description<D>(
        self,
        description: D,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        D: TryInto<String>,
        validation_errors::SingleFieldError: From<<D as TryInto<String>>::Error>;
}
impl ColorSettable for InsertableColorBuilder {
    type Attributes = crate::codegen::structs_codegen::tables::insertables::ColorAttribute;
    /// Sets the value of the `public.colors.name` column.
    fn name<N>(
        mut self,
        name: N,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        N: TryInto<String>,
        validation_errors::SingleFieldError: From<<N as TryInto<String>>::Error>,
    {
        let name = name.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err).rename_field(ColorAttribute::Name)
        })?;
        self.name = Some(name);
        Ok(self)
    }
    /// Sets the value of the `public.colors.hexadecimal_value` column.
    fn hexadecimal_value<HV>(
        mut self,
        hexadecimal_value: HV,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        HV: TryInto<String>,
        validation_errors::SingleFieldError: From<<HV as TryInto<String>>::Error>,
    {
        let hexadecimal_value = hexadecimal_value.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(ColorAttribute::HexadecimalValue)
        })?;
        self.hexadecimal_value = Some(hexadecimal_value);
        Ok(self)
    }
    /// Sets the value of the `public.colors.description` column.
    fn description<D>(
        mut self,
        description: D,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        D: TryInto<String>,
        validation_errors::SingleFieldError: From<<D as TryInto<String>>::Error>,
    {
        let description = description.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err).rename_field(ColorAttribute::Description)
        })?;
        self.description = Some(description);
        Ok(self)
    }
}
impl web_common_traits::prelude::SetPrimaryKey for InsertableColorBuilder {
    type PrimaryKey = i16;
    fn set_primary_key(self, _primary_key: Self::PrimaryKey) -> Self {
        self
    }
}
impl<C> web_common_traits::database::TryInsertGeneric<C> for InsertableColorBuilder
where
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::colors::Color,
            Attribute = ColorAttribute,
        >,
{
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<Self::Attribute>> {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::colors::Color =
            self.insert(user_id, conn)?;
        Ok(insertable.id())
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableColorAttributes {
    Name,
    HexadecimalValue,
    Description,
    Id,
}
impl core::str::FromStr for InsertableColorAttributes {
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
impl core::fmt::Display for InsertableColorAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Name => write!(f, "name"),
            Self::HexadecimalValue => write!(f, "hexadecimal_value"),
            Self::Description => write!(f, "description"),
            Self::Id => write!(f, "id"),
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
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableColorBuilder {
    pub(crate) name: Option<String>,
    pub(crate) hexadecimal_value: Option<String>,
    pub(crate) description: Option<String>,
}
impl web_common_traits::database::ExtendableBuilder for InsertableColorBuilder {
    type Attributes = InsertableColorAttributes;
    fn extend_builder(
        mut self,
        other: Self,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let Some(name) = other.name {
            self = self.name(name)?;
        }
        if let Some(hexadecimal_value) = other.hexadecimal_value {
            self = self.hexadecimal_value(hexadecimal_value)?;
        }
        if let Some(description) = other.description {
            self = self.description(description)?;
        }
        Ok(self)
    }
}
impl web_common_traits::prelude::SetPrimaryKey for InsertableColorBuilder {
    type PrimaryKey = i16;
    fn set_primary_key(self, _primary_key: Self::PrimaryKey) -> Self {
        self
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableColorBuilder {
    /// Sets the value of the `colors.description` column from table `colors`.
    pub fn description<Description>(
        mut self,
        description: Description,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableColorAttributes>>
    where
        Description: TryInto<String>,
        <Description as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let description =
            description.try_into().map_err(|err: <Description as TryInto<String>>::Error| {
                Into::into(err).rename_field(InsertableColorAttributes::Description)
            })?;
        self.description = Some(description);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableColorBuilder {
    /// Sets the value of the `colors.hexadecimal_value` column from table
    /// `colors`.
    pub fn hexadecimal_value<HexadecimalValue>(
        mut self,
        hexadecimal_value: HexadecimalValue,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableColorAttributes>>
    where
        HexadecimalValue: TryInto<String>,
        <HexadecimalValue as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let hexadecimal_value = hexadecimal_value.try_into().map_err(
            |err: <HexadecimalValue as TryInto<String>>::Error| {
                Into::into(err).rename_field(InsertableColorAttributes::HexadecimalValue)
            },
        )?;
        self.hexadecimal_value = Some(hexadecimal_value);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableColorBuilder {
    /// Sets the value of the `colors.name` column from table `colors`.
    pub fn name<Name>(
        mut self,
        name: Name,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableColorAttributes>>
    where
        Name: TryInto<String>,
        <Name as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let name = name.try_into().map_err(|err: <Name as TryInto<String>>::Error| {
            Into::into(err).rename_field(InsertableColorAttributes::Name)
        })?;
        self.name = Some(name);
        Ok(self)
    }
}
impl<C> web_common_traits::database::TryInsertGeneric<C> for InsertableColorBuilder
where
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::colors::Color,
            Error = web_common_traits::database::InsertError<InsertableColorAttributes>,
        >,
{
    type Attributes = InsertableColorAttributes;
    fn is_complete(&self) -> bool {
        self.name.is_some() && self.hexadecimal_value.is_some() && self.description.is_some()
    }
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<Self::Attributes>> {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::colors::Color =
            self.insert(user_id, conn)?;
        Ok(insertable.id())
    }
}

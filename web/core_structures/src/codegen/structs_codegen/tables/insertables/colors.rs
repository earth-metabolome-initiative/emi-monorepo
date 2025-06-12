#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableColorAttributes {
    Name,
    HexadecimalValue,
    Description,
}
impl core::fmt::Display for InsertableColorAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableColorAttributes::Name => write!(f, "name"),
            InsertableColorAttributes::HexadecimalValue => write!(f, "hexadecimal_value"),
            InsertableColorAttributes::Description => write!(f, "description"),
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
    name: String,
    hexadecimal_value: String,
    description: String,
}
impl InsertableColor {}
#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableColorBuilder {
    pub(crate) name: Option<String>,
    pub(crate) hexadecimal_value: Option<String>,
    pub(crate) description: Option<String>,
}
impl InsertableColorBuilder {
    pub fn name<P>(
        mut self,
        name: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableColorAttributes>>
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let name = name.try_into().map_err(|err: <P as TryInto<String>>::Error| {
            Into::into(err).rename_field(InsertableColorAttributes::Name)
        })?;
        self.name = Some(name);
        Ok(self)
    }
    pub fn hexadecimal_value<P>(
        mut self,
        hexadecimal_value: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableColorAttributes>>
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let hexadecimal_value =
            hexadecimal_value.try_into().map_err(|err: <P as TryInto<String>>::Error| {
                Into::into(err).rename_field(InsertableColorAttributes::HexadecimalValue)
            })?;
        self.hexadecimal_value = Some(hexadecimal_value);
        Ok(self)
    }
    pub fn description<P>(
        mut self,
        description: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableColorAttributes>>
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let description =
            description.try_into().map_err(|err: <P as TryInto<String>>::Error| {
                Into::into(err).rename_field(InsertableColorAttributes::Description)
            })?;
        self.description = Some(description);
        Ok(self)
    }
}
impl TryFrom<InsertableColorBuilder> for InsertableColor {
    type Error = common_traits::prelude::BuilderError<InsertableColorAttributes>;
    fn try_from(builder: InsertableColorBuilder) -> Result<InsertableColor, Self::Error> {
        Ok(Self {
            name: builder.name.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableColorAttributes::Name,
            ))?,
            hexadecimal_value: builder.hexadecimal_value.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableColorAttributes::HexadecimalValue,
                ),
            )?,
            description: builder.description.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableColorAttributes::Description,
                ),
            )?,
        })
    }
}

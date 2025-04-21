#[derive(Clone, core::fmt::Debug)]
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
#[derive(Default)]
pub struct InsertableColorBuilder {
    name: Option<String>,
    hexadecimal_value: Option<String>,
    description: Option<String>,
}
impl InsertableColorBuilder {
    pub fn name(
        mut self,
        name: String,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.name = Some(name);
        Ok(self)
    }
    pub fn hexadecimal_value(
        mut self,
        hexadecimal_value: String,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.hexadecimal_value = Some(hexadecimal_value);
        Ok(self)
    }
    pub fn description(
        mut self,
        description: String,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.description = Some(description);
        Ok(self)
    }
}
impl common_traits::prelude::Builder for InsertableColorBuilder {
    type Error = web_common_traits::database::InsertError<InsertableColorAttributes>;
    type Object = InsertableColor;
    type Attribute = InsertableColorAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            name: self.name.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableColorAttributes::Name,
            ))?,
            hexadecimal_value: self.hexadecimal_value.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableColorAttributes::HexadecimalValue,
                ),
            )?,
            description: self.description.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableColorAttributes::Description,
                ),
            )?,
        })
    }
}
impl TryFrom<InsertableColor> for InsertableColorBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableColor) -> Result<Self, Self::Error> {
        Self::default()
            .name(insertable_variant.name)?
            .hexadecimal_value(insertable_variant.hexadecimal_value)?
            .description(insertable_variant.description)
    }
}

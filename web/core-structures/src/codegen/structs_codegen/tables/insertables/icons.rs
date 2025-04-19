#[derive(Clone, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableIconAttributes {
    Name,
    Description,
}
impl core::fmt::Display for InsertableIconAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableIconAttributes::Name => write!(f, "name"),
            InsertableIconAttributes::Description => write!(f, "description"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(table_name = crate::codegen::diesel_codegen::tables::icons::icons)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableIcon {
    name: String,
    description: String,
}
impl InsertableIcon {}
#[derive(Default)]
pub struct InsertableIconBuilder {
    name: Option<String>,
    description: Option<String>,
}
impl InsertableIconBuilder {
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
}
impl common_traits::prelude::Builder for InsertableIconBuilder {
    type Error = web_common_traits::database::InsertError<InsertableIconAttributes>;
    type Object = InsertableIcon;
    type Attribute = InsertableIconAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            name: self.name.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableIconAttributes::Name,
                )
            })?,
            description: self.description.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableIconAttributes::Description,
                )
            })?,
        })
    }
}
impl TryFrom<InsertableIcon> for InsertableIconBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableIcon) -> Result<Self, Self::Error> {
        Ok(Self::default()
            .name(insertable_variant.name)?
            .description(insertable_variant.description)?)
    }
}

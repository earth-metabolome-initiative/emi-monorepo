#[derive(Clone, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableInstrumentCategoryAttributes {
    Name,
    Description,
    Icon,
}
impl core::fmt::Display for InsertableInstrumentCategoryAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableInstrumentCategoryAttributes::Name => write!(f, "name"),
            InsertableInstrumentCategoryAttributes::Description => {
                write!(f, "description")
            }
            InsertableInstrumentCategoryAttributes::Icon => write!(f, "icon"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::instrument_categories::instrument_categories
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableInstrumentCategory {
    name: String,
    description: String,
    icon: String,
}
impl InsertableInstrumentCategory {}
#[derive(Default)]
pub struct InsertableInstrumentCategoryBuilder {
    name: Option<String>,
    description: Option<String>,
    icon: Option<String>,
}
impl InsertableInstrumentCategoryBuilder {
    pub fn name<P: Into<String>>(
        mut self,
        name: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let name = name.into();
        self.name = Some(name);
        Ok(self)
    }
    pub fn description<P: Into<String>>(
        mut self,
        description: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let description = description.into();
        self.description = Some(description);
        Ok(self)
    }
    pub fn icon<P: Into<String>>(
        mut self,
        icon: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let icon = icon.into();
        self.icon = Some(icon);
        Ok(self)
    }
}
impl common_traits::prelude::Builder for InsertableInstrumentCategoryBuilder {
    type Error = web_common_traits::database::InsertError<InsertableInstrumentCategoryAttributes>;
    type Object = InsertableInstrumentCategory;
    type Attribute = InsertableInstrumentCategoryAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            name: self.name.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableInstrumentCategoryAttributes::Name,
            ))?,
            description: self.description.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableInstrumentCategoryAttributes::Description,
                ),
            )?,
            icon: self.icon.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableInstrumentCategoryAttributes::Icon,
            ))?,
        })
    }
}
impl TryFrom<InsertableInstrumentCategory> for InsertableInstrumentCategoryBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableInstrumentCategory) -> Result<Self, Self::Error> {
        Self::default()
            .name(insertable_variant.name)?
            .description(insertable_variant.description)?
            .icon(insertable_variant.icon)
    }
}

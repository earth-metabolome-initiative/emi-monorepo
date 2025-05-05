#[derive(Clone, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableToolCategoryAttributes {
    Name,
    Description,
    Icon,
}
impl core::fmt::Display for InsertableToolCategoryAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableToolCategoryAttributes::Name => write!(f, "name"),
            InsertableToolCategoryAttributes::Description => write!(f, "description"),
            InsertableToolCategoryAttributes::Icon => write!(f, "icon"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::tool_categories::tool_categories
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableToolCategory {
    name: String,
    description: String,
    icon: String,
}
impl InsertableToolCategory {}
#[derive(Default)]
pub struct InsertableToolCategoryBuilder {
    name: Option<String>,
    description: Option<String>,
    icon: Option<String>,
}
impl InsertableToolCategoryBuilder {
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
    pub fn icon(
        mut self,
        icon: String,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.icon = Some(icon);
        Ok(self)
    }
}
impl common_traits::prelude::Builder for InsertableToolCategoryBuilder {
    type Error = web_common_traits::database::InsertError<InsertableToolCategoryAttributes>;
    type Object = InsertableToolCategory;
    type Attribute = InsertableToolCategoryAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            name: self.name.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableToolCategoryAttributes::Name,
            ))?,
            description: self.description.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableToolCategoryAttributes::Description,
                ),
            )?,
            icon: self.icon.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableToolCategoryAttributes::Icon,
            ))?,
        })
    }
}
impl TryFrom<InsertableToolCategory> for InsertableToolCategoryBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableToolCategory) -> Result<Self, Self::Error> {
        Self::default()
            .name(insertable_variant.name)?
            .description(insertable_variant.description)?
            .icon(insertable_variant.icon)
    }
}

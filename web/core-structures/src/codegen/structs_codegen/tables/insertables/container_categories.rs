#[derive(Clone, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableContainerCategoryAttributes {
    Name,
    Description,
    Icon,
}
impl core::fmt::Display for InsertableContainerCategoryAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableContainerCategoryAttributes::Name => write!(f, "name"),
            InsertableContainerCategoryAttributes::Description => {
                write!(f, "description")
            }
            InsertableContainerCategoryAttributes::Icon => write!(f, "icon"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::container_categories::container_categories
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableContainerCategory {
    name: String,
    description: String,
    icon: String,
}
impl InsertableContainerCategory {}
#[derive(Default)]
pub struct InsertableContainerCategoryBuilder {
    name: Option<String>,
    description: Option<String>,
    icon: Option<String>,
}
impl InsertableContainerCategoryBuilder {
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
impl common_traits::prelude::Builder for InsertableContainerCategoryBuilder {
    type Error = web_common_traits::database::InsertError<InsertableContainerCategoryAttributes>;
    type Object = InsertableContainerCategory;
    type Attribute = InsertableContainerCategoryAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            name: self.name.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableContainerCategoryAttributes::Name,
            ))?,
            description: self.description.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableContainerCategoryAttributes::Description,
                ),
            )?,
            icon: self.icon.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableContainerCategoryAttributes::Icon,
            ))?,
        })
    }
}
impl TryFrom<InsertableContainerCategory> for InsertableContainerCategoryBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableContainerCategory) -> Result<Self, Self::Error> {
        Self::default()
            .name(insertable_variant.name)?
            .description(insertable_variant.description)?
            .icon(insertable_variant.icon)
    }
}

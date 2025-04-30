#[derive(Clone, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableStepModelCategoryAttributes {
    Name,
    Description,
    Icon,
}
impl core::fmt::Display for InsertableStepModelCategoryAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableStepModelCategoryAttributes::Name => write!(f, "name"),
            InsertableStepModelCategoryAttributes::Description => {
                write!(f, "description")
            }
            InsertableStepModelCategoryAttributes::Icon => write!(f, "icon"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::step_model_categories::step_model_categories
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableStepModelCategory {
    name: String,
    description: String,
    icon: font_awesome_icons::FAIcon,
}
impl InsertableStepModelCategory {}
#[derive(Default)]
pub struct InsertableStepModelCategoryBuilder {
    name: Option<String>,
    description: Option<String>,
    icon: Option<font_awesome_icons::FAIcon>,
}
impl InsertableStepModelCategoryBuilder {
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
        icon: font_awesome_icons::FAIcon,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.icon = Some(icon);
        Ok(self)
    }
}
impl common_traits::prelude::Builder for InsertableStepModelCategoryBuilder {
    type Error = web_common_traits::database::InsertError<InsertableStepModelCategoryAttributes>;
    type Object = InsertableStepModelCategory;
    type Attribute = InsertableStepModelCategoryAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            name: self.name.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableStepModelCategoryAttributes::Name,
            ))?,
            description: self.description.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableStepModelCategoryAttributes::Description,
                ),
            )?,
            icon: self.icon.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableStepModelCategoryAttributes::Icon,
            ))?,
        })
    }
}
impl TryFrom<InsertableStepModelCategory> for InsertableStepModelCategoryBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableStepModelCategory) -> Result<Self, Self::Error> {
        Self::default()
            .name(insertable_variant.name)?
            .description(insertable_variant.description)?
            .icon(insertable_variant.icon)
    }
}

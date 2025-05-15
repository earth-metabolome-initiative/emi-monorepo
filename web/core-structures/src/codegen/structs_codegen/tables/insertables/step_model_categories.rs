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
    icon: String,
}
impl InsertableStepModelCategory {}
#[derive(Default)]
pub struct InsertableStepModelCategoryBuilder {
    name: Option<String>,
    description: Option<String>,
    icon: Option<String>,
}
impl InsertableStepModelCategoryBuilder {
    pub fn name<P>(
        mut self,
        name: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let name = name.try_into().map_err(|err: <P as TryInto<String>>::Error| {
            Into::into(err).rename_field(InsertableStepModelCategoryAttributes::Name)
        })?;
        self.name = Some(name);
        Ok(self)
    }
    pub fn description<P>(
        mut self,
        description: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let description =
            description.try_into().map_err(|err: <P as TryInto<String>>::Error| {
                Into::into(err).rename_field(InsertableStepModelCategoryAttributes::Description)
            })?;
        self.description = Some(description);
        Ok(self)
    }
    pub fn icon<P>(
        mut self,
        icon: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let icon = icon.try_into().map_err(|err: <P as TryInto<String>>::Error| {
            Into::into(err).rename_field(InsertableStepModelCategoryAttributes::Icon)
        })?;
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

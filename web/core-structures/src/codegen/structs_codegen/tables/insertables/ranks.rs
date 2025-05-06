#[derive(Clone, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableRankAttributes {
    Name,
    Description,
}
impl core::fmt::Display for InsertableRankAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableRankAttributes::Name => write!(f, "name"),
            InsertableRankAttributes::Description => write!(f, "description"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(table_name = crate::codegen::diesel_codegen::tables::ranks::ranks)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableRank {
    name: String,
    description: String,
}
impl InsertableRank {}
#[derive(Default)]
pub struct InsertableRankBuilder {
    name: Option<String>,
    description: Option<String>,
}
impl InsertableRankBuilder {
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
}
impl common_traits::prelude::Builder for InsertableRankBuilder {
    type Error = web_common_traits::database::InsertError<InsertableRankAttributes>;
    type Object = InsertableRank;
    type Attribute = InsertableRankAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            name: self.name.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableRankAttributes::Name,
            ))?,
            description: self.description.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableRankAttributes::Description,
                ),
            )?,
        })
    }
}
impl TryFrom<InsertableRank> for InsertableRankBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableRank) -> Result<Self, Self::Error> {
        Self::default().name(insertable_variant.name)?.description(insertable_variant.description)
    }
}

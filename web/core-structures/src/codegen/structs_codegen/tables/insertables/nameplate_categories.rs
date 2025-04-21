#[derive(Clone, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableNameplateCategoryAttributes {
    Name,
    PermanenceCategoryId,
    Description,
    IconId,
    ColorId,
}
impl core::fmt::Display for InsertableNameplateCategoryAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableNameplateCategoryAttributes::Name => write!(f, "name"),
            InsertableNameplateCategoryAttributes::PermanenceCategoryId => {
                write!(f, "permanence_category_id")
            }
            InsertableNameplateCategoryAttributes::Description => {
                write!(f, "description")
            }
            InsertableNameplateCategoryAttributes::IconId => write!(f, "icon_id"),
            InsertableNameplateCategoryAttributes::ColorId => write!(f, "color_id"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::nameplate_categories::nameplate_categories
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableNameplateCategory {
    name: String,
    permanence_category_id: i16,
    description: String,
    icon_id: i16,
    color_id: i16,
}
impl InsertableNameplateCategory {
    #[cfg(feature = "postgres")]
    pub async fn permanence_category(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::permanence_categories::PermanenceCategory,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::permanence_categories::PermanenceCategory::table()
            .filter(
                crate::codegen::diesel_codegen::tables::permanence_categories::permanence_categories::dsl::id
                    .eq(&self.permanence_category_id),
            )
            .first::<
                crate::codegen::structs_codegen::tables::permanence_categories::PermanenceCategory,
            >(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn icon(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::icons::Icon, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::icons::Icon::table()
            .filter(crate::codegen::diesel_codegen::tables::icons::icons::dsl::id.eq(&self.icon_id))
            .first::<crate::codegen::structs_codegen::tables::icons::Icon>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn color(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::colors::Color, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::colors::Color::table()
            .filter(
                crate::codegen::diesel_codegen::tables::colors::colors::dsl::id.eq(&self.color_id),
            )
            .first::<crate::codegen::structs_codegen::tables::colors::Color>(conn)
            .await
    }
}
#[derive(Default)]
pub struct InsertableNameplateCategoryBuilder {
    name: Option<String>,
    permanence_category_id: Option<i16>,
    description: Option<String>,
    icon_id: Option<i16>,
    color_id: Option<i16>,
}
impl InsertableNameplateCategoryBuilder {
    pub fn name(
        mut self,
        name: String,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.name = Some(name);
        Ok(self)
    }
    pub fn permanence_category_id(
        mut self,
        permanence_category_id: i16,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.permanence_category_id = Some(permanence_category_id);
        Ok(self)
    }
    pub fn description(
        mut self,
        description: String,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.description = Some(description);
        Ok(self)
    }
    pub fn icon_id(
        mut self,
        icon_id: i16,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.icon_id = Some(icon_id);
        Ok(self)
    }
    pub fn color_id(
        mut self,
        color_id: i16,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.color_id = Some(color_id);
        Ok(self)
    }
}
impl common_traits::prelude::Builder for InsertableNameplateCategoryBuilder {
    type Error = web_common_traits::database::InsertError<InsertableNameplateCategoryAttributes>;
    type Object = InsertableNameplateCategory;
    type Attribute = InsertableNameplateCategoryAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            name: self.name.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableNameplateCategoryAttributes::Name,
            ))?,
            permanence_category_id: self.permanence_category_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableNameplateCategoryAttributes::PermanenceCategoryId,
                ),
            )?,
            description: self.description.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableNameplateCategoryAttributes::Description,
                ),
            )?,
            icon_id: self.icon_id.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableNameplateCategoryAttributes::IconId,
            ))?,
            color_id: self.color_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableNameplateCategoryAttributes::ColorId,
                ),
            )?,
        })
    }
}
impl TryFrom<InsertableNameplateCategory> for InsertableNameplateCategoryBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableNameplateCategory) -> Result<Self, Self::Error> {
        Self::default()
            .name(insertable_variant.name)?
            .permanence_category_id(insertable_variant.permanence_category_id)?
            .description(insertable_variant.description)?
            .icon_id(insertable_variant.icon_id)?
            .color_id(insertable_variant.color_id)?
    }
}

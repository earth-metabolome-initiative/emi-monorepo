#[derive(Clone, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableMaterialAttributes {
    Name,
    Description,
    Icon,
    ColorId,
}
impl core::fmt::Display for InsertableMaterialAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableMaterialAttributes::Name => write!(f, "name"),
            InsertableMaterialAttributes::Description => write!(f, "description"),
            InsertableMaterialAttributes::Icon => write!(f, "icon"),
            InsertableMaterialAttributes::ColorId => write!(f, "color_id"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(table_name = crate::codegen::diesel_codegen::tables::materials::materials)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableMaterial {
    name: String,
    description: String,
    icon: String,
    color_id: i16,
}
impl InsertableMaterial {
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
pub struct InsertableMaterialBuilder {
    name: Option<String>,
    description: Option<String>,
    icon: Option<String>,
    color_id: Option<i16>,
}
impl InsertableMaterialBuilder {
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
    pub fn color_id<P: Into<i16>>(
        mut self,
        color_id: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let color_id = color_id.into();
        self.color_id = Some(color_id);
        Ok(self)
    }
}
impl common_traits::prelude::Builder for InsertableMaterialBuilder {
    type Error = web_common_traits::database::InsertError<InsertableMaterialAttributes>;
    type Object = InsertableMaterial;
    type Attribute = InsertableMaterialAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            name: self.name.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableMaterialAttributes::Name,
            ))?,
            description: self.description.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableMaterialAttributes::Description,
                ),
            )?,
            icon: self.icon.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableMaterialAttributes::Icon,
            ))?,
            color_id: self.color_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableMaterialAttributes::ColorId,
                ),
            )?,
        })
    }
}
impl TryFrom<InsertableMaterial> for InsertableMaterialBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableMaterial) -> Result<Self, Self::Error> {
        Self::default()
            .name(insertable_variant.name)?
            .description(insertable_variant.description)?
            .icon(insertable_variant.icon)?
            .color_id(insertable_variant.color_id)
    }
}

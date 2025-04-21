#[derive(Clone, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableUnitAttributes {
    Name,
    Unit,
    IconId,
    ColorId,
}
impl core::fmt::Display for InsertableUnitAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableUnitAttributes::Name => write!(f, "name"),
            InsertableUnitAttributes::Unit => write!(f, "unit"),
            InsertableUnitAttributes::IconId => write!(f, "icon_id"),
            InsertableUnitAttributes::ColorId => write!(f, "color_id"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(table_name = crate::codegen::diesel_codegen::tables::units::units)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableUnit {
    name: String,
    unit: String,
    icon_id: i16,
    color_id: i16,
}
impl InsertableUnit {
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
pub struct InsertableUnitBuilder {
    name: Option<String>,
    unit: Option<String>,
    icon_id: Option<i16>,
    color_id: Option<i16>,
}
impl InsertableUnitBuilder {
    pub fn name(
        mut self,
        name: String,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.name = Some(name);
        Ok(self)
    }
    pub fn unit(
        mut self,
        unit: String,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.unit = Some(unit);
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
impl common_traits::prelude::Builder for InsertableUnitBuilder {
    type Error = web_common_traits::database::InsertError<InsertableUnitAttributes>;
    type Object = InsertableUnit;
    type Attribute = InsertableUnitAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            name: self.name.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableUnitAttributes::Name,
            ))?,
            unit: self.unit.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableUnitAttributes::Unit,
            ))?,
            icon_id: self.icon_id.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableUnitAttributes::IconId,
            ))?,
            color_id: self.color_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableUnitAttributes::ColorId,
                ),
            )?,
        })
    }
}
impl TryFrom<InsertableUnit> for InsertableUnitBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableUnit) -> Result<Self, Self::Error> {
        Self::default()
            .name(insertable_variant.name)?
            .unit(insertable_variant.unit)?
            .icon_id(insertable_variant.icon_id)?
            .color_id(insertable_variant.color_id)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableUnitAttributes {
    Name,
    Unit,
    Icon,
    ColorId,
}
impl core::fmt::Display for InsertableUnitAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableUnitAttributes::Name => write!(f, "name"),
            InsertableUnitAttributes::Unit => write!(f, "unit"),
            InsertableUnitAttributes::Icon => write!(f, "icon"),
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
    icon: String,
    color_id: i16,
}
impl InsertableUnit {
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
    icon: Option<String>,
    color_id: Option<i16>,
}
impl InsertableUnitBuilder {
    pub fn name<P>(
        mut self,
        name: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let name = name.try_into().map_err(|err: <P as TryInto<String>>::Error| {
            Into::into(err).rename_field(InsertableUnitAttributes::Name)
        })?;
        self.name = Some(name);
        Ok(self)
    }
    pub fn unit<P>(
        mut self,
        unit: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let unit = unit.try_into().map_err(|err: <P as TryInto<String>>::Error| {
            Into::into(err).rename_field(InsertableUnitAttributes::Unit)
        })?;
        self.unit = Some(unit);
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
            Into::into(err).rename_field(InsertableUnitAttributes::Icon)
        })?;
        self.icon = Some(icon);
        Ok(self)
    }
    pub fn color_id<P>(
        mut self,
        color_id: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<i16>,
        <P as TryInto<i16>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let color_id = color_id.try_into().map_err(|err: <P as TryInto<i16>>::Error| {
            Into::into(err).rename_field(InsertableUnitAttributes::ColorId)
        })?;
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
            icon: self.icon.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableUnitAttributes::Icon,
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
            .icon(insertable_variant.icon)?
            .color_id(insertable_variant.color_id)
    }
}

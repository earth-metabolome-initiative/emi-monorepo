#[derive(Clone, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableInstrumentStateAttributes {
    Name,
    Description,
    IconId,
    ColorId,
}
impl core::fmt::Display for InsertableInstrumentStateAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableInstrumentStateAttributes::Name => write!(f, "name"),
            InsertableInstrumentStateAttributes::Description => write!(f, "description"),
            InsertableInstrumentStateAttributes::IconId => write!(f, "icon_id"),
            InsertableInstrumentStateAttributes::ColorId => write!(f, "color_id"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::instrument_states::instrument_states
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableInstrumentState {
    name: String,
    description: String,
    icon_id: i16,
    color_id: i16,
}
impl InsertableInstrumentState {
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
pub struct InsertableInstrumentStateBuilder {
    name: Option<String>,
    description: Option<String>,
    icon_id: Option<i16>,
    color_id: Option<i16>,
}
impl InsertableInstrumentStateBuilder {
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
impl common_traits::prelude::Builder for InsertableInstrumentStateBuilder {
    type Error = web_common_traits::database::InsertError<InsertableInstrumentStateAttributes>;
    type Object = InsertableInstrumentState;
    type Attribute = InsertableInstrumentStateAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            name: self.name.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableInstrumentStateAttributes::Name,
            ))?,
            description: self.description.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableInstrumentStateAttributes::Description,
                ),
            )?,
            icon_id: self.icon_id.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableInstrumentStateAttributes::IconId,
            ))?,
            color_id: self.color_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableInstrumentStateAttributes::ColorId,
                ),
            )?,
        })
    }
}
impl TryFrom<InsertableInstrumentState> for InsertableInstrumentStateBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableInstrumentState) -> Result<Self, Self::Error> {
        Self::default()
            .name(insertable_variant.name)?
            .description(insertable_variant.description)?
            .icon_id(insertable_variant.icon_id)?
            .color_id(insertable_variant.color_id)?
    }
}

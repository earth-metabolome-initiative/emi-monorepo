#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableInstrumentStateAttributes {
    Name,
    Description,
    Icon,
    ColorId,
}
impl core::fmt::Display for InsertableInstrumentStateAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableInstrumentStateAttributes::Name => write!(f, "name"),
            InsertableInstrumentStateAttributes::Description => write!(f, "description"),
            InsertableInstrumentStateAttributes::Icon => write!(f, "icon"),
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
    icon: String,
    color_id: i16,
}
impl InsertableInstrumentState {
    pub fn color<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::colors::Color,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::colors::Color: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::colors::Color as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::colors::Color as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::colors::Color as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::colors::Color as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::colors::Color as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::colors::Color as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::colors::Color,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::colors::Color::table(),
                self.color_id,
            ),
            conn,
        )
    }
}
#[derive(Default)]
pub struct InsertableInstrumentStateBuilder {
    name: Option<String>,
    description: Option<String>,
    icon: Option<String>,
    color_id: Option<i16>,
}
impl InsertableInstrumentStateBuilder {
    pub fn name<P>(
        mut self,
        name: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableInstrumentStateAttributes>>
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let name = name.try_into().map_err(|err: <P as TryInto<String>>::Error| {
            Into::into(err).rename_field(InsertableInstrumentStateAttributes::Name)
        })?;
        self.name = Some(name);
        Ok(self)
    }
    pub fn description<P>(
        mut self,
        description: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableInstrumentStateAttributes>>
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let description =
            description.try_into().map_err(|err: <P as TryInto<String>>::Error| {
                Into::into(err).rename_field(InsertableInstrumentStateAttributes::Description)
            })?;
        self.description = Some(description);
        Ok(self)
    }
    pub fn icon<P>(
        mut self,
        icon: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableInstrumentStateAttributes>>
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let icon = icon.try_into().map_err(|err: <P as TryInto<String>>::Error| {
            Into::into(err).rename_field(InsertableInstrumentStateAttributes::Icon)
        })?;
        self.icon = Some(icon);
        Ok(self)
    }
    pub fn color_id<P>(
        mut self,
        color_id: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableInstrumentStateAttributes>>
    where
        P: TryInto<i16>,
        <P as TryInto<i16>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let color_id = color_id.try_into().map_err(|err: <P as TryInto<i16>>::Error| {
            Into::into(err).rename_field(InsertableInstrumentStateAttributes::ColorId)
        })?;
        self.color_id = Some(color_id);
        Ok(self)
    }
}
impl TryFrom<InsertableInstrumentStateBuilder> for InsertableInstrumentState {
    type Error = common_traits::prelude::BuilderError<InsertableInstrumentStateAttributes>;
    fn try_from(
        builder: InsertableInstrumentStateBuilder,
    ) -> Result<InsertableInstrumentState, Self::Error> {
        Ok(Self {
            name: builder.name.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableInstrumentStateAttributes::Name,
            ))?,
            description: builder.description.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableInstrumentStateAttributes::Description,
                ),
            )?,
            icon: builder.icon.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableInstrumentStateAttributes::Icon,
            ))?,
            color_id: builder.color_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableInstrumentStateAttributes::ColorId,
                ),
            )?,
        })
    }
}

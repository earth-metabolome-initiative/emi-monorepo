#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableInstrumentStateAttributes {
    Name,
    Description,
    Icon,
    ColorId,
    Id,
}
impl core::fmt::Display for InsertableInstrumentStateAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Name => write!(f, "name"),
            Self::Description => write!(f, "description"),
            Self::Icon => write!(f, "icon"),
            Self::ColorId => write!(f, "color_id"),
            Self::Id => write!(f, "id"),
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
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) icon: String,
    pub(crate) color_id: i16,
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
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableInstrumentStateBuilder {
    pub(crate) name: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) icon: Option<String>,
    pub(crate) color_id: Option<i16>,
}
impl web_common_traits::database::ExtendableBuilder for InsertableInstrumentStateBuilder {
    type Attributes = InsertableInstrumentStateAttributes;
    fn extend_builder(
        mut self,
        other: Self,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let Some(name) = other.name {
            self = self.name(name)?;
        }
        if let Some(description) = other.description {
            self = self.description(description)?;
        }
        if let Some(icon) = other.icon {
            self = self.icon(icon)?;
        }
        if let Some(color_id) = other.color_id {
            self = self.color(color_id)?;
        }
        Ok(self)
    }
}
impl web_common_traits::prelude::SetPrimaryKey for InsertableInstrumentStateBuilder {
    type PrimaryKey = i16;
    fn set_primary_key(self, _primary_key: Self::PrimaryKey) -> Self {
        self
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentStateBuilder {
    /// Sets the value of the `instrument_states.color_id` column from table
    /// `instrument_states`.
    pub fn color(
        mut self,
        color_id: i16,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableInstrumentStateAttributes>>
    {
        self.color_id = Some(color_id);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentStateBuilder {
    /// Sets the value of the `instrument_states.description` column from table
    /// `instrument_states`.
    pub fn description<Description>(
        mut self,
        description: Description,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableInstrumentStateAttributes>>
    where
        Description: TryInto<String>,
        <Description as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let description =
            description.try_into().map_err(|err: <Description as TryInto<String>>::Error| {
                Into::into(err).rename_field(InsertableInstrumentStateAttributes::Description)
            })?;
        self.description = Some(description);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentStateBuilder {
    /// Sets the value of the `instrument_states.icon` column from table
    /// `instrument_states`.
    pub fn icon<Icon>(
        mut self,
        icon: Icon,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableInstrumentStateAttributes>>
    where
        Icon: TryInto<String>,
        <Icon as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let icon = icon.try_into().map_err(|err: <Icon as TryInto<String>>::Error| {
            Into::into(err).rename_field(InsertableInstrumentStateAttributes::Icon)
        })?;
        self.icon = Some(icon);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentStateBuilder {
    /// Sets the value of the `instrument_states.name` column from table
    /// `instrument_states`.
    pub fn name<Name>(
        mut self,
        name: Name,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableInstrumentStateAttributes>>
    where
        Name: TryInto<String>,
        <Name as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let name = name.try_into().map_err(|err: <Name as TryInto<String>>::Error| {
            Into::into(err).rename_field(InsertableInstrumentStateAttributes::Name)
        })?;
        self.name = Some(name);
        Ok(self)
    }
}
impl<C> web_common_traits::database::TryInsertGeneric<C> for InsertableInstrumentStateBuilder
where
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::instrument_states::InstrumentState,
            Error = web_common_traits::database::InsertError<InsertableInstrumentStateAttributes>,
        >,
{
    type Attributes = InsertableInstrumentStateAttributes;
    fn is_complete(&self) -> bool {
        self.name.is_some()
            && self.description.is_some()
            && self.icon.is_some()
            && self.color_id.is_some()
    }
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<Self::Attributes>> {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::instrument_states::InstrumentState = self
            .insert(user_id, conn)?;
        Ok(insertable.id())
    }
}

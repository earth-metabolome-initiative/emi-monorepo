#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableUnitAttributes {
    Name,
    Unit,
    Icon,
    ColorId,
    Id,
}
impl core::fmt::Display for InsertableUnitAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Name => write!(f, "name"),
            Self::Unit => write!(f, "unit"),
            Self::Icon => write!(f, "icon"),
            Self::ColorId => write!(f, "color_id"),
            Self::Id => write!(f, "id"),
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
    pub(crate) name: String,
    pub(crate) unit: String,
    pub(crate) icon: String,
    pub(crate) color_id: i16,
}
impl InsertableUnit {
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
pub struct InsertableUnitBuilder {
    pub(crate) name: Option<String>,
    pub(crate) unit: Option<String>,
    pub(crate) icon: Option<String>,
    pub(crate) color_id: Option<i16>,
}
impl web_common_traits::database::ExtendableBuilder for InsertableUnitBuilder {
    type Attributes = InsertableUnitAttributes;
    fn extend_builder(
        mut self,
        other: Self,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let Some(name) = other.name {
            self = self.name(name)?;
        }
        if let Some(unit) = other.unit {
            self = self.unit(unit)?;
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
impl web_common_traits::prelude::SetPrimaryKey for InsertableUnitBuilder {
    type PrimaryKey = i16;
    fn set_primary_key(self, _primary_key: Self::PrimaryKey) -> Self {
        self
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableUnitBuilder {
    /// Sets the value of the `units.color_id` column from table `units`.
    pub fn color(
        mut self,
        color_id: i16,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableUnitAttributes>> {
        self.color_id = Some(color_id);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableUnitBuilder {
    /// Sets the value of the `units.icon` column from table `units`.
    pub fn icon<P>(
        mut self,
        icon: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableUnitAttributes>>
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
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableUnitBuilder {
    /// Sets the value of the `units.name` column from table `units`.
    pub fn name<P>(
        mut self,
        name: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableUnitAttributes>>
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
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableUnitBuilder {
    /// Sets the value of the `units.unit` column from table `units`.
    pub fn unit<P>(
        mut self,
        unit: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableUnitAttributes>>
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
}
impl<C> web_common_traits::database::TryInsertGeneric<C> for InsertableUnitBuilder
where
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::units::Unit,
            Error = web_common_traits::database::InsertError<InsertableUnitAttributes>,
        >,
{
    type Attributes = InsertableUnitAttributes;
    fn is_complete(&self) -> bool {
        self.name.is_some() && self.unit.is_some() && self.icon.is_some() && self.color_id.is_some()
    }
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<Self::Attributes>> {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::units::Unit =
            self.insert(user_id, conn)?;
        Ok(insertable.id())
    }
}

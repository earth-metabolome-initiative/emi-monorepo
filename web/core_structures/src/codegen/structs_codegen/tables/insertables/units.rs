#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UnitAttribute {
    Name,
    Unit,
    Icon,
    ColorId,
    Id,
}
impl core::str::FromStr for UnitAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Name" => Ok(Self::Name),
            "Unit" => Ok(Self::Unit),
            "Icon" => Ok(Self::Icon),
            "ColorId" => Ok(Self::ColorId),
            "name" => Ok(Self::Name),
            "unit" => Ok(Self::Unit),
            "icon" => Ok(Self::Icon),
            "color_id" => Ok(Self::ColorId),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl core::fmt::Display for UnitAttribute {
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
impl From<InsertableUnitBuilder>
    for web_common_traits::database::IdOrBuilder<i16, InsertableUnitBuilder>
{
    fn from(builder: InsertableUnitBuilder) -> Self {
        Self::Builder(builder)
    }
}
/// Trait defining setters for attributes of an instance of `Unit` or descendant
/// tables.
pub trait UnitSettable: Sized {
    /// Attributes required to build the insertable.
    type Attributes;
    /// Sets the value of the `public.units.name` column.
    ///
    /// # Arguments
    /// * `name`: The value to set for the `public.units.name` column.
    ///
    /// # Implementation details
    /// This method accepts a reference to a generic value which can be
    /// converted to the required type for the column. This allows passing
    /// values of different types, as long as they can be converted to the
    /// required type using the `TryFrom` trait. The method, additionally,
    /// employs same-as and inferred same-as rules to ensure that the
    /// schema-defined ancestral tables and associated table values associated
    /// to the current column (if any) are also set appropriately.
    ///
    /// # Errors
    /// * If the provided value cannot be converted to the required type
    ///   `String`.
    /// * If the provided value does not pass schema-defined validation.
    fn name<N>(
        self,
        name: N,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        N: TryInto<String>,
        validation_errors::SingleFieldError: From<<N as TryInto<String>>::Error>;
    /// Sets the value of the `public.units.unit` column.
    ///
    /// # Arguments
    /// * `unit`: The value to set for the `public.units.unit` column.
    ///
    /// # Implementation details
    /// This method accepts a reference to a generic value which can be
    /// converted to the required type for the column. This allows passing
    /// values of different types, as long as they can be converted to the
    /// required type using the `TryFrom` trait. The method, additionally,
    /// employs same-as and inferred same-as rules to ensure that the
    /// schema-defined ancestral tables and associated table values associated
    /// to the current column (if any) are also set appropriately.
    ///
    /// # Errors
    /// * If the provided value cannot be converted to the required type
    ///   `String`.
    /// * If the provided value does not pass schema-defined validation.
    fn unit<U>(
        self,
        unit: U,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        U: TryInto<String>,
        validation_errors::SingleFieldError: From<<U as TryInto<String>>::Error>;
    /// Sets the value of the `public.units.icon` column.
    ///
    /// # Arguments
    /// * `icon`: The value to set for the `public.units.icon` column.
    ///
    /// # Implementation details
    /// This method accepts a reference to a generic value which can be
    /// converted to the required type for the column. This allows passing
    /// values of different types, as long as they can be converted to the
    /// required type using the `TryFrom` trait. The method, additionally,
    /// employs same-as and inferred same-as rules to ensure that the
    /// schema-defined ancestral tables and associated table values associated
    /// to the current column (if any) are also set appropriately.
    ///
    /// # Errors
    /// * If the provided value cannot be converted to the required type
    ///   `String`.
    /// * If the provided value does not pass schema-defined validation.
    fn icon<I>(
        self,
        icon: I,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        I: TryInto<String>,
        validation_errors::SingleFieldError: From<<I as TryInto<String>>::Error>;
    /// Sets the value of the `public.units.color_id` column.
    ///
    /// # Arguments
    /// * `color_id`: The value to set for the `public.units.color_id` column.
    ///
    /// # Implementation details
    /// This method accepts a reference to a generic value which can be
    /// converted to the required type for the column. This allows passing
    /// values of different types, as long as they can be converted to the
    /// required type using the `TryFrom` trait. The method, additionally,
    /// employs same-as and inferred same-as rules to ensure that the
    /// schema-defined ancestral tables and associated table values associated
    /// to the current column (if any) are also set appropriately.
    ///
    /// # Errors
    /// * If the provided value cannot be converted to the required type `i16`.
    /// * If the provided value does not pass schema-defined validation.
    fn color(
        self,
        color_id: i16,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
}
impl UnitSettable for InsertableUnitBuilder {
    type Attributes = crate::codegen::structs_codegen::tables::insertables::UnitAttribute;
    /// Sets the value of the `public.units.name` column.
    fn name<N>(
        mut self,
        name: N,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        N: TryInto<String>,
        validation_errors::SingleFieldError: From<<N as TryInto<String>>::Error>,
    {
        let name = name.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err).rename_field(UnitAttribute::Name)
        })?;
        self.name = Some(name);
        Ok(self)
    }
    /// Sets the value of the `public.units.unit` column.
    fn unit<U>(
        mut self,
        unit: U,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        U: TryInto<String>,
        validation_errors::SingleFieldError: From<<U as TryInto<String>>::Error>,
    {
        let unit = unit.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err).rename_field(UnitAttribute::Unit)
        })?;
        self.unit = Some(unit);
        Ok(self)
    }
    /// Sets the value of the `public.units.icon` column.
    fn icon<I>(
        mut self,
        icon: I,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        I: TryInto<String>,
        validation_errors::SingleFieldError: From<<I as TryInto<String>>::Error>,
    {
        let icon = icon.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err).rename_field(UnitAttribute::Icon)
        })?;
        self.icon = Some(icon);
        Ok(self)
    }
    /// Sets the value of the `public.units.color_id` column.
    fn color(
        mut self,
        color_id: i16,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.color_id = Some(color_id);
        Ok(self)
    }
}
impl web_common_traits::prelude::SetPrimaryKey for InsertableUnitBuilder {
    type PrimaryKey = i16;
    fn set_primary_key(self, _primary_key: Self::PrimaryKey) -> Self {
        self
    }
}
impl<C> web_common_traits::database::TryInsertGeneric<C> for InsertableUnitBuilder
where
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::units::Unit,
            Error = web_common_traits::database::InsertError<UnitAttribute>,
        >,
{
    type Attributes = UnitAttribute;
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

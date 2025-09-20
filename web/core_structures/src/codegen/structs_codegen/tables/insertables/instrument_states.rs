#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InstrumentStateAttribute {
    Name,
    Description,
    Icon,
    ColorId,
    Id,
}
impl core::str::FromStr for InstrumentStateAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Name" => Ok(Self::Name),
            "Description" => Ok(Self::Description),
            "Icon" => Ok(Self::Icon),
            "ColorId" => Ok(Self::ColorId),
            "name" => Ok(Self::Name),
            "description" => Ok(Self::Description),
            "icon" => Ok(Self::Icon),
            "color_id" => Ok(Self::ColorId),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl common_traits::builder::Attributed
    for crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentStateBuilder
{
    type Attribute = InstrumentStateAttribute;
}
impl web_common_traits::database::TableField for InstrumentStateAttribute {}
impl web_common_traits::database::HasTableType for InstrumentStateAttribute {
    type Table = crate::codegen::tables::table_names::TableName;
}
impl core::fmt::Display for InstrumentStateAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Name => write!(f, "instrument_states.name"),
            Self::Description => write!(f, "instrument_states.description"),
            Self::Icon => write!(f, "instrument_states.icon"),
            Self::ColorId => write!(f, "instrument_states.color_id"),
            Self::Id => write!(f, "instrument_states.id"),
        }
    }
}
#[derive(Debug)]
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
    ) -> Result<crate::codegen::structs_codegen::tables::colors::Color, diesel::result::Error>
    where
        crate::codegen::structs_codegen::tables::colors::Color:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::colors::Color::read(self.color_id, conn)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Hash, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Builder for creating and inserting a new
/// [`InstrumentState`](crate::codegen::structs_codegen::tables::instrument_states::InstrumentState).
///
///
/// # Implementation details
/// While this builder implements several methods, a reasonably complete
/// **basic** usage example (*which may not apply to your own specific use case,
/// please adapt accordingly*) is as follows:
///
/// ```rust,ignore
/// use core_structures::InstrumentState;
/// use core_structures::tables::insertables::InstrumentStateSettable;
/// use web_common_traits::database::Insertable;
/// use web_common_traits::database::InsertableVariant;
///
/// let instrument_state = InstrumentState::new()
///    // Set mandatory fields
///    .color(color_id)?
///    .description(description)?
///    .icon(icon)?
///    .name(name)?
///    // Finally, insert the new record in the database
///    .insert(user.id, conn)?;
/// ```
pub struct InsertableInstrumentStateBuilder {
    pub(crate) name: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) icon: Option<String>,
    pub(crate) color_id: Option<i16>,
}
impl diesel::associations::HasTable for InsertableInstrumentStateBuilder {
    type Table =
        crate::codegen::diesel_codegen::tables::instrument_states::instrument_states::table;
    fn table() -> Self::Table {
        crate::codegen::diesel_codegen::tables::instrument_states::instrument_states::table
    }
}
impl From<InsertableInstrumentStateBuilder>
    for web_common_traits::database::IdOrBuilder<i16, InsertableInstrumentStateBuilder>
{
    fn from(builder: InsertableInstrumentStateBuilder) -> Self {
        Self::Builder(builder)
    }
}
impl common_traits::builder::IsCompleteBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentStateBuilder
{
    fn is_complete(&self) -> bool {
        self.name.is_some()
            && self.description.is_some()
            && self.icon.is_some()
            && self.color_id.is_some()
    }
}
/// Trait defining setters for attributes of an instance of `InstrumentState` or
/// descendant tables.
pub trait InstrumentStateSettable: Sized {
    /// Error type returned when setting attributes.
    type Error;
    /// Sets the value of the `public.instrument_states.name` column.
    ///
    /// # Arguments
    /// * `name`: The value to set for the `public.instrument_states.name`
    ///   column.
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
    fn name<N>(self, name: N) -> Result<Self, Self::Error>
    where
        N: TryInto<String>,
        validation_errors::SingleFieldError: From<<N as TryInto<String>>::Error>;
    /// Sets the value of the `public.instrument_states.description` column.
    ///
    /// # Arguments
    /// * `description`: The value to set for the
    ///   `public.instrument_states.description` column.
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
    fn description<D>(self, description: D) -> Result<Self, Self::Error>
    where
        D: TryInto<String>,
        validation_errors::SingleFieldError: From<<D as TryInto<String>>::Error>;
    /// Sets the value of the `public.instrument_states.icon` column.
    ///
    /// # Arguments
    /// * `icon`: The value to set for the `public.instrument_states.icon`
    ///   column.
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
    fn icon<I>(self, icon: I) -> Result<Self, Self::Error>
    where
        I: TryInto<String>,
        validation_errors::SingleFieldError: From<<I as TryInto<String>>::Error>;
    /// Sets the value of the `public.instrument_states.color_id` column.
    ///
    /// # Arguments
    /// * `color_id`: The value to set for the
    ///   `public.instrument_states.color_id` column.
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
    fn color<CI>(self, color_id: CI) -> Result<Self, Self::Error>
    where
        CI: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i16>;
}
impl InstrumentStateSettable for InsertableInstrumentStateBuilder
where
    Self: common_traits::builder::Attributed<
        Attribute = crate::codegen::structs_codegen::tables::insertables::InstrumentStateAttribute,
    >,
{
    type Error = web_common_traits::database::InsertError<
        <Self as common_traits::builder::Attributed>::Attribute,
    >;
    ///Sets the value of the `public.instrument_states.name` column.
    fn name<N>(mut self, name: N) -> Result<Self, Self::Error>
    where
        N: TryInto<String>,
        validation_errors::SingleFieldError: From<<N as TryInto<String>>::Error>,
    {
        let name = name
            .try_into()
            .map_err(|err| {
                validation_errors::SingleFieldError::from(err)
                    .rename_field(InstrumentStateAttribute::Name)
            })?;
        self.name = Some(name);
        Ok(self)
    }
    ///Sets the value of the `public.instrument_states.description` column.
    fn description<D>(mut self, description: D) -> Result<Self, Self::Error>
    where
        D: TryInto<String>,
        validation_errors::SingleFieldError: From<<D as TryInto<String>>::Error>,
    {
        let description = description
            .try_into()
            .map_err(|err| {
                validation_errors::SingleFieldError::from(err)
                    .rename_field(InstrumentStateAttribute::Description)
            })?;
        self.description = Some(description);
        Ok(self)
    }
    ///Sets the value of the `public.instrument_states.icon` column.
    fn icon<I>(mut self, icon: I) -> Result<Self, Self::Error>
    where
        I: TryInto<String>,
        validation_errors::SingleFieldError: From<<I as TryInto<String>>::Error>,
    {
        let icon = icon
            .try_into()
            .map_err(|err| {
                validation_errors::SingleFieldError::from(err)
                    .rename_field(InstrumentStateAttribute::Icon)
            })?;
        self.icon = Some(icon);
        Ok(self)
    }
    ///Sets the value of the `public.instrument_states.color_id` column.
    fn color<CI>(mut self, color_id: CI) -> Result<Self, Self::Error>
    where
        CI: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i16>,
    {
        let color_id = <CI as web_common_traits::database::PrimaryKeyLike>::primary_key(
            &color_id,
        );
        self.color_id = Some(color_id);
        Ok(self)
    }
}
impl web_common_traits::prelude::SetPrimaryKey for InsertableInstrumentStateBuilder {
    type PrimaryKey = i16;
    fn set_primary_key(self, _primary_key: Self::PrimaryKey) -> Self {
        self
    }
}
impl<C> web_common_traits::database::TryInsertGeneric<C> for InsertableInstrumentStateBuilder
where
    Self: web_common_traits::database::DispatchableInsertableVariant<
            C,
            Row = crate::codegen::structs_codegen::tables::instrument_states::InstrumentState,
            Error = web_common_traits::database::InsertError<InstrumentStateAttribute>,
        > + web_common_traits::database::SetPrimaryKey<PrimaryKey = i16>
        + common_traits::builder::IsCompleteBuilder,
{
    type Error = web_common_traits::database::InsertError<InstrumentStateAttribute>;
    fn mint_primary_key(self, user_id: i32, conn: &mut C) -> Result<Self::PrimaryKey, Self::Error> {
        use diesel::Identifiable;
        use web_common_traits::database::DispatchableInsertableVariant;
        Ok(self.insert(user_id, conn)?.id())
    }
}

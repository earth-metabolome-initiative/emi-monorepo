#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableTeamStateAttributes {
    Name,
    Description,
    Icon,
    ColorId,
    Id,
}
impl core::str::FromStr for InsertableTeamStateAttributes {
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
impl core::fmt::Display for InsertableTeamStateAttributes {
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
    diesel(table_name = crate::codegen::diesel_codegen::tables::team_states::team_states)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableTeamState {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) icon: String,
    pub(crate) color_id: i16,
}
impl InsertableTeamState {
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
pub struct InsertableTeamStateBuilder {
    pub(crate) name: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) icon: Option<String>,
    pub(crate) color_id: Option<i16>,
}
/// Trait defining setters for attributes of an instance of `TeamState` or
/// descendant tables.
pub trait TeamStateBuildable: std::marker::Sized {
    /// Attributes required to build the insertable.
    type Attributes;
    /// Sets the value of the `public.team_states.name` column.
    ///
    /// # Arguments
    /// * `name`: The value to set for the `public.team_states.name` column.
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
    fn name<'N, N>(
        self,
        name: &'N N,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        &'N N: TryInto<String>,
        validation_errors::SingleFieldError: From<<&'N N as TryInto<String>>::Error>;
    /// Sets the value of the `public.team_states.description` column.
    ///
    /// # Arguments
    /// * `description`: The value to set for the
    ///   `public.team_states.description` column.
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
    fn description<'D, D>(
        self,
        description: &'D D,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        &'D D: TryInto<String>,
        validation_errors::SingleFieldError: From<<&'D D as TryInto<String>>::Error>;
    /// Sets the value of the `public.team_states.icon` column.
    ///
    /// # Arguments
    /// * `icon`: The value to set for the `public.team_states.icon` column.
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
    fn icon<'I, I>(
        self,
        icon: &'I I,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        &'I I: TryInto<String>,
        validation_errors::SingleFieldError: From<<&'I I as TryInto<String>>::Error>;
    /// Sets the value of the `public.team_states.color_id` column.
    ///
    /// # Arguments
    /// * `color_id`: The value to set for the `public.team_states.color_id`
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
    /// * If the provided value cannot be converted to the required type `i16`.
    /// * If the provided value does not pass schema-defined validation.
    fn color(
        self,
        color_id: i16,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
}
impl TeamStateBuildable for Option<i16> {
    type Attributes =
        crate::codegen::structs_codegen::tables::insertables::InsertableTeamStateAttributes;
    fn name<'N, N>(
        self,
        _name: &'N N,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        &'N N: TryInto<String>,
        validation_errors::SingleFieldError: From<<&'N N as TryInto<String>>::Error>,
    {
        Ok(self)
    }
    fn description<'D, D>(
        self,
        _description: &'D D,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        &'D D: TryInto<String>,
        validation_errors::SingleFieldError: From<<&'D D as TryInto<String>>::Error>,
    {
        Ok(self)
    }
    fn icon<'I, I>(
        self,
        _icon: &'I I,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        &'I I: TryInto<String>,
        validation_errors::SingleFieldError: From<<&'I I as TryInto<String>>::Error>,
    {
        Ok(self)
    }
    fn color(
        self,
        _color_id: i16,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        Ok(self)
    }
}
impl TeamStateBuildable for InsertableTeamStateBuilder {
    type Attributes =
        crate::codegen::structs_codegen::tables::insertables::InsertableTeamStateAttributes;
    /// Sets the value of the `public.team_states.name` column.
    fn name<'N, N>(
        mut self,
        name: &'N N,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        &'N N: TryInto<String>,
        validation_errors::SingleFieldError: From<<&'N N as TryInto<String>>::Error>,
    {
        let name = name.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(InsertableTeamStateAttributes::Name)
        })?;
        self.name = Some(name);
        Ok(self)
    }
    /// Sets the value of the `public.team_states.description` column.
    fn description<'D, D>(
        mut self,
        description: &'D D,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        &'D D: TryInto<String>,
        validation_errors::SingleFieldError: From<<&'D D as TryInto<String>>::Error>,
    {
        let description = description.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(InsertableTeamStateAttributes::Description)
        })?;
        self.description = Some(description);
        Ok(self)
    }
    /// Sets the value of the `public.team_states.icon` column.
    fn icon<'I, I>(
        mut self,
        icon: &'I I,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        &'I I: TryInto<String>,
        validation_errors::SingleFieldError: From<<&'I I as TryInto<String>>::Error>,
    {
        let icon = icon.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(InsertableTeamStateAttributes::Icon)
        })?;
        self.icon = Some(icon);
        Ok(self)
    }
    /// Sets the value of the `public.team_states.color_id` column.
    fn color(
        mut self,
        color_id: i16,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        let color_id = color_id.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(InsertableTeamStateAttributes::ColorId)
        })?;
        self.color_id = Some(color_id);
        Ok(self)
    }
}
impl web_common_traits::prelude::SetPrimaryKey for InsertableTeamStateBuilder {
    type PrimaryKey = i16;
    fn set_primary_key(self, _primary_key: Self::PrimaryKey) -> Self {
        self
    }
}
impl<C> web_common_traits::database::TryInsertGeneric<C> for InsertableTeamStateBuilder
where
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::team_states::TeamState,
            Error = web_common_traits::database::InsertError<InsertableTeamStateAttributes>,
        >,
{
    type Attributes = InsertableTeamStateAttributes;
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
        let insertable: crate::codegen::structs_codegen::tables::team_states::TeamState =
            self.insert(user_id, conn)?;
        Ok(insertable.id())
    }
}

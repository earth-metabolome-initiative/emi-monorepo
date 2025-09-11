#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ObservationSubjectAttribute {
    Name,
    Description,
    Icon,
    ColorId,
    Id,
}
impl core::str::FromStr for ObservationSubjectAttribute {
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
impl core::fmt::Display for ObservationSubjectAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Name => write!(f, "observation_subjects.name"),
            Self::Description => write!(f, "observation_subjects.description"),
            Self::Icon => write!(f, "observation_subjects.icon"),
            Self::ColorId => write!(f, "observation_subjects.color_id"),
            Self::Id => write!(f, "observation_subjects.id"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::observation_subjects::observation_subjects
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableObservationSubject {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) icon: String,
    pub(crate) color_id: i16,
}
impl InsertableObservationSubject {
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
pub struct InsertableObservationSubjectBuilder {
    pub(crate) name: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) icon: Option<String>,
    pub(crate) color_id: Option<i16>,
}
impl From<InsertableObservationSubjectBuilder>
    for web_common_traits::database::IdOrBuilder<i16, InsertableObservationSubjectBuilder>
{
    fn from(builder: InsertableObservationSubjectBuilder) -> Self {
        Self::Builder(builder)
    }
}
impl common_traits::builder::IsCompleteBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableObservationSubjectBuilder
{
    fn is_complete(&self) -> bool {
        self.name.is_some()
            && self.description.is_some()
            && self.icon.is_some()
            && self.color_id.is_some()
    }
}
/// Trait defining setters for attributes of an instance of `ObservationSubject`
/// or descendant tables.
pub trait ObservationSubjectSettable: Sized {
    /// Attributes required to build the insertable.
    type Attributes;
    /// Sets the value of the `public.observation_subjects.name` column.
    ///
    /// # Arguments
    /// * `name`: The value to set for the `public.observation_subjects.name`
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
    fn name<N>(
        self,
        name: N,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        N: TryInto<String>,
        validation_errors::SingleFieldError: From<<N as TryInto<String>>::Error>;
    /// Sets the value of the `public.observation_subjects.description` column.
    ///
    /// # Arguments
    /// * `description`: The value to set for the
    ///   `public.observation_subjects.description` column.
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
    fn description<D>(
        self,
        description: D,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        D: TryInto<String>,
        validation_errors::SingleFieldError: From<<D as TryInto<String>>::Error>;
    /// Sets the value of the `public.observation_subjects.icon` column.
    ///
    /// # Arguments
    /// * `icon`: The value to set for the `public.observation_subjects.icon`
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
    fn icon<I>(
        self,
        icon: I,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        I: TryInto<String>,
        validation_errors::SingleFieldError: From<<I as TryInto<String>>::Error>;
    /// Sets the value of the `public.observation_subjects.color_id` column.
    ///
    /// # Arguments
    /// * `color_id`: The value to set for the
    ///   `public.observation_subjects.color_id` column.
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
    fn color<CI>(
        self,
        color_id: CI,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        CI: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i16>;
}
impl ObservationSubjectSettable for InsertableObservationSubjectBuilder {
    type Attributes =
        crate::codegen::structs_codegen::tables::insertables::ObservationSubjectAttribute;
    /// Sets the value of the `public.observation_subjects.name` column.
    fn name<N>(
        mut self,
        name: N,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        N: TryInto<String>,
        validation_errors::SingleFieldError: From<<N as TryInto<String>>::Error>,
    {
        let name = name.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(ObservationSubjectAttribute::Name)
        })?;
        self.name = Some(name);
        Ok(self)
    }
    /// Sets the value of the `public.observation_subjects.description` column.
    fn description<D>(
        mut self,
        description: D,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        D: TryInto<String>,
        validation_errors::SingleFieldError: From<<D as TryInto<String>>::Error>,
    {
        let description = description.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(ObservationSubjectAttribute::Description)
        })?;
        self.description = Some(description);
        Ok(self)
    }
    /// Sets the value of the `public.observation_subjects.icon` column.
    fn icon<I>(
        mut self,
        icon: I,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        I: TryInto<String>,
        validation_errors::SingleFieldError: From<<I as TryInto<String>>::Error>,
    {
        let icon = icon.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(ObservationSubjectAttribute::Icon)
        })?;
        self.icon = Some(icon);
        Ok(self)
    }
    /// Sets the value of the `public.observation_subjects.color_id` column.
    fn color<CI>(
        mut self,
        color_id: CI,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        CI: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i16>,
    {
        let color_id = <CI as web_common_traits::database::PrimaryKeyLike>::primary_key(&color_id);
        self.color_id = Some(color_id);
        Ok(self)
    }
}
impl web_common_traits::prelude::SetPrimaryKey for InsertableObservationSubjectBuilder {
    type PrimaryKey = i16;
    fn set_primary_key(self, _primary_key: Self::PrimaryKey) -> Self {
        self
    }
}
impl<C> web_common_traits::database::TryInsertGeneric<C> for InsertableObservationSubjectBuilder
where
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject,
            Error = web_common_traits::database::InsertError<ObservationSubjectAttribute>,
        >,
{
    type Attribute = ObservationSubjectAttribute;
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<Self::Attribute>> {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject = self
            .insert(user_id, conn)?;
        Ok(insertable.id())
    }
}

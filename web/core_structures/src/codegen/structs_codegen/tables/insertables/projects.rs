#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ProjectAttribute {
    Id,
    Name,
    Description,
    StateId,
    Icon,
    ColorId,
    ParentProjectId,
    Budget,
    Expenses,
    CreatedBy,
    CreatedAt,
    UpdatedBy,
    UpdatedAt,
    ExpectedEndDate,
    EndDate,
}
impl core::str::FromStr for ProjectAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Id" => Ok(Self::Id),
            "Name" => Ok(Self::Name),
            "Description" => Ok(Self::Description),
            "StateId" => Ok(Self::StateId),
            "Icon" => Ok(Self::Icon),
            "ColorId" => Ok(Self::ColorId),
            "ParentProjectId" => Ok(Self::ParentProjectId),
            "Budget" => Ok(Self::Budget),
            "Expenses" => Ok(Self::Expenses),
            "CreatedBy" => Ok(Self::CreatedBy),
            "CreatedAt" => Ok(Self::CreatedAt),
            "UpdatedBy" => Ok(Self::UpdatedBy),
            "UpdatedAt" => Ok(Self::UpdatedAt),
            "ExpectedEndDate" => Ok(Self::ExpectedEndDate),
            "EndDate" => Ok(Self::EndDate),
            "id" => Ok(Self::Id),
            "name" => Ok(Self::Name),
            "description" => Ok(Self::Description),
            "state_id" => Ok(Self::StateId),
            "icon" => Ok(Self::Icon),
            "color_id" => Ok(Self::ColorId),
            "parent_project_id" => Ok(Self::ParentProjectId),
            "budget" => Ok(Self::Budget),
            "expenses" => Ok(Self::Expenses),
            "created_by" => Ok(Self::CreatedBy),
            "created_at" => Ok(Self::CreatedAt),
            "updated_by" => Ok(Self::UpdatedBy),
            "updated_at" => Ok(Self::UpdatedAt),
            "expected_end_date" => Ok(Self::ExpectedEndDate),
            "end_date" => Ok(Self::EndDate),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl common_traits::builder::Attributed
    for crate::codegen::structs_codegen::tables::insertables::InsertableProjectBuilder
{
    type Attribute = ProjectAttribute;
}
impl core::fmt::Display for ProjectAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Id => write!(f, "projects.id"),
            Self::Name => write!(f, "projects.name"),
            Self::Description => write!(f, "projects.description"),
            Self::StateId => write!(f, "projects.state_id"),
            Self::Icon => write!(f, "projects.icon"),
            Self::ColorId => write!(f, "projects.color_id"),
            Self::ParentProjectId => write!(f, "projects.parent_project_id"),
            Self::Budget => write!(f, "projects.budget"),
            Self::Expenses => write!(f, "projects.expenses"),
            Self::CreatedBy => write!(f, "projects.created_by"),
            Self::CreatedAt => write!(f, "projects.created_at"),
            Self::UpdatedBy => write!(f, "projects.updated_by"),
            Self::UpdatedAt => write!(f, "projects.updated_at"),
            Self::ExpectedEndDate => write!(f, "projects.expected_end_date"),
            Self::EndDate => write!(f, "projects.end_date"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(table_name = crate::codegen::diesel_codegen::tables::projects::projects)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableProject {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) state_id: i16,
    pub(crate) icon: String,
    pub(crate) color_id: i16,
    pub(crate) parent_project_id: Option<i32>,
    pub(crate) budget: Option<f64>,
    pub(crate) expenses: Option<f64>,
    pub(crate) created_by: i32,
    pub(crate) created_at: ::rosetta_timestamp::TimestampUTC,
    pub(crate) updated_by: i32,
    pub(crate) updated_at: ::rosetta_timestamp::TimestampUTC,
    pub(crate) expected_end_date: ::rosetta_timestamp::TimestampUTC,
    pub(crate) end_date: ::rosetta_timestamp::TimestampUTC,
}
impl InsertableProject {
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
    pub fn created_by<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::codegen::structs_codegen::tables::users::User, diesel::result::Error>
    where
        crate::codegen::structs_codegen::tables::users::User: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::users::User::read(self.created_by, conn)
    }
    pub fn parent_project<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::projects::Project>,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::projects::Project:
            web_common_traits::database::Read<C>,
    {
        use diesel::OptionalExtension;
        use web_common_traits::database::Read;
        let Some(parent_project_id) = self.parent_project_id else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::projects::Project::read(parent_project_id, conn)
            .optional()
    }
    pub fn state<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::project_states::ProjectState,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::project_states::ProjectState:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::project_states::ProjectState::read(
            self.state_id,
            conn,
        )
    }
    pub fn updated_by<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::codegen::structs_codegen::tables::users::User, diesel::result::Error>
    where
        crate::codegen::structs_codegen::tables::users::User: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::users::User::read(self.updated_by, conn)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Builder for creating and inserting a new
/// [`Project`](crate::codegen::structs_codegen::tables::projects::Project).
///
/// # Implementation details
/// While this builder implements several methods, a reasonably complete
/// **basic** usage example (*which may not apply to your own specific use case,
/// please adapt accordingly*) is as follows:
///
/// ```rust,ignore
/// use core_structures::Project;
/// use core_structures::tables::insertables::ProjectSettable;
/// use web_common_traits::database::Insertable;
/// use web_common_traits::database::InsertableVariant;
///
/// let project = Project::new()
///    // Set mandatory fields
///    .created_by(created_by)?
///    .description(description)?
///    .end_date(end_date)?
///    .expected_end_date(expected_end_date)?
///    .icon(icon)?
///    .id(id)?
///    .name(name)?
///    // Note: `updated_by` is automatically set by the `created by` column.
///    .updated_by(updated_by)?
///    // Optionally set fields with default values
///    .color(color_id)?
///    .created_at(created_at)?
///    .state(state_id)?
///    .updated_at(updated_at)?
///    // Optionally set optional fields
///    .budget(budget)?
///    .expenses(expenses)?
///    .parent_project(parent_project_id)?
///    // Finally, insert the new record in the database
///    .insert(user.id, conn)?;
/// ```
pub struct InsertableProjectBuilder {
    pub(crate) id: Option<i32>,
    pub(crate) name: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) state_id: Option<i16>,
    pub(crate) icon: Option<String>,
    pub(crate) color_id: Option<i16>,
    pub(crate) parent_project_id: Option<i32>,
    pub(crate) budget: Option<f64>,
    pub(crate) expenses: Option<f64>,
    pub(crate) created_by: Option<i32>,
    pub(crate) created_at: Option<::rosetta_timestamp::TimestampUTC>,
    pub(crate) updated_by: Option<i32>,
    pub(crate) updated_at: Option<::rosetta_timestamp::TimestampUTC>,
    pub(crate) expected_end_date: Option<::rosetta_timestamp::TimestampUTC>,
    pub(crate) end_date: Option<::rosetta_timestamp::TimestampUTC>,
}
impl From<InsertableProjectBuilder>
    for web_common_traits::database::IdOrBuilder<i32, InsertableProjectBuilder>
{
    fn from(builder: InsertableProjectBuilder) -> Self {
        Self::Builder(builder)
    }
}
impl Default for InsertableProjectBuilder {
    fn default() -> Self {
        Self {
            id: Default::default(),
            name: Default::default(),
            description: Default::default(),
            state_id: Some(1i16),
            icon: Default::default(),
            color_id: Some(1i16),
            parent_project_id: Default::default(),
            budget: Default::default(),
            expenses: Default::default(),
            created_by: Default::default(),
            created_at: Some(rosetta_timestamp::TimestampUTC::default()),
            updated_by: Default::default(),
            updated_at: Some(rosetta_timestamp::TimestampUTC::default()),
            expected_end_date: Default::default(),
            end_date: Default::default(),
        }
    }
}
impl common_traits::builder::IsCompleteBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableProjectBuilder
{
    fn is_complete(&self) -> bool {
        self.id.is_some()
            && self.name.is_some()
            && self.description.is_some()
            && self.state_id.is_some()
            && self.icon.is_some()
            && self.color_id.is_some()
            && self.created_by.is_some()
            && self.created_at.is_some()
            && self.updated_by.is_some()
            && self.updated_at.is_some()
            && self.expected_end_date.is_some()
            && self.end_date.is_some()
    }
}
/// Trait defining setters for attributes of an instance of `Project` or
/// descendant tables.
pub trait ProjectSettable: Sized {
    /// Attributes required to build the insertable.
    type Attributes;
    /// Sets the value of the `public.projects.id` column.
    ///
    /// # Arguments
    /// * `id`: The value to set for the `public.projects.id` column.
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
    /// * If the provided value cannot be converted to the required type `i32`.
    /// * If the provided value does not pass schema-defined validation.
    fn id<I>(
        self,
        id: I,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        I: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the `public.projects.name` column.
    ///
    /// # Arguments
    /// * `name`: The value to set for the `public.projects.name` column.
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
    /// Sets the value of the `public.projects.description` column.
    ///
    /// # Arguments
    /// * `description`: The value to set for the `public.projects.description`
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
    fn description<D>(
        self,
        description: D,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        D: TryInto<String>,
        validation_errors::SingleFieldError: From<<D as TryInto<String>>::Error>;
    /// Sets the value of the `public.projects.state_id` column.
    ///
    /// # Arguments
    /// * `state_id`: The value to set for the `public.projects.state_id`
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
    fn state<SI>(
        self,
        state_id: SI,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        SI: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i16>;
    /// Sets the value of the `public.projects.icon` column.
    ///
    /// # Arguments
    /// * `icon`: The value to set for the `public.projects.icon` column.
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
    /// Sets the value of the `public.projects.color_id` column.
    ///
    /// # Arguments
    /// * `color_id`: The value to set for the `public.projects.color_id`
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
    fn color<CI>(
        self,
        color_id: CI,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        CI: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i16>;
    /// Sets the value of the `public.projects.parent_project_id` column.
    ///
    /// # Arguments
    /// * `parent_project_id`: The value to set for the
    ///   `public.projects.parent_project_id` column.
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
    /// * If the provided value cannot be converted to the required type `i32`.
    /// * If the provided value does not pass schema-defined validation.
    fn parent_project<PPI>(
        self,
        parent_project_id: PPI,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PPI: web_common_traits::database::MaybePrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the `public.projects.budget` column.
    ///
    /// # Arguments
    /// * `budget`: The value to set for the `public.projects.budget` column.
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
    /// * If the provided value cannot be converted to the required type `f64`.
    /// * If the provided value does not pass schema-defined validation.
    fn budget<B>(
        self,
        budget: B,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        B: TryInto<Option<f64>>,
        validation_errors::SingleFieldError: From<<B as TryInto<Option<f64>>>::Error>;
    /// Sets the value of the `public.projects.expenses` column.
    ///
    /// # Arguments
    /// * `expenses`: The value to set for the `public.projects.expenses`
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
    /// * If the provided value cannot be converted to the required type `f64`.
    /// * If the provided value does not pass schema-defined validation.
    fn expenses<E>(
        self,
        expenses: E,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        E: TryInto<Option<f64>>,
        validation_errors::SingleFieldError: From<<E as TryInto<Option<f64>>>::Error>;
    /// Sets the value of the `public.projects.created_by` column.
    ///
    /// # Arguments
    /// * `created_by`: The value to set for the `public.projects.created_by`
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
    /// * If the provided value cannot be converted to the required type `i32`.
    /// * If the provided value does not pass schema-defined validation.
    fn created_by<CB>(
        self,
        created_by: CB,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        CB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the `public.projects.created_at` column.
    ///
    /// # Arguments
    /// * `created_at`: The value to set for the `public.projects.created_at`
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
    ///   `::rosetta_timestamp::TimestampUTC`.
    /// * If the provided value does not pass schema-defined validation.
    fn created_at<CA>(
        self,
        created_at: CA,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        CA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::SingleFieldError:
            From<<CA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error>;
    /// Sets the value of the `public.projects.updated_by` column.
    ///
    /// # Arguments
    /// * `updated_by`: The value to set for the `public.projects.updated_by`
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
    /// * If the provided value cannot be converted to the required type `i32`.
    /// * If the provided value does not pass schema-defined validation.
    fn updated_by<UB>(
        self,
        updated_by: UB,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        UB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the `public.projects.updated_at` column.
    ///
    /// # Arguments
    /// * `updated_at`: The value to set for the `public.projects.updated_at`
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
    ///   `::rosetta_timestamp::TimestampUTC`.
    /// * If the provided value does not pass schema-defined validation.
    fn updated_at<UA>(
        self,
        updated_at: UA,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        UA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::SingleFieldError:
            From<<UA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error>;
    /// Sets the value of the `public.projects.expected_end_date` column.
    ///
    /// # Arguments
    /// * `expected_end_date`: The value to set for the
    ///   `public.projects.expected_end_date` column.
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
    ///   `::rosetta_timestamp::TimestampUTC`.
    /// * If the provided value does not pass schema-defined validation.
    fn expected_end_date<EED>(
        self,
        expected_end_date: EED,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        EED: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::SingleFieldError:
            From<<EED as TryInto<::rosetta_timestamp::TimestampUTC>>::Error>;
    /// Sets the value of the `public.projects.end_date` column.
    ///
    /// # Arguments
    /// * `end_date`: The value to set for the `public.projects.end_date`
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
    ///   `::rosetta_timestamp::TimestampUTC`.
    /// * If the provided value does not pass schema-defined validation.
    fn end_date<ED>(
        self,
        end_date: ED,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        ED: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::SingleFieldError:
            From<<ED as TryInto<::rosetta_timestamp::TimestampUTC>>::Error>;
}
impl ProjectSettable for InsertableProjectBuilder {
    type Attributes = crate::codegen::structs_codegen::tables::insertables::ProjectAttribute;
    /// Sets the value of the `public.projects.id` column.
    fn id<I>(
        mut self,
        id: I,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        I: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let id = <I as web_common_traits::database::PrimaryKeyLike>::primary_key(&id);
        if let Some(parent_project_id) = self.parent_project_id {
            pgrx_validation::must_be_distinct_i32(parent_project_id, id)
                .map_err(|e| {
                    e
                        .rename_fields(
                            crate::codegen::structs_codegen::tables::insertables::ProjectAttribute::ParentProjectId,
                            crate::codegen::structs_codegen::tables::insertables::ProjectAttribute::Id,
                        )
                })?;
        }
        self.id = Some(id);
        Ok(self)
    }
    /// Sets the value of the `public.projects.name` column.
    fn name<N>(
        mut self,
        name: N,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        N: TryInto<String>,
        validation_errors::SingleFieldError: From<<N as TryInto<String>>::Error>,
    {
        let name = name.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err).rename_field(ProjectAttribute::Name)
        })?;
        if let Some(description) = self.description.as_ref() {
            pgrx_validation::must_be_distinct(name.as_ref(), description)
                .map_err(|e| {
                    e
                        .rename_fields(
                            crate::codegen::structs_codegen::tables::insertables::ProjectAttribute::Name,
                            crate::codegen::structs_codegen::tables::insertables::ProjectAttribute::Description,
                        )
                })?;
        }
        pgrx_validation::must_be_paragraph(name.as_ref()).map_err(|e| {
            e.rename_field(
                crate::codegen::structs_codegen::tables::insertables::ProjectAttribute::Name,
            )
        })?;
        self.name = Some(name);
        Ok(self)
    }
    /// Sets the value of the `public.projects.description` column.
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
                .rename_field(ProjectAttribute::Description)
        })?;
        if let Some(name) = self.name.as_ref() {
            pgrx_validation::must_be_distinct(name, description.as_ref())
                .map_err(|e| {
                    e
                        .rename_fields(
                            crate::codegen::structs_codegen::tables::insertables::ProjectAttribute::Name,
                            crate::codegen::structs_codegen::tables::insertables::ProjectAttribute::Description,
                        )
                })?;
        }
        pgrx_validation::must_be_paragraph(description.as_ref()).map_err(|e| {
            e.rename_field(
                crate::codegen::structs_codegen::tables::insertables::ProjectAttribute::Description,
            )
        })?;
        self.description = Some(description);
        Ok(self)
    }
    /// Sets the value of the `public.projects.state_id` column.
    fn state<SI>(
        mut self,
        state_id: SI,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        SI: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i16>,
    {
        let state_id = <SI as web_common_traits::database::PrimaryKeyLike>::primary_key(&state_id);
        self.state_id = Some(state_id);
        Ok(self)
    }
    /// Sets the value of the `public.projects.icon` column.
    fn icon<I>(
        mut self,
        icon: I,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        I: TryInto<String>,
        validation_errors::SingleFieldError: From<<I as TryInto<String>>::Error>,
    {
        let icon = icon.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err).rename_field(ProjectAttribute::Icon)
        })?;
        pgrx_validation::must_be_font_awesome_class(icon.as_ref()).map_err(|e| {
            e.rename_field(
                crate::codegen::structs_codegen::tables::insertables::ProjectAttribute::Icon,
            )
        })?;
        self.icon = Some(icon);
        Ok(self)
    }
    /// Sets the value of the `public.projects.color_id` column.
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
    /// Sets the value of the `public.projects.parent_project_id` column.
    fn parent_project<PPI>(
        mut self,
        parent_project_id: PPI,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PPI: web_common_traits::database::MaybePrimaryKeyLike<PrimaryKey = i32>,
    {
        let parent_project_id =
            <PPI as web_common_traits::database::MaybePrimaryKeyLike>::maybe_primary_key(
                &parent_project_id,
            );
        if let (Some(id), Some(parent_project_id)) = (self.id, parent_project_id) {
            pgrx_validation::must_be_distinct_i32(parent_project_id, id)
                .map_err(|e| {
                    e
                        .rename_fields(
                            crate::codegen::structs_codegen::tables::insertables::ProjectAttribute::ParentProjectId,
                            crate::codegen::structs_codegen::tables::insertables::ProjectAttribute::Id,
                        )
                })?;
        }
        self.parent_project_id = parent_project_id;
        Ok(self)
    }
    /// Sets the value of the `public.projects.budget` column.
    fn budget<B>(
        mut self,
        budget: B,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        B: TryInto<Option<f64>>,
        validation_errors::SingleFieldError: From<<B as TryInto<Option<f64>>>::Error>,
    {
        let budget = budget.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err).rename_field(ProjectAttribute::Budget)
        })?;
        self.budget = budget;
        Ok(self)
    }
    /// Sets the value of the `public.projects.expenses` column.
    fn expenses<E>(
        mut self,
        expenses: E,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        E: TryInto<Option<f64>>,
        validation_errors::SingleFieldError: From<<E as TryInto<Option<f64>>>::Error>,
    {
        let expenses = expenses.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err).rename_field(ProjectAttribute::Expenses)
        })?;
        self.expenses = expenses;
        Ok(self)
    }
    /// Sets the value of the `public.projects.created_by` column.
    ///
    /// # Implementation notes
    /// This method also set the values of other columns, due to
    /// same-as relationships or inferred values.
    ///
    /// ## Mermaid illustration
    ///
    /// ```mermaid
    /// flowchart BT
    /// classDef column-of-interest stroke: #f0746c,fill: #f49f9a
    /// classDef directly-involved-column stroke: #6c74f0,fill: #9a9ff4
    /// v0@{shape: rounded, label: "created_by"}
    /// class v0 column-of-interest
    /// v1@{shape: rounded, label: "updated_by"}
    /// class v1 directly-involved-column
    /// ```
    fn created_by<CB>(
        mut self,
        created_by: CB,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        CB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let created_by =
            <CB as web_common_traits::database::PrimaryKeyLike>::primary_key(&created_by);
        self = self.updated_by(created_by)?;
        self.created_by = Some(created_by);
        Ok(self)
    }
    /// Sets the value of the `public.projects.created_at` column.
    fn created_at<CA>(
        mut self,
        created_at: CA,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        CA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::SingleFieldError:
            From<<CA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error>,
    {
        let created_at = created_at.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err).rename_field(ProjectAttribute::CreatedAt)
        })?;
        if let Some(updated_at) = self.updated_at {
            pgrx_validation::must_be_smaller_than_utc(created_at, updated_at)
                .map_err(|e| {
                    e
                        .rename_fields(
                            crate::codegen::structs_codegen::tables::insertables::ProjectAttribute::CreatedAt,
                            crate::codegen::structs_codegen::tables::insertables::ProjectAttribute::UpdatedAt,
                        )
                })?;
        }
        self.created_at = Some(created_at);
        Ok(self)
    }
    /// Sets the value of the `public.projects.updated_by` column.
    fn updated_by<UB>(
        mut self,
        updated_by: UB,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        UB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let updated_by =
            <UB as web_common_traits::database::PrimaryKeyLike>::primary_key(&updated_by);
        self.updated_by = Some(updated_by);
        Ok(self)
    }
    /// Sets the value of the `public.projects.updated_at` column.
    fn updated_at<UA>(
        mut self,
        updated_at: UA,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        UA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::SingleFieldError:
            From<<UA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error>,
    {
        let updated_at = updated_at.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err).rename_field(ProjectAttribute::UpdatedAt)
        })?;
        if let Some(created_at) = self.created_at {
            pgrx_validation::must_be_smaller_than_utc(created_at, updated_at)
                .map_err(|e| {
                    e
                        .rename_fields(
                            crate::codegen::structs_codegen::tables::insertables::ProjectAttribute::CreatedAt,
                            crate::codegen::structs_codegen::tables::insertables::ProjectAttribute::UpdatedAt,
                        )
                })?;
        }
        self.updated_at = Some(updated_at);
        Ok(self)
    }
    /// Sets the value of the `public.projects.expected_end_date` column.
    fn expected_end_date<EED>(
        mut self,
        expected_end_date: EED,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        EED: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::SingleFieldError:
            From<<EED as TryInto<::rosetta_timestamp::TimestampUTC>>::Error>,
    {
        let expected_end_date = expected_end_date.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(ProjectAttribute::ExpectedEndDate)
        })?;
        self.expected_end_date = Some(expected_end_date);
        Ok(self)
    }
    /// Sets the value of the `public.projects.end_date` column.
    fn end_date<ED>(
        mut self,
        end_date: ED,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        ED: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::SingleFieldError:
            From<<ED as TryInto<::rosetta_timestamp::TimestampUTC>>::Error>,
    {
        let end_date = end_date.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err).rename_field(ProjectAttribute::EndDate)
        })?;
        self.end_date = Some(end_date);
        Ok(self)
    }
}
impl web_common_traits::prelude::SetPrimaryKey for InsertableProjectBuilder {
    type PrimaryKey = i32;
    fn set_primary_key(self, _primary_key: Self::PrimaryKey) -> Self {
        self
    }
}
impl<C> web_common_traits::database::TryInsertGeneric<C> for InsertableProjectBuilder
where
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::projects::Project,
            Attribute = ProjectAttribute,
        >,
{
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<Self::Attribute>> {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::projects::Project =
            self.insert(user_id, conn)?;
        Ok(insertable.id())
    }
}

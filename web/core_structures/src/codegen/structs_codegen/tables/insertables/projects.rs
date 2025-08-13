#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableProjectAttributes {
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
impl core::str::FromStr for InsertableProjectAttributes {
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
impl core::fmt::Display for InsertableProjectAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Id => write!(f, "id"),
            Self::Name => write!(f, "name"),
            Self::Description => write!(f, "description"),
            Self::StateId => write!(f, "state_id"),
            Self::Icon => write!(f, "icon"),
            Self::ColorId => write!(f, "color_id"),
            Self::ParentProjectId => write!(f, "parent_project_id"),
            Self::Budget => write!(f, "budget"),
            Self::Expenses => write!(f, "expenses"),
            Self::CreatedBy => write!(f, "created_by"),
            Self::CreatedAt => write!(f, "created_at"),
            Self::UpdatedBy => write!(f, "updated_by"),
            Self::UpdatedAt => write!(f, "updated_at"),
            Self::ExpectedEndDate => write!(f, "expected_end_date"),
            Self::EndDate => write!(f, "end_date"),
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
    pub fn created_by<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::users::User,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::users::User: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::users::User as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::users::User as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::users::User as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::users::User as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::users::User as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::users::User as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::users::User,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::users::User::table(),
                self.created_by,
            ),
            conn,
        )
    }
    pub fn updated_by<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::users::User,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::users::User: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::users::User as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::users::User as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::users::User as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::users::User as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::users::User as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::users::User as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::users::User,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::users::User::table(),
                self.updated_by,
            ),
            conn,
        )
    }
    pub fn state<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::project_states::ProjectState,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::project_states::ProjectState: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::project_states::ProjectState as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::project_states::ProjectState as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::project_states::ProjectState as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::project_states::ProjectState as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::project_states::ProjectState as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::project_states::ProjectState as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::project_states::ProjectState,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::project_states::ProjectState::table(),
                self.state_id,
            ),
            conn,
        )
    }
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
    pub fn parent_project<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::projects::Project>,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::projects::Project: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::projects::Project as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::projects::Project as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::projects::Project as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::projects::Project as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::projects::Project as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::projects::Project as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::projects::Project,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        let Some(parent_project_id) = self.parent_project_id else {
            return Ok(None);
        };
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::projects::Project::table(),
                parent_project_id,
            ),
            conn,
        )
        .map(Some)
    }
}
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
impl web_common_traits::database::ExtendableBuilder for InsertableProjectBuilder {
    type Attributes = InsertableProjectAttributes;
    fn extend_builder(
        mut self,
        other: Self,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        match (other.id, other.parent_project_id) {
            (Some(id), Some(parent_project_id)) => {
                self = self.id_and_parent_project(id, parent_project_id)?;
            }
            (None, Some(parent_project_id)) => {
                self = self.parent_project(Some(parent_project_id))?;
            }
            (Some(id), None) => {
                self = self.id(id)?;
            }
            (None, None) => {}
        }
        match (other.name, other.description) {
            (Some(name), Some(description)) => {
                self = self.name_and_description(name, description)?;
            }
            (None, Some(description)) => {
                self = self.description(description)?;
            }
            (Some(name), None) => {
                self = self.name(name)?;
            }
            (None, None) => {}
        }
        match (other.created_at, other.updated_at) {
            (Some(created_at), Some(updated_at)) => {
                self = self.created_at_and_updated_at(created_at, updated_at)?;
            }
            (None, Some(updated_at)) => {
                self = self.updated_at(updated_at)?;
            }
            (Some(created_at), None) => {
                self = self.created_at(created_at)?;
            }
            (None, None) => {}
        }
        if let Some(state_id) = other.state_id {
            self = self.state(state_id)?;
        }
        if let Some(icon) = other.icon {
            self = self.icon(icon)?;
        }
        if let Some(color_id) = other.color_id {
            self = self.color(color_id)?;
        }
        if let Some(budget) = other.budget {
            self = self.budget(Some(budget))?;
        }
        if let Some(expenses) = other.expenses {
            self = self.expenses(Some(expenses))?;
        }
        if let Some(created_by) = other.created_by {
            self = self.created_by(created_by)?;
        }
        if let Some(updated_by) = other.updated_by {
            self = self.updated_by(updated_by)?;
        }
        if let Some(expected_end_date) = other.expected_end_date {
            self = self.expected_end_date(expected_end_date)?;
        }
        if let Some(end_date) = other.end_date {
            self = self.end_date(end_date)?;
        }
        Ok(self)
    }
}
impl web_common_traits::prelude::SetPrimaryKey for InsertableProjectBuilder {
    type PrimaryKey = i32;
    fn set_primary_key(self, _primary_key: Self::PrimaryKey) -> Self {
        self
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableProjectBuilder {
    /// Sets the value of the `projects.budget` column from table `projects`.
    pub fn budget<Budget>(
        mut self,
        budget: Budget,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableProjectAttributes>>
    where
        Budget: TryInto<Option<f64>>,
        <Budget as TryInto<Option<f64>>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let budget =
            budget.try_into().map_err(|err: <Budget as TryInto<Option<f64>>>::Error| {
                Into::into(err).rename_field(InsertableProjectAttributes::Budget)
            })?;
        self.budget = budget;
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableProjectBuilder {
    /// Sets the value of the `projects.color_id` column from table `projects`.
    pub fn color(
        mut self,
        color_id: i16,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableProjectAttributes>> {
        self.color_id = Some(color_id);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableProjectBuilder {
    /// Sets the value of the `projects.created_at` column from table
    /// `projects`.
    pub fn created_at<CreatedAt>(
        mut self,
        created_at: CreatedAt,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableProjectAttributes>>
    where
        CreatedAt: TryInto<::rosetta_timestamp::TimestampUTC>,
        <CreatedAt as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let created_at = created_at.try_into().map_err(
            |err: <CreatedAt as TryInto<::rosetta_timestamp::TimestampUTC>>::Error| {
                Into::into(err).rename_field(InsertableProjectAttributes::CreatedAt)
            },
        )?;
        if let Some(updated_at) = self.updated_at {
            pgrx_validation::must_be_smaller_than_utc(created_at, updated_at)
                .map_err(|e| {
                    e
                        .rename_fields(
                            crate::codegen::structs_codegen::tables::insertables::InsertableProjectAttributes::CreatedAt,
                            crate::codegen::structs_codegen::tables::insertables::InsertableProjectAttributes::UpdatedAt,
                        )
                })?;
        }
        self.created_at = Some(created_at);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableProjectBuilder {
    /// Sets the value of the `projects.created_at`, `projects.updated_at`
    /// columns from table `projects`.
    pub fn created_at_and_updated_at<CreatedAt, UpdatedAt>(
        mut self,
        created_at: CreatedAt,
        updated_at: UpdatedAt,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableProjectAttributes>>
    where
        CreatedAt: TryInto<::rosetta_timestamp::TimestampUTC>,
        <CreatedAt as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
        UpdatedAt: TryInto<::rosetta_timestamp::TimestampUTC>,
        <UpdatedAt as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let created_at = created_at.try_into().map_err(
            |err: <CreatedAt as TryInto<::rosetta_timestamp::TimestampUTC>>::Error| {
                Into::into(err).rename_field(InsertableProjectAttributes::CreatedAt)
            },
        )?;
        let updated_at = updated_at.try_into().map_err(
            |err: <UpdatedAt as TryInto<::rosetta_timestamp::TimestampUTC>>::Error| {
                Into::into(err).rename_field(InsertableProjectAttributes::UpdatedAt)
            },
        )?;
        pgrx_validation::must_be_smaller_than_utc(created_at, updated_at)
            .map_err(|e| {
                e
                    .rename_fields(
                        crate::codegen::structs_codegen::tables::insertables::InsertableProjectAttributes::CreatedAt,
                        crate::codegen::structs_codegen::tables::insertables::InsertableProjectAttributes::UpdatedAt,
                    )
            })?;
        self.created_at = Some(created_at);
        self.updated_at = Some(updated_at);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableProjectBuilder {
    /// Sets the value of the `projects.created_by` column from table
    /// `projects`.
    pub fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableProjectAttributes>> {
        self.created_by = Some(created_by);
        self = self.updated_by(created_by)?;
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableProjectBuilder {
    /// Sets the value of the `projects.description` column from table
    /// `projects`.
    pub fn description<Description>(
        mut self,
        description: Description,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableProjectAttributes>>
    where
        Description: TryInto<String>,
        <Description as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let description =
            description.try_into().map_err(|err: <Description as TryInto<String>>::Error| {
                Into::into(err).rename_field(InsertableProjectAttributes::Description)
            })?;
        if let Some(name) = self.name.as_ref() {
            pgrx_validation::must_be_distinct(name, description.as_ref())
                .map_err(|e| {
                    e
                        .rename_fields(
                            crate::codegen::structs_codegen::tables::insertables::InsertableProjectAttributes::Name,
                            crate::codegen::structs_codegen::tables::insertables::InsertableProjectAttributes::Description,
                        )
                })?;
        }
        pgrx_validation::must_be_paragraph(description.as_ref())
            .map_err(|e| {
                e
                    .rename_field(
                        crate::codegen::structs_codegen::tables::insertables::InsertableProjectAttributes::Description,
                    )
            })?;
        self.description = Some(description);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableProjectBuilder {
    /// Sets the value of the `projects.end_date` column from table `projects`.
    pub fn end_date<EndDate>(
        mut self,
        end_date: EndDate,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableProjectAttributes>>
    where
        EndDate: TryInto<::rosetta_timestamp::TimestampUTC>,
        <EndDate as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let end_date = end_date.try_into().map_err(
            |err: <EndDate as TryInto<::rosetta_timestamp::TimestampUTC>>::Error| {
                Into::into(err).rename_field(InsertableProjectAttributes::EndDate)
            },
        )?;
        self.end_date = Some(end_date);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableProjectBuilder {
    /// Sets the value of the `projects.expected_end_date` column from table
    /// `projects`.
    pub fn expected_end_date<ExpectedEndDate>(
        mut self,
        expected_end_date: ExpectedEndDate,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableProjectAttributes>>
    where
        ExpectedEndDate: TryInto<::rosetta_timestamp::TimestampUTC>,
        <ExpectedEndDate as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let expected_end_date = expected_end_date.try_into().map_err(
            |err: <ExpectedEndDate as TryInto<::rosetta_timestamp::TimestampUTC>>::Error| {
                Into::into(err).rename_field(InsertableProjectAttributes::ExpectedEndDate)
            },
        )?;
        self.expected_end_date = Some(expected_end_date);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableProjectBuilder {
    /// Sets the value of the `projects.expenses` column from table `projects`.
    pub fn expenses<Expenses>(
        mut self,
        expenses: Expenses,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableProjectAttributes>>
    where
        Expenses: TryInto<Option<f64>>,
        <Expenses as TryInto<Option<f64>>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let expenses =
            expenses.try_into().map_err(|err: <Expenses as TryInto<Option<f64>>>::Error| {
                Into::into(err).rename_field(InsertableProjectAttributes::Expenses)
            })?;
        self.expenses = expenses;
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableProjectBuilder {
    /// Sets the value of the `projects.icon` column from table `projects`.
    pub fn icon<Icon>(
        mut self,
        icon: Icon,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableProjectAttributes>>
    where
        Icon: TryInto<String>,
        <Icon as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let icon = icon.try_into().map_err(|err: <Icon as TryInto<String>>::Error| {
            Into::into(err).rename_field(InsertableProjectAttributes::Icon)
        })?;
        pgrx_validation::must_be_font_awesome_class(icon.as_ref())
            .map_err(|e| {
                e
                    .rename_field(
                        crate::codegen::structs_codegen::tables::insertables::InsertableProjectAttributes::Icon,
                    )
            })?;
        self.icon = Some(icon);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableProjectBuilder {
    /// Sets the value of the `projects.id` column from table `projects`.
    pub fn id<Id>(
        mut self,
        id: Id,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableProjectAttributes>>
    where
        Id: TryInto<i32>,
        <Id as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let id = id.try_into().map_err(|err: <Id as TryInto<i32>>::Error| {
            Into::into(err).rename_field(InsertableProjectAttributes::Id)
        })?;
        if let Some(parent_project_id) = self.parent_project_id {
            pgrx_validation::must_be_distinct_i32(parent_project_id, id)
                .map_err(|e| {
                    e
                        .rename_fields(
                            crate::codegen::structs_codegen::tables::insertables::InsertableProjectAttributes::ParentProjectId,
                            crate::codegen::structs_codegen::tables::insertables::InsertableProjectAttributes::Id,
                        )
                })?;
        }
        self.id = Some(id);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableProjectBuilder {
    /// Sets the value of the `projects.id`, `projects.parent_project_id`
    /// columns from table `projects`.
    pub fn id_and_parent_project<Id>(
        mut self,
        id: Id,
        parent_project_id: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableProjectAttributes>>
    where
        Id: TryInto<i32>,
        <Id as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let id = id.try_into().map_err(|err: <Id as TryInto<i32>>::Error| {
            Into::into(err).rename_field(InsertableProjectAttributes::Id)
        })?;
        pgrx_validation::must_be_distinct_i32(parent_project_id, id)
            .map_err(|e| {
                e
                    .rename_fields(
                        crate::codegen::structs_codegen::tables::insertables::InsertableProjectAttributes::ParentProjectId,
                        crate::codegen::structs_codegen::tables::insertables::InsertableProjectAttributes::Id,
                    )
            })?;
        self.id = Some(id);
        self.parent_project_id = Some(parent_project_id);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableProjectBuilder {
    /// Sets the value of the `projects.name` column from table `projects`.
    pub fn name<Name>(
        mut self,
        name: Name,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableProjectAttributes>>
    where
        Name: TryInto<String>,
        <Name as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let name = name.try_into().map_err(|err: <Name as TryInto<String>>::Error| {
            Into::into(err).rename_field(InsertableProjectAttributes::Name)
        })?;
        if let Some(description) = self.description.as_ref() {
            pgrx_validation::must_be_distinct(name.as_ref(), description)
                .map_err(|e| {
                    e
                        .rename_fields(
                            crate::codegen::structs_codegen::tables::insertables::InsertableProjectAttributes::Name,
                            crate::codegen::structs_codegen::tables::insertables::InsertableProjectAttributes::Description,
                        )
                })?;
        }
        pgrx_validation::must_be_paragraph(name.as_ref())
            .map_err(|e| {
                e
                    .rename_field(
                        crate::codegen::structs_codegen::tables::insertables::InsertableProjectAttributes::Name,
                    )
            })?;
        self.name = Some(name);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableProjectBuilder {
    /// Sets the value of the `projects.name`, `projects.description` columns
    /// from table `projects`.
    pub fn name_and_description<Name, Description>(
        mut self,
        name: Name,
        description: Description,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableProjectAttributes>>
    where
        Name: TryInto<String>,
        <Name as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
        Description: TryInto<String>,
        <Description as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let name = name.try_into().map_err(|err: <Name as TryInto<String>>::Error| {
            Into::into(err).rename_field(InsertableProjectAttributes::Name)
        })?;
        let description =
            description.try_into().map_err(|err: <Description as TryInto<String>>::Error| {
                Into::into(err).rename_field(InsertableProjectAttributes::Description)
            })?;
        pgrx_validation::must_be_distinct(name.as_ref(), description.as_ref())
            .map_err(|e| {
                e
                    .rename_fields(
                        crate::codegen::structs_codegen::tables::insertables::InsertableProjectAttributes::Name,
                        crate::codegen::structs_codegen::tables::insertables::InsertableProjectAttributes::Description,
                    )
            })?;
        pgrx_validation::must_be_paragraph(description.as_ref())
            .map_err(|e| {
                e
                    .rename_field(
                        crate::codegen::structs_codegen::tables::insertables::InsertableProjectAttributes::Description,
                    )
            })?;
        pgrx_validation::must_be_paragraph(name.as_ref())
            .map_err(|e| {
                e
                    .rename_field(
                        crate::codegen::structs_codegen::tables::insertables::InsertableProjectAttributes::Name,
                    )
            })?;
        self.name = Some(name);
        self.description = Some(description);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableProjectBuilder {
    /// Sets the value of the `projects.parent_project_id` column from table
    /// `projects`.
    pub fn parent_project(
        mut self,
        parent_project_id: Option<i32>,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableProjectAttributes>> {
        if let (Some(id), Some(parent_project_id)) = (self.id, parent_project_id) {
            pgrx_validation::must_be_distinct_i32(parent_project_id, id)
                .map_err(|e| {
                    e
                        .rename_fields(
                            crate::codegen::structs_codegen::tables::insertables::InsertableProjectAttributes::ParentProjectId,
                            crate::codegen::structs_codegen::tables::insertables::InsertableProjectAttributes::Id,
                        )
                })?;
        }
        self.parent_project_id = parent_project_id;
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableProjectBuilder {
    /// Sets the value of the `projects.state_id` column from table `projects`.
    pub fn state(
        mut self,
        state_id: i16,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableProjectAttributes>> {
        self.state_id = Some(state_id);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableProjectBuilder {
    /// Sets the value of the `projects.updated_at` column from table
    /// `projects`.
    pub fn updated_at<UpdatedAt>(
        mut self,
        updated_at: UpdatedAt,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableProjectAttributes>>
    where
        UpdatedAt: TryInto<::rosetta_timestamp::TimestampUTC>,
        <UpdatedAt as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let updated_at = updated_at.try_into().map_err(
            |err: <UpdatedAt as TryInto<::rosetta_timestamp::TimestampUTC>>::Error| {
                Into::into(err).rename_field(InsertableProjectAttributes::UpdatedAt)
            },
        )?;
        if let Some(created_at) = self.created_at {
            pgrx_validation::must_be_smaller_than_utc(created_at, updated_at)
                .map_err(|e| {
                    e
                        .rename_fields(
                            crate::codegen::structs_codegen::tables::insertables::InsertableProjectAttributes::CreatedAt,
                            crate::codegen::structs_codegen::tables::insertables::InsertableProjectAttributes::UpdatedAt,
                        )
                })?;
        }
        self.updated_at = Some(updated_at);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableProjectBuilder {
    /// Sets the value of the `projects.updated_by` column from table
    /// `projects`.
    pub fn updated_by(
        mut self,
        updated_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableProjectAttributes>> {
        self.updated_by = Some(updated_by);
        Ok(self)
    }
}
impl<C> web_common_traits::database::TryInsertGeneric<C> for InsertableProjectBuilder
where
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::projects::Project,
            Error = web_common_traits::database::InsertError<InsertableProjectAttributes>,
        >,
{
    type Attributes = InsertableProjectAttributes;
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
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<Self::Attributes>> {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::projects::Project =
            self.insert(user_id, conn)?;
        Ok(insertable.id())
    }
}

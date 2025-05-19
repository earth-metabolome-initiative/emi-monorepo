#[derive(Clone, core::fmt::Debug)]
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
impl core::fmt::Display for InsertableProjectAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableProjectAttributes::Id => write!(f, "id"),
            InsertableProjectAttributes::Name => write!(f, "name"),
            InsertableProjectAttributes::Description => write!(f, "description"),
            InsertableProjectAttributes::StateId => write!(f, "state_id"),
            InsertableProjectAttributes::Icon => write!(f, "icon"),
            InsertableProjectAttributes::ColorId => write!(f, "color_id"),
            InsertableProjectAttributes::ParentProjectId => {
                write!(f, "parent_project_id")
            }
            InsertableProjectAttributes::Budget => write!(f, "budget"),
            InsertableProjectAttributes::Expenses => write!(f, "expenses"),
            InsertableProjectAttributes::CreatedBy => write!(f, "created_by"),
            InsertableProjectAttributes::CreatedAt => write!(f, "created_at"),
            InsertableProjectAttributes::UpdatedBy => write!(f, "updated_by"),
            InsertableProjectAttributes::UpdatedAt => write!(f, "updated_at"),
            InsertableProjectAttributes::ExpectedEndDate => {
                write!(f, "expected_end_date")
            }
            InsertableProjectAttributes::EndDate => write!(f, "end_date"),
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
    id: i32,
    name: String,
    description: String,
    state_id: i16,
    icon: String,
    color_id: i16,
    parent_project_id: Option<i32>,
    budget: Option<f64>,
    expenses: Option<f64>,
    created_by: i32,
    created_at: rosetta_timestamp::TimestampUTC,
    updated_by: i32,
    updated_at: rosetta_timestamp::TimestampUTC,
    expected_end_date: rosetta_timestamp::TimestampUTC,
    end_date: rosetta_timestamp::TimestampUTC,
}
impl InsertableProject {
    #[cfg(feature = "postgres")]
    pub async fn state(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::project_states::ProjectState,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::project_states::ProjectState::table()
            .filter(
                crate::codegen::diesel_codegen::tables::project_states::project_states::dsl::id
                    .eq(&self.state_id),
            )
            .first::<crate::codegen::structs_codegen::tables::project_states::ProjectState>(conn)
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
    #[cfg(feature = "postgres")]
    pub async fn parent_project(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::projects::Project>,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        let Some(parent_project_id) = self.parent_project_id.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::projects::Project::table()
            .filter(
                crate::codegen::diesel_codegen::tables::projects::projects::dsl::id
                    .eq(parent_project_id),
            )
            .first::<crate::codegen::structs_codegen::tables::projects::Project>(conn)
            .await
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub async fn created_by(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::users::User, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::users::User::table()
            .filter(
                crate::codegen::diesel_codegen::tables::users::users::dsl::id.eq(&self.created_by),
            )
            .first::<crate::codegen::structs_codegen::tables::users::User>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn updated_by(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::users::User, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::users::User::table()
            .filter(
                crate::codegen::diesel_codegen::tables::users::users::dsl::id.eq(&self.updated_by),
            )
            .first::<crate::codegen::structs_codegen::tables::users::User>(conn)
            .await
    }
}
pub struct InsertableProjectBuilder {
    id: Option<i32>,
    name: Option<String>,
    description: Option<String>,
    state_id: Option<i16>,
    icon: Option<String>,
    color_id: Option<i16>,
    parent_project_id: Option<i32>,
    budget: Option<f64>,
    expenses: Option<f64>,
    created_by: Option<i32>,
    created_at: Option<rosetta_timestamp::TimestampUTC>,
    updated_by: Option<i32>,
    updated_at: Option<rosetta_timestamp::TimestampUTC>,
    expected_end_date: Option<rosetta_timestamp::TimestampUTC>,
    end_date: Option<rosetta_timestamp::TimestampUTC>,
}
impl Default for InsertableProjectBuilder {
    fn default() -> Self {
        Self {
            id: None,
            name: None,
            description: None,
            state_id: Some(1i16),
            icon: None,
            color_id: Some(1i16),
            parent_project_id: None,
            budget: None,
            expenses: None,
            created_by: None,
            created_at: Some(rosetta_timestamp::TimestampUTC::default()),
            updated_by: None,
            updated_at: Some(rosetta_timestamp::TimestampUTC::default()),
            expected_end_date: None,
            end_date: None,
        }
    }
}
impl InsertableProjectBuilder {
    pub fn id<P>(mut self, id: P) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let id = id.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
            Into::into(err).rename_field(InsertableProjectAttributes::Id)
        })?;
        if let Some(parent_project_id) = self.parent_project_id {
            pgrx_validation::must_be_distinct_i32(parent_project_id, id).map_err(|e| {
                e.rename_fields(
                    InsertableProjectAttributes::ParentProjectId,
                    InsertableProjectAttributes::Id,
                )
            })?;
        }
        self.id = Some(id);
        Ok(self)
    }
    pub fn name<P>(
        mut self,
        name: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let name = name.try_into().map_err(|err: <P as TryInto<String>>::Error| {
            Into::into(err).rename_field(InsertableProjectAttributes::Name)
        })?;
        if let Some(description) = self.description.as_ref() {
            pgrx_validation::must_be_distinct(name.as_ref(), description).map_err(|e| {
                e.rename_fields(
                    InsertableProjectAttributes::Name,
                    InsertableProjectAttributes::Description,
                )
            })?;
        }
        pgrx_validation::must_be_paragraph(name.as_ref())
            .map_err(|e| e.rename_field(InsertableProjectAttributes::Name))?;
        self.name = Some(name);
        Ok(self)
    }
    pub fn description<P>(
        mut self,
        description: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let description =
            description.try_into().map_err(|err: <P as TryInto<String>>::Error| {
                Into::into(err).rename_field(InsertableProjectAttributes::Description)
            })?;
        if let Some(name) = self.name.as_ref() {
            pgrx_validation::must_be_distinct(name, description.as_ref()).map_err(|e| {
                e.rename_fields(
                    InsertableProjectAttributes::Name,
                    InsertableProjectAttributes::Description,
                )
            })?;
        }
        pgrx_validation::must_be_paragraph(description.as_ref())
            .map_err(|e| e.rename_field(InsertableProjectAttributes::Description))?;
        self.description = Some(description);
        Ok(self)
    }
    pub fn state_id<P>(
        mut self,
        state_id: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<i16>,
        <P as TryInto<i16>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let state_id = state_id.try_into().map_err(|err: <P as TryInto<i16>>::Error| {
            Into::into(err).rename_field(InsertableProjectAttributes::StateId)
        })?;
        self.state_id = Some(state_id);
        Ok(self)
    }
    pub fn icon<P>(
        mut self,
        icon: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let icon = icon.try_into().map_err(|err: <P as TryInto<String>>::Error| {
            Into::into(err).rename_field(InsertableProjectAttributes::Icon)
        })?;
        pgrx_validation::must_be_font_awesome_class(icon.as_ref())
            .map_err(|e| e.rename_field(InsertableProjectAttributes::Icon))?;
        self.icon = Some(icon);
        Ok(self)
    }
    pub fn color_id<P>(
        mut self,
        color_id: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<i16>,
        <P as TryInto<i16>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let color_id = color_id.try_into().map_err(|err: <P as TryInto<i16>>::Error| {
            Into::into(err).rename_field(InsertableProjectAttributes::ColorId)
        })?;
        self.color_id = Some(color_id);
        Ok(self)
    }
    pub fn parent_project_id<P>(
        mut self,
        parent_project_id: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<Option<i32>>,
        <P as TryInto<Option<i32>>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let parent_project_id =
            parent_project_id.try_into().map_err(|err: <P as TryInto<Option<i32>>>::Error| {
                Into::into(err).rename_field(InsertableProjectAttributes::ParentProjectId)
            })?;
        if let (Some(id), Some(parent_project_id)) = (self.id, parent_project_id) {
            pgrx_validation::must_be_distinct_i32(parent_project_id, id).map_err(|e| {
                e.rename_fields(
                    InsertableProjectAttributes::ParentProjectId,
                    InsertableProjectAttributes::Id,
                )
            })?;
        }
        self.parent_project_id = parent_project_id;
        Ok(self)
    }
    pub fn budget<P>(
        mut self,
        budget: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<Option<f64>>,
        <P as TryInto<Option<f64>>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let budget = budget.try_into().map_err(|err: <P as TryInto<Option<f64>>>::Error| {
            Into::into(err).rename_field(InsertableProjectAttributes::Budget)
        })?;
        self.budget = budget;
        Ok(self)
    }
    pub fn expenses<P>(
        mut self,
        expenses: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<Option<f64>>,
        <P as TryInto<Option<f64>>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let expenses = expenses.try_into().map_err(|err: <P as TryInto<Option<f64>>>::Error| {
            Into::into(err).rename_field(InsertableProjectAttributes::Expenses)
        })?;
        self.expenses = expenses;
        Ok(self)
    }
    pub fn created_by<P>(
        mut self,
        created_by: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let created_by = created_by.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
            Into::into(err).rename_field(InsertableProjectAttributes::CreatedBy)
        })?;
        self.created_by = Some(created_by);
        self = self.updated_by(created_by)?;
        Ok(self)
    }
    pub fn created_at<P>(
        mut self,
        created_at: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<rosetta_timestamp::TimestampUTC>,
        <P as TryInto<rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let created_at = created_at.try_into().map_err(
            |err: <P as TryInto<rosetta_timestamp::TimestampUTC>>::Error| {
                Into::into(err).rename_field(InsertableProjectAttributes::CreatedAt)
            },
        )?;
        self.created_at = Some(created_at);
        Ok(self)
    }
    pub fn updated_by<P>(
        mut self,
        updated_by: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let updated_by = updated_by.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
            Into::into(err).rename_field(InsertableProjectAttributes::UpdatedBy)
        })?;
        self.updated_by = Some(updated_by);
        Ok(self)
    }
    pub fn updated_at<P>(
        mut self,
        updated_at: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<rosetta_timestamp::TimestampUTC>,
        <P as TryInto<rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let updated_at = updated_at.try_into().map_err(
            |err: <P as TryInto<rosetta_timestamp::TimestampUTC>>::Error| {
                Into::into(err).rename_field(InsertableProjectAttributes::UpdatedAt)
            },
        )?;
        self.updated_at = Some(updated_at);
        Ok(self)
    }
    pub fn expected_end_date<P>(
        mut self,
        expected_end_date: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<rosetta_timestamp::TimestampUTC>,
        <P as TryInto<rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let expected_end_date = expected_end_date.try_into().map_err(
            |err: <P as TryInto<rosetta_timestamp::TimestampUTC>>::Error| {
                Into::into(err).rename_field(InsertableProjectAttributes::ExpectedEndDate)
            },
        )?;
        self.expected_end_date = Some(expected_end_date);
        Ok(self)
    }
    pub fn end_date<P>(
        mut self,
        end_date: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<rosetta_timestamp::TimestampUTC>,
        <P as TryInto<rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let end_date = end_date.try_into().map_err(
            |err: <P as TryInto<rosetta_timestamp::TimestampUTC>>::Error| {
                Into::into(err).rename_field(InsertableProjectAttributes::EndDate)
            },
        )?;
        self.end_date = Some(end_date);
        Ok(self)
    }
}
impl common_traits::prelude::Builder for InsertableProjectBuilder {
    type Error = web_common_traits::database::InsertError<InsertableProjectAttributes>;
    type Object = InsertableProject;
    type Attribute = InsertableProjectAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            id: self.id.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableProjectAttributes::Id,
            ))?,
            name: self.name.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableProjectAttributes::Name,
            ))?,
            description: self.description.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProjectAttributes::Description,
                ),
            )?,
            state_id: self.state_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProjectAttributes::StateId,
                ),
            )?,
            icon: self.icon.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableProjectAttributes::Icon,
            ))?,
            color_id: self.color_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProjectAttributes::ColorId,
                ),
            )?,
            parent_project_id: self.parent_project_id,
            budget: self.budget,
            expenses: self.expenses,
            created_by: self.created_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProjectAttributes::CreatedBy,
                ),
            )?,
            created_at: self.created_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProjectAttributes::CreatedAt,
                ),
            )?,
            updated_by: self.updated_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProjectAttributes::UpdatedBy,
                ),
            )?,
            updated_at: self.updated_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProjectAttributes::UpdatedAt,
                ),
            )?,
            expected_end_date: self.expected_end_date.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProjectAttributes::ExpectedEndDate,
                ),
            )?,
            end_date: self.end_date.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProjectAttributes::EndDate,
                ),
            )?,
        })
    }
}
impl TryFrom<InsertableProject> for InsertableProjectBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableProject) -> Result<Self, Self::Error> {
        Self::default()
            .id(insertable_variant.id)?
            .name(insertable_variant.name)?
            .description(insertable_variant.description)?
            .state_id(insertable_variant.state_id)?
            .icon(insertable_variant.icon)?
            .color_id(insertable_variant.color_id)?
            .parent_project_id(insertable_variant.parent_project_id)?
            .budget(insertable_variant.budget)?
            .expenses(insertable_variant.expenses)?
            .created_by(insertable_variant.created_by)?
            .created_at(insertable_variant.created_at)?
            .updated_by(insertable_variant.updated_by)?
            .updated_at(insertable_variant.updated_at)?
            .expected_end_date(insertable_variant.expected_end_date)?
            .end_date(insertable_variant.end_date)
    }
}

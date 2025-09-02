#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableTeamProjectAttributes {
    TeamId,
    ProjectId,
}
impl core::str::FromStr for InsertableTeamProjectAttributes {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "TeamId" => Ok(Self::TeamId),
            "ProjectId" => Ok(Self::ProjectId),
            "team_id" => Ok(Self::TeamId),
            "project_id" => Ok(Self::ProjectId),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl core::fmt::Display for InsertableTeamProjectAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::TeamId => write!(f, "team_id"),
            Self::ProjectId => write!(f, "project_id"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::team_projects::team_projects
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableTeamProject {
    pub(crate) team_id: i32,
    pub(crate) project_id: i32,
}
impl InsertableTeamProject {
    pub fn team<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::teams::Team,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::teams::Team: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::teams::Team as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::teams::Team as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::teams::Team as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::teams::Team as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::teams::Team as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::teams::Team as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::teams::Team,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::teams::Team::table(),
                self.team_id,
            ),
            conn,
        )
    }
    pub fn project<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::projects::Project,
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
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::projects::Project::table(),
                self.project_id,
            ),
            conn,
        )
    }
}
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableTeamProjectBuilder {
    pub(crate) team_id: Option<i32>,
    pub(crate) project_id: Option<i32>,
}
/// Trait defining setters for attributes of an instance of `TeamProject` or
/// descendant tables.
pub trait TeamProjectBuildable: Sized {
    /// Attributes required to build the insertable.
    type Attributes;
    /// Sets the value of the `public.team_projects.team_id` column.
    ///
    /// # Arguments
    /// * `team_id`: The value to set for the `public.team_projects.team_id`
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
    fn team(
        self,
        team_id: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the `public.team_projects.project_id` column.
    ///
    /// # Arguments
    /// * `project_id`: The value to set for the
    ///   `public.team_projects.project_id` column.
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
    fn project(
        self,
        project_id: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
}
impl TeamProjectBuildable for InsertableTeamProjectBuilder {
    type Attributes =
        crate::codegen::structs_codegen::tables::insertables::InsertableTeamProjectAttributes;
    /// Sets the value of the `public.team_projects.team_id` column.
    fn team(
        mut self,
        team_id: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        let team_id = team_id.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(InsertableTeamProjectAttributes::TeamId)
        })?;
        self.team_id = Some(team_id);
        Ok(self)
    }
    /// Sets the value of the `public.team_projects.project_id` column.
    fn project(
        mut self,
        project_id: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        let project_id = project_id.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(InsertableTeamProjectAttributes::ProjectId)
        })?;
        self.project_id = Some(project_id);
        Ok(self)
    }
}
impl web_common_traits::prelude::SetPrimaryKey for InsertableTeamProjectBuilder {
    type PrimaryKey = (i32, i32);
    fn set_primary_key(self, _primary_key: Self::PrimaryKey) -> Self {
        self
    }
}
impl<C> web_common_traits::database::TryInsertGeneric<C> for InsertableTeamProjectBuilder
where
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::team_projects::TeamProject,
            Error = web_common_traits::database::InsertError<InsertableTeamProjectAttributes>,
        >,
{
    type Attributes = InsertableTeamProjectAttributes;
    fn is_complete(&self) -> bool {
        self.team_id.is_some() && self.project_id.is_some()
    }
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<Self::Attributes>> {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::team_projects::TeamProject =
            self.insert(user_id, conn)?;
        Ok(insertable.id())
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableTeamProjectAttributes {
    TeamId,
    ProjectId,
}
impl core::fmt::Display for InsertableTeamProjectAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableTeamProjectAttributes::TeamId => write!(f, "team_id"),
            InsertableTeamProjectAttributes::ProjectId => write!(f, "project_id"),
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
    team_id: i32,
    project_id: i32,
}
impl InsertableTeamProject {
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
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, RunQueryDsl};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::projects::Project::table(),
                self.project_id,
            ),
            conn,
        )
    }
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
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, RunQueryDsl};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::teams::Team::table(),
                self.team_id,
            ),
            conn,
        )
    }
}
#[derive(Default)]
pub struct InsertableTeamProjectBuilder {
    pub(crate) team_id: Option<i32>,
    pub(crate) project_id: Option<i32>,
}
impl InsertableTeamProjectBuilder {
    pub fn team_id<P>(
        mut self,
        team_id: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableTeamProjectAttributes>>
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let team_id = team_id.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
            Into::into(err).rename_field(InsertableTeamProjectAttributes::TeamId)
        })?;
        self.team_id = Some(team_id);
        Ok(self)
    }
    pub fn project_id<P>(
        mut self,
        project_id: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableTeamProjectAttributes>>
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let project_id = project_id.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
            Into::into(err).rename_field(InsertableTeamProjectAttributes::ProjectId)
        })?;
        self.project_id = Some(project_id);
        Ok(self)
    }
}
impl TryFrom<InsertableTeamProjectBuilder> for InsertableTeamProject {
    type Error = common_traits::prelude::BuilderError<InsertableTeamProjectAttributes>;
    fn try_from(
        builder: InsertableTeamProjectBuilder,
    ) -> Result<InsertableTeamProject, Self::Error> {
        Ok(Self {
            team_id: builder.team_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableTeamProjectAttributes::TeamId,
                ),
            )?,
            project_id: builder.project_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableTeamProjectAttributes::ProjectId,
                ),
            )?,
        })
    }
}

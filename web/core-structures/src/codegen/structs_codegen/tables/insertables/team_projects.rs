#[derive(Clone, core::fmt::Debug)]
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
    #[cfg(feature = "postgres")]
    pub async fn team(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::teams::Team, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::teams::Team::table()
            .filter(crate::codegen::diesel_codegen::tables::teams::teams::dsl::id.eq(&self.team_id))
            .first::<crate::codegen::structs_codegen::tables::teams::Team>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn project(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::projects::Project, diesel::result::Error>
    {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::projects::Project::table()
            .filter(
                crate::codegen::diesel_codegen::tables::projects::projects::dsl::id
                    .eq(&self.project_id),
            )
            .first::<crate::codegen::structs_codegen::tables::projects::Project>(conn)
            .await
    }
}
#[derive(Default)]
pub struct InsertableTeamProjectBuilder {
    team_id: Option<i32>,
    project_id: Option<i32>,
}
impl InsertableTeamProjectBuilder {
    pub fn team_id(
        mut self,
        team_id: i32,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.team_id = Some(team_id);
        Ok(self)
    }
    pub fn project_id(
        mut self,
        project_id: i32,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.project_id = Some(project_id);
        Ok(self)
    }
}
impl common_traits::prelude::Builder for InsertableTeamProjectBuilder {
    type Error = web_common_traits::database::InsertError<InsertableTeamProjectAttributes>;
    type Object = InsertableTeamProject;
    type Attribute = InsertableTeamProjectAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            team_id: self.team_id.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableTeamProjectAttributes::TeamId,
                )
            })?,
            project_id: self.project_id.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableTeamProjectAttributes::ProjectId,
                )
            })?,
        })
    }
}
impl TryFrom<InsertableTeamProject> for InsertableTeamProjectBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableTeamProject) -> Result<Self, Self::Error> {
        Ok(Self::default()
            .team_id(insertable_variant.team_id)?
            .project_id(insertable_variant.project_id)?)
    }
}

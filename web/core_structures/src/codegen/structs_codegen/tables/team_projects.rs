#[derive(Debug, Clone, PartialEq, Copy, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(diesel::Selectable, diesel::Insertable, diesel::Queryable, diesel::Identifiable)]
#[cfg_attr(feature = "yew", derive(yew::prelude::Properties))]
#[diesel(primary_key(team_id, project_id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::team_projects::team_projects
)]
pub struct TeamProject {
    pub team_id: i32,
    pub project_id: i32,
}
impl web_common_traits::prelude::TableName for TeamProject {
    const TABLE_NAME: &'static str = "team_projects";
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::team_projects::TeamProject,
    > for TeamProject
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a (i32, i32)>,
{
}
impl diesel::Identifiable for TeamProject {
    type Id = (i32, i32);
    fn id(self) -> Self::Id {
        (self.team_id, self.project_id)
    }
}
impl TeamProject {
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
    #[cfg(feature = "postgres")]
    pub fn from_team_id(
        team_id: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::team_projects::team_projects;
        Self::table()
            .filter(team_projects::team_id.eq(team_id))
            .order_by((team_projects::team_id.asc(), team_projects::project_id.asc()))
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_project_id(
        project_id: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::team_projects::team_projects;
        Self::table()
            .filter(team_projects::project_id.eq(project_id))
            .order_by((team_projects::team_id.asc(), team_projects::project_id.asc()))
            .load::<Self>(conn)
    }
}
impl AsRef<TeamProject> for TeamProject {
    fn as_ref(&self) -> &TeamProject {
        self
    }
}

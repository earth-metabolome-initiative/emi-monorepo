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
impl diesel::Identifiable for TeamProject {
    type Id = (i32, i32);
    fn id(self) -> Self::Id {
        (self.team_id, self.project_id)
    }
}
impl TeamProject {
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
    #[cfg(feature = "postgres")]
    pub async fn from_team_id(
        conn: &mut diesel_async::AsyncPgConnection,
        team_id: &crate::codegen::structs_codegen::tables::teams::Team,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::team_projects::team_projects::dsl::team_id
                    .eq(team_id.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_project_id(
        conn: &mut diesel_async::AsyncPgConnection,
        project_id: &crate::codegen::structs_codegen::tables::projects::Project,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::team_projects::team_projects::dsl::project_id
                    .eq(project_id.id),
            )
            .load::<Self>(conn)
            .await
    }
}

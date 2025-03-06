#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "diesel", derive(diesel::Selectable, diesel::Queryable, diesel::Identifiable))]
#[cfg_attr(feature = "diesel", diesel(primary_key(id)))]
# [cfg_attr (feature = "diesel" , diesel (table_name = crate :: codegen :: diesel_codegen :: tables :: teams :: teams))]
pub struct Team {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub parent_team_id: Option<i32>,
    pub created_by: i32,
    pub updated_by: i32,
    pub icon_id: i16,
    pub color_id: i16,
    pub state_id: i16,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
impl Team {
    #[cfg(feature = "postgres")]
    pub async fn parent_team(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<Option<crate::codegen::structs_codegen::tables::teams::Team>, diesel::result::Error>
    {
        use diesel::{associations::HasTable, QueryDsl};
        use diesel_async::RunQueryDsl;
        let Some(parent_team_id) = self.parent_team_id.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::teams::Team::table()
            .find(parent_team_id)
            .first::<crate::codegen::structs_codegen::tables::teams::Team>(conn)
            .await
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub async fn created_by(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::users::User, diesel::result::Error> {
        use diesel::{associations::HasTable, QueryDsl};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::users::User::table()
            .find(&self.created_by)
            .first::<crate::codegen::structs_codegen::tables::users::User>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn updated_by(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::users::User, diesel::result::Error> {
        use diesel::{associations::HasTable, QueryDsl};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::users::User::table()
            .find(&self.updated_by)
            .first::<crate::codegen::structs_codegen::tables::users::User>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn icon(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::icons::Icon, diesel::result::Error> {
        use diesel::{associations::HasTable, QueryDsl};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::icons::Icon::table()
            .find(&self.icon_id)
            .first::<crate::codegen::structs_codegen::tables::icons::Icon>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn color(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::colors::Color, diesel::result::Error> {
        use diesel::{associations::HasTable, QueryDsl};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::colors::Color::table()
            .find(&self.color_id)
            .first::<crate::codegen::structs_codegen::tables::colors::Color>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn state(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::team_states::TeamState,
        diesel::result::Error,
    > {
        use diesel::{associations::HasTable, QueryDsl};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::team_states::TeamState::table()
            .find(&self.state_id)
            .first::<crate::codegen::structs_codegen::tables::team_states::TeamState>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_name(
        name: &str,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{associations::HasTable, OptionalExtension, QueryDsl};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(diesel::ExpressionMethods::eq(
                crate::codegen::diesel_codegen::tables::teams::teams::name,
                name,
            ))
            .first::<Self>(conn)
            .await
            .optional()
    }
}

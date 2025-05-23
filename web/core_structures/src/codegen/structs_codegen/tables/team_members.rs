#[derive(Debug, Clone, PartialEq, Copy, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(diesel::Selectable, diesel::Insertable, diesel::Queryable, diesel::Identifiable)]
#[cfg_attr(feature = "yew", derive(yew::prelude::Properties))]
#[diesel(primary_key(team_id, member_id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::team_members::team_members
)]
pub struct TeamMember {
    pub team_id: i32,
    pub member_id: i32,
}
impl diesel::Identifiable for TeamMember {
    type Id = (i32, i32);
    fn id(self) -> Self::Id {
        (self.team_id, self.member_id)
    }
}
impl TeamMember {
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
    pub async fn member(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::users::User, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::users::User::table()
            .filter(
                crate::codegen::diesel_codegen::tables::users::users::dsl::id.eq(&self.member_id),
            )
            .first::<crate::codegen::structs_codegen::tables::users::User>(conn)
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
                crate::codegen::diesel_codegen::tables::team_members::team_members::dsl::team_id
                    .eq(team_id.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_member_id(
        conn: &mut diesel_async::AsyncPgConnection,
        member_id: &crate::codegen::structs_codegen::tables::users::User,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::team_members::team_members::dsl::member_id
                    .eq(member_id.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_team_id_and_member_id(
        team_id: &i32,
        member_id: &i32,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, OptionalExtension, QueryDsl,
            associations::HasTable,
        };
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::team_members::team_members::team_id
                    .eq(team_id)
                    .and(
                        crate::codegen::diesel_codegen::tables::team_members::team_members::member_id
                            .eq(member_id),
                    ),
            )
            .first::<Self>(conn)
            .await
            .optional()
    }
}

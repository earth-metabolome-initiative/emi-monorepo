#[derive(Clone, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableTeamMemberAttributes {
    TeamId,
    MemberId,
}
impl core::fmt::Display for InsertableTeamMemberAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableTeamMemberAttributes::TeamId => write!(f, "team_id"),
            InsertableTeamMemberAttributes::MemberId => write!(f, "member_id"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::team_members::team_members
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableTeamMember {
    team_id: i32,
    member_id: i32,
}
impl InsertableTeamMember {
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
}
#[derive(Default)]
pub struct InsertableTeamMemberBuilder {
    team_id: Option<i32>,
    member_id: Option<i32>,
}
impl InsertableTeamMemberBuilder {
    pub fn team_id<P: Into<i32>>(
        mut self,
        team_id: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let team_id = team_id.into();
        self.team_id = Some(team_id);
        Ok(self)
    }
    pub fn member_id<P: Into<i32>>(
        mut self,
        member_id: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let member_id = member_id.into();
        self.member_id = Some(member_id);
        Ok(self)
    }
}
impl common_traits::prelude::Builder for InsertableTeamMemberBuilder {
    type Error = web_common_traits::database::InsertError<InsertableTeamMemberAttributes>;
    type Object = InsertableTeamMember;
    type Attribute = InsertableTeamMemberAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            team_id: self.team_id.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableTeamMemberAttributes::TeamId,
            ))?,
            member_id: self.member_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableTeamMemberAttributes::MemberId,
                ),
            )?,
        })
    }
}
impl TryFrom<InsertableTeamMember> for InsertableTeamMemberBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableTeamMember) -> Result<Self, Self::Error> {
        Self::default().team_id(insertable_variant.team_id)?.member_id(insertable_variant.member_id)
    }
}

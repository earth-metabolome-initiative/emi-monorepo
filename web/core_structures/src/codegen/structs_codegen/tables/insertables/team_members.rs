#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
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
    pub fn member<C: diesel::connection::LoadConnection>(
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
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, RunQueryDsl};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::users::User::table(),
                self.member_id,
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
pub struct InsertableTeamMemberBuilder {
    pub(crate) team_id: Option<i32>,
    pub(crate) member_id: Option<i32>,
}
impl InsertableTeamMemberBuilder {
    pub fn team_id<P>(
        mut self,
        team_id: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableTeamMemberAttributes>>
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let team_id = team_id.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
            Into::into(err).rename_field(InsertableTeamMemberAttributes::TeamId)
        })?;
        self.team_id = Some(team_id);
        Ok(self)
    }
    pub fn member_id<P>(
        mut self,
        member_id: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableTeamMemberAttributes>>
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let member_id = member_id.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
            Into::into(err).rename_field(InsertableTeamMemberAttributes::MemberId)
        })?;
        self.member_id = Some(member_id);
        Ok(self)
    }
}
impl TryFrom<InsertableTeamMemberBuilder> for InsertableTeamMember {
    type Error = common_traits::prelude::BuilderError<InsertableTeamMemberAttributes>;
    fn try_from(builder: InsertableTeamMemberBuilder) -> Result<InsertableTeamMember, Self::Error> {
        Ok(Self {
            team_id: builder.team_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableTeamMemberAttributes::TeamId,
                ),
            )?,
            member_id: builder.member_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableTeamMemberAttributes::MemberId,
                ),
            )?,
        })
    }
}

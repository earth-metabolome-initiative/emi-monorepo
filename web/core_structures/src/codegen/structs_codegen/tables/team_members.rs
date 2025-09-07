#[derive(Debug, Clone, PartialEq, Copy, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    diesel::Selectable,
    diesel::Insertable,
    diesel::Queryable,
    diesel::Identifiable,
    diesel::Associations,
)]
#[cfg_attr(feature = "yew", derive(yew::prelude::Properties))]
#[diesel(belongs_to(crate::User, foreign_key = member_id))]
#[diesel(belongs_to(crate::Team, foreign_key = team_id))]
#[diesel(primary_key(team_id, member_id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::team_members::team_members
)]
pub struct TeamMember {
    pub team_id: i32,
    pub member_id: i32,
}
impl web_common_traits::prelude::TableName for TeamMember {
    const TABLE_NAME: &'static str = "team_members";
}
impl web_common_traits::prelude::ExtensionTable<crate::TeamMember> for TeamMember where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a (i32, i32)>
{
}
impl diesel::Identifiable for TeamMember {
    type Id = (i32, i32);
    fn id(self) -> Self::Id {
        (self.team_id, self.member_id)
    }
}
impl TeamMember {
    pub fn member<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::User, diesel::result::Error>
    where
        crate::User: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::User::read(self.member_id, conn)
    }
    pub fn team<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::Team, diesel::result::Error>
    where
        crate::Team: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::Team::read(self.team_id, conn)
    }
}
impl AsRef<TeamMember> for TeamMember {
    fn as_ref(&self) -> &TeamMember {
        self
    }
}

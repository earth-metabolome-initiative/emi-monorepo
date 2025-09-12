#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TeamMemberAttribute {
    TeamId,
    MemberId,
}
impl core::str::FromStr for TeamMemberAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "TeamId" => Ok(Self::TeamId),
            "MemberId" => Ok(Self::MemberId),
            "team_id" => Ok(Self::TeamId),
            "member_id" => Ok(Self::MemberId),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl core::fmt::Display for TeamMemberAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::TeamId => write!(f, "team_members.team_id"),
            Self::MemberId => write!(f, "team_members.member_id"),
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
    pub(crate) team_id: i32,
    pub(crate) member_id: i32,
}
impl InsertableTeamMember {
    pub fn member<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::codegen::structs_codegen::tables::users::User, diesel::result::Error>
    where
        crate::codegen::structs_codegen::tables::users::User: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::users::User::read(self.member_id, conn)
    }
    pub fn team<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::codegen::structs_codegen::tables::teams::Team, diesel::result::Error>
    where
        crate::codegen::structs_codegen::tables::teams::Team: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::teams::Team::read(self.team_id, conn)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Hash, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Builder for creating and inserting a new
/// [`TeamMember`](crate::codegen::structs_codegen::tables::team_members::TeamMember).
///
///
/// # Implementation details
/// While this builder implements several methods, a reasonably complete
/// **basic** usage example (*which may not apply to your own specific use case,
/// please adapt accordingly*) is as follows:
///
/// ```rust,ignore
/// use core_structures::TeamMember;
/// use core_structures::tables::insertables::TeamMemberSettable;
/// use web_common_traits::database::Insertable;
/// use web_common_traits::database::InsertableVariant;
///
/// let team_member = TeamMember::new()
///    // Set mandatory fields
///    .member(member_id)?
///    .team(team_id)?
///    // Finally, insert the new record in the database
///    .insert(user.id, conn)?;
/// ```
pub struct InsertableTeamMemberBuilder {
    pub(crate) team_id: Option<i32>,
    pub(crate) member_id: Option<i32>,
}
impl common_traits::builder::IsCompleteBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableTeamMemberBuilder
{
    fn is_complete(&self) -> bool {
        self.team_id.is_some() && self.member_id.is_some()
    }
}
/// Trait defining setters for attributes of an instance of `TeamMember` or
/// descendant tables.
pub trait TeamMemberSettable: Sized {
    /// Attributes required to build the insertable.
    type Attributes;
    /// Sets the value of the `public.team_members.team_id` column.
    ///
    /// # Arguments
    /// * `team_id`: The value to set for the `public.team_members.team_id`
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
    fn team<TI>(
        self,
        team_id: TI,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        TI: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the `public.team_members.member_id` column.
    ///
    /// # Arguments
    /// * `member_id`: The value to set for the `public.team_members.member_id`
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
    fn member<MI>(
        self,
        member_id: MI,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        MI: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
}
impl TeamMemberSettable for InsertableTeamMemberBuilder {
    type Attributes = crate::codegen::structs_codegen::tables::insertables::TeamMemberAttribute;
    /// Sets the value of the `public.team_members.team_id` column.
    fn team<TI>(
        mut self,
        team_id: TI,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        TI: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let team_id = <TI as web_common_traits::database::PrimaryKeyLike>::primary_key(&team_id);
        self.team_id = Some(team_id);
        Ok(self)
    }
    /// Sets the value of the `public.team_members.member_id` column.
    fn member<MI>(
        mut self,
        member_id: MI,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        MI: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let member_id =
            <MI as web_common_traits::database::PrimaryKeyLike>::primary_key(&member_id);
        self.member_id = Some(member_id);
        Ok(self)
    }
}
impl web_common_traits::prelude::SetPrimaryKey for InsertableTeamMemberBuilder {
    type PrimaryKey = (i32, i32);
    fn set_primary_key(self, _primary_key: Self::PrimaryKey) -> Self {
        self
    }
}
impl<C> web_common_traits::database::TryInsertGeneric<C> for InsertableTeamMemberBuilder
where
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::team_members::TeamMember,
            Error = web_common_traits::database::InsertError<TeamMemberAttribute>,
        >,
{
    type Attribute = TeamMemberAttribute;
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<Self::Attribute>> {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::team_members::TeamMember =
            self.insert(user_id, conn)?;
        Ok(insertable.id())
    }
}

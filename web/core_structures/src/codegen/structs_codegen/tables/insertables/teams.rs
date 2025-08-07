#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableTeamAttributes {
    Id,
    Name,
    Description,
    Icon,
    ColorId,
    StateId,
    ParentTeamId,
    CreatedBy,
    CreatedAt,
    UpdatedBy,
    UpdatedAt,
}
impl core::fmt::Display for InsertableTeamAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Id => write!(f, "id"),
            Self::Name => write!(f, "name"),
            Self::Description => write!(f, "description"),
            Self::Icon => write!(f, "icon"),
            Self::ColorId => write!(f, "color_id"),
            Self::StateId => write!(f, "state_id"),
            Self::ParentTeamId => write!(f, "parent_team_id"),
            Self::CreatedBy => write!(f, "created_by"),
            Self::CreatedAt => write!(f, "created_at"),
            Self::UpdatedBy => write!(f, "updated_by"),
            Self::UpdatedAt => write!(f, "updated_at"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(table_name = crate::codegen::diesel_codegen::tables::teams::teams)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableTeam {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) icon: String,
    pub(crate) color_id: i16,
    pub(crate) state_id: i16,
    pub(crate) parent_team_id: Option<i32>,
    pub(crate) created_by: i32,
    pub(crate) created_at: ::rosetta_timestamp::TimestampUTC,
    pub(crate) updated_by: i32,
    pub(crate) updated_at: ::rosetta_timestamp::TimestampUTC,
}
impl InsertableTeam {
    pub fn created_by<C: diesel::connection::LoadConnection>(
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
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::users::User::table(),
                self.created_by,
            ),
            conn,
        )
    }
    pub fn updated_by<C: diesel::connection::LoadConnection>(
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
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::users::User::table(),
                self.updated_by,
            ),
            conn,
        )
    }
    pub fn color<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::colors::Color,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::colors::Color: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::colors::Color as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::colors::Color as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::colors::Color as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::colors::Color as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::colors::Color as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::colors::Color as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::colors::Color,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::colors::Color::table(),
                self.color_id,
            ),
            conn,
        )
    }
    pub fn state<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::team_states::TeamState,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::team_states::TeamState: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::team_states::TeamState as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::team_states::TeamState as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::team_states::TeamState as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::team_states::TeamState as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::team_states::TeamState as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::team_states::TeamState as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::team_states::TeamState,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::team_states::TeamState::table(),
                self.state_id,
            ),
            conn,
        )
    }
    pub fn parent_team<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::teams::Team>,
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
        let Some(parent_team_id) = self.parent_team_id else {
            return Ok(None);
        };
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::teams::Team::table(),
                parent_team_id,
            ),
            conn,
        )
        .map(Some)
    }
}
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableTeamBuilder {
    pub(crate) id: Option<i32>,
    pub(crate) name: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) icon: Option<String>,
    pub(crate) color_id: Option<i16>,
    pub(crate) state_id: Option<i16>,
    pub(crate) parent_team_id: Option<i32>,
    pub(crate) created_by: Option<i32>,
    pub(crate) created_at: Option<::rosetta_timestamp::TimestampUTC>,
    pub(crate) updated_by: Option<i32>,
    pub(crate) updated_at: Option<::rosetta_timestamp::TimestampUTC>,
}
impl Default for InsertableTeamBuilder {
    fn default() -> Self {
        Self {
            id: Default::default(),
            name: Default::default(),
            description: Default::default(),
            icon: Default::default(),
            color_id: Some(15i16),
            state_id: Some(1i16),
            parent_team_id: Default::default(),
            created_by: Default::default(),
            created_at: Some(rosetta_timestamp::TimestampUTC::default()),
            updated_by: Default::default(),
            updated_at: Some(rosetta_timestamp::TimestampUTC::default()),
        }
    }
}
impl web_common_traits::database::ExtendableBuilder for InsertableTeamBuilder {
    type Attributes = InsertableTeamAttributes;
    fn extend_builder(
        mut self,
        other: Self,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let Some(id) = other.id {
            self = self.id(id)?;
        }
        if let Some(name) = other.name {
            self = self.name(name)?;
        }
        if let Some(description) = other.description {
            self = self.description(description)?;
        }
        if let Some(icon) = other.icon {
            self = self.icon(icon)?;
        }
        if let Some(color_id) = other.color_id {
            self = self.color(color_id)?;
        }
        if let Some(state_id) = other.state_id {
            self = self.state(state_id)?;
        }
        if let Some(parent_team_id) = other.parent_team_id {
            self = self.parent_team(Some(parent_team_id))?;
        }
        if let Some(created_by) = other.created_by {
            self = self.created_by(created_by)?;
        }
        if let Some(created_at) = other.created_at {
            self = self.created_at(created_at)?;
        }
        if let Some(updated_by) = other.updated_by {
            self = self.updated_by(updated_by)?;
        }
        if let Some(updated_at) = other.updated_at {
            self = self.updated_at(updated_at)?;
        }
        Ok(self)
    }
}
impl web_common_traits::prelude::SetPrimaryKey for InsertableTeamBuilder {
    type PrimaryKey = i32;
    fn set_primary_key(self, _primary_key: Self::PrimaryKey) -> Self {
        self
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableTeamBuilder {
    /// Sets the value of the `teams.color_id` column from table `teams`.
    pub fn color(
        mut self,
        color_id: i16,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableTeamAttributes>> {
        self.color_id = Some(color_id);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableTeamBuilder {
    /// Sets the value of the `teams.created_at` column from table `teams`.
    pub fn created_at<P>(
        mut self,
        created_at: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableTeamAttributes>>
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let created_at = created_at.try_into().map_err(
            |err: <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error| {
                Into::into(err).rename_field(InsertableTeamAttributes::CreatedAt)
            },
        )?;
        if let Some(updated_at) = self.updated_at {
            pgrx_validation::must_be_smaller_than_utc(created_at, updated_at).map_err(|e| {
                e.rename_fields(
                    InsertableTeamAttributes::CreatedAt,
                    InsertableTeamAttributes::UpdatedAt,
                )
            })?;
        }
        self.created_at = Some(created_at);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableTeamBuilder {
    /// Sets the value of the `teams.created_by` column from table `teams`.
    pub fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableTeamAttributes>> {
        self.created_by = Some(created_by);
        self = self.updated_by(created_by)?;
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableTeamBuilder {
    /// Sets the value of the `teams.description` column from table `teams`.
    pub fn description<P>(
        mut self,
        description: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableTeamAttributes>>
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let description =
            description.try_into().map_err(|err: <P as TryInto<String>>::Error| {
                Into::into(err).rename_field(InsertableTeamAttributes::Description)
            })?;
        self.description = Some(description);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableTeamBuilder {
    /// Sets the value of the `teams.icon` column from table `teams`.
    pub fn icon<P>(
        mut self,
        icon: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableTeamAttributes>>
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let icon = icon.try_into().map_err(|err: <P as TryInto<String>>::Error| {
            Into::into(err).rename_field(InsertableTeamAttributes::Icon)
        })?;
        pgrx_validation::must_be_font_awesome_class(icon.as_ref())
            .map_err(|e| e.rename_field(InsertableTeamAttributes::Icon))?;
        self.icon = Some(icon);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableTeamBuilder {
    /// Sets the value of the `teams.id` column from table `teams`.
    pub fn id<P>(
        mut self,
        id: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableTeamAttributes>>
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let id = id.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
            Into::into(err).rename_field(InsertableTeamAttributes::Id)
        })?;
        if let Some(parent_team_id) = self.parent_team_id {
            pgrx_validation::must_be_distinct_i32(parent_team_id, id).map_err(|e| {
                e.rename_fields(
                    InsertableTeamAttributes::ParentTeamId,
                    InsertableTeamAttributes::Id,
                )
            })?;
        }
        self.id = Some(id);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableTeamBuilder {
    /// Sets the value of the `teams.name` column from table `teams`.
    pub fn name<P>(
        mut self,
        name: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableTeamAttributes>>
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let name = name.try_into().map_err(|err: <P as TryInto<String>>::Error| {
            Into::into(err).rename_field(InsertableTeamAttributes::Name)
        })?;
        pgrx_validation::must_be_paragraph(name.as_ref())
            .map_err(|e| e.rename_field(InsertableTeamAttributes::Name))?;
        self.name = Some(name);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableTeamBuilder {
    /// Sets the value of the `teams.parent_team_id` column from table `teams`.
    pub fn parent_team(
        mut self,
        parent_team_id: Option<i32>,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableTeamAttributes>> {
        self.parent_team_id = parent_team_id;
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableTeamBuilder {
    /// Sets the value of the `teams.state_id` column from table `teams`.
    pub fn state(
        mut self,
        state_id: i16,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableTeamAttributes>> {
        self.state_id = Some(state_id);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableTeamBuilder {
    /// Sets the value of the `teams.updated_at` column from table `teams`.
    pub fn updated_at<P>(
        mut self,
        updated_at: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableTeamAttributes>>
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let updated_at = updated_at.try_into().map_err(
            |err: <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error| {
                Into::into(err).rename_field(InsertableTeamAttributes::UpdatedAt)
            },
        )?;
        if let Some(created_at) = self.created_at {
            pgrx_validation::must_be_smaller_than_utc(created_at, updated_at).map_err(|e| {
                e.rename_fields(
                    InsertableTeamAttributes::CreatedAt,
                    InsertableTeamAttributes::UpdatedAt,
                )
            })?;
        }
        self.updated_at = Some(updated_at);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableTeamBuilder {
    /// Sets the value of the `teams.updated_by` column from table `teams`.
    pub fn updated_by(
        mut self,
        updated_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableTeamAttributes>> {
        self.updated_by = Some(updated_by);
        Ok(self)
    }
}
impl<C> web_common_traits::database::TryInsertGeneric<C> for InsertableTeamBuilder
where
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::teams::Team,
            Error = web_common_traits::database::InsertError<InsertableTeamAttributes>,
        >,
{
    type Attributes = InsertableTeamAttributes;
    fn is_complete(&self) -> bool {
        self.id.is_some()
            && self.name.is_some()
            && self.description.is_some()
            && self.icon.is_some()
            && self.color_id.is_some()
            && self.state_id.is_some()
            && self.created_by.is_some()
            && self.created_at.is_some()
            && self.updated_by.is_some()
            && self.updated_at.is_some()
    }
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<Self::Attributes>> {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::teams::Team =
            self.insert(user_id, conn)?;
        Ok(insertable.id())
    }
}

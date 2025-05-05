#[derive(Clone, core::fmt::Debug)]
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
            InsertableTeamAttributes::Id => write!(f, "id"),
            InsertableTeamAttributes::Name => write!(f, "name"),
            InsertableTeamAttributes::Description => write!(f, "description"),
            InsertableTeamAttributes::Icon => write!(f, "icon"),
            InsertableTeamAttributes::ColorId => write!(f, "color_id"),
            InsertableTeamAttributes::StateId => write!(f, "state_id"),
            InsertableTeamAttributes::ParentTeamId => write!(f, "parent_team_id"),
            InsertableTeamAttributes::CreatedBy => write!(f, "created_by"),
            InsertableTeamAttributes::CreatedAt => write!(f, "created_at"),
            InsertableTeamAttributes::UpdatedBy => write!(f, "updated_by"),
            InsertableTeamAttributes::UpdatedAt => write!(f, "updated_at"),
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
    id: i32,
    name: String,
    description: String,
    icon: String,
    color_id: i16,
    state_id: i16,
    parent_team_id: Option<i32>,
    created_by: i32,
    created_at: rosetta_timestamp::TimestampUTC,
    updated_by: i32,
    updated_at: rosetta_timestamp::TimestampUTC,
}
impl InsertableTeam {
    #[cfg(feature = "postgres")]
    pub async fn color(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::colors::Color, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::colors::Color::table()
            .filter(
                crate::codegen::diesel_codegen::tables::colors::colors::dsl::id.eq(&self.color_id),
            )
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
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::team_states::TeamState::table()
            .filter(
                crate::codegen::diesel_codegen::tables::team_states::team_states::dsl::id
                    .eq(&self.state_id),
            )
            .first::<crate::codegen::structs_codegen::tables::team_states::TeamState>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn parent_team(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<Option<crate::codegen::structs_codegen::tables::teams::Team>, diesel::result::Error>
    {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        let Some(parent_team_id) = self.parent_team_id.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::teams::Team::table()
            .filter(
                crate::codegen::diesel_codegen::tables::teams::teams::dsl::id.eq(parent_team_id),
            )
            .first::<crate::codegen::structs_codegen::tables::teams::Team>(conn)
            .await
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub async fn created_by(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::users::User, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::users::User::table()
            .filter(
                crate::codegen::diesel_codegen::tables::users::users::dsl::id.eq(&self.created_by),
            )
            .first::<crate::codegen::structs_codegen::tables::users::User>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn updated_by(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::users::User, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::users::User::table()
            .filter(
                crate::codegen::diesel_codegen::tables::users::users::dsl::id.eq(&self.updated_by),
            )
            .first::<crate::codegen::structs_codegen::tables::users::User>(conn)
            .await
    }
}
#[derive(Default)]
pub struct InsertableTeamBuilder {
    id: Option<i32>,
    name: Option<String>,
    description: Option<String>,
    icon: Option<String>,
    color_id: Option<i16>,
    state_id: Option<i16>,
    parent_team_id: Option<i32>,
    created_by: Option<i32>,
    created_at: Option<rosetta_timestamp::TimestampUTC>,
    updated_by: Option<i32>,
    updated_at: Option<rosetta_timestamp::TimestampUTC>,
}
impl InsertableTeamBuilder {
    pub fn id(mut self, id: i32) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
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
    pub fn name(
        mut self,
        name: String,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        pgrx_validation::must_not_be_empty(name.as_ref())
            .map_err(|e| e.rename_field(InsertableTeamAttributes::Name))?;
        self.name = Some(name);
        Ok(self)
    }
    pub fn description(
        mut self,
        description: String,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.description = Some(description);
        Ok(self)
    }
    pub fn icon(
        mut self,
        icon: String,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        pgrx_validation::must_be_font_awesome_class(icon.as_ref())
            .map_err(|e| e.rename_field(InsertableTeamAttributes::Icon))?;
        self.icon = Some(icon);
        Ok(self)
    }
    pub fn color_id(
        mut self,
        color_id: i16,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.color_id = Some(color_id);
        Ok(self)
    }
    pub fn state_id(
        mut self,
        state_id: i16,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.state_id = Some(state_id);
        Ok(self)
    }
    pub fn parent_team_id(
        mut self,
        parent_team_id: Option<i32>,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        if let (Some(id), Some(parent_team_id)) = (self.id, parent_team_id) {
            pgrx_validation::must_be_distinct_i32(parent_team_id, id).map_err(|e| {
                e.rename_fields(
                    InsertableTeamAttributes::ParentTeamId,
                    InsertableTeamAttributes::Id,
                )
            })?;
        }
        self.parent_team_id = parent_team_id;
        Ok(self)
    }
    pub fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.created_by = Some(created_by);
        Ok(self)
    }
    pub fn created_at(
        mut self,
        created_at: rosetta_timestamp::TimestampUTC,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.created_at = Some(created_at);
        Ok(self)
    }
    pub fn updated_by(
        mut self,
        updated_by: i32,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.updated_by = Some(updated_by);
        Ok(self)
    }
    pub fn updated_at(
        mut self,
        updated_at: rosetta_timestamp::TimestampUTC,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.updated_at = Some(updated_at);
        Ok(self)
    }
}
impl common_traits::prelude::Builder for InsertableTeamBuilder {
    type Error = web_common_traits::database::InsertError<InsertableTeamAttributes>;
    type Object = InsertableTeam;
    type Attribute = InsertableTeamAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            id: self.id.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableTeamAttributes::Id,
            ))?,
            name: self.name.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableTeamAttributes::Name,
            ))?,
            description: self.description.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableTeamAttributes::Description,
                ),
            )?,
            icon: self.icon.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableTeamAttributes::Icon,
            ))?,
            color_id: self.color_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableTeamAttributes::ColorId,
                ),
            )?,
            state_id: self.state_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableTeamAttributes::StateId,
                ),
            )?,
            parent_team_id: self.parent_team_id,
            created_by: self.created_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableTeamAttributes::CreatedBy,
                ),
            )?,
            created_at: self.created_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableTeamAttributes::CreatedAt,
                ),
            )?,
            updated_by: self.updated_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableTeamAttributes::UpdatedBy,
                ),
            )?,
            updated_at: self.updated_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableTeamAttributes::UpdatedAt,
                ),
            )?,
        })
    }
}
impl TryFrom<InsertableTeam> for InsertableTeamBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableTeam) -> Result<Self, Self::Error> {
        Self::default()
            .id(insertable_variant.id)?
            .name(insertable_variant.name)?
            .description(insertable_variant.description)?
            .icon(insertable_variant.icon)?
            .color_id(insertable_variant.color_id)?
            .state_id(insertable_variant.state_id)?
            .parent_team_id(insertable_variant.parent_team_id)?
            .created_by(insertable_variant.created_by)?
            .created_at(insertable_variant.created_at)?
            .updated_by(insertable_variant.updated_by)?
            .updated_at(insertable_variant.updated_at)
    }
}

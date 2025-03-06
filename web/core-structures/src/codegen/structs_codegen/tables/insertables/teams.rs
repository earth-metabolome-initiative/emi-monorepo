#[derive(Clone, core :: fmt :: Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableTeamAttributes {
    Id,
    Name,
    Description,
    ParentTeamId,
    CreatedBy,
    UpdatedBy,
    IconId,
    ColorId,
    StateId,
}
impl core::fmt::Display for InsertableTeamAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableTeamAttributes::Id => write!(f, "id"),
            InsertableTeamAttributes::Name => write!(f, "name"),
            InsertableTeamAttributes::Description => write!(f, "description"),
            InsertableTeamAttributes::ParentTeamId => write!(f, "parent_team_id"),
            InsertableTeamAttributes::CreatedBy => write!(f, "created_by"),
            InsertableTeamAttributes::UpdatedBy => write!(f, "updated_by"),
            InsertableTeamAttributes::IconId => write!(f, "icon_id"),
            InsertableTeamAttributes::ColorId => write!(f, "color_id"),
            InsertableTeamAttributes::StateId => write!(f, "state_id"),
        }
    }
}
#[cfg_attr(feature = "diesel", derive(diesel::Insertable))]
# [cfg_attr (feature = "diesel" , diesel (table_name = crate :: codegen :: diesel_codegen :: tables :: teams :: teams))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableTeam {
    id: i32,
    name: String,
    description: String,
    parent_team_id: Option<i32>,
    created_by: i32,
    updated_by: i32,
    icon_id: i16,
    color_id: i16,
    state_id: i16,
}
#[derive(Default)]
pub struct InsertableTeamBuilder {
    id: Option<i32>,
    name: Option<String>,
    description: Option<String>,
    parent_team_id: Option<i32>,
    created_by: Option<i32>,
    updated_by: Option<i32>,
    icon_id: Option<i16>,
    color_id: Option<i16>,
    state_id: Option<i16>,
}
impl InsertableTeamBuilder {
    pub fn id(mut self, id: i32) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.id = Some(id);
        Ok(self)
    }
    pub fn name(
        mut self,
        name: String,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
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
    pub fn parent_team_id(
        mut self,
        parent_team_id: Option<i32>,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
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
    pub fn updated_by(
        mut self,
        updated_by: i32,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.updated_by = Some(updated_by);
        Ok(self)
    }
    pub fn icon_id(
        mut self,
        icon_id: i16,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.icon_id = Some(icon_id);
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
}
impl common_traits::prelude::Builder for InsertableTeamBuilder {
    type Error = web_common_traits::database::InsertError<InsertableTeamAttributes>;
    type Object = InsertableTeam;
    type Attribute = InsertableTeamAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            id: self.id.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(InsertableTeamAttributes::Id)
            })?,
            name: self.name.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableTeamAttributes::Name,
                )
            })?,
            description: self.description.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableTeamAttributes::Description,
                )
            })?,
            parent_team_id: self.parent_team_id,
            created_by: self.created_by.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableTeamAttributes::CreatedBy,
                )
            })?,
            updated_by: self.updated_by.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableTeamAttributes::UpdatedBy,
                )
            })?,
            icon_id: self.icon_id.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableTeamAttributes::IconId,
                )
            })?,
            color_id: self.color_id.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableTeamAttributes::ColorId,
                )
            })?,
            state_id: self.state_id.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableTeamAttributes::StateId,
                )
            })?,
        })
    }
}
impl TryFrom<InsertableTeam> for InsertableTeamBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableTeam) -> Result<Self, Self::Error> {
        Ok(Self::default()
            .id(insertable_variant.id)?
            .name(insertable_variant.name)?
            .description(insertable_variant.description)?
            .parent_team_id(insertable_variant.parent_team_id)?
            .created_by(insertable_variant.created_by)?
            .updated_by(insertable_variant.updated_by)?
            .icon_id(insertable_variant.icon_id)?
            .color_id(insertable_variant.color_id)?
            .state_id(insertable_variant.state_id)?)
    }
}

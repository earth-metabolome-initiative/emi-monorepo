#[derive(Clone, core :: fmt :: Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableSampleAttributes {
    Id,
    ContainerId,
    Notes,
    ProjectId,
    CreatedBy,
    SampledBy,
    UpdatedBy,
    StateId,
}
impl core::fmt::Display for InsertableSampleAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableSampleAttributes::Id => write!(f, "id"),
            InsertableSampleAttributes::ContainerId => write!(f, "container_id"),
            InsertableSampleAttributes::Notes => write!(f, "notes"),
            InsertableSampleAttributes::ProjectId => write!(f, "project_id"),
            InsertableSampleAttributes::CreatedBy => write!(f, "created_by"),
            InsertableSampleAttributes::SampledBy => write!(f, "sampled_by"),
            InsertableSampleAttributes::UpdatedBy => write!(f, "updated_by"),
            InsertableSampleAttributes::StateId => write!(f, "state_id"),
        }
    }
}
#[cfg_attr(feature = "diesel", derive(diesel::Insertable))]
# [cfg_attr (feature = "diesel" , diesel (table_name = crate :: codegen :: diesel_codegen :: tables :: samples :: samples))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableSample {
    id: uuid::Uuid,
    container_id: i32,
    notes: Option<String>,
    project_id: i32,
    created_by: i32,
    sampled_by: i32,
    updated_by: i32,
    state_id: i16,
}
#[derive(Default)]
pub struct InsertableSampleBuilder {
    id: Option<uuid::Uuid>,
    container_id: Option<i32>,
    notes: Option<String>,
    project_id: Option<i32>,
    created_by: Option<i32>,
    sampled_by: Option<i32>,
    updated_by: Option<i32>,
    state_id: Option<i16>,
}
impl InsertableSampleBuilder {
    pub fn id(
        mut self,
        id: uuid::Uuid,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.id = Some(id);
        Ok(self)
    }
    pub fn container_id(
        mut self,
        container_id: i32,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.container_id = Some(container_id);
        Ok(self)
    }
    pub fn notes(
        mut self,
        notes: Option<String>,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.notes = notes;
        Ok(self)
    }
    pub fn project_id(
        mut self,
        project_id: i32,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.project_id = Some(project_id);
        Ok(self)
    }
    pub fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.created_by = Some(created_by);
        Ok(self)
    }
    pub fn sampled_by(
        mut self,
        sampled_by: i32,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.sampled_by = Some(sampled_by);
        Ok(self)
    }
    pub fn updated_by(
        mut self,
        updated_by: i32,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.updated_by = Some(updated_by);
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
impl common_traits::prelude::Builder for InsertableSampleBuilder {
    type Error = web_common_traits::database::InsertError<InsertableSampleAttributes>;
    type Object = InsertableSample;
    type Attribute = InsertableSampleAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            id: self.id.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableSampleAttributes::Id,
                )
            })?,
            container_id: self.container_id.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableSampleAttributes::ContainerId,
                )
            })?,
            notes: self.notes,
            project_id: self.project_id.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableSampleAttributes::ProjectId,
                )
            })?,
            created_by: self.created_by.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableSampleAttributes::CreatedBy,
                )
            })?,
            sampled_by: self.sampled_by.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableSampleAttributes::SampledBy,
                )
            })?,
            updated_by: self.updated_by.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableSampleAttributes::UpdatedBy,
                )
            })?,
            state_id: self.state_id.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableSampleAttributes::StateId,
                )
            })?,
        })
    }
}
impl TryFrom<InsertableSample> for InsertableSampleBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableSample) -> Result<Self, Self::Error> {
        Ok(Self::default()
            .id(insertable_variant.id)?
            .container_id(insertable_variant.container_id)?
            .notes(insertable_variant.notes)?
            .project_id(insertable_variant.project_id)?
            .created_by(insertable_variant.created_by)?
            .sampled_by(insertable_variant.sampled_by)?
            .updated_by(insertable_variant.updated_by)?
            .state_id(insertable_variant.state_id)?)
    }
}

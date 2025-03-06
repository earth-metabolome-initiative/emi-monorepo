#[derive(Clone, core :: fmt :: Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableObservationAttributes {
    Id,
    ParentObservationId,
    CreatedBy,
    UpdatedBy,
    ProjectId,
    OrganismId,
    SampleId,
    SubjectId,
    Picture,
    Notes,
}
impl core::fmt::Display for InsertableObservationAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableObservationAttributes::Id => write!(f, "id"),
            InsertableObservationAttributes::ParentObservationId => {
                write!(f, "parent_observation_id")
            }
            InsertableObservationAttributes::CreatedBy => write!(f, "created_by"),
            InsertableObservationAttributes::UpdatedBy => write!(f, "updated_by"),
            InsertableObservationAttributes::ProjectId => write!(f, "project_id"),
            InsertableObservationAttributes::OrganismId => write!(f, "organism_id"),
            InsertableObservationAttributes::SampleId => write!(f, "sample_id"),
            InsertableObservationAttributes::SubjectId => write!(f, "subject_id"),
            InsertableObservationAttributes::Picture => write!(f, "picture"),
            InsertableObservationAttributes::Notes => write!(f, "notes"),
        }
    }
}
#[cfg_attr(feature = "diesel", derive(diesel::Insertable))]
# [cfg_attr (feature = "diesel" , diesel (table_name = crate :: codegen :: diesel_codegen :: tables :: observations :: observations))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableObservation {
    id: uuid::Uuid,
    parent_observation_id: Option<uuid::Uuid>,
    created_by: i32,
    updated_by: i32,
    project_id: i32,
    organism_id: Option<uuid::Uuid>,
    sample_id: Option<uuid::Uuid>,
    subject_id: i16,
    picture: Vec<u8>,
    notes: Option<String>,
}
#[derive(Default)]
pub struct InsertableObservationBuilder {
    id: Option<uuid::Uuid>,
    parent_observation_id: Option<uuid::Uuid>,
    created_by: Option<i32>,
    updated_by: Option<i32>,
    project_id: Option<i32>,
    organism_id: Option<uuid::Uuid>,
    sample_id: Option<uuid::Uuid>,
    subject_id: Option<i16>,
    picture: Option<Vec<u8>>,
    notes: Option<String>,
}
impl InsertableObservationBuilder {
    pub fn id(
        mut self,
        id: uuid::Uuid,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.id = Some(id);
        Ok(self)
    }
    pub fn parent_observation_id(
        mut self,
        parent_observation_id: Option<uuid::Uuid>,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.parent_observation_id = parent_observation_id;
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
    pub fn project_id(
        mut self,
        project_id: i32,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.project_id = Some(project_id);
        Ok(self)
    }
    pub fn organism_id(
        mut self,
        organism_id: Option<uuid::Uuid>,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.organism_id = organism_id;
        Ok(self)
    }
    pub fn sample_id(
        mut self,
        sample_id: Option<uuid::Uuid>,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.sample_id = sample_id;
        Ok(self)
    }
    pub fn subject_id(
        mut self,
        subject_id: i16,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.subject_id = Some(subject_id);
        Ok(self)
    }
    pub fn picture(
        mut self,
        picture: Vec<u8>,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.picture = Some(picture);
        Ok(self)
    }
    pub fn notes(
        mut self,
        notes: Option<String>,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.notes = notes;
        Ok(self)
    }
}
impl common_traits::prelude::Builder for InsertableObservationBuilder {
    type Error = web_common_traits::database::InsertError<InsertableObservationAttributes>;
    type Object = InsertableObservation;
    type Attribute = InsertableObservationAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            id: self.id.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableObservationAttributes::Id,
                )
            })?,
            parent_observation_id: self.parent_observation_id,
            created_by: self.created_by.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableObservationAttributes::CreatedBy,
                )
            })?,
            updated_by: self.updated_by.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableObservationAttributes::UpdatedBy,
                )
            })?,
            project_id: self.project_id.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableObservationAttributes::ProjectId,
                )
            })?,
            organism_id: self.organism_id,
            sample_id: self.sample_id,
            subject_id: self.subject_id.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableObservationAttributes::SubjectId,
                )
            })?,
            picture: self.picture.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableObservationAttributes::Picture,
                )
            })?,
            notes: self.notes,
        })
    }
}
impl TryFrom<InsertableObservation> for InsertableObservationBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableObservation) -> Result<Self, Self::Error> {
        Ok(Self::default()
            .id(insertable_variant.id)?
            .parent_observation_id(insertable_variant.parent_observation_id)?
            .created_by(insertable_variant.created_by)?
            .updated_by(insertable_variant.updated_by)?
            .project_id(insertable_variant.project_id)?
            .organism_id(insertable_variant.organism_id)?
            .sample_id(insertable_variant.sample_id)?
            .subject_id(insertable_variant.subject_id)?
            .picture(insertable_variant.picture)?
            .notes(insertable_variant.notes)?)
    }
}

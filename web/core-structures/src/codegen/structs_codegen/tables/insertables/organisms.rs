#[derive(Clone, core :: fmt :: Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableOrganismAttributes {
    Id,
    HostOrganismId,
    SampleId,
    Notes,
    NameplateId,
    ProjectId,
    CreatedBy,
    UpdatedBy,
    Wild,
}
impl core::fmt::Display for InsertableOrganismAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableOrganismAttributes::Id => write!(f, "id"),
            InsertableOrganismAttributes::HostOrganismId => write!(f, "host_organism_id"),
            InsertableOrganismAttributes::SampleId => write!(f, "sample_id"),
            InsertableOrganismAttributes::Notes => write!(f, "notes"),
            InsertableOrganismAttributes::NameplateId => write!(f, "nameplate_id"),
            InsertableOrganismAttributes::ProjectId => write!(f, "project_id"),
            InsertableOrganismAttributes::CreatedBy => write!(f, "created_by"),
            InsertableOrganismAttributes::UpdatedBy => write!(f, "updated_by"),
            InsertableOrganismAttributes::Wild => write!(f, "wild"),
        }
    }
}
#[cfg_attr(feature = "diesel", derive(diesel::Insertable))]
# [cfg_attr (feature = "diesel" , diesel (table_name = crate :: codegen :: diesel_codegen :: tables :: organisms :: organisms))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableOrganism {
    id: uuid::Uuid,
    host_organism_id: Option<uuid::Uuid>,
    sample_id: Option<uuid::Uuid>,
    notes: Option<String>,
    nameplate_id: i32,
    project_id: i32,
    created_by: i32,
    updated_by: i32,
    wild: bool,
}
#[derive(Default)]
pub struct InsertableOrganismBuilder {
    id: Option<uuid::Uuid>,
    host_organism_id: Option<uuid::Uuid>,
    sample_id: Option<uuid::Uuid>,
    notes: Option<String>,
    nameplate_id: Option<i32>,
    project_id: Option<i32>,
    created_by: Option<i32>,
    updated_by: Option<i32>,
    wild: Option<bool>,
}
impl InsertableOrganismBuilder {
    pub fn id(
        mut self,
        id: uuid::Uuid,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.id = Some(id);
        Ok(self)
    }
    pub fn host_organism_id(
        mut self,
        host_organism_id: Option<uuid::Uuid>,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.host_organism_id = host_organism_id;
        Ok(self)
    }
    pub fn sample_id(
        mut self,
        sample_id: Option<uuid::Uuid>,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.sample_id = sample_id;
        Ok(self)
    }
    pub fn notes(
        mut self,
        notes: Option<String>,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.notes = notes;
        Ok(self)
    }
    pub fn nameplate_id(
        mut self,
        nameplate_id: i32,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.nameplate_id = Some(nameplate_id);
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
    pub fn updated_by(
        mut self,
        updated_by: i32,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.updated_by = Some(updated_by);
        Ok(self)
    }
    pub fn wild(
        mut self,
        wild: bool,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.wild = Some(wild);
        Ok(self)
    }
}
impl common_traits::prelude::Builder for InsertableOrganismBuilder {
    type Error = web_common_traits::database::InsertError<InsertableOrganismAttributes>;
    type Object = InsertableOrganism;
    type Attribute = InsertableOrganismAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            id: self.id.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableOrganismAttributes::Id,
                )
            })?,
            host_organism_id: self.host_organism_id,
            sample_id: self.sample_id,
            notes: self.notes,
            nameplate_id: self.nameplate_id.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableOrganismAttributes::NameplateId,
                )
            })?,
            project_id: self.project_id.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableOrganismAttributes::ProjectId,
                )
            })?,
            created_by: self.created_by.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableOrganismAttributes::CreatedBy,
                )
            })?,
            updated_by: self.updated_by.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableOrganismAttributes::UpdatedBy,
                )
            })?,
            wild: self.wild.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableOrganismAttributes::Wild,
                )
            })?,
        })
    }
}
impl TryFrom<InsertableOrganism> for InsertableOrganismBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableOrganism) -> Result<Self, Self::Error> {
        Ok(Self::default()
            .id(insertable_variant.id)?
            .host_organism_id(insertable_variant.host_organism_id)?
            .sample_id(insertable_variant.sample_id)?
            .notes(insertable_variant.notes)?
            .nameplate_id(insertable_variant.nameplate_id)?
            .project_id(insertable_variant.project_id)?
            .created_by(insertable_variant.created_by)?
            .updated_by(insertable_variant.updated_by)?
            .wild(insertable_variant.wild)?)
    }
}

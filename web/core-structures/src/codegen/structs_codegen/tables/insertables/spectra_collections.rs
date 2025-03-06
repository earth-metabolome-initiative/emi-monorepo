#[derive(Clone, core :: fmt :: Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableSpectraCollectionAttributes {
    Id,
    Notes,
    SampleId,
    CreatedBy,
    UpdatedBy,
}
impl core::fmt::Display for InsertableSpectraCollectionAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableSpectraCollectionAttributes::Id => write!(f, "id"),
            InsertableSpectraCollectionAttributes::Notes => write!(f, "notes"),
            InsertableSpectraCollectionAttributes::SampleId => write!(f, "sample_id"),
            InsertableSpectraCollectionAttributes::CreatedBy => write!(f, "created_by"),
            InsertableSpectraCollectionAttributes::UpdatedBy => write!(f, "updated_by"),
        }
    }
}
#[cfg_attr(feature = "diesel", derive(diesel::Insertable))]
# [cfg_attr (feature = "diesel" , diesel (table_name = crate :: codegen :: diesel_codegen :: tables :: spectra_collections :: spectra_collections))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableSpectraCollection {
    id: i32,
    notes: Option<String>,
    sample_id: uuid::Uuid,
    created_by: i32,
    updated_by: i32,
}
#[derive(Default)]
pub struct InsertableSpectraCollectionBuilder {
    id: Option<i32>,
    notes: Option<String>,
    sample_id: Option<uuid::Uuid>,
    created_by: Option<i32>,
    updated_by: Option<i32>,
}
impl InsertableSpectraCollectionBuilder {
    pub fn id(mut self, id: i32) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.id = Some(id);
        Ok(self)
    }
    pub fn notes(
        mut self,
        notes: Option<String>,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.notes = notes;
        Ok(self)
    }
    pub fn sample_id(
        mut self,
        sample_id: uuid::Uuid,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.sample_id = Some(sample_id);
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
}
impl common_traits::prelude::Builder for InsertableSpectraCollectionBuilder {
    type Error = web_common_traits::database::InsertError<InsertableSpectraCollectionAttributes>;
    type Object = InsertableSpectraCollection;
    type Attribute = InsertableSpectraCollectionAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            id: self.id.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableSpectraCollectionAttributes::Id,
                )
            })?,
            notes: self.notes,
            sample_id: self.sample_id.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableSpectraCollectionAttributes::SampleId,
                )
            })?,
            created_by: self.created_by.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableSpectraCollectionAttributes::CreatedBy,
                )
            })?,
            updated_by: self.updated_by.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableSpectraCollectionAttributes::UpdatedBy,
                )
            })?,
        })
    }
}
impl TryFrom<InsertableSpectraCollection> for InsertableSpectraCollectionBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableSpectraCollection) -> Result<Self, Self::Error> {
        Ok(Self::default()
            .id(insertable_variant.id)?
            .notes(insertable_variant.notes)?
            .sample_id(insertable_variant.sample_id)?
            .created_by(insertable_variant.created_by)?
            .updated_by(insertable_variant.updated_by)?)
    }
}

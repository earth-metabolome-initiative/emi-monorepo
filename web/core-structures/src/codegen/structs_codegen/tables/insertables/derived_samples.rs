#[derive(Clone, core :: fmt :: Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableDerivedSampleAttributes {
    CreatedBy,
    UpdatedBy,
    ParentSampleId,
    ChildSampleId,
    Quantity,
    UnitId,
}
impl core::fmt::Display for InsertableDerivedSampleAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableDerivedSampleAttributes::CreatedBy => write!(f, "created_by"),
            InsertableDerivedSampleAttributes::UpdatedBy => write!(f, "updated_by"),
            InsertableDerivedSampleAttributes::ParentSampleId => write!(f, "parent_sample_id"),
            InsertableDerivedSampleAttributes::ChildSampleId => write!(f, "child_sample_id"),
            InsertableDerivedSampleAttributes::Quantity => write!(f, "quantity"),
            InsertableDerivedSampleAttributes::UnitId => write!(f, "unit_id"),
        }
    }
}
#[cfg_attr(feature = "diesel", derive(diesel::Insertable))]
# [cfg_attr (feature = "diesel" , diesel (table_name = crate :: codegen :: diesel_codegen :: tables :: derived_samples :: derived_samples))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableDerivedSample {
    created_by: i32,
    updated_by: i32,
    parent_sample_id: uuid::Uuid,
    child_sample_id: uuid::Uuid,
    quantity: f64,
    unit_id: i16,
}
#[derive(Default)]
pub struct InsertableDerivedSampleBuilder {
    created_by: Option<i32>,
    updated_by: Option<i32>,
    parent_sample_id: Option<uuid::Uuid>,
    child_sample_id: Option<uuid::Uuid>,
    quantity: Option<f64>,
    unit_id: Option<i16>,
}
impl InsertableDerivedSampleBuilder {
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
    pub fn parent_sample_id(
        mut self,
        parent_sample_id: uuid::Uuid,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.parent_sample_id = Some(parent_sample_id);
        Ok(self)
    }
    pub fn child_sample_id(
        mut self,
        child_sample_id: uuid::Uuid,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.child_sample_id = Some(child_sample_id);
        Ok(self)
    }
    pub fn quantity(
        mut self,
        quantity: f64,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.quantity = Some(quantity);
        Ok(self)
    }
    pub fn unit_id(
        mut self,
        unit_id: i16,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.unit_id = Some(unit_id);
        Ok(self)
    }
}
impl common_traits::prelude::Builder for InsertableDerivedSampleBuilder {
    type Error = web_common_traits::database::InsertError<InsertableDerivedSampleAttributes>;
    type Object = InsertableDerivedSample;
    type Attribute = InsertableDerivedSampleAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            created_by: self.created_by.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableDerivedSampleAttributes::CreatedBy,
                )
            })?,
            updated_by: self.updated_by.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableDerivedSampleAttributes::UpdatedBy,
                )
            })?,
            parent_sample_id: self.parent_sample_id.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableDerivedSampleAttributes::ParentSampleId,
                )
            })?,
            child_sample_id: self.child_sample_id.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableDerivedSampleAttributes::ChildSampleId,
                )
            })?,
            quantity: self.quantity.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableDerivedSampleAttributes::Quantity,
                )
            })?,
            unit_id: self.unit_id.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableDerivedSampleAttributes::UnitId,
                )
            })?,
        })
    }
}
impl TryFrom<InsertableDerivedSample> for InsertableDerivedSampleBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableDerivedSample) -> Result<Self, Self::Error> {
        Ok(Self::default()
            .created_by(insertable_variant.created_by)?
            .updated_by(insertable_variant.updated_by)?
            .parent_sample_id(insertable_variant.parent_sample_id)?
            .child_sample_id(insertable_variant.child_sample_id)?
            .quantity(insertable_variant.quantity)?
            .unit_id(insertable_variant.unit_id)?)
    }
}

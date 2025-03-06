#[derive(Clone, core :: fmt :: Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableSampleContainerAttributes {
    Id,
    Barcode,
    ProjectId,
    CreatedBy,
    UpdatedBy,
    CategoryId,
}
impl core::fmt::Display for InsertableSampleContainerAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableSampleContainerAttributes::Id => write!(f, "id"),
            InsertableSampleContainerAttributes::Barcode => write!(f, "barcode"),
            InsertableSampleContainerAttributes::ProjectId => write!(f, "project_id"),
            InsertableSampleContainerAttributes::CreatedBy => write!(f, "created_by"),
            InsertableSampleContainerAttributes::UpdatedBy => write!(f, "updated_by"),
            InsertableSampleContainerAttributes::CategoryId => write!(f, "category_id"),
        }
    }
}
#[cfg_attr(feature = "diesel", derive(diesel::Insertable))]
# [cfg_attr (feature = "diesel" , diesel (table_name = crate :: codegen :: diesel_codegen :: tables :: sample_containers :: sample_containers))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableSampleContainer {
    id: i32,
    barcode: String,
    project_id: i32,
    created_by: i32,
    updated_by: i32,
    category_id: i16,
}
#[derive(Default)]
pub struct InsertableSampleContainerBuilder {
    id: Option<i32>,
    barcode: Option<String>,
    project_id: Option<i32>,
    created_by: Option<i32>,
    updated_by: Option<i32>,
    category_id: Option<i16>,
}
impl InsertableSampleContainerBuilder {
    pub fn id(mut self, id: i32) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.id = Some(id);
        Ok(self)
    }
    pub fn barcode(
        mut self,
        barcode: String,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.barcode = Some(barcode);
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
    pub fn category_id(
        mut self,
        category_id: i16,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.category_id = Some(category_id);
        Ok(self)
    }
}
impl common_traits::prelude::Builder for InsertableSampleContainerBuilder {
    type Error = web_common_traits::database::InsertError<InsertableSampleContainerAttributes>;
    type Object = InsertableSampleContainer;
    type Attribute = InsertableSampleContainerAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            id: self.id.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableSampleContainerAttributes::Id,
                )
            })?,
            barcode: self.barcode.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableSampleContainerAttributes::Barcode,
                )
            })?,
            project_id: self.project_id.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableSampleContainerAttributes::ProjectId,
                )
            })?,
            created_by: self.created_by.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableSampleContainerAttributes::CreatedBy,
                )
            })?,
            updated_by: self.updated_by.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableSampleContainerAttributes::UpdatedBy,
                )
            })?,
            category_id: self.category_id.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableSampleContainerAttributes::CategoryId,
                )
            })?,
        })
    }
}
impl TryFrom<InsertableSampleContainer> for InsertableSampleContainerBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableSampleContainer) -> Result<Self, Self::Error> {
        Ok(Self::default()
            .id(insertable_variant.id)?
            .barcode(insertable_variant.barcode)?
            .project_id(insertable_variant.project_id)?
            .created_by(insertable_variant.created_by)?
            .updated_by(insertable_variant.updated_by)?
            .category_id(insertable_variant.category_id)?)
    }
}

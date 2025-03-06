#[derive(Clone, core :: fmt :: Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableNameplateAttributes {
    Id,
    Barcode,
    ProjectId,
    Geolocation,
    CreatedBy,
    UpdatedBy,
    CategoryId,
}
impl core::fmt::Display for InsertableNameplateAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableNameplateAttributes::Id => write!(f, "id"),
            InsertableNameplateAttributes::Barcode => write!(f, "barcode"),
            InsertableNameplateAttributes::ProjectId => write!(f, "project_id"),
            InsertableNameplateAttributes::Geolocation => write!(f, "geolocation"),
            InsertableNameplateAttributes::CreatedBy => write!(f, "created_by"),
            InsertableNameplateAttributes::UpdatedBy => write!(f, "updated_by"),
            InsertableNameplateAttributes::CategoryId => write!(f, "category_id"),
        }
    }
}
#[cfg_attr(feature = "diesel", derive(diesel::Insertable))]
# [cfg_attr (feature = "diesel" , diesel (table_name = crate :: codegen :: diesel_codegen :: tables :: nameplates :: nameplates))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableNameplate {
    id: i32,
    barcode: String,
    project_id: i32,
    geolocation: postgis_diesel::types::Point,
    created_by: i32,
    updated_by: i32,
    category_id: i16,
}
#[derive(Default)]
pub struct InsertableNameplateBuilder {
    id: Option<i32>,
    barcode: Option<String>,
    project_id: Option<i32>,
    geolocation: Option<postgis_diesel::types::Point>,
    created_by: Option<i32>,
    updated_by: Option<i32>,
    category_id: Option<i16>,
}
impl InsertableNameplateBuilder {
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
    pub fn geolocation(
        mut self,
        geolocation: postgis_diesel::types::Point,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.geolocation = Some(geolocation);
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
impl common_traits::prelude::Builder for InsertableNameplateBuilder {
    type Error = web_common_traits::database::InsertError<InsertableNameplateAttributes>;
    type Object = InsertableNameplate;
    type Attribute = InsertableNameplateAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            id: self.id.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableNameplateAttributes::Id,
                )
            })?,
            barcode: self.barcode.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableNameplateAttributes::Barcode,
                )
            })?,
            project_id: self.project_id.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableNameplateAttributes::ProjectId,
                )
            })?,
            geolocation: self.geolocation.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableNameplateAttributes::Geolocation,
                )
            })?,
            created_by: self.created_by.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableNameplateAttributes::CreatedBy,
                )
            })?,
            updated_by: self.updated_by.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableNameplateAttributes::UpdatedBy,
                )
            })?,
            category_id: self.category_id.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableNameplateAttributes::CategoryId,
                )
            })?,
        })
    }
}
impl TryFrom<InsertableNameplate> for InsertableNameplateBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableNameplate) -> Result<Self, Self::Error> {
        Ok(Self::default()
            .id(insertable_variant.id)?
            .barcode(insertable_variant.barcode)?
            .project_id(insertable_variant.project_id)?
            .geolocation(insertable_variant.geolocation)?
            .created_by(insertable_variant.created_by)?
            .updated_by(insertable_variant.updated_by)?
            .category_id(insertable_variant.category_id)?)
    }
}

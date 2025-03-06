#[derive(Clone, core :: fmt :: Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableSampleTaxaAttributes {
    CreatedBy,
    SampleId,
    TaxonId,
}
impl core::fmt::Display for InsertableSampleTaxaAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableSampleTaxaAttributes::CreatedBy => write!(f, "created_by"),
            InsertableSampleTaxaAttributes::SampleId => write!(f, "sample_id"),
            InsertableSampleTaxaAttributes::TaxonId => write!(f, "taxon_id"),
        }
    }
}
#[cfg_attr(feature = "diesel", derive(diesel::Insertable))]
# [cfg_attr (feature = "diesel" , diesel (table_name = crate :: codegen :: diesel_codegen :: tables :: sample_taxa :: sample_taxa))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableSampleTaxa {
    created_by: i32,
    sample_id: uuid::Uuid,
    taxon_id: i32,
}
#[derive(Default)]
pub struct InsertableSampleTaxaBuilder {
    created_by: Option<i32>,
    sample_id: Option<uuid::Uuid>,
    taxon_id: Option<i32>,
}
impl InsertableSampleTaxaBuilder {
    pub fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.created_by = Some(created_by);
        Ok(self)
    }
    pub fn sample_id(
        mut self,
        sample_id: uuid::Uuid,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.sample_id = Some(sample_id);
        Ok(self)
    }
    pub fn taxon_id(
        mut self,
        taxon_id: i32,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.taxon_id = Some(taxon_id);
        Ok(self)
    }
}
impl common_traits::prelude::Builder for InsertableSampleTaxaBuilder {
    type Error = web_common_traits::database::InsertError<InsertableSampleTaxaAttributes>;
    type Object = InsertableSampleTaxa;
    type Attribute = InsertableSampleTaxaAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            created_by: self.created_by.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableSampleTaxaAttributes::CreatedBy,
                )
            })?,
            sample_id: self.sample_id.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableSampleTaxaAttributes::SampleId,
                )
            })?,
            taxon_id: self.taxon_id.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableSampleTaxaAttributes::TaxonId,
                )
            })?,
        })
    }
}
impl TryFrom<InsertableSampleTaxa> for InsertableSampleTaxaBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableSampleTaxa) -> Result<Self, Self::Error> {
        Ok(Self::default()
            .created_by(insertable_variant.created_by)?
            .sample_id(insertable_variant.sample_id)?
            .taxon_id(insertable_variant.taxon_id)?)
    }
}

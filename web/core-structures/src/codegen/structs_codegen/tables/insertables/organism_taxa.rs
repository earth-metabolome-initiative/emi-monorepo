#[derive(Clone, core :: fmt :: Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableOrganismTaxaAttributes {
    CreatedBy,
    OrganismId,
    TaxonId,
}
impl core::fmt::Display for InsertableOrganismTaxaAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableOrganismTaxaAttributes::CreatedBy => write!(f, "created_by"),
            InsertableOrganismTaxaAttributes::OrganismId => write!(f, "organism_id"),
            InsertableOrganismTaxaAttributes::TaxonId => write!(f, "taxon_id"),
        }
    }
}
#[cfg_attr(feature = "diesel", derive(diesel::Insertable))]
# [cfg_attr (feature = "diesel" , diesel (table_name = crate :: codegen :: diesel_codegen :: tables :: organism_taxa :: organism_taxa))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableOrganismTaxa {
    created_by: i32,
    organism_id: uuid::Uuid,
    taxon_id: i32,
}
#[derive(Default)]
pub struct InsertableOrganismTaxaBuilder {
    created_by: Option<i32>,
    organism_id: Option<uuid::Uuid>,
    taxon_id: Option<i32>,
}
impl InsertableOrganismTaxaBuilder {
    pub fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.created_by = Some(created_by);
        Ok(self)
    }
    pub fn organism_id(
        mut self,
        organism_id: uuid::Uuid,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.organism_id = Some(organism_id);
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
impl common_traits::prelude::Builder for InsertableOrganismTaxaBuilder {
    type Error = web_common_traits::database::InsertError<InsertableOrganismTaxaAttributes>;
    type Object = InsertableOrganismTaxa;
    type Attribute = InsertableOrganismTaxaAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            created_by: self.created_by.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableOrganismTaxaAttributes::CreatedBy,
                )
            })?,
            organism_id: self.organism_id.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableOrganismTaxaAttributes::OrganismId,
                )
            })?,
            taxon_id: self.taxon_id.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableOrganismTaxaAttributes::TaxonId,
                )
            })?,
        })
    }
}
impl TryFrom<InsertableOrganismTaxa> for InsertableOrganismTaxaBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableOrganismTaxa) -> Result<Self, Self::Error> {
        Ok(Self::default()
            .created_by(insertable_variant.created_by)?
            .organism_id(insertable_variant.organism_id)?
            .taxon_id(insertable_variant.taxon_id)?)
    }
}

impl From<crate::codegen::structs_codegen::tables::organism_taxa::OrganismTaxon> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::organism_taxa::OrganismTaxon) -> Self {
        super::Row::OrganismTaxon(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::organism_taxa::OrganismTaxon>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::OrganismTaxon(v) => Some(v),
            _ => None,
        }
    }
}

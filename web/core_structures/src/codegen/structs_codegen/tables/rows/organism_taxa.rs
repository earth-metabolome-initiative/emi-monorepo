impl From<crate::codegen::structs_codegen::tables::organism_taxa::OrganismTaxon> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::organism_taxa::OrganismTaxon) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::organism_taxa::OrganismTaxon>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::organism_taxa::OrganismTaxon>,
    ) -> Self {
        super::Rows::OrganismTaxon(value)
    }
}
impl From<super::Rows>
    for Option<Vec<crate::codegen::structs_codegen::tables::organism_taxa::OrganismTaxon>>
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::OrganismTaxon(v) => Some(v),
            _ => None,
        }
    }
}

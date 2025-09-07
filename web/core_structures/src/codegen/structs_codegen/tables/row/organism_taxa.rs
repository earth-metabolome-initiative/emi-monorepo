impl From<crate::OrganismTaxon> for super::Row {
    fn from(value: crate::OrganismTaxon) -> Self {
        super::Row::OrganismTaxon(value)
    }
}
impl TryFrom<super::Row> for crate::OrganismTaxon {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::OrganismTaxon(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}

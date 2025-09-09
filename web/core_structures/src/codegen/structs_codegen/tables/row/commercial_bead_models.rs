impl From<crate::codegen::structs_codegen::tables::commercial_bead_models::CommercialBeadModel>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_bead_models::CommercialBeadModel,
    ) -> Self {
        super::Row::CommercialBeadModel(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::commercial_bead_models::CommercialBeadModel
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::CommercialBeadModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}

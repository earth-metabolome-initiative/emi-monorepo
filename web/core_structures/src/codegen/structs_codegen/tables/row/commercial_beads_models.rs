impl From<crate::codegen::structs_codegen::tables::commercial_beads_models::CommercialBeadsModel>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_beads_models::CommercialBeadsModel,
    ) -> Self {
        super::Row::CommercialBeadsModel(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::commercial_beads_models::CommercialBeadsModel
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::CommercialBeadsModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}

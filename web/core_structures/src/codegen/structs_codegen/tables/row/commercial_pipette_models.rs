impl
    From<crate::codegen::structs_codegen::tables::commercial_pipette_models::CommercialPipetteModel>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_pipette_models::CommercialPipetteModel,
    ) -> Self {
        super::Row::CommercialPipetteModel(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::commercial_pipette_models::CommercialPipetteModel
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::CommercialPipetteModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}

impl From<crate::codegen::structs_codegen::tables::commercial_bead_models::CommercialBeadModel>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_bead_models::CommercialBeadModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::commercial_bead_models::CommercialBeadModel>>
    for super::Rows
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::commercial_bead_models::CommercialBeadModel,
        >,
    ) -> Self {
        super::Rows::CommercialBeadModel(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::commercial_bead_models::CommercialBeadModel>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::CommercialBeadModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}

impl From<crate::codegen::structs_codegen::tables::commercial_cap_lots::CommercialCapLot>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_cap_lots::CommercialCapLot,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::commercial_cap_lots::CommercialCapLot>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::commercial_cap_lots::CommercialCapLot>,
    ) -> Self {
        super::Rows::CommercialCapLot(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::commercial_cap_lots::CommercialCapLot>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::CommercialCapLot(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}

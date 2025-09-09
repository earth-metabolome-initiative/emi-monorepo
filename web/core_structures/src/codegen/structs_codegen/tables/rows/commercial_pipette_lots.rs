impl From<crate::codegen::structs_codegen::tables::commercial_pipette_lots::CommercialPipetteLot>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_pipette_lots::CommercialPipetteLot,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl
    From<
        Vec<crate::codegen::structs_codegen::tables::commercial_pipette_lots::CommercialPipetteLot>,
    > for super::Rows
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::commercial_pipette_lots::CommercialPipetteLot,
        >,
    ) -> Self {
        super::Rows::CommercialPipetteLot(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::commercial_pipette_lots::CommercialPipetteLot>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::CommercialPipetteLot(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}

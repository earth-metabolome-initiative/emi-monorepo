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
impl From<super::Rows>
    for Option<
        Vec<crate::codegen::structs_codegen::tables::commercial_pipette_lots::CommercialPipetteLot>,
    >
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::CommercialPipetteLot(v) => Some(v),
            _ => None,
        }
    }
}

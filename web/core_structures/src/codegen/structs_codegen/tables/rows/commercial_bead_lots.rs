impl From<crate::codegen::structs_codegen::tables::commercial_bead_lots::CommercialBeadLot>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_bead_lots::CommercialBeadLot,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::commercial_bead_lots::CommercialBeadLot>>
    for super::Rows
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::commercial_bead_lots::CommercialBeadLot,
        >,
    ) -> Self {
        super::Rows::CommercialBeadLot(value)
    }
}
impl From<super::Rows>
    for Option<
        Vec<crate::codegen::structs_codegen::tables::commercial_bead_lots::CommercialBeadLot>,
    >
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::CommercialBeadLot(v) => Some(v),
            _ => None,
        }
    }
}

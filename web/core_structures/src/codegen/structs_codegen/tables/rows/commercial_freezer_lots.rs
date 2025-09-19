impl From<crate::codegen::structs_codegen::tables::commercial_freezer_lots::CommercialFreezerLot>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_freezer_lots::CommercialFreezerLot,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl
    From<
        Vec<crate::codegen::structs_codegen::tables::commercial_freezer_lots::CommercialFreezerLot>,
    > for super::Rows
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::commercial_freezer_lots::CommercialFreezerLot,
        >,
    ) -> Self {
        super::Rows::CommercialFreezerLot(value)
    }
}
impl From<super::Rows>
    for Option<
        Vec<crate::codegen::structs_codegen::tables::commercial_freezer_lots::CommercialFreezerLot>,
    >
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::CommercialFreezerLot(v) => Some(v),
            _ => None,
        }
    }
}

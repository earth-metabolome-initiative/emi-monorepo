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
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::commercial_freezer_lots::CommercialFreezerLot>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::CommercialFreezerLot(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}

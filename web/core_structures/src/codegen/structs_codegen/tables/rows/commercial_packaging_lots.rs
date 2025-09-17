impl
    From<crate::codegen::structs_codegen::tables::commercial_packaging_lots::CommercialPackagingLot>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_packaging_lots::CommercialPackagingLot,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::commercial_packaging_lots::CommercialPackagingLot,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::commercial_packaging_lots::CommercialPackagingLot,
        >,
    ) -> Self {
        super::Rows::CommercialPackagingLot(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<
        crate::codegen::structs_codegen::tables::commercial_packaging_lots::CommercialPackagingLot,
    >
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::CommercialPackagingLot(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}

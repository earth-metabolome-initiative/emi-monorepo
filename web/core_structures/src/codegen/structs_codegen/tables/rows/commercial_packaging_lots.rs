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
impl From<super::Rows>
for Option<
    Vec<
        crate::codegen::structs_codegen::tables::commercial_packaging_lots::CommercialPackagingLot,
    >,
> {
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::CommercialPackagingLot(v) => Some(v),
            _ => None,
        }
    }
}

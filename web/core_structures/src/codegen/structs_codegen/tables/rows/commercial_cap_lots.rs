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
impl From<super::Rows>
    for Option<Vec<crate::codegen::structs_codegen::tables::commercial_cap_lots::CommercialCapLot>>
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::CommercialCapLot(v) => Some(v),
            _ => None,
        }
    }
}

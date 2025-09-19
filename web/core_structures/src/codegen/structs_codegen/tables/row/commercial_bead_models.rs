impl From<crate::codegen::structs_codegen::tables::commercial_bead_models::CommercialBeadModel>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_bead_models::CommercialBeadModel,
    ) -> Self {
        super::Row::CommercialBeadModel(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::commercial_bead_models::CommercialBeadModel>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::CommercialBeadModel(v) => Some(v),
            _ => None,
        }
    }
}

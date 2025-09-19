impl
    From<crate::codegen::structs_codegen::tables::commercial_freezer_models::CommercialFreezerModel>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_freezer_models::CommercialFreezerModel,
    ) -> Self {
        super::Row::CommercialFreezerModel(value)
    }
}
impl From<super::Row>
    for Option<
        crate::codegen::structs_codegen::tables::commercial_freezer_models::CommercialFreezerModel,
    >
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::CommercialFreezerModel(v) => Some(v),
            _ => None,
        }
    }
}

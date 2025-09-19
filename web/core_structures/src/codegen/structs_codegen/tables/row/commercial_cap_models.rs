impl From<crate::codegen::structs_codegen::tables::commercial_cap_models::CommercialCapModel>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_cap_models::CommercialCapModel,
    ) -> Self {
        super::Row::CommercialCapModel(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::commercial_cap_models::CommercialCapModel>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::CommercialCapModel(v) => Some(v),
            _ => None,
        }
    }
}

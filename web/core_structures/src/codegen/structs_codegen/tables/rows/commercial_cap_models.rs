impl From<crate::codegen::structs_codegen::tables::commercial_cap_models::CommercialCapModel>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_cap_models::CommercialCapModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::commercial_cap_models::CommercialCapModel>>
    for super::Rows
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::commercial_cap_models::CommercialCapModel,
        >,
    ) -> Self {
        super::Rows::CommercialCapModel(value)
    }
}
impl From<super::Rows>
    for Option<
        Vec<crate::codegen::structs_codegen::tables::commercial_cap_models::CommercialCapModel>,
    >
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::CommercialCapModel(v) => Some(v),
            _ => None,
        }
    }
}

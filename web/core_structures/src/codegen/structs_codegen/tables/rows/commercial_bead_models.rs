impl From<crate::codegen::structs_codegen::tables::commercial_bead_models::CommercialBeadModel>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_bead_models::CommercialBeadModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::commercial_bead_models::CommercialBeadModel>>
    for super::Rows
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::commercial_bead_models::CommercialBeadModel,
        >,
    ) -> Self {
        super::Rows::CommercialBeadModel(value)
    }
}
impl From<super::Rows>
    for Option<
        Vec<crate::codegen::structs_codegen::tables::commercial_bead_models::CommercialBeadModel>,
    >
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::CommercialBeadModel(v) => Some(v),
            _ => None,
        }
    }
}

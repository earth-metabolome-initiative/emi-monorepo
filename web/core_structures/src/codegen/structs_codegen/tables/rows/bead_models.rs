impl From<crate::codegen::structs_codegen::tables::bead_models::BeadModel> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::bead_models::BeadModel) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::bead_models::BeadModel>> for super::Rows {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::bead_models::BeadModel>) -> Self {
        super::Rows::BeadModel(value)
    }
}
impl From<super::Rows>
    for Option<Vec<crate::codegen::structs_codegen::tables::bead_models::BeadModel>>
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::BeadModel(v) => Some(v),
            _ => None,
        }
    }
}

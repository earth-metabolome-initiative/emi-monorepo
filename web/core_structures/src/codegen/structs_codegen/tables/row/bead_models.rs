impl From<crate::codegen::structs_codegen::tables::bead_models::BeadModel> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::bead_models::BeadModel) -> Self {
        super::Row::BeadModel(value)
    }
}
impl From<super::Row> for Option<crate::codegen::structs_codegen::tables::bead_models::BeadModel> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::BeadModel(v) => Some(v),
            _ => None,
        }
    }
}

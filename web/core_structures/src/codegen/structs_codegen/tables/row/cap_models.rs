impl From<crate::codegen::structs_codegen::tables::cap_models::CapModel> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::cap_models::CapModel) -> Self {
        super::Row::CapModel(value)
    }
}
impl From<super::Row> for Option<crate::codegen::structs_codegen::tables::cap_models::CapModel> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::CapModel(v) => Some(v),
            _ => None,
        }
    }
}

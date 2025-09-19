impl From<crate::codegen::structs_codegen::tables::cap_models::CapModel> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::cap_models::CapModel) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::cap_models::CapModel>> for super::Rows {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::cap_models::CapModel>) -> Self {
        super::Rows::CapModel(value)
    }
}
impl From<super::Rows>
    for Option<Vec<crate::codegen::structs_codegen::tables::cap_models::CapModel>>
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::CapModel(v) => Some(v),
            _ => None,
        }
    }
}

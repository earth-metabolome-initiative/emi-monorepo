impl From<crate::codegen::structs_codegen::tables::phone_models::PhoneModel> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::phone_models::PhoneModel) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::phone_models::PhoneModel>> for super::Rows {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::phone_models::PhoneModel>) -> Self {
        super::Rows::PhoneModel(value)
    }
}
impl From<super::Rows>
    for Option<Vec<crate::codegen::structs_codegen::tables::phone_models::PhoneModel>>
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::PhoneModel(v) => Some(v),
            _ => None,
        }
    }
}

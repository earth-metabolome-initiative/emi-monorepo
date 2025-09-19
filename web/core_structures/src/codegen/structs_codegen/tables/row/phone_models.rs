impl From<crate::codegen::structs_codegen::tables::phone_models::PhoneModel> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::phone_models::PhoneModel) -> Self {
        super::Row::PhoneModel(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::phone_models::PhoneModel>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::PhoneModel(v) => Some(v),
            _ => None,
        }
    }
}

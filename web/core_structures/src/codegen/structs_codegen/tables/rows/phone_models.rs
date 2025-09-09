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
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::phone_models::PhoneModel>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::PhoneModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}

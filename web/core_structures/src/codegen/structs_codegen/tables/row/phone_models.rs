impl From<crate::codegen::structs_codegen::tables::phone_models::PhoneModel> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::phone_models::PhoneModel) -> Self {
        super::Row::PhoneModel(value)
    }
}
impl TryFrom<super::Row> for crate::codegen::structs_codegen::tables::phone_models::PhoneModel {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::PhoneModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}

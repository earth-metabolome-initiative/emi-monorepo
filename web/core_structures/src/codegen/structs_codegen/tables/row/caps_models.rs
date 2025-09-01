impl From<crate::codegen::structs_codegen::tables::caps_models::CapsModel> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::caps_models::CapsModel) -> Self {
        super::Row::CapsModel(value)
    }
}
impl TryFrom<super::Row> for crate::codegen::structs_codegen::tables::caps_models::CapsModel {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::CapsModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}

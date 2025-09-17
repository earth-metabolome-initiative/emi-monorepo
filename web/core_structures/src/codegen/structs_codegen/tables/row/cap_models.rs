impl From<crate::codegen::structs_codegen::tables::cap_models::CapModel> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::cap_models::CapModel) -> Self {
        super::Row::CapModel(value)
    }
}
impl TryFrom<super::Row> for crate::codegen::structs_codegen::tables::cap_models::CapModel {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::CapModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}

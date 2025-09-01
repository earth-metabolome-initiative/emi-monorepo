impl From<crate::codegen::structs_codegen::tables::beads_models::BeadsModel> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::beads_models::BeadsModel) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::beads_models::BeadsModel>> for super::Rows {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::beads_models::BeadsModel>) -> Self {
        super::Rows::BeadsModel(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::beads_models::BeadsModel>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::BeadsModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}

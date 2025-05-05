impl From<crate::codegen::structs_codegen::tables::nameplate_models::NameplateModel>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::nameplate_models::NameplateModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::nameplate_models::NameplateModel>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::nameplate_models::NameplateModel>,
    ) -> Self {
        super::Rows::NameplateModel(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::nameplate_models::NameplateModel>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::NameplateModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}

impl From<crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel>,
    ) -> Self {
        super::Rows::CentrifugeModel(value)
    }
}
impl From<super::Rows>
    for Option<Vec<crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel>>
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::CentrifugeModel(v) => Some(v),
            _ => None,
        }
    }
}

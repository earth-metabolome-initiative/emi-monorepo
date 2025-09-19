impl From<crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel,
    ) -> Self {
        super::Row::CentrifugeModel(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::CentrifugeModel(v) => Some(v),
            _ => None,
        }
    }
}

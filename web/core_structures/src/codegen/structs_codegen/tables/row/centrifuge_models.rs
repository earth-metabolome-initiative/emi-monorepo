impl From<crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel,
    ) -> Self {
        super::Row::CentrifugeModel(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::CentrifugeModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}

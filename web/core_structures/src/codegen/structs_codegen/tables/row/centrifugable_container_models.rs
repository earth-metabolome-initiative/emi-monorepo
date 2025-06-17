impl From<
    crate::codegen::structs_codegen::tables::centrifugable_container_models::CentrifugableContainerModel,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::centrifugable_container_models::CentrifugableContainerModel,
    ) -> Self {
        super::Row::CentrifugableContainerModel(value)
    }
}
impl TryFrom<super::Row>
for crate::codegen::structs_codegen::tables::centrifugable_container_models::CentrifugableContainerModel {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::CentrifugableContainerModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}

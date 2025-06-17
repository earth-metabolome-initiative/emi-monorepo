impl From<
    crate::codegen::structs_codegen::tables::centrifugable_container_models::CentrifugableContainerModel,
> for super::Rows {
    fn from(
        value: crate::codegen::structs_codegen::tables::centrifugable_container_models::CentrifugableContainerModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::centrifugable_container_models::CentrifugableContainerModel,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::centrifugable_container_models::CentrifugableContainerModel,
        >,
    ) -> Self {
        super::Rows::CentrifugableContainerModel(value)
    }
}
impl TryFrom<super::Rows>
for Vec<
    crate::codegen::structs_codegen::tables::centrifugable_container_models::CentrifugableContainerModel,
> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::CentrifugableContainerModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}

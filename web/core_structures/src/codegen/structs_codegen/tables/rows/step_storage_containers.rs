impl From<crate::codegen::structs_codegen::tables::step_storage_containers::StepStorageContainer>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::step_storage_containers::StepStorageContainer,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl
    From<
        Vec<crate::codegen::structs_codegen::tables::step_storage_containers::StepStorageContainer>,
    > for super::Rows
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::step_storage_containers::StepStorageContainer,
        >,
    ) -> Self {
        super::Rows::StepStorageContainer(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::step_storage_containers::StepStorageContainer>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::StepStorageContainer(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}

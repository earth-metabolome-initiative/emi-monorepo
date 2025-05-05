impl From<crate::codegen::structs_codegen::tables::storage_containers::StorageContainer>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::storage_containers::StorageContainer,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::storage_containers::StorageContainer>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::storage_containers::StorageContainer>,
    ) -> Self {
        super::Rows::StorageContainer(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::storage_containers::StorageContainer>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::StorageContainer(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}

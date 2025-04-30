impl From<crate::codegen::structs_codegen::tables::storage_containers::StorageContainer>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::storage_containers::StorageContainer,
    ) -> Self {
        super::Row::StorageContainer(std::rc::Rc::from(value))
    }
}
impl
    From<std::rc::Rc<crate::codegen::structs_codegen::tables::storage_containers::StorageContainer>>
    for super::Row
{
    fn from(
        value: std::rc::Rc<
            crate::codegen::structs_codegen::tables::storage_containers::StorageContainer,
        >,
    ) -> Self {
        super::Row::StorageContainer(std::rc::Rc::from(value))
    }
}
impl TryFrom<super::Row>
    for std::rc::Rc<crate::codegen::structs_codegen::tables::storage_containers::StorageContainer>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::StorageContainer(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}

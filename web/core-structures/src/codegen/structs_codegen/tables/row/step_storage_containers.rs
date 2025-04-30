impl From<crate::codegen::structs_codegen::tables::step_storage_containers::StepStorageContainer>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::step_storage_containers::StepStorageContainer,
    ) -> Self {
        super::Row::StepStorageContainer(std::rc::Rc::from(value))
    }
}
impl
    From<
        std::rc::Rc<
            crate::codegen::structs_codegen::tables::step_storage_containers::StepStorageContainer,
        >,
    > for super::Row
{
    fn from(
        value: std::rc::Rc<
            crate::codegen::structs_codegen::tables::step_storage_containers::StepStorageContainer,
        >,
    ) -> Self {
        super::Row::StepStorageContainer(std::rc::Rc::from(value))
    }
}
impl TryFrom<super::Row>
    for std::rc::Rc<
        crate::codegen::structs_codegen::tables::step_storage_containers::StepStorageContainer,
    >
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::StepStorageContainer(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}

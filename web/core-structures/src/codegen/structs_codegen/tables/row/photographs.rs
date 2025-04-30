impl From<crate::codegen::structs_codegen::tables::photographs::Photograph> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::photographs::Photograph) -> Self {
        super::Row::Photograph(std::rc::Rc::from(value))
    }
}
impl From<std::rc::Rc<crate::codegen::structs_codegen::tables::photographs::Photograph>>
    for super::Row
{
    fn from(
        value: std::rc::Rc<crate::codegen::structs_codegen::tables::photographs::Photograph>,
    ) -> Self {
        super::Row::Photograph(std::rc::Rc::from(value))
    }
}
impl TryFrom<super::Row>
    for std::rc::Rc<crate::codegen::structs_codegen::tables::photographs::Photograph>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::Photograph(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}

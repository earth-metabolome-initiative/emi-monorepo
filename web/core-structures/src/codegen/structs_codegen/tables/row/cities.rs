impl From<crate::codegen::structs_codegen::tables::cities::City> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::cities::City) -> Self {
        super::Row::City(std::rc::Rc::from(value))
    }
}
impl From<std::rc::Rc<crate::codegen::structs_codegen::tables::cities::City>> for super::Row {
    fn from(value: std::rc::Rc<crate::codegen::structs_codegen::tables::cities::City>) -> Self {
        super::Row::City(std::rc::Rc::from(value))
    }
}
impl TryFrom<super::Row> for std::rc::Rc<crate::codegen::structs_codegen::tables::cities::City> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::City(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}

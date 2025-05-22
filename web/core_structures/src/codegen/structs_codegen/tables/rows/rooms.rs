impl From<crate::codegen::structs_codegen::tables::rooms::Room> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::rooms::Room) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::rooms::Room>> for super::Rows {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::rooms::Room>) -> Self {
        super::Rows::Room(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::codegen::structs_codegen::tables::rooms::Room> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::Room(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}

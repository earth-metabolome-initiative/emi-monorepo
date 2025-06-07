impl From<crate::codegen::structs_codegen::tables::processables::Processable> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::processables::Processable) -> Self {
        super::Row::Processable(value)
    }
}
impl TryFrom<super::Row> for crate::codegen::structs_codegen::tables::processables::Processable {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::Processable(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}

impl From<crate::codegen::structs_codegen::tables::processables::Processable> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::processables::Processable) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::processables::Processable>> for super::Rows {
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::processables::Processable>,
    ) -> Self {
        super::Rows::Processable(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::processables::Processable>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::Processable(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}

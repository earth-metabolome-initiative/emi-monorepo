impl From<crate::codegen::structs_codegen::tables::cameras::Camera> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::cameras::Camera) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::cameras::Camera>> for super::Rows {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::cameras::Camera>) -> Self {
        super::Rows::Camera(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::codegen::structs_codegen::tables::cameras::Camera> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::Camera(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}

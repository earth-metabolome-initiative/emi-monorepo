impl From<crate::codegen::structs_codegen::tables::cameras::Camera> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::cameras::Camera) -> Self {
        super::Row::Camera(value)
    }
}
impl TryFrom<super::Row> for crate::codegen::structs_codegen::tables::cameras::Camera {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::Camera(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}

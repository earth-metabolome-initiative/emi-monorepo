impl From<crate::codegen::structs_codegen::tables::cameras::Camera> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::cameras::Camera) -> Self {
        super::Row::Camera(value)
    }
}
impl From<super::Row> for Option<crate::codegen::structs_codegen::tables::cameras::Camera> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::Camera(v) => Some(v),
            _ => None,
        }
    }
}

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
impl From<super::Rows> for Option<Vec<crate::codegen::structs_codegen::tables::cameras::Camera>> {
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::Camera(v) => Some(v),
            _ => None,
        }
    }
}

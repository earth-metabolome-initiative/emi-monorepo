impl From<crate::codegen::structs_codegen::tables::materials::Material> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::materials::Material) -> Self {
        super::Row::Material(value)
    }
}
impl From<super::Row> for Option<crate::codegen::structs_codegen::tables::materials::Material> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::Material(v) => Some(v),
            _ => None,
        }
    }
}

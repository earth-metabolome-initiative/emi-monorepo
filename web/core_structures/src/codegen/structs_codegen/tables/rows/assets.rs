impl From<crate::codegen::structs_codegen::tables::assets::Asset> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::assets::Asset) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::assets::Asset>> for super::Rows {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::assets::Asset>) -> Self {
        super::Rows::Asset(value)
    }
}
impl From<super::Rows> for Option<Vec<crate::codegen::structs_codegen::tables::assets::Asset>> {
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::Asset(v) => Some(v),
            _ => None,
        }
    }
}

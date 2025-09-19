impl From<crate::codegen::structs_codegen::tables::assets::Asset> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::assets::Asset) -> Self {
        super::Row::Asset(value)
    }
}
impl From<super::Row> for Option<crate::codegen::structs_codegen::tables::assets::Asset> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::Asset(v) => Some(v),
            _ => None,
        }
    }
}

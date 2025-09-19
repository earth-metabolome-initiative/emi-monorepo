impl From<crate::codegen::structs_codegen::tables::addresses::Address> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::addresses::Address) -> Self {
        super::Row::Address(value)
    }
}
impl From<super::Row> for Option<crate::codegen::structs_codegen::tables::addresses::Address> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::Address(v) => Some(v),
            _ => None,
        }
    }
}

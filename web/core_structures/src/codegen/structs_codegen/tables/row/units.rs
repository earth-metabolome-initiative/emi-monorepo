impl From<crate::codegen::structs_codegen::tables::units::Unit> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::units::Unit) -> Self {
        super::Row::Unit(value)
    }
}
impl From<super::Row> for Option<crate::codegen::structs_codegen::tables::units::Unit> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::Unit(v) => Some(v),
            _ => None,
        }
    }
}

impl From<crate::codegen::structs_codegen::tables::procedures::Procedure> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::procedures::Procedure) -> Self {
        super::Row::Procedure(value)
    }
}
impl From<super::Row> for Option<crate::codegen::structs_codegen::tables::procedures::Procedure> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::Procedure(v) => Some(v),
            _ => None,
        }
    }
}

impl From<crate::codegen::structs_codegen::tables::organizations::Organization> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::organizations::Organization) -> Self {
        super::Row::Organization(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::organizations::Organization>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::Organization(v) => Some(v),
            _ => None,
        }
    }
}

impl From<crate::codegen::structs_codegen::tables::permanence_categories::PermanenceCategory>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::permanence_categories::PermanenceCategory,
    ) -> Self {
        super::Row::PermanenceCategory(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::permanence_categories::PermanenceCategory>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::PermanenceCategory(v) => Some(v),
            _ => None,
        }
    }
}

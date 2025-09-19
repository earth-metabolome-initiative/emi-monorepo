impl From<crate::codegen::structs_codegen::tables::permanence_categories::PermanenceCategory>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::permanence_categories::PermanenceCategory,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::permanence_categories::PermanenceCategory>>
    for super::Rows
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::permanence_categories::PermanenceCategory,
        >,
    ) -> Self {
        super::Rows::PermanenceCategory(value)
    }
}
impl From<super::Rows>
    for Option<
        Vec<crate::codegen::structs_codegen::tables::permanence_categories::PermanenceCategory>,
    >
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::PermanenceCategory(v) => Some(v),
            _ => None,
        }
    }
}

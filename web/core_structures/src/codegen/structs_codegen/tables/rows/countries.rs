impl From<crate::codegen::structs_codegen::tables::countries::Country> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::countries::Country) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::countries::Country>> for super::Rows {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::countries::Country>) -> Self {
        super::Rows::Country(value)
    }
}
impl From<super::Rows>
    for Option<Vec<crate::codegen::structs_codegen::tables::countries::Country>>
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::Country(v) => Some(v),
            _ => None,
        }
    }
}

impl From<crate::codegen::structs_codegen::tables::addresses::Address> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::addresses::Address) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::addresses::Address>> for super::Rows {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::addresses::Address>) -> Self {
        super::Rows::Address(value)
    }
}
impl From<super::Rows>
    for Option<Vec<crate::codegen::structs_codegen::tables::addresses::Address>>
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::Address(v) => Some(v),
            _ => None,
        }
    }
}

impl From<crate::codegen::structs_codegen::tables::centrifuges::Centrifuge> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::centrifuges::Centrifuge) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::centrifuges::Centrifuge>> for super::Rows {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::centrifuges::Centrifuge>) -> Self {
        super::Rows::Centrifuge(value)
    }
}
impl From<super::Rows>
    for Option<Vec<crate::codegen::structs_codegen::tables::centrifuges::Centrifuge>>
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::Centrifuge(v) => Some(v),
            _ => None,
        }
    }
}

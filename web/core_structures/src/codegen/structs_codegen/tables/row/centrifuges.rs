impl From<crate::codegen::structs_codegen::tables::centrifuges::Centrifuge> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::centrifuges::Centrifuge) -> Self {
        super::Row::Centrifuge(value)
    }
}
impl From<super::Row> for Option<crate::codegen::structs_codegen::tables::centrifuges::Centrifuge> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::Centrifuge(v) => Some(v),
            _ => None,
        }
    }
}

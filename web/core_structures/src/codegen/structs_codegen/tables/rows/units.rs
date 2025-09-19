impl From<crate::codegen::structs_codegen::tables::units::Unit> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::units::Unit) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::units::Unit>> for super::Rows {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::units::Unit>) -> Self {
        super::Rows::Unit(value)
    }
}
impl From<super::Rows> for Option<Vec<crate::codegen::structs_codegen::tables::units::Unit>> {
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::Unit(v) => Some(v),
            _ => None,
        }
    }
}

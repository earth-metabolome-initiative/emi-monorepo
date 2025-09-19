impl From<crate::codegen::structs_codegen::tables::pipettes::Pipette> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::pipettes::Pipette) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::pipettes::Pipette>> for super::Rows {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::pipettes::Pipette>) -> Self {
        super::Rows::Pipette(value)
    }
}
impl From<super::Rows> for Option<Vec<crate::codegen::structs_codegen::tables::pipettes::Pipette>> {
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::Pipette(v) => Some(v),
            _ => None,
        }
    }
}

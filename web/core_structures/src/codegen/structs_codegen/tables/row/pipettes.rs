impl From<crate::codegen::structs_codegen::tables::pipettes::Pipette> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::pipettes::Pipette) -> Self {
        super::Row::Pipette(value)
    }
}
impl From<super::Row> for Option<crate::codegen::structs_codegen::tables::pipettes::Pipette> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::Pipette(v) => Some(v),
            _ => None,
        }
    }
}

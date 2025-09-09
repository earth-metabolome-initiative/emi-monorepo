impl From<crate::codegen::structs_codegen::tables::pipettes::Pipette> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::pipettes::Pipette) -> Self {
        super::Row::Pipette(value)
    }
}
impl TryFrom<super::Row> for crate::codegen::structs_codegen::tables::pipettes::Pipette {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::Pipette(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}

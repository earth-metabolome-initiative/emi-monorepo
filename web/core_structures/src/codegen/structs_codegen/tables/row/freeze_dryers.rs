impl From<crate::codegen::structs_codegen::tables::freeze_dryers::FreezeDryer> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::freeze_dryers::FreezeDryer) -> Self {
        super::Row::FreezeDryer(value)
    }
}
impl TryFrom<super::Row> for crate::codegen::structs_codegen::tables::freeze_dryers::FreezeDryer {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::FreezeDryer(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}

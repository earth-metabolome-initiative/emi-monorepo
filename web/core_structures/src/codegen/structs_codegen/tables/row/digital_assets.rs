impl From<crate::codegen::structs_codegen::tables::digital_assets::DigitalAsset> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::digital_assets::DigitalAsset) -> Self {
        super::Row::DigitalAsset(value)
    }
}
impl TryFrom<super::Row> for crate::codegen::structs_codegen::tables::digital_assets::DigitalAsset {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::DigitalAsset(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}

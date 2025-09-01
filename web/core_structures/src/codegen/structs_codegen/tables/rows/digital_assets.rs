impl From<crate::codegen::structs_codegen::tables::digital_assets::DigitalAsset> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::digital_assets::DigitalAsset) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::digital_assets::DigitalAsset>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::digital_assets::DigitalAsset>,
    ) -> Self {
        super::Rows::DigitalAsset(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::digital_assets::DigitalAsset>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::DigitalAsset(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}

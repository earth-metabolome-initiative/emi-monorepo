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
impl From<super::Rows>
    for Option<Vec<crate::codegen::structs_codegen::tables::digital_assets::DigitalAsset>>
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::DigitalAsset(v) => Some(v),
            _ => None,
        }
    }
}

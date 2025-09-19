impl From<crate::codegen::structs_codegen::tables::digital_assets::DigitalAsset> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::digital_assets::DigitalAsset) -> Self {
        super::Row::DigitalAsset(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::digital_assets::DigitalAsset>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::DigitalAsset(v) => Some(v),
            _ => None,
        }
    }
}

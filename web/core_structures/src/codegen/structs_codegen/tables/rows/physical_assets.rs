impl From<crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset> for super::Rows {
    fn from(
        value: crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset>,
    ) -> Self {
        super::Rows::PhysicalAsset(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::PhysicalAsset(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}

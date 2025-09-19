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
impl From<super::Rows>
    for Option<Vec<crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset>>
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::PhysicalAsset(v) => Some(v),
            _ => None,
        }
    }
}

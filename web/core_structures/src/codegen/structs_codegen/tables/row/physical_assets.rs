impl From<crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset,
    ) -> Self {
        super::Row::PhysicalAsset(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::PhysicalAsset(v) => Some(v),
            _ => None,
        }
    }
}

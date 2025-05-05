impl From<crate::codegen::structs_codegen::tables::nameplate_categories::NameplateCategory>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::nameplate_categories::NameplateCategory,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::nameplate_categories::NameplateCategory>>
    for super::Rows
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::nameplate_categories::NameplateCategory,
        >,
    ) -> Self {
        super::Rows::NameplateCategory(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::nameplate_categories::NameplateCategory>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::NameplateCategory(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}

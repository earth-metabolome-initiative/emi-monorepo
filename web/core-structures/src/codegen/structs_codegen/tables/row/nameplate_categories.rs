impl From<crate::codegen::structs_codegen::tables::nameplate_categories::NameplateCategory>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::nameplate_categories::NameplateCategory,
    ) -> Self {
        super::Row::NameplateCategory(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::nameplate_categories::NameplateCategory
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::NameplateCategory(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}

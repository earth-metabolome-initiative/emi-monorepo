impl From<crate::codegen::structs_codegen::tables::trackable_categories::TrackableCategory>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::trackable_categories::TrackableCategory,
    ) -> Self {
        super::Row::TrackableCategory(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::trackable_categories::TrackableCategory
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::TrackableCategory(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}

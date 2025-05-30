impl From<crate::codegen::structs_codegen::tables::trackable_categories::TrackableCategory>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::trackable_categories::TrackableCategory,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::trackable_categories::TrackableCategory>>
    for super::Rows
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::trackable_categories::TrackableCategory,
        >,
    ) -> Self {
        super::Rows::TrackableCategory(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::trackable_categories::TrackableCategory>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::TrackableCategory(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}

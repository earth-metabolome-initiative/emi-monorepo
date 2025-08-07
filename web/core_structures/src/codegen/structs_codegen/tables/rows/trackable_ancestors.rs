impl From<crate::codegen::structs_codegen::tables::trackable_ancestors::TrackableAncestor>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::trackable_ancestors::TrackableAncestor,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::trackable_ancestors::TrackableAncestor>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::trackable_ancestors::TrackableAncestor>,
    ) -> Self {
        super::Rows::TrackableAncestor(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::trackable_ancestors::TrackableAncestor>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::TrackableAncestor(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}

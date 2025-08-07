impl From<crate::codegen::structs_codegen::tables::trackable_ancestors::TrackableAncestor>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::trackable_ancestors::TrackableAncestor,
    ) -> Self {
        super::Row::TrackableAncestor(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::trackable_ancestors::TrackableAncestor
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::TrackableAncestor(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}

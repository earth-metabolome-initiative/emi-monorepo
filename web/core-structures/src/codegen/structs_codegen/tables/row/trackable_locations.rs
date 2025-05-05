impl From<crate::codegen::structs_codegen::tables::trackable_locations::TrackableLocation>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::trackable_locations::TrackableLocation,
    ) -> Self {
        super::Row::TrackableLocation(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::trackable_locations::TrackableLocation
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::TrackableLocation(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}

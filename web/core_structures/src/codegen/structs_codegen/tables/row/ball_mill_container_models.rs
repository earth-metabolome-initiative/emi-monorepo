impl
    From<
        crate::codegen::structs_codegen::tables::ball_mill_container_models::BallMillContainerModel,
    > for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::ball_mill_container_models::BallMillContainerModel,
    ) -> Self {
        super::Row::BallMillContainerModel(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::ball_mill_container_models::BallMillContainerModel
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::BallMillContainerModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}

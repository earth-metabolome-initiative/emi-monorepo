impl
    From<
        crate::codegen::structs_codegen::tables::ball_mill_container_models::BallMillContainerModel,
    > for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::ball_mill_container_models::BallMillContainerModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::ball_mill_container_models::BallMillContainerModel,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::ball_mill_container_models::BallMillContainerModel,
        >,
    ) -> Self {
        super::Rows::BallMillContainerModel(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<
        crate::codegen::structs_codegen::tables::ball_mill_container_models::BallMillContainerModel,
    >
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::BallMillContainerModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}

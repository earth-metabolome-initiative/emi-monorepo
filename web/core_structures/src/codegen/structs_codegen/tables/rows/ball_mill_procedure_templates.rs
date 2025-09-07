impl From<crate::BallMillProcedureTemplate> for super::Rows {
    fn from(value: crate::BallMillProcedureTemplate) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::BallMillProcedureTemplate>> for super::Rows {
    fn from(value: Vec<crate::BallMillProcedureTemplate>) -> Self {
        super::Rows::BallMillProcedureTemplate(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::BallMillProcedureTemplate> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::BallMillProcedureTemplate(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}

impl From<crate::BallMillMachine> for super::Rows {
    fn from(value: crate::BallMillMachine) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::BallMillMachine>> for super::Rows {
    fn from(value: Vec<crate::BallMillMachine>) -> Self {
        super::Rows::BallMillMachine(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::BallMillMachine> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::BallMillMachine(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}

impl From<crate::BallMillMachine> for super::Row {
    fn from(value: crate::BallMillMachine) -> Self {
        super::Row::BallMillMachine(value)
    }
}
impl TryFrom<super::Row> for crate::BallMillMachine {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::BallMillMachine(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}

impl From<crate::Room> for super::Rows {
    fn from(value: crate::Room) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::Room>> for super::Rows {
    fn from(value: Vec<crate::Room>) -> Self {
        super::Rows::Room(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::Room> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::Room(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}

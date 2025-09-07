impl From<crate::Room> for super::Row {
    fn from(value: crate::Room) -> Self {
        super::Row::Room(value)
    }
}
impl TryFrom<super::Row> for crate::Room {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::Room(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}

impl From<crate::Color> for super::Row {
    fn from(value: crate::Color) -> Self {
        super::Row::Color(value)
    }
}
impl TryFrom<super::Row> for crate::Color {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::Color(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}

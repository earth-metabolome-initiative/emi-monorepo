impl From<crate::Organism> for super::Rows {
    fn from(value: crate::Organism) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::Organism>> for super::Rows {
    fn from(value: Vec<crate::Organism>) -> Self {
        super::Rows::Organism(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::Organism> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::Organism(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}

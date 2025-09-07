impl From<crate::Pipette> for super::Rows {
    fn from(value: crate::Pipette) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::Pipette>> for super::Rows {
    fn from(value: Vec<crate::Pipette>) -> Self {
        super::Rows::Pipette(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::Pipette> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::Pipette(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}

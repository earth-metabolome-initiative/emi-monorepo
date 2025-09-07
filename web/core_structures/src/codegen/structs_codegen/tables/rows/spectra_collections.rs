impl From<crate::SpectraCollection> for super::Rows {
    fn from(value: crate::SpectraCollection) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::SpectraCollection>> for super::Rows {
    fn from(value: Vec<crate::SpectraCollection>) -> Self {
        super::Rows::SpectraCollection(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::SpectraCollection> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::SpectraCollection(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}

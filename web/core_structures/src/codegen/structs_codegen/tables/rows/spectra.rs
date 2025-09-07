impl From<crate::Spectrum> for super::Rows {
    fn from(value: crate::Spectrum) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::Spectrum>> for super::Rows {
    fn from(value: Vec<crate::Spectrum>) -> Self {
        super::Rows::Spectrum(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::Spectrum> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::Spectrum(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}

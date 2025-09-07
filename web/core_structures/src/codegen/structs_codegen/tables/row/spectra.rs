impl From<crate::Spectrum> for super::Row {
    fn from(value: crate::Spectrum) -> Self {
        super::Row::Spectrum(value)
    }
}
impl TryFrom<super::Row> for crate::Spectrum {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::Spectrum(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}

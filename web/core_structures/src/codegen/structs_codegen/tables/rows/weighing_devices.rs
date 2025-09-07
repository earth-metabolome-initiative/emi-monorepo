impl From<crate::WeighingDevice> for super::Rows {
    fn from(value: crate::WeighingDevice) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::WeighingDevice>> for super::Rows {
    fn from(value: Vec<crate::WeighingDevice>) -> Self {
        super::Rows::WeighingDevice(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::WeighingDevice> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::WeighingDevice(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}

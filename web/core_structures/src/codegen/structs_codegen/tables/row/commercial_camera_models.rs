impl From<crate::CommercialCameraModel> for super::Row {
    fn from(value: crate::CommercialCameraModel) -> Self {
        super::Row::CommercialCameraModel(value)
    }
}
impl TryFrom<super::Row> for crate::CommercialCameraModel {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::CommercialCameraModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}

impl From<crate::CommercialCameraModel> for super::Rows {
    fn from(value: crate::CommercialCameraModel) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::CommercialCameraModel>> for super::Rows {
    fn from(value: Vec<crate::CommercialCameraModel>) -> Self {
        super::Rows::CommercialCameraModel(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::CommercialCameraModel> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::CommercialCameraModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}

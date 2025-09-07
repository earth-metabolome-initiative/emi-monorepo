impl From<crate::CameraModel> for super::Rows {
    fn from(value: crate::CameraModel) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::CameraModel>> for super::Rows {
    fn from(value: Vec<crate::CameraModel>) -> Self {
        super::Rows::CameraModel(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::CameraModel> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::CameraModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}

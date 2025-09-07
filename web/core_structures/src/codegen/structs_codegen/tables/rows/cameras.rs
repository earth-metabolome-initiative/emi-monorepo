impl From<crate::Camera> for super::Rows {
    fn from(value: crate::Camera) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::Camera>> for super::Rows {
    fn from(value: Vec<crate::Camera>) -> Self {
        super::Rows::Camera(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::Camera> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::Camera(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}

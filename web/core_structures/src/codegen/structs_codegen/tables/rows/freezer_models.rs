impl From<crate::FreezerModel> for super::Rows {
    fn from(value: crate::FreezerModel) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::FreezerModel>> for super::Rows {
    fn from(value: Vec<crate::FreezerModel>) -> Self {
        super::Rows::FreezerModel(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::FreezerModel> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::FreezerModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}

impl From<crate::AssetCompatibilityRule> for super::Rows {
    fn from(value: crate::AssetCompatibilityRule) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::AssetCompatibilityRule>> for super::Rows {
    fn from(value: Vec<crate::AssetCompatibilityRule>) -> Self {
        super::Rows::AssetCompatibilityRule(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::AssetCompatibilityRule> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::AssetCompatibilityRule(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}

impl From<crate::AssetCompatibilityRule> for super::Row {
    fn from(value: crate::AssetCompatibilityRule) -> Self {
        super::Row::AssetCompatibilityRule(value)
    }
}
impl TryFrom<super::Row> for crate::AssetCompatibilityRule {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::AssetCompatibilityRule(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}

impl From<crate::ContainerCompatibilityRule> for super::Rows {
    fn from(value: crate::ContainerCompatibilityRule) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::ContainerCompatibilityRule>> for super::Rows {
    fn from(value: Vec<crate::ContainerCompatibilityRule>) -> Self {
        super::Rows::ContainerCompatibilityRule(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::ContainerCompatibilityRule> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::ContainerCompatibilityRule(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}

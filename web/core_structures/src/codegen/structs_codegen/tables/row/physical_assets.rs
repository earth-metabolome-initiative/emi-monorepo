impl From<crate::PhysicalAsset> for super::Row {
    fn from(value: crate::PhysicalAsset) -> Self {
        super::Row::PhysicalAsset(value)
    }
}
impl TryFrom<super::Row> for crate::PhysicalAsset {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::PhysicalAsset(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}

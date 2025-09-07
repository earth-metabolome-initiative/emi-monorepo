impl From<crate::AssetModelAncestor> for super::Rows {
    fn from(value: crate::AssetModelAncestor) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::AssetModelAncestor>> for super::Rows {
    fn from(value: Vec<crate::AssetModelAncestor>) -> Self {
        super::Rows::AssetModelAncestor(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::AssetModelAncestor> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::AssetModelAncestor(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}

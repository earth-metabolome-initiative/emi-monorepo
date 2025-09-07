impl From<crate::AssetModelAncestor> for super::Row {
    fn from(value: crate::AssetModelAncestor) -> Self {
        super::Row::AssetModelAncestor(value)
    }
}
impl TryFrom<super::Row> for crate::AssetModelAncestor {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::AssetModelAncestor(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}

impl From<crate::UserOrganization> for super::Row {
    fn from(value: crate::UserOrganization) -> Self {
        super::Row::UserOrganization(value)
    }
}
impl TryFrom<super::Row> for crate::UserOrganization {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::UserOrganization(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}

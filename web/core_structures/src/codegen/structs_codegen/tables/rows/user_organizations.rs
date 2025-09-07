impl From<crate::UserOrganization> for super::Rows {
    fn from(value: crate::UserOrganization) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::UserOrganization>> for super::Rows {
    fn from(value: Vec<crate::UserOrganization>) -> Self {
        super::Rows::UserOrganization(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::UserOrganization> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::UserOrganization(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}

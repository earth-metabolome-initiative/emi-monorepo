impl From<crate::PouringProcedureTemplate> for super::Rows {
    fn from(value: crate::PouringProcedureTemplate) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::PouringProcedureTemplate>> for super::Rows {
    fn from(value: Vec<crate::PouringProcedureTemplate>) -> Self {
        super::Rows::PouringProcedureTemplate(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::PouringProcedureTemplate> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::PouringProcedureTemplate(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}

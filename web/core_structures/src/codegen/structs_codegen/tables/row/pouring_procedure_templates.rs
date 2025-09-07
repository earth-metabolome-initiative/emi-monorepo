impl From<crate::PouringProcedureTemplate> for super::Row {
    fn from(value: crate::PouringProcedureTemplate) -> Self {
        super::Row::PouringProcedureTemplate(value)
    }
}
impl TryFrom<super::Row> for crate::PouringProcedureTemplate {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::PouringProcedureTemplate(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}

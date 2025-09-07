impl From<crate::PouringProcedure> for super::Row {
    fn from(value: crate::PouringProcedure) -> Self {
        super::Row::PouringProcedure(value)
    }
}
impl TryFrom<super::Row> for crate::PouringProcedure {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::PouringProcedure(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}

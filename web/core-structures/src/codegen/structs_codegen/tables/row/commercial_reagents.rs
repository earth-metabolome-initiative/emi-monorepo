impl From<crate::codegen::structs_codegen::tables::commercial_reagents::CommercialReagent>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_reagents::CommercialReagent,
    ) -> Self {
        super::Row::CommercialReagent(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::commercial_reagents::CommercialReagent
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::CommercialReagent(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}

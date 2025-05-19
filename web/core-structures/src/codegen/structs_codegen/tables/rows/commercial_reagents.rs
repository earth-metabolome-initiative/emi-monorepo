impl From<crate::codegen::structs_codegen::tables::commercial_reagents::CommercialReagent>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_reagents::CommercialReagent,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::commercial_reagents::CommercialReagent>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::commercial_reagents::CommercialReagent>,
    ) -> Self {
        super::Rows::CommercialReagent(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::commercial_reagents::CommercialReagent>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::CommercialReagent(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}

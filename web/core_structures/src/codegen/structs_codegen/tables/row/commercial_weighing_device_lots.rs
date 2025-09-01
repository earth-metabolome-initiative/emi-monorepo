impl From<
    crate::codegen::structs_codegen::tables::commercial_weighing_device_lots::CommercialWeighingDeviceLot,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_weighing_device_lots::CommercialWeighingDeviceLot,
    ) -> Self {
        super::Row::CommercialWeighingDeviceLot(value)
    }
}
impl TryFrom<super::Row>
for crate::codegen::structs_codegen::tables::commercial_weighing_device_lots::CommercialWeighingDeviceLot {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::CommercialWeighingDeviceLot(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}

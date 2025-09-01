impl From<
    crate::codegen::structs_codegen::tables::commercial_positioning_device_lots::CommercialPositioningDeviceLot,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_positioning_device_lots::CommercialPositioningDeviceLot,
    ) -> Self {
        super::Row::CommercialPositioningDeviceLot(value)
    }
}
impl TryFrom<super::Row>
for crate::codegen::structs_codegen::tables::commercial_positioning_device_lots::CommercialPositioningDeviceLot {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::CommercialPositioningDeviceLot(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}

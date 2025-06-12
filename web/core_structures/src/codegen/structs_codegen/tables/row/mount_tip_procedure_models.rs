impl
    From<
        crate::codegen::structs_codegen::tables::mount_tip_procedure_models::MountTipProcedureModel,
    > for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::mount_tip_procedure_models::MountTipProcedureModel,
    ) -> Self {
        super::Row::MountTipProcedureModel(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::mount_tip_procedure_models::MountTipProcedureModel
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::MountTipProcedureModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}

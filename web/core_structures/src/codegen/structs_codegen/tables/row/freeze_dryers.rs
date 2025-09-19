impl From<crate::codegen::structs_codegen::tables::freeze_dryers::FreezeDryer> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::freeze_dryers::FreezeDryer) -> Self {
        super::Row::FreezeDryer(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::freeze_dryers::FreezeDryer>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::FreezeDryer(v) => Some(v),
            _ => None,
        }
    }
}

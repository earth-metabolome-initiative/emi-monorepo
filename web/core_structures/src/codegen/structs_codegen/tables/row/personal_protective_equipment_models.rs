impl From<
    crate::codegen::structs_codegen::tables::personal_protective_equipment_models::PersonalProtectiveEquipmentModel,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::personal_protective_equipment_models::PersonalProtectiveEquipmentModel,
    ) -> Self {
        super::Row::PersonalProtectiveEquipmentModel(value)
    }
}
impl From<super::Row>
for Option<
    crate::codegen::structs_codegen::tables::personal_protective_equipment_models::PersonalProtectiveEquipmentModel,
> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::PersonalProtectiveEquipmentModel(v) => Some(v),
            _ => None,
        }
    }
}

impl From<
    crate::codegen::structs_codegen::tables::personal_protective_equipment_models::PersonalProtectiveEquipmentModel,
> for super::Rows {
    fn from(
        value: crate::codegen::structs_codegen::tables::personal_protective_equipment_models::PersonalProtectiveEquipmentModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::personal_protective_equipment_models::PersonalProtectiveEquipmentModel,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::personal_protective_equipment_models::PersonalProtectiveEquipmentModel,
        >,
    ) -> Self {
        super::Rows::PersonalProtectiveEquipmentModel(value)
    }
}
impl From<super::Rows>
for Option<
    Vec<
        crate::codegen::structs_codegen::tables::personal_protective_equipment_models::PersonalProtectiveEquipmentModel,
    >,
> {
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::PersonalProtectiveEquipmentModel(v) => Some(v),
            _ => None,
        }
    }
}

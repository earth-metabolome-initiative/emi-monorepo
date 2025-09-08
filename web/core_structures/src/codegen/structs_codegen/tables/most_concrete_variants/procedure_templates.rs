#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Enum representing the most concrete variant of the `procedure_templates`
/// table DAG.
///
/// # Mermaid illustration of the DAG:
/// ```mermaid
/// flowchart BT
/// v0@{shape: rect, label: "aliquoting_procedure_templates"}
/// v1@{shape: rect, label: "ball_mill_procedure_templates"}
/// v2@{shape: rect, label: "capping_procedure_templates"}
/// v3@{shape: rect, label: "centrifuge_procedure_templates"}
/// v4@{shape: rect, label: "disposal_procedure_templates"}
/// v5@{shape: rect, label: "fractioning_procedure_templates"}
/// v6@{shape: rect, label: "freeze_drying_procedure_templates"}
/// v7@{shape: rect, label: "freezing_procedure_templates"}
/// v8@{shape: rect, label: "geolocation_procedure_templates"}
/// v9@{shape: rect, label: "packaging_procedure_templates"}
/// v10@{shape: rect, label: "photograph_procedure_templates"}
/// v11@{shape: rect, label: "pouring_procedure_templates"}
/// v12@{shape: rect, label: "procedure_templates"}
/// v13@{shape: rect, label: "storage_procedure_templates"}
/// v14@{shape: rect, label: "supernatant_procedure_templates"}
/// v15@{shape: rect, label: "weighing_procedure_templates"}
/// v4 --->|"`extends`"| v12
/// v0 --->|"`extends`"| v12
/// v10 --->|"`extends`"| v12
/// v14 --->|"`extends`"| v12
/// v11 --->|"`extends`"| v12
/// v13 --->|"`extends`"| v12
/// v9 --->|"`extends`"| v12
/// v6 --->|"`extends`"| v12
/// v7 --->|"`extends`"| v12
/// v1 --->|"`extends`"| v12
/// v8 --->|"`extends`"| v12
/// v3 --->|"`extends`"| v12
/// v15 --->|"`extends`"| v12
/// v5 --->|"`extends`"| v12
/// v2 --->|"`extends`"| v12
/// ```
pub enum ProcedureTemplateDAG {
    /// Variant representing the `aliquoting_procedure_templates` table.
    AliquotingProcedureTemplate(crate::AliquotingProcedureTemplate),
    /// Variant representing the `ball_mill_procedure_templates` table.
    BallMillProcedureTemplate(crate::BallMillProcedureTemplate),
    /// Variant representing the `capping_procedure_templates` table.
    CappingProcedureTemplate(crate::CappingProcedureTemplate),
    /// Variant representing the `centrifuge_procedure_templates` table.
    CentrifugeProcedureTemplate(crate::CentrifugeProcedureTemplate),
    /// Variant representing the `disposal_procedure_templates` table.
    DisposalProcedureTemplate(crate::DisposalProcedureTemplate),
    /// Variant representing the `fractioning_procedure_templates` table.
    FractioningProcedureTemplate(crate::FractioningProcedureTemplate),
    /// Variant representing the `freeze_drying_procedure_templates` table.
    FreezeDryingProcedureTemplate(crate::FreezeDryingProcedureTemplate),
    /// Variant representing the `freezing_procedure_templates` table.
    FreezingProcedureTemplate(crate::FreezingProcedureTemplate),
    /// Variant representing the `geolocation_procedure_templates` table.
    GeolocationProcedureTemplate(crate::GeolocationProcedureTemplate),
    /// Variant representing the `packaging_procedure_templates` table.
    PackagingProcedureTemplate(crate::PackagingProcedureTemplate),
    /// Variant representing the `photograph_procedure_templates` table.
    PhotographProcedureTemplate(crate::PhotographProcedureTemplate),
    /// Variant representing the `pouring_procedure_templates` table.
    PouringProcedureTemplate(crate::PouringProcedureTemplate),
    /// Variant representing the `procedure_templates` table.
    ProcedureTemplate(crate::ProcedureTemplate),
    /// Variant representing the `storage_procedure_templates` table.
    StorageProcedureTemplate(crate::StorageProcedureTemplate),
    /// Variant representing the `supernatant_procedure_templates` table.
    SupernatantProcedureTemplate(crate::SupernatantProcedureTemplate),
    /// Variant representing the `weighing_procedure_templates` table.
    WeighingProcedureTemplate(crate::WeighingProcedureTemplate),
}
impl From<crate::AliquotingProcedureTemplate> for ProcedureTemplateDAG {
    fn from(value: crate::AliquotingProcedureTemplate) -> Self {
        ProcedureTemplateDAG::AliquotingProcedureTemplate(value)
    }
}
impl From<crate::BallMillProcedureTemplate> for ProcedureTemplateDAG {
    fn from(value: crate::BallMillProcedureTemplate) -> Self {
        ProcedureTemplateDAG::BallMillProcedureTemplate(value)
    }
}
impl From<crate::CappingProcedureTemplate> for ProcedureTemplateDAG {
    fn from(value: crate::CappingProcedureTemplate) -> Self {
        ProcedureTemplateDAG::CappingProcedureTemplate(value)
    }
}
impl From<crate::CentrifugeProcedureTemplate> for ProcedureTemplateDAG {
    fn from(value: crate::CentrifugeProcedureTemplate) -> Self {
        ProcedureTemplateDAG::CentrifugeProcedureTemplate(value)
    }
}
impl From<crate::DisposalProcedureTemplate> for ProcedureTemplateDAG {
    fn from(value: crate::DisposalProcedureTemplate) -> Self {
        ProcedureTemplateDAG::DisposalProcedureTemplate(value)
    }
}
impl From<crate::FractioningProcedureTemplate> for ProcedureTemplateDAG {
    fn from(value: crate::FractioningProcedureTemplate) -> Self {
        ProcedureTemplateDAG::FractioningProcedureTemplate(value)
    }
}
impl From<crate::FreezeDryingProcedureTemplate> for ProcedureTemplateDAG {
    fn from(value: crate::FreezeDryingProcedureTemplate) -> Self {
        ProcedureTemplateDAG::FreezeDryingProcedureTemplate(value)
    }
}
impl From<crate::FreezingProcedureTemplate> for ProcedureTemplateDAG {
    fn from(value: crate::FreezingProcedureTemplate) -> Self {
        ProcedureTemplateDAG::FreezingProcedureTemplate(value)
    }
}
impl From<crate::GeolocationProcedureTemplate> for ProcedureTemplateDAG {
    fn from(value: crate::GeolocationProcedureTemplate) -> Self {
        ProcedureTemplateDAG::GeolocationProcedureTemplate(value)
    }
}
impl From<crate::PackagingProcedureTemplate> for ProcedureTemplateDAG {
    fn from(value: crate::PackagingProcedureTemplate) -> Self {
        ProcedureTemplateDAG::PackagingProcedureTemplate(value)
    }
}
impl From<crate::PhotographProcedureTemplate> for ProcedureTemplateDAG {
    fn from(value: crate::PhotographProcedureTemplate) -> Self {
        ProcedureTemplateDAG::PhotographProcedureTemplate(value)
    }
}
impl From<crate::PouringProcedureTemplate> for ProcedureTemplateDAG {
    fn from(value: crate::PouringProcedureTemplate) -> Self {
        ProcedureTemplateDAG::PouringProcedureTemplate(value)
    }
}
impl From<crate::ProcedureTemplate> for ProcedureTemplateDAG {
    fn from(value: crate::ProcedureTemplate) -> Self {
        ProcedureTemplateDAG::ProcedureTemplate(value)
    }
}
impl From<crate::StorageProcedureTemplate> for ProcedureTemplateDAG {
    fn from(value: crate::StorageProcedureTemplate) -> Self {
        ProcedureTemplateDAG::StorageProcedureTemplate(value)
    }
}
impl From<crate::SupernatantProcedureTemplate> for ProcedureTemplateDAG {
    fn from(value: crate::SupernatantProcedureTemplate) -> Self {
        ProcedureTemplateDAG::SupernatantProcedureTemplate(value)
    }
}
impl From<crate::WeighingProcedureTemplate> for ProcedureTemplateDAG {
    fn from(value: crate::WeighingProcedureTemplate) -> Self {
        ProcedureTemplateDAG::WeighingProcedureTemplate(value)
    }
}
impl<C> web_common_traits::database::MostConcreteVariant<C> for crate::ProcedureTemplate
where
    crate::AliquotingProcedureTemplate: web_common_traits::database::Read<C>,
    crate::BallMillProcedureTemplate: web_common_traits::database::Read<C>,
    crate::CappingProcedureTemplate: web_common_traits::database::Read<C>,
    crate::CentrifugeProcedureTemplate: web_common_traits::database::Read<C>,
    crate::DisposalProcedureTemplate: web_common_traits::database::Read<C>,
    crate::FractioningProcedureTemplate: web_common_traits::database::Read<C>,
    crate::FreezeDryingProcedureTemplate: web_common_traits::database::Read<C>,
    crate::FreezingProcedureTemplate: web_common_traits::database::Read<C>,
    crate::GeolocationProcedureTemplate: web_common_traits::database::Read<C>,
    crate::PackagingProcedureTemplate: web_common_traits::database::Read<C>,
    crate::PhotographProcedureTemplate: web_common_traits::database::Read<C>,
    crate::PouringProcedureTemplate: web_common_traits::database::Read<C>,
    crate::StorageProcedureTemplate: web_common_traits::database::Read<C>,
    crate::SupernatantProcedureTemplate: web_common_traits::database::Read<C>,
    crate::WeighingProcedureTemplate: web_common_traits::database::Read<C>,
{
    type Variant = ProcedureTemplateDAG;
    fn most_concrete_variant(&self, conn: &mut C) -> Result<Self::Variant, diesel::result::Error> {
        use diesel::Identifiable;
        Ok(
            match self.most_concrete_table.as_str() {
                "aliquoting_procedure_templates" => {
                    <crate::AliquotingProcedureTemplate as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "ball_mill_procedure_templates" => {
                    <crate::BallMillProcedureTemplate as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "capping_procedure_templates" => {
                    <crate::CappingProcedureTemplate as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "centrifuge_procedure_templates" => {
                    <crate::CentrifugeProcedureTemplate as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "disposal_procedure_templates" => {
                    <crate::DisposalProcedureTemplate as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "fractioning_procedure_templates" => {
                    <crate::FractioningProcedureTemplate as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "freeze_drying_procedure_templates" => {
                    <crate::FreezeDryingProcedureTemplate as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "freezing_procedure_templates" => {
                    <crate::FreezingProcedureTemplate as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "geolocation_procedure_templates" => {
                    <crate::GeolocationProcedureTemplate as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "packaging_procedure_templates" => {
                    <crate::PackagingProcedureTemplate as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "photograph_procedure_templates" => {
                    <crate::PhotographProcedureTemplate as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "pouring_procedure_templates" => {
                    <crate::PouringProcedureTemplate as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "procedure_templates" => self.clone().into(),
                "storage_procedure_templates" => {
                    <crate::StorageProcedureTemplate as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "supernatant_procedure_templates" => {
                    <crate::SupernatantProcedureTemplate as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "weighing_procedure_templates" => {
                    <crate::WeighingProcedureTemplate as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                _ => unreachable!("Database and codegen are out of sync."),
            },
        )
    }
}

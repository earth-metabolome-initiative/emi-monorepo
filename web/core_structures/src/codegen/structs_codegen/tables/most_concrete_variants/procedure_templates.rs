mod builder;
pub use builder::ProcedureTemplateBuilderDAG;
#[derive(Debug, Clone, PartialEq)]
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
/// v13 --->|"`extends`"| v12
/// v14 --->|"`extends`"| v12
/// v8 --->|"`extends`"| v12
/// v11 --->|"`extends`"| v12
/// v0 --->|"`extends`"| v12
/// v10 --->|"`extends`"| v12
/// v4 --->|"`extends`"| v12
/// v2 --->|"`extends`"| v12
/// v6 --->|"`extends`"| v12
/// v3 --->|"`extends`"| v12
/// v15 --->|"`extends`"| v12
/// v9 --->|"`extends`"| v12
/// v5 --->|"`extends`"| v12
/// v7 --->|"`extends`"| v12
/// v1 --->|"`extends`"| v12
/// ```
pub enum ProcedureTemplateDAG {
    ///Variant representing the `aliquoting_procedure_templates` table.
    AliquotingProcedureTemplate(
        crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate,
    ),
    ///Variant representing the `ball_mill_procedure_templates` table.
    BallMillProcedureTemplate(
        crate::codegen::structs_codegen::tables::ball_mill_procedure_templates::BallMillProcedureTemplate,
    ),
    ///Variant representing the `capping_procedure_templates` table.
    CappingProcedureTemplate(
        crate::codegen::structs_codegen::tables::capping_procedure_templates::CappingProcedureTemplate,
    ),
    ///Variant representing the `centrifuge_procedure_templates` table.
    CentrifugeProcedureTemplate(
        crate::codegen::structs_codegen::tables::centrifuge_procedure_templates::CentrifugeProcedureTemplate,
    ),
    ///Variant representing the `disposal_procedure_templates` table.
    DisposalProcedureTemplate(
        crate::codegen::structs_codegen::tables::disposal_procedure_templates::DisposalProcedureTemplate,
    ),
    ///Variant representing the `fractioning_procedure_templates` table.
    FractioningProcedureTemplate(
        crate::codegen::structs_codegen::tables::fractioning_procedure_templates::FractioningProcedureTemplate,
    ),
    ///Variant representing the `freeze_drying_procedure_templates` table.
    FreezeDryingProcedureTemplate(
        crate::codegen::structs_codegen::tables::freeze_drying_procedure_templates::FreezeDryingProcedureTemplate,
    ),
    ///Variant representing the `freezing_procedure_templates` table.
    FreezingProcedureTemplate(
        crate::codegen::structs_codegen::tables::freezing_procedure_templates::FreezingProcedureTemplate,
    ),
    ///Variant representing the `geolocation_procedure_templates` table.
    GeolocationProcedureTemplate(
        crate::codegen::structs_codegen::tables::geolocation_procedure_templates::GeolocationProcedureTemplate,
    ),
    ///Variant representing the `packaging_procedure_templates` table.
    PackagingProcedureTemplate(
        crate::codegen::structs_codegen::tables::packaging_procedure_templates::PackagingProcedureTemplate,
    ),
    ///Variant representing the `photograph_procedure_templates` table.
    PhotographProcedureTemplate(
        crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate,
    ),
    ///Variant representing the `pouring_procedure_templates` table.
    PouringProcedureTemplate(
        crate::codegen::structs_codegen::tables::pouring_procedure_templates::PouringProcedureTemplate,
    ),
    ///Variant representing the `procedure_templates` table.
    ProcedureTemplate(
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
    ),
    ///Variant representing the `storage_procedure_templates` table.
    StorageProcedureTemplate(
        crate::codegen::structs_codegen::tables::storage_procedure_templates::StorageProcedureTemplate,
    ),
    ///Variant representing the `supernatant_procedure_templates` table.
    SupernatantProcedureTemplate(
        crate::codegen::structs_codegen::tables::supernatant_procedure_templates::SupernatantProcedureTemplate,
    ),
    ///Variant representing the `weighing_procedure_templates` table.
    WeighingProcedureTemplate(
        crate::codegen::structs_codegen::tables::weighing_procedure_templates::WeighingProcedureTemplate,
    ),
}
impl From<
    crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate,
> for ProcedureTemplateDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate,
    ) -> Self {
        ProcedureTemplateDAG::AliquotingProcedureTemplate(value)
    }
}
impl From<
    crate::codegen::structs_codegen::tables::ball_mill_procedure_templates::BallMillProcedureTemplate,
> for ProcedureTemplateDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::ball_mill_procedure_templates::BallMillProcedureTemplate,
    ) -> Self {
        ProcedureTemplateDAG::BallMillProcedureTemplate(value)
    }
}
impl From<
    crate::codegen::structs_codegen::tables::capping_procedure_templates::CappingProcedureTemplate,
> for ProcedureTemplateDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::capping_procedure_templates::CappingProcedureTemplate,
    ) -> Self {
        ProcedureTemplateDAG::CappingProcedureTemplate(value)
    }
}
impl From<
    crate::codegen::structs_codegen::tables::centrifuge_procedure_templates::CentrifugeProcedureTemplate,
> for ProcedureTemplateDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::centrifuge_procedure_templates::CentrifugeProcedureTemplate,
    ) -> Self {
        ProcedureTemplateDAG::CentrifugeProcedureTemplate(value)
    }
}
impl From<
    crate::codegen::structs_codegen::tables::disposal_procedure_templates::DisposalProcedureTemplate,
> for ProcedureTemplateDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::disposal_procedure_templates::DisposalProcedureTemplate,
    ) -> Self {
        ProcedureTemplateDAG::DisposalProcedureTemplate(value)
    }
}
impl From<
    crate::codegen::structs_codegen::tables::fractioning_procedure_templates::FractioningProcedureTemplate,
> for ProcedureTemplateDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::fractioning_procedure_templates::FractioningProcedureTemplate,
    ) -> Self {
        ProcedureTemplateDAG::FractioningProcedureTemplate(value)
    }
}
impl From<
    crate::codegen::structs_codegen::tables::freeze_drying_procedure_templates::FreezeDryingProcedureTemplate,
> for ProcedureTemplateDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::freeze_drying_procedure_templates::FreezeDryingProcedureTemplate,
    ) -> Self {
        ProcedureTemplateDAG::FreezeDryingProcedureTemplate(value)
    }
}
impl From<
    crate::codegen::structs_codegen::tables::freezing_procedure_templates::FreezingProcedureTemplate,
> for ProcedureTemplateDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::freezing_procedure_templates::FreezingProcedureTemplate,
    ) -> Self {
        ProcedureTemplateDAG::FreezingProcedureTemplate(value)
    }
}
impl From<
    crate::codegen::structs_codegen::tables::geolocation_procedure_templates::GeolocationProcedureTemplate,
> for ProcedureTemplateDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::geolocation_procedure_templates::GeolocationProcedureTemplate,
    ) -> Self {
        ProcedureTemplateDAG::GeolocationProcedureTemplate(value)
    }
}
impl From<
    crate::codegen::structs_codegen::tables::packaging_procedure_templates::PackagingProcedureTemplate,
> for ProcedureTemplateDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::packaging_procedure_templates::PackagingProcedureTemplate,
    ) -> Self {
        ProcedureTemplateDAG::PackagingProcedureTemplate(value)
    }
}
impl From<
    crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate,
> for ProcedureTemplateDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate,
    ) -> Self {
        ProcedureTemplateDAG::PhotographProcedureTemplate(value)
    }
}
impl From<
    crate::codegen::structs_codegen::tables::pouring_procedure_templates::PouringProcedureTemplate,
> for ProcedureTemplateDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::pouring_procedure_templates::PouringProcedureTemplate,
    ) -> Self {
        ProcedureTemplateDAG::PouringProcedureTemplate(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate>
    for ProcedureTemplateDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
    ) -> Self {
        ProcedureTemplateDAG::ProcedureTemplate(value)
    }
}
impl From<
    crate::codegen::structs_codegen::tables::storage_procedure_templates::StorageProcedureTemplate,
> for ProcedureTemplateDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::storage_procedure_templates::StorageProcedureTemplate,
    ) -> Self {
        ProcedureTemplateDAG::StorageProcedureTemplate(value)
    }
}
impl From<
    crate::codegen::structs_codegen::tables::supernatant_procedure_templates::SupernatantProcedureTemplate,
> for ProcedureTemplateDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::supernatant_procedure_templates::SupernatantProcedureTemplate,
    ) -> Self {
        ProcedureTemplateDAG::SupernatantProcedureTemplate(value)
    }
}
impl From<
    crate::codegen::structs_codegen::tables::weighing_procedure_templates::WeighingProcedureTemplate,
> for ProcedureTemplateDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::weighing_procedure_templates::WeighingProcedureTemplate,
    ) -> Self {
        ProcedureTemplateDAG::WeighingProcedureTemplate(value)
    }
}
impl<C> web_common_traits::database::MostConcreteVariant<C>
for crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate
where
    crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::ball_mill_procedure_templates::BallMillProcedureTemplate: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::capping_procedure_templates::CappingProcedureTemplate: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::centrifuge_procedure_templates::CentrifugeProcedureTemplate: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::disposal_procedure_templates::DisposalProcedureTemplate: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::fractioning_procedure_templates::FractioningProcedureTemplate: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::freeze_drying_procedure_templates::FreezeDryingProcedureTemplate: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::freezing_procedure_templates::FreezingProcedureTemplate: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::geolocation_procedure_templates::GeolocationProcedureTemplate: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::packaging_procedure_templates::PackagingProcedureTemplate: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::pouring_procedure_templates::PouringProcedureTemplate: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::storage_procedure_templates::StorageProcedureTemplate: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::supernatant_procedure_templates::SupernatantProcedureTemplate: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::weighing_procedure_templates::WeighingProcedureTemplate: web_common_traits::database::Read<
        C,
    >,
{
    type Variant = ProcedureTemplateDAG;
    fn most_concrete_variant(
        &self,
        conn: &mut C,
    ) -> Result<Self::Variant, diesel::result::Error> {
        use diesel::Identifiable;
        Ok(
            match self.most_concrete_table.as_str() {
                "aliquoting_procedure_templates" => {
                    <crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "ball_mill_procedure_templates" => {
                    <crate::codegen::structs_codegen::tables::ball_mill_procedure_templates::BallMillProcedureTemplate as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "capping_procedure_templates" => {
                    <crate::codegen::structs_codegen::tables::capping_procedure_templates::CappingProcedureTemplate as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "centrifuge_procedure_templates" => {
                    <crate::codegen::structs_codegen::tables::centrifuge_procedure_templates::CentrifugeProcedureTemplate as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "disposal_procedure_templates" => {
                    <crate::codegen::structs_codegen::tables::disposal_procedure_templates::DisposalProcedureTemplate as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "fractioning_procedure_templates" => {
                    <crate::codegen::structs_codegen::tables::fractioning_procedure_templates::FractioningProcedureTemplate as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "freeze_drying_procedure_templates" => {
                    <crate::codegen::structs_codegen::tables::freeze_drying_procedure_templates::FreezeDryingProcedureTemplate as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "freezing_procedure_templates" => {
                    <crate::codegen::structs_codegen::tables::freezing_procedure_templates::FreezingProcedureTemplate as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "geolocation_procedure_templates" => {
                    <crate::codegen::structs_codegen::tables::geolocation_procedure_templates::GeolocationProcedureTemplate as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "packaging_procedure_templates" => {
                    <crate::codegen::structs_codegen::tables::packaging_procedure_templates::PackagingProcedureTemplate as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "photograph_procedure_templates" => {
                    <crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "pouring_procedure_templates" => {
                    <crate::codegen::structs_codegen::tables::pouring_procedure_templates::PouringProcedureTemplate as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "procedure_templates" => self.clone().into(),
                "storage_procedure_templates" => {
                    <crate::codegen::structs_codegen::tables::storage_procedure_templates::StorageProcedureTemplate as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "supernatant_procedure_templates" => {
                    <crate::codegen::structs_codegen::tables::supernatant_procedure_templates::SupernatantProcedureTemplate as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "weighing_procedure_templates" => {
                    <crate::codegen::structs_codegen::tables::weighing_procedure_templates::WeighingProcedureTemplate as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                _ => unreachable!("Database and codegen are out of sync."),
            },
        )
    }
}

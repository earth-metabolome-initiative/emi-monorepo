mod builder;
pub use builder::ProcedureBuilderDAG;
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Enum representing the most concrete variant of the `procedures` table DAG.
///
/// # Mermaid illustration of the DAG:
/// ```mermaid
/// flowchart BT
/// v0@{shape: rect, label: "aliquoting_procedures"}
/// v1@{shape: rect, label: "ball_mill_procedures"}
/// v2@{shape: rect, label: "capping_procedures"}
/// v3@{shape: rect, label: "centrifuge_procedures"}
/// v4@{shape: rect, label: "disposal_procedures"}
/// v5@{shape: rect, label: "fractioning_procedures"}
/// v6@{shape: rect, label: "freeze_drying_procedures"}
/// v7@{shape: rect, label: "freezing_procedures"}
/// v8@{shape: rect, label: "geolocation_procedures"}
/// v9@{shape: rect, label: "packaging_procedures"}
/// v10@{shape: rect, label: "photograph_procedures"}
/// v11@{shape: rect, label: "pouring_procedures"}
/// v12@{shape: rect, label: "procedures"}
/// v13@{shape: rect, label: "storage_procedures"}
/// v14@{shape: rect, label: "supernatant_procedures"}
/// v15@{shape: rect, label: "weighing_procedures"}
/// v8 --->|"`extends`"| v12
/// v6 --->|"`extends`"| v12
/// v7 --->|"`extends`"| v12
/// v14 --->|"`extends`"| v12
/// v10 --->|"`extends`"| v12
/// v15 --->|"`extends`"| v12
/// v5 --->|"`extends`"| v12
/// v0 --->|"`extends`"| v12
/// v1 --->|"`extends`"| v12
/// v9 --->|"`extends`"| v12
/// v11 --->|"`extends`"| v12
/// v3 --->|"`extends`"| v12
/// v13 --->|"`extends`"| v12
/// v4 --->|"`extends`"| v12
/// v2 --->|"`extends`"| v12
/// ```
pub enum ProcedureDAG {
    /// Variant representing the `aliquoting_procedures` table.
    AliquotingProcedure(crate::AliquotingProcedure),
    /// Variant representing the `ball_mill_procedures` table.
    BallMillProcedure(crate::BallMillProcedure),
    /// Variant representing the `capping_procedures` table.
    CappingProcedure(crate::CappingProcedure),
    /// Variant representing the `centrifuge_procedures` table.
    CentrifugeProcedure(crate::CentrifugeProcedure),
    /// Variant representing the `disposal_procedures` table.
    DisposalProcedure(crate::DisposalProcedure),
    /// Variant representing the `fractioning_procedures` table.
    FractioningProcedure(crate::FractioningProcedure),
    /// Variant representing the `freeze_drying_procedures` table.
    FreezeDryingProcedure(crate::FreezeDryingProcedure),
    /// Variant representing the `freezing_procedures` table.
    FreezingProcedure(crate::FreezingProcedure),
    /// Variant representing the `geolocation_procedures` table.
    GeolocationProcedure(crate::GeolocationProcedure),
    /// Variant representing the `packaging_procedures` table.
    PackagingProcedure(crate::PackagingProcedure),
    /// Variant representing the `photograph_procedures` table.
    PhotographProcedure(crate::PhotographProcedure),
    /// Variant representing the `pouring_procedures` table.
    PouringProcedure(crate::PouringProcedure),
    /// Variant representing the `procedures` table.
    Procedure(crate::Procedure),
    /// Variant representing the `storage_procedures` table.
    StorageProcedure(crate::StorageProcedure),
    /// Variant representing the `supernatant_procedures` table.
    SupernatantProcedure(crate::SupernatantProcedure),
    /// Variant representing the `weighing_procedures` table.
    WeighingProcedure(crate::WeighingProcedure),
}
impl From<crate::AliquotingProcedure> for ProcedureDAG {
    fn from(value: crate::AliquotingProcedure) -> Self {
        ProcedureDAG::AliquotingProcedure(value)
    }
}
impl From<crate::BallMillProcedure> for ProcedureDAG {
    fn from(value: crate::BallMillProcedure) -> Self {
        ProcedureDAG::BallMillProcedure(value)
    }
}
impl From<crate::CappingProcedure> for ProcedureDAG {
    fn from(value: crate::CappingProcedure) -> Self {
        ProcedureDAG::CappingProcedure(value)
    }
}
impl From<crate::CentrifugeProcedure> for ProcedureDAG {
    fn from(value: crate::CentrifugeProcedure) -> Self {
        ProcedureDAG::CentrifugeProcedure(value)
    }
}
impl From<crate::DisposalProcedure> for ProcedureDAG {
    fn from(value: crate::DisposalProcedure) -> Self {
        ProcedureDAG::DisposalProcedure(value)
    }
}
impl From<crate::FractioningProcedure> for ProcedureDAG {
    fn from(value: crate::FractioningProcedure) -> Self {
        ProcedureDAG::FractioningProcedure(value)
    }
}
impl From<crate::FreezeDryingProcedure> for ProcedureDAG {
    fn from(value: crate::FreezeDryingProcedure) -> Self {
        ProcedureDAG::FreezeDryingProcedure(value)
    }
}
impl From<crate::FreezingProcedure> for ProcedureDAG {
    fn from(value: crate::FreezingProcedure) -> Self {
        ProcedureDAG::FreezingProcedure(value)
    }
}
impl From<crate::GeolocationProcedure> for ProcedureDAG {
    fn from(value: crate::GeolocationProcedure) -> Self {
        ProcedureDAG::GeolocationProcedure(value)
    }
}
impl From<crate::PackagingProcedure> for ProcedureDAG {
    fn from(value: crate::PackagingProcedure) -> Self {
        ProcedureDAG::PackagingProcedure(value)
    }
}
impl From<crate::PhotographProcedure> for ProcedureDAG {
    fn from(value: crate::PhotographProcedure) -> Self {
        ProcedureDAG::PhotographProcedure(value)
    }
}
impl From<crate::PouringProcedure> for ProcedureDAG {
    fn from(value: crate::PouringProcedure) -> Self {
        ProcedureDAG::PouringProcedure(value)
    }
}
impl From<crate::Procedure> for ProcedureDAG {
    fn from(value: crate::Procedure) -> Self {
        ProcedureDAG::Procedure(value)
    }
}
impl From<crate::StorageProcedure> for ProcedureDAG {
    fn from(value: crate::StorageProcedure) -> Self {
        ProcedureDAG::StorageProcedure(value)
    }
}
impl From<crate::SupernatantProcedure> for ProcedureDAG {
    fn from(value: crate::SupernatantProcedure) -> Self {
        ProcedureDAG::SupernatantProcedure(value)
    }
}
impl From<crate::WeighingProcedure> for ProcedureDAG {
    fn from(value: crate::WeighingProcedure) -> Self {
        ProcedureDAG::WeighingProcedure(value)
    }
}
impl<C> web_common_traits::database::MostConcreteVariant<C> for crate::Procedure
where
    crate::AliquotingProcedure: web_common_traits::database::Read<C>,
    crate::BallMillProcedure: web_common_traits::database::Read<C>,
    crate::CappingProcedure: web_common_traits::database::Read<C>,
    crate::CentrifugeProcedure: web_common_traits::database::Read<C>,
    crate::DisposalProcedure: web_common_traits::database::Read<C>,
    crate::FractioningProcedure: web_common_traits::database::Read<C>,
    crate::FreezeDryingProcedure: web_common_traits::database::Read<C>,
    crate::FreezingProcedure: web_common_traits::database::Read<C>,
    crate::GeolocationProcedure: web_common_traits::database::Read<C>,
    crate::PackagingProcedure: web_common_traits::database::Read<C>,
    crate::PhotographProcedure: web_common_traits::database::Read<C>,
    crate::PouringProcedure: web_common_traits::database::Read<C>,
    crate::StorageProcedure: web_common_traits::database::Read<C>,
    crate::SupernatantProcedure: web_common_traits::database::Read<C>,
    crate::WeighingProcedure: web_common_traits::database::Read<C>,
{
    type Variant = ProcedureDAG;
    fn most_concrete_variant(&self, conn: &mut C) -> Result<Self::Variant, diesel::result::Error> {
        use diesel::Identifiable;
        Ok(match self.most_concrete_table.as_str() {
            "aliquoting_procedures" => {
                <crate::AliquotingProcedure as web_common_traits::database::Read<C>>::read(
                    *self.id(),
                    conn,
                )?
                .into()
            }
            "ball_mill_procedures" => {
                <crate::BallMillProcedure as web_common_traits::database::Read<C>>::read(
                    *self.id(),
                    conn,
                )?
                .into()
            }
            "capping_procedures" => {
                <crate::CappingProcedure as web_common_traits::database::Read<C>>::read(
                    *self.id(),
                    conn,
                )?
                .into()
            }
            "centrifuge_procedures" => {
                <crate::CentrifugeProcedure as web_common_traits::database::Read<C>>::read(
                    *self.id(),
                    conn,
                )?
                .into()
            }
            "disposal_procedures" => {
                <crate::DisposalProcedure as web_common_traits::database::Read<C>>::read(
                    *self.id(),
                    conn,
                )?
                .into()
            }
            "fractioning_procedures" => {
                <crate::FractioningProcedure as web_common_traits::database::Read<C>>::read(
                    *self.id(),
                    conn,
                )?
                .into()
            }
            "freeze_drying_procedures" => {
                <crate::FreezeDryingProcedure as web_common_traits::database::Read<C>>::read(
                    *self.id(),
                    conn,
                )?
                .into()
            }
            "freezing_procedures" => {
                <crate::FreezingProcedure as web_common_traits::database::Read<C>>::read(
                    *self.id(),
                    conn,
                )?
                .into()
            }
            "geolocation_procedures" => {
                <crate::GeolocationProcedure as web_common_traits::database::Read<C>>::read(
                    *self.id(),
                    conn,
                )?
                .into()
            }
            "packaging_procedures" => {
                <crate::PackagingProcedure as web_common_traits::database::Read<C>>::read(
                    *self.id(),
                    conn,
                )?
                .into()
            }
            "photograph_procedures" => {
                <crate::PhotographProcedure as web_common_traits::database::Read<C>>::read(
                    *self.id(),
                    conn,
                )?
                .into()
            }
            "pouring_procedures" => {
                <crate::PouringProcedure as web_common_traits::database::Read<C>>::read(
                    *self.id(),
                    conn,
                )?
                .into()
            }
            "procedures" => self.clone().into(),
            "storage_procedures" => {
                <crate::StorageProcedure as web_common_traits::database::Read<C>>::read(
                    *self.id(),
                    conn,
                )?
                .into()
            }
            "supernatant_procedures" => {
                <crate::SupernatantProcedure as web_common_traits::database::Read<C>>::read(
                    *self.id(),
                    conn,
                )?
                .into()
            }
            "weighing_procedures" => {
                <crate::WeighingProcedure as web_common_traits::database::Read<C>>::read(
                    *self.id(),
                    conn,
                )?
                .into()
            }
            _ => unreachable!("Database and codegen are out of sync."),
        })
    }
}

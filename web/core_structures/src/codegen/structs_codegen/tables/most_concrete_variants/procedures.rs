mod builder;
mod insert_error;
pub use builder::ProcedureBuilderDAG;
pub use insert_error::ProcedureInsertErrorDAG;
#[derive(Debug, Clone, PartialEq)]
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
/// v9@{shape: rect, label: "harvesting_procedures"}
/// v10@{shape: rect, label: "packaging_procedures"}
/// v11@{shape: rect, label: "photograph_procedures"}
/// v12@{shape: rect, label: "placing_procedures"}
/// v13@{shape: rect, label: "pouring_procedures"}
/// v14@{shape: rect, label: "procedures"}
/// v15@{shape: rect, label: "storage_procedures"}
/// v16@{shape: rect, label: "supernatant_procedures"}
/// v17@{shape: rect, label: "weighing_procedures"}
/// v0 --->|"`extends`"| v14
/// v1 --->|"`extends`"| v14
/// v2 --->|"`extends`"| v14
/// v3 --->|"`extends`"| v14
/// v4 --->|"`extends`"| v14
/// v5 --->|"`extends`"| v14
/// v6 --->|"`extends`"| v14
/// v7 --->|"`extends`"| v14
/// v8 --->|"`extends`"| v14
/// v9 --->|"`extends`"| v14
/// v10 --->|"`extends`"| v14
/// v11 --->|"`extends`"| v14
/// v12 --->|"`extends`"| v8
/// v13 --->|"`extends`"| v14
/// v15 --->|"`extends`"| v14
/// v16 --->|"`extends`"| v14
/// v17 --->|"`extends`"| v14
/// ```
pub enum ProcedureDAG {
    /// Variant representing the `aliquoting_procedures` table.
    AliquotingProcedure(
        crate::codegen::structs_codegen::tables::aliquoting_procedures::AliquotingProcedure,
    ),
    /// Variant representing the `ball_mill_procedures` table.
    BallMillProcedure(
        crate::codegen::structs_codegen::tables::ball_mill_procedures::BallMillProcedure,
    ),
    /// Variant representing the `capping_procedures` table.
    CappingProcedure(crate::codegen::structs_codegen::tables::capping_procedures::CappingProcedure),
    /// Variant representing the `centrifuge_procedures` table.
    CentrifugeProcedure(
        crate::codegen::structs_codegen::tables::centrifuge_procedures::CentrifugeProcedure,
    ),
    /// Variant representing the `disposal_procedures` table.
    DisposalProcedure(
        crate::codegen::structs_codegen::tables::disposal_procedures::DisposalProcedure,
    ),
    /// Variant representing the `fractioning_procedures` table.
    FractioningProcedure(
        crate::codegen::structs_codegen::tables::fractioning_procedures::FractioningProcedure,
    ),
    /// Variant representing the `freeze_drying_procedures` table.
    FreezeDryingProcedure(
        crate::codegen::structs_codegen::tables::freeze_drying_procedures::FreezeDryingProcedure,
    ),
    /// Variant representing the `freezing_procedures` table.
    FreezingProcedure(
        crate::codegen::structs_codegen::tables::freezing_procedures::FreezingProcedure,
    ),
    /// Variant representing the `geolocation_procedures` table.
    GeolocationProcedure(
        crate::codegen::structs_codegen::tables::geolocation_procedures::GeolocationProcedure,
    ),
    /// Variant representing the `harvesting_procedures` table.
    HarvestingProcedure(
        crate::codegen::structs_codegen::tables::harvesting_procedures::HarvestingProcedure,
    ),
    /// Variant representing the `packaging_procedures` table.
    PackagingProcedure(
        crate::codegen::structs_codegen::tables::packaging_procedures::PackagingProcedure,
    ),
    /// Variant representing the `photograph_procedures` table.
    PhotographProcedure(
        crate::codegen::structs_codegen::tables::photograph_procedures::PhotographProcedure,
    ),
    /// Variant representing the `placing_procedures` table.
    PlacingProcedure(crate::codegen::structs_codegen::tables::placing_procedures::PlacingProcedure),
    /// Variant representing the `pouring_procedures` table.
    PouringProcedure(crate::codegen::structs_codegen::tables::pouring_procedures::PouringProcedure),
    /// Variant representing the `procedures` table.
    Procedure(crate::codegen::structs_codegen::tables::procedures::Procedure),
    /// Variant representing the `storage_procedures` table.
    StorageProcedure(crate::codegen::structs_codegen::tables::storage_procedures::StorageProcedure),
    /// Variant representing the `supernatant_procedures` table.
    SupernatantProcedure(
        crate::codegen::structs_codegen::tables::supernatant_procedures::SupernatantProcedure,
    ),
    /// Variant representing the `weighing_procedures` table.
    WeighingProcedure(
        crate::codegen::structs_codegen::tables::weighing_procedures::WeighingProcedure,
    ),
}
impl From<crate::codegen::structs_codegen::tables::aliquoting_procedures::AliquotingProcedure>
    for ProcedureDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::aliquoting_procedures::AliquotingProcedure,
    ) -> Self {
        ProcedureDAG::AliquotingProcedure(value)
    }
}
impl From<ProcedureDAG>
    for Option<crate::codegen::structs_codegen::tables::aliquoting_procedures::AliquotingProcedure>
{
    fn from(value: ProcedureDAG) -> Self {
        match value {
            ProcedureDAG::AliquotingProcedure(v) => Some(v),
            _ => None,
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::ball_mill_procedures::BallMillProcedure>
    for ProcedureDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::ball_mill_procedures::BallMillProcedure,
    ) -> Self {
        ProcedureDAG::BallMillProcedure(value)
    }
}
impl From<ProcedureDAG>
    for Option<crate::codegen::structs_codegen::tables::ball_mill_procedures::BallMillProcedure>
{
    fn from(value: ProcedureDAG) -> Self {
        match value {
            ProcedureDAG::BallMillProcedure(v) => Some(v),
            _ => None,
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::capping_procedures::CappingProcedure>
    for ProcedureDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::capping_procedures::CappingProcedure,
    ) -> Self {
        ProcedureDAG::CappingProcedure(value)
    }
}
impl From<ProcedureDAG>
    for Option<crate::codegen::structs_codegen::tables::capping_procedures::CappingProcedure>
{
    fn from(value: ProcedureDAG) -> Self {
        match value {
            ProcedureDAG::CappingProcedure(v) => Some(v),
            _ => None,
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::centrifuge_procedures::CentrifugeProcedure>
    for ProcedureDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::centrifuge_procedures::CentrifugeProcedure,
    ) -> Self {
        ProcedureDAG::CentrifugeProcedure(value)
    }
}
impl From<ProcedureDAG>
    for Option<crate::codegen::structs_codegen::tables::centrifuge_procedures::CentrifugeProcedure>
{
    fn from(value: ProcedureDAG) -> Self {
        match value {
            ProcedureDAG::CentrifugeProcedure(v) => Some(v),
            _ => None,
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::disposal_procedures::DisposalProcedure>
    for ProcedureDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::disposal_procedures::DisposalProcedure,
    ) -> Self {
        ProcedureDAG::DisposalProcedure(value)
    }
}
impl From<ProcedureDAG>
    for Option<crate::codegen::structs_codegen::tables::disposal_procedures::DisposalProcedure>
{
    fn from(value: ProcedureDAG) -> Self {
        match value {
            ProcedureDAG::DisposalProcedure(v) => Some(v),
            _ => None,
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::fractioning_procedures::FractioningProcedure>
    for ProcedureDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::fractioning_procedures::FractioningProcedure,
    ) -> Self {
        ProcedureDAG::FractioningProcedure(value)
    }
}
impl From<ProcedureDAG>
    for Option<
        crate::codegen::structs_codegen::tables::fractioning_procedures::FractioningProcedure,
    >
{
    fn from(value: ProcedureDAG) -> Self {
        match value {
            ProcedureDAG::FractioningProcedure(v) => Some(v),
            _ => None,
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::freeze_drying_procedures::FreezeDryingProcedure>
    for ProcedureDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::freeze_drying_procedures::FreezeDryingProcedure,
    ) -> Self {
        ProcedureDAG::FreezeDryingProcedure(value)
    }
}
impl From<ProcedureDAG>
    for Option<
        crate::codegen::structs_codegen::tables::freeze_drying_procedures::FreezeDryingProcedure,
    >
{
    fn from(value: ProcedureDAG) -> Self {
        match value {
            ProcedureDAG::FreezeDryingProcedure(v) => Some(v),
            _ => None,
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::freezing_procedures::FreezingProcedure>
    for ProcedureDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::freezing_procedures::FreezingProcedure,
    ) -> Self {
        ProcedureDAG::FreezingProcedure(value)
    }
}
impl From<ProcedureDAG>
    for Option<crate::codegen::structs_codegen::tables::freezing_procedures::FreezingProcedure>
{
    fn from(value: ProcedureDAG) -> Self {
        match value {
            ProcedureDAG::FreezingProcedure(v) => Some(v),
            _ => None,
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::geolocation_procedures::GeolocationProcedure>
    for ProcedureDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::geolocation_procedures::GeolocationProcedure,
    ) -> Self {
        ProcedureDAG::GeolocationProcedure(value)
    }
}
impl From<ProcedureDAG>
    for Option<
        crate::codegen::structs_codegen::tables::geolocation_procedures::GeolocationProcedure,
    >
{
    fn from(value: ProcedureDAG) -> Self {
        match value {
            ProcedureDAG::GeolocationProcedure(v) => Some(v),
            _ => None,
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::harvesting_procedures::HarvestingProcedure>
    for ProcedureDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::harvesting_procedures::HarvestingProcedure,
    ) -> Self {
        ProcedureDAG::HarvestingProcedure(value)
    }
}
impl From<ProcedureDAG>
    for Option<crate::codegen::structs_codegen::tables::harvesting_procedures::HarvestingProcedure>
{
    fn from(value: ProcedureDAG) -> Self {
        match value {
            ProcedureDAG::HarvestingProcedure(v) => Some(v),
            _ => None,
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::packaging_procedures::PackagingProcedure>
    for ProcedureDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::packaging_procedures::PackagingProcedure,
    ) -> Self {
        ProcedureDAG::PackagingProcedure(value)
    }
}
impl From<ProcedureDAG>
    for Option<crate::codegen::structs_codegen::tables::packaging_procedures::PackagingProcedure>
{
    fn from(value: ProcedureDAG) -> Self {
        match value {
            ProcedureDAG::PackagingProcedure(v) => Some(v),
            _ => None,
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::photograph_procedures::PhotographProcedure>
    for ProcedureDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::photograph_procedures::PhotographProcedure,
    ) -> Self {
        ProcedureDAG::PhotographProcedure(value)
    }
}
impl From<ProcedureDAG>
    for Option<crate::codegen::structs_codegen::tables::photograph_procedures::PhotographProcedure>
{
    fn from(value: ProcedureDAG) -> Self {
        match value {
            ProcedureDAG::PhotographProcedure(v) => Some(v),
            _ => None,
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::placing_procedures::PlacingProcedure>
    for ProcedureDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::placing_procedures::PlacingProcedure,
    ) -> Self {
        ProcedureDAG::PlacingProcedure(value)
    }
}
impl From<ProcedureDAG>
    for Option<crate::codegen::structs_codegen::tables::placing_procedures::PlacingProcedure>
{
    fn from(value: ProcedureDAG) -> Self {
        match value {
            ProcedureDAG::PlacingProcedure(v) => Some(v),
            _ => None,
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::pouring_procedures::PouringProcedure>
    for ProcedureDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::pouring_procedures::PouringProcedure,
    ) -> Self {
        ProcedureDAG::PouringProcedure(value)
    }
}
impl From<ProcedureDAG>
    for Option<crate::codegen::structs_codegen::tables::pouring_procedures::PouringProcedure>
{
    fn from(value: ProcedureDAG) -> Self {
        match value {
            ProcedureDAG::PouringProcedure(v) => Some(v),
            _ => None,
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::procedures::Procedure> for ProcedureDAG {
    fn from(value: crate::codegen::structs_codegen::tables::procedures::Procedure) -> Self {
        ProcedureDAG::Procedure(value)
    }
}
impl From<ProcedureDAG> for Option<crate::codegen::structs_codegen::tables::procedures::Procedure> {
    fn from(value: ProcedureDAG) -> Self {
        match value {
            ProcedureDAG::Procedure(v) => Some(v),
            _ => None,
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::storage_procedures::StorageProcedure>
    for ProcedureDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::storage_procedures::StorageProcedure,
    ) -> Self {
        ProcedureDAG::StorageProcedure(value)
    }
}
impl From<ProcedureDAG>
    for Option<crate::codegen::structs_codegen::tables::storage_procedures::StorageProcedure>
{
    fn from(value: ProcedureDAG) -> Self {
        match value {
            ProcedureDAG::StorageProcedure(v) => Some(v),
            _ => None,
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::supernatant_procedures::SupernatantProcedure>
    for ProcedureDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::supernatant_procedures::SupernatantProcedure,
    ) -> Self {
        ProcedureDAG::SupernatantProcedure(value)
    }
}
impl From<ProcedureDAG>
    for Option<
        crate::codegen::structs_codegen::tables::supernatant_procedures::SupernatantProcedure,
    >
{
    fn from(value: ProcedureDAG) -> Self {
        match value {
            ProcedureDAG::SupernatantProcedure(v) => Some(v),
            _ => None,
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::weighing_procedures::WeighingProcedure>
    for ProcedureDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::weighing_procedures::WeighingProcedure,
    ) -> Self {
        ProcedureDAG::WeighingProcedure(value)
    }
}
impl From<ProcedureDAG>
    for Option<crate::codegen::structs_codegen::tables::weighing_procedures::WeighingProcedure>
{
    fn from(value: ProcedureDAG) -> Self {
        match value {
            ProcedureDAG::WeighingProcedure(v) => Some(v),
            _ => None,
        }
    }
}
impl web_common_traits::database::MostConcreteVariantMetadata
    for crate::codegen::structs_codegen::tables::procedures::Procedure
{
    type Variant = ProcedureDAG;
}
impl web_common_traits::database::PrimaryKeyLike for ProcedureDAG {
    type PrimaryKey = ::rosetta_uuid::Uuid;
    fn primary_key(&self) -> Self::PrimaryKey {
        match self {
            Self::AliquotingProcedure(variant) => variant.primary_key(),
            Self::BallMillProcedure(variant) => variant.primary_key(),
            Self::CappingProcedure(variant) => variant.primary_key(),
            Self::CentrifugeProcedure(variant) => variant.primary_key(),
            Self::DisposalProcedure(variant) => variant.primary_key(),
            Self::FractioningProcedure(variant) => variant.primary_key(),
            Self::FreezeDryingProcedure(variant) => variant.primary_key(),
            Self::FreezingProcedure(variant) => variant.primary_key(),
            Self::GeolocationProcedure(variant) => variant.primary_key(),
            Self::HarvestingProcedure(variant) => variant.primary_key(),
            Self::PackagingProcedure(variant) => variant.primary_key(),
            Self::PhotographProcedure(variant) => variant.primary_key(),
            Self::PlacingProcedure(variant) => variant.primary_key(),
            Self::PouringProcedure(variant) => variant.primary_key(),
            Self::Procedure(variant) => variant.primary_key(),
            Self::StorageProcedure(variant) => variant.primary_key(),
            Self::SupernatantProcedure(variant) => variant.primary_key(),
            Self::WeighingProcedure(variant) => variant.primary_key(),
        }
    }
}
impl<C> web_common_traits::database::MostConcreteVariant<C>
    for crate::codegen::structs_codegen::tables::procedures::Procedure
where
    crate::codegen::structs_codegen::tables::aliquoting_procedures::AliquotingProcedure:
        web_common_traits::database::Read<C>,
    crate::codegen::structs_codegen::tables::ball_mill_procedures::BallMillProcedure:
        web_common_traits::database::Read<C>,
    crate::codegen::structs_codegen::tables::capping_procedures::CappingProcedure:
        web_common_traits::database::Read<C>,
    crate::codegen::structs_codegen::tables::centrifuge_procedures::CentrifugeProcedure:
        web_common_traits::database::Read<C>,
    crate::codegen::structs_codegen::tables::disposal_procedures::DisposalProcedure:
        web_common_traits::database::Read<C>,
    crate::codegen::structs_codegen::tables::fractioning_procedures::FractioningProcedure:
        web_common_traits::database::Read<C>,
    crate::codegen::structs_codegen::tables::freeze_drying_procedures::FreezeDryingProcedure:
        web_common_traits::database::Read<C>,
    crate::codegen::structs_codegen::tables::freezing_procedures::FreezingProcedure:
        web_common_traits::database::Read<C>,
    crate::codegen::structs_codegen::tables::geolocation_procedures::GeolocationProcedure:
        web_common_traits::database::Read<C>,
    crate::codegen::structs_codegen::tables::harvesting_procedures::HarvestingProcedure:
        web_common_traits::database::Read<C>,
    crate::codegen::structs_codegen::tables::packaging_procedures::PackagingProcedure:
        web_common_traits::database::Read<C>,
    crate::codegen::structs_codegen::tables::photograph_procedures::PhotographProcedure:
        web_common_traits::database::Read<C>,
    crate::codegen::structs_codegen::tables::placing_procedures::PlacingProcedure:
        web_common_traits::database::Read<C>,
    crate::codegen::structs_codegen::tables::pouring_procedures::PouringProcedure:
        web_common_traits::database::Read<C>,
    crate::codegen::structs_codegen::tables::procedures::Procedure:
        web_common_traits::database::Read<C>,
    crate::codegen::structs_codegen::tables::storage_procedures::StorageProcedure:
        web_common_traits::database::Read<C>,
    crate::codegen::structs_codegen::tables::supernatant_procedures::SupernatantProcedure:
        web_common_traits::database::Read<C>,
    crate::codegen::structs_codegen::tables::weighing_procedures::WeighingProcedure:
        web_common_traits::database::Read<C>,
{
    fn most_concrete_variant(&self, conn: &mut C) -> Result<Self::Variant, diesel::result::Error> {
        use diesel::Identifiable;
        Ok(
            match self.most_concrete_table.as_str() {
                "aliquoting_procedures" => {
                    <crate::codegen::structs_codegen::tables::aliquoting_procedures::AliquotingProcedure as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "ball_mill_procedures" => {
                    <crate::codegen::structs_codegen::tables::ball_mill_procedures::BallMillProcedure as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "capping_procedures" => {
                    <crate::codegen::structs_codegen::tables::capping_procedures::CappingProcedure as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "centrifuge_procedures" => {
                    <crate::codegen::structs_codegen::tables::centrifuge_procedures::CentrifugeProcedure as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "disposal_procedures" => {
                    <crate::codegen::structs_codegen::tables::disposal_procedures::DisposalProcedure as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "fractioning_procedures" => {
                    <crate::codegen::structs_codegen::tables::fractioning_procedures::FractioningProcedure as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "freeze_drying_procedures" => {
                    <crate::codegen::structs_codegen::tables::freeze_drying_procedures::FreezeDryingProcedure as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "freezing_procedures" => {
                    <crate::codegen::structs_codegen::tables::freezing_procedures::FreezingProcedure as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "geolocation_procedures" => {
                    <crate::codegen::structs_codegen::tables::geolocation_procedures::GeolocationProcedure as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "harvesting_procedures" => {
                    <crate::codegen::structs_codegen::tables::harvesting_procedures::HarvestingProcedure as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "packaging_procedures" => {
                    <crate::codegen::structs_codegen::tables::packaging_procedures::PackagingProcedure as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "photograph_procedures" => {
                    <crate::codegen::structs_codegen::tables::photograph_procedures::PhotographProcedure as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "placing_procedures" => {
                    <crate::codegen::structs_codegen::tables::placing_procedures::PlacingProcedure as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "pouring_procedures" => {
                    <crate::codegen::structs_codegen::tables::pouring_procedures::PouringProcedure as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "procedures" => self.clone().into(),
                "storage_procedures" => {
                    <crate::codegen::structs_codegen::tables::storage_procedures::StorageProcedure as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "supernatant_procedures" => {
                    <crate::codegen::structs_codegen::tables::supernatant_procedures::SupernatantProcedure as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "weighing_procedures" => {
                    <crate::codegen::structs_codegen::tables::weighing_procedures::WeighingProcedure as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                _ => unreachable!("Database and codegen are out of sync."),
            },
        )
    }
}

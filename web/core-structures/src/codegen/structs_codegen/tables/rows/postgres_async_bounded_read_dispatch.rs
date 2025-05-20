#[cfg(feature = "postgres")]
impl web_common_traits::prelude::AsyncBoundedReadDispatch<diesel_async::AsyncPgConnection>
    for super::Rows
{
    type TableName = crate::codegen::tables::table_names::TableName;
    async fn bounded_read(
        table_name: Self::TableName,
        offset: u16,
        limit: u16,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<Self, diesel::result::Error> {
        match table_name {
            crate::codegen::tables::table_names::TableName::Address => {
                <crate::codegen::structs_codegen::tables::addresses::Address as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::AliquotingInstrumentModel => {
                <crate::codegen::structs_codegen::tables::aliquoting_instrument_models::AliquotingInstrumentModel as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::AliquotingStepModel => {
                <crate::codegen::structs_codegen::tables::aliquoting_step_models::AliquotingStepModel as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::AliquotingStep => {
                <crate::codegen::structs_codegen::tables::aliquoting_steps::AliquotingStep as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::BallMillStepModel => {
                <crate::codegen::structs_codegen::tables::ball_mill_step_models::BallMillStepModel as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::BallMillStep => {
                <crate::codegen::structs_codegen::tables::ball_mill_steps::BallMillStep as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Brand => {
                <crate::codegen::structs_codegen::tables::brands::Brand as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CentrifugeStepModel => {
                <crate::codegen::structs_codegen::tables::centrifuge_step_models::CentrifugeStepModel as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CentrifugeStep => {
                <crate::codegen::structs_codegen::tables::centrifuge_steps::CentrifugeStep as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::ChemicalEntity => {
                <crate::codegen::structs_codegen::tables::chemical_entities::ChemicalEntity as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::City => {
                <crate::codegen::structs_codegen::tables::cities::City as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Color => {
                <crate::codegen::structs_codegen::tables::colors::Color as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialProductLot => {
                <crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialProduct => {
                <crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialReagentModel => {
                <crate::codegen::structs_codegen::tables::commercial_reagent_models::CommercialReagentModel as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialReagent => {
                <crate::codegen::structs_codegen::tables::commercial_reagents::CommercialReagent as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::ContainerModel => {
                <crate::codegen::structs_codegen::tables::container_models::ContainerModel as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Country => {
                <crate::codegen::structs_codegen::tables::countries::Country as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::DisposalStepModel => {
                <crate::codegen::structs_codegen::tables::disposal_step_models::DisposalStepModel as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::DisposalStep => {
                <crate::codegen::structs_codegen::tables::disposal_steps::DisposalStep as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::DocumentFormat => {
                <crate::codegen::structs_codegen::tables::document_formats::DocumentFormat as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::EmailProvider => {
                <crate::codegen::structs_codegen::tables::email_providers::EmailProvider as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::FractioningStepModel => {
                <crate::codegen::structs_codegen::tables::fractioning_step_models::FractioningStepModel as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::FractioningStep => {
                <crate::codegen::structs_codegen::tables::fractioning_steps::FractioningStep as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::FreezeDryingStepModel => {
                <crate::codegen::structs_codegen::tables::freeze_drying_step_models::FreezeDryingStepModel as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::InstrumentLocation => {
                <crate::codegen::structs_codegen::tables::instrument_locations::InstrumentLocation as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::InstrumentModelCategory => {
                <crate::codegen::structs_codegen::tables::instrument_model_categories::InstrumentModelCategory as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::InstrumentModel => {
                <crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::InstrumentState => {
                <crate::codegen::structs_codegen::tables::instrument_states::InstrumentState as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Instrument => {
                <crate::codegen::structs_codegen::tables::instruments::Instrument as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::LoginProvider => {
                <crate::codegen::structs_codegen::tables::login_providers::LoginProvider as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Material => {
                <crate::codegen::structs_codegen::tables::materials::Material as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::NameplateModel => {
                <crate::codegen::structs_codegen::tables::nameplate_models::NameplateModel as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::ObservationSubject => {
                <crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::OrganismObservation => {
                <crate::codegen::structs_codegen::tables::organism_observations::OrganismObservation as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::OrganismSamplingStepModel => {
                <crate::codegen::structs_codegen::tables::organism_sampling_step_models::OrganismSamplingStepModel as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::OrganismTaxon => {
                <crate::codegen::structs_codegen::tables::organism_taxa::OrganismTaxon as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Organism => {
                <crate::codegen::structs_codegen::tables::organisms::Organism as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Organization => {
                <crate::codegen::structs_codegen::tables::organizations::Organization as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::PackagingModel => {
                <crate::codegen::structs_codegen::tables::packaging_models::PackagingModel as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::PackagingStepModel => {
                <crate::codegen::structs_codegen::tables::packaging_step_models::PackagingStepModel as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::PermanenceCategory => {
                <crate::codegen::structs_codegen::tables::permanence_categories::PermanenceCategory as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Photograph => {
                <crate::codegen::structs_codegen::tables::photographs::Photograph as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::ProcedureModelContainerCategory => {
                <crate::codegen::structs_codegen::tables::procedure_model_container_categories::ProcedureModelContainerCategory as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::ProcedureModelInstrumentCategory => {
                <crate::codegen::structs_codegen::tables::procedure_model_instrument_categories::ProcedureModelInstrumentCategory as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::ProcedureModelNameplateCategory => {
                <crate::codegen::structs_codegen::tables::procedure_model_nameplate_categories::ProcedureModelNameplateCategory as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::ProcedureModelReagent => {
                <crate::codegen::structs_codegen::tables::procedure_model_reagents::ProcedureModelReagent as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::ProcedureModelToolCategory => {
                <crate::codegen::structs_codegen::tables::procedure_model_tool_categories::ProcedureModelToolCategory as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::ProcedureModel => {
                <crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::ProcedureStepModel => {
                <crate::codegen::structs_codegen::tables::procedure_step_models::ProcedureStepModel as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Procedure => {
                <crate::codegen::structs_codegen::tables::procedures::Procedure as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Processable => {
                <crate::codegen::structs_codegen::tables::processables::Processable as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::ProcessingStep => {
                <crate::codegen::structs_codegen::tables::processing_steps::ProcessingStep as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::ProjectState => {
                <crate::codegen::structs_codegen::tables::project_states::ProjectState as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::ProjectWorkflowModel => {
                <crate::codegen::structs_codegen::tables::project_workflow_models::ProjectWorkflowModel as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Project => {
                <crate::codegen::structs_codegen::tables::projects::Project as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Rank => {
                <crate::codegen::structs_codegen::tables::ranks::Rank as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Reagent => {
                <crate::codegen::structs_codegen::tables::reagents::Reagent as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Role => {
                <crate::codegen::structs_codegen::tables::roles::Role as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Room => {
                <crate::codegen::structs_codegen::tables::rooms::Room as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::SampleState => {
                <crate::codegen::structs_codegen::tables::sample_states::SampleState as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::SamplingStepModel => {
                <crate::codegen::structs_codegen::tables::sampling_step_models::SamplingStepModel as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::SamplingStep => {
                <crate::codegen::structs_codegen::tables::sampling_steps::SamplingStep as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::ShakingStepModel => {
                <crate::codegen::structs_codegen::tables::shaking_step_models::ShakingStepModel as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::ShakingStep => {
                <crate::codegen::structs_codegen::tables::shaking_steps::ShakingStep as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::SpatialRefSy => {
                <crate::codegen::structs_codegen::tables::spatial_ref_sys::SpatialRefSy as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Spectrum => {
                <crate::codegen::structs_codegen::tables::spectra::Spectrum as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::SpectraCollection => {
                <crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::StepContainerModel => {
                <crate::codegen::structs_codegen::tables::step_container_models::StepContainerModel as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::StepInstrument => {
                <crate::codegen::structs_codegen::tables::step_instruments::StepInstrument as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::StepModelCategory => {
                <crate::codegen::structs_codegen::tables::step_model_categories::StepModelCategory as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::StepModelContainerCategory => {
                <crate::codegen::structs_codegen::tables::step_model_container_categories::StepModelContainerCategory as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::StepModelInstrumentCategory => {
                <crate::codegen::structs_codegen::tables::step_model_instrument_categories::StepModelInstrumentCategory as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::StepModelInstrumentModel => {
                <crate::codegen::structs_codegen::tables::step_model_instrument_models::StepModelInstrumentModel as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::StepModelInstrument => {
                <crate::codegen::structs_codegen::tables::step_model_instruments::StepModelInstrument as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::StepModelNameplateCategory => {
                <crate::codegen::structs_codegen::tables::step_model_nameplate_categories::StepModelNameplateCategory as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::StepModelToolCategory => {
                <crate::codegen::structs_codegen::tables::step_model_tool_categories::StepModelToolCategory as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::StepModel => {
                <crate::codegen::structs_codegen::tables::step_models::StepModel as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::StepNameplateModel => {
                <crate::codegen::structs_codegen::tables::step_nameplate_models::StepNameplateModel as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::StepStorageContainer => {
                <crate::codegen::structs_codegen::tables::step_storage_containers::StepStorageContainer as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::StepToolModel => {
                <crate::codegen::structs_codegen::tables::step_tool_models::StepToolModel as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Step => {
                <crate::codegen::structs_codegen::tables::steps::Step as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::StorageContainer => {
                <crate::codegen::structs_codegen::tables::storage_containers::StorageContainer as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Taxon => {
                <crate::codegen::structs_codegen::tables::taxa::Taxon as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::TeamMember => {
                <crate::codegen::structs_codegen::tables::team_members::TeamMember as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::TeamProject => {
                <crate::codegen::structs_codegen::tables::team_projects::TeamProject as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::TeamState => {
                <crate::codegen::structs_codegen::tables::team_states::TeamState as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Team => {
                <crate::codegen::structs_codegen::tables::teams::Team as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::TemporaryUser => {
                <crate::codegen::structs_codegen::tables::temporary_user::TemporaryUser as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::ToolModel => {
                <crate::codegen::structs_codegen::tables::tool_models::ToolModel as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::TrackableLocation => {
                <crate::codegen::structs_codegen::tables::trackable_locations::TrackableLocation as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::TrackableState => {
                <crate::codegen::structs_codegen::tables::trackable_states::TrackableState as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Trackable => {
                <crate::codegen::structs_codegen::tables::trackables::Trackable as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Unit => {
                <crate::codegen::structs_codegen::tables::units::Unit as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::UserEmail => {
                <crate::codegen::structs_codegen::tables::user_emails::UserEmail as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::UserOrganization => {
                <crate::codegen::structs_codegen::tables::user_organizations::UserOrganization as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::User => {
                <crate::codegen::structs_codegen::tables::users::User as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::VolumetricProcessable => {
                <crate::codegen::structs_codegen::tables::volumetric_processables::VolumetricProcessable as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::WeighingInstrumentModel => {
                <crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::WeighingStepModel => {
                <crate::codegen::structs_codegen::tables::weighing_step_models::WeighingStepModel as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::WeighingStep => {
                <crate::codegen::structs_codegen::tables::weighing_steps::WeighingStep as web_common_traits::database::AsyncBoundedRead<
                    diesel_async::AsyncPgConnection,
                >>::bounded_read_async(offset, limit, conn)
                    .await
                    .map(super::Rows::from)
            }
        }
    }
}

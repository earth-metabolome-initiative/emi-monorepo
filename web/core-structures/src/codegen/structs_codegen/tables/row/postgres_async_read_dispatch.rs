#[cfg(feature = "postgres")]
impl web_common_traits::prelude::AsyncReadDispatch<diesel_async::AsyncPgConnection> for super::Row {
    type PrimaryKey = crate::codegen::tables::table_primary_keys::TablePrimaryKey;
    async fn read(
        primary_key: Self::PrimaryKey,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        match primary_key {
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Address(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::addresses::Address as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::AliquotingInstrumentModel(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::aliquoting_instrument_models::AliquotingInstrumentModel as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::AliquotingStepModel(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::aliquoting_step_models::AliquotingStepModel as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::AliquotingStep(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::aliquoting_steps::AliquotingStep as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::BallMillStepModel(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::ball_mill_step_models::BallMillStepModel as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::BallMillStep(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::ball_mill_steps::BallMillStep as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Brand(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::brands::Brand as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::CentrifugeStepModel(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::centrifuge_step_models::CentrifugeStepModel as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::CentrifugeStep(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::centrifuge_steps::CentrifugeStep as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::City(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::cities::City as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Color(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::colors::Color as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialProductLot(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialProduct(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialReagentModel(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::commercial_reagent_models::CommercialReagentModel as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialReagent(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::commercial_reagents::CommercialReagent as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ContainerModel(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::container_models::ContainerModel as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Country(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::countries::Country as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::DisposalStepModel(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::disposal_step_models::DisposalStepModel as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::DisposalStep(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::disposal_steps::DisposalStep as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::DocumentFormat(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::document_formats::DocumentFormat as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::EmailProvider(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::email_providers::EmailProvider as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::FractioningStepModel(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::fractioning_step_models::FractioningStepModel as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::FractioningStep(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::fractioning_steps::FractioningStep as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::FreezeDryingStepModel(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::freeze_drying_step_models::FreezeDryingStepModel as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::InstrumentLocation(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::instrument_locations::InstrumentLocation as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::InstrumentModelCategory(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::instrument_model_categories::InstrumentModelCategory as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::InstrumentModel(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::InstrumentState(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::instrument_states::InstrumentState as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Instrument(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::instruments::Instrument as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::LoginProvider(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::login_providers::LoginProvider as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Material(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::materials::Material as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::NameplateModel(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::nameplate_models::NameplateModel as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ObservationSubject(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::OrganismObservation(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::organism_observations::OrganismObservation as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::OrganismSamplingStepModel(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::organism_sampling_step_models::OrganismSamplingStepModel as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::OrganismTaxon(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::organism_taxa::OrganismTaxon as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Organism(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::organisms::Organism as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Organization(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::organizations::Organization as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::PackagingModel(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::packaging_models::PackagingModel as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::PackagingStepModel(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::packaging_step_models::PackagingStepModel as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::PermanenceCategory(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::permanence_categories::PermanenceCategory as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Photograph(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::photographs::Photograph as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureModelContainerCategory(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::procedure_model_container_categories::ProcedureModelContainerCategory as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureModelInstrumentCategory(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::procedure_model_instrument_categories::ProcedureModelInstrumentCategory as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureModelNameplateCategory(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::procedure_model_nameplate_categories::ProcedureModelNameplateCategory as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureModelToolCategory(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::procedure_model_tool_categories::ProcedureModelToolCategory as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureModel(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureStepModel(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::procedure_step_models::ProcedureStepModel as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Procedure(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::procedures::Procedure as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Processable(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::processables::Processable as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcessingStep(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::processing_steps::ProcessingStep as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProjectState(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::project_states::ProjectState as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProjectWorkflowModel(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::project_workflow_models::ProjectWorkflowModel as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Project(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::projects::Project as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Rank(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::ranks::Rank as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Reagent(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::reagents::Reagent as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Role(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::roles::Role as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Room(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::rooms::Room as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::SampleState(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::sample_states::SampleState as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::SamplingStepModel(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::sampling_step_models::SamplingStepModel as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::SamplingStep(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::sampling_steps::SamplingStep as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ShakingStepModel(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::shaking_step_models::ShakingStepModel as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ShakingStep(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::shaking_steps::ShakingStep as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::SpatialRefSy(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::spatial_ref_sys::SpatialRefSy as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Spectrum(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::spectra::Spectrum as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::SpectraCollection(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::StepContainerModel(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::step_container_models::StepContainerModel as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::StepInstrument(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::step_instruments::StepInstrument as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::StepModelCategory(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::step_model_categories::StepModelCategory as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::StepModelContainerCategory(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::step_model_container_categories::StepModelContainerCategory as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::StepModelInstrumentCategory(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::step_model_instrument_categories::StepModelInstrumentCategory as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::StepModelInstrumentModel(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::step_model_instrument_models::StepModelInstrumentModel as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::StepModelInstrument(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::step_model_instruments::StepModelInstrument as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::StepModelNameplateCategory(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::step_model_nameplate_categories::StepModelNameplateCategory as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::StepModelToolCategory(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::step_model_tool_categories::StepModelToolCategory as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::StepModel(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::step_models::StepModel as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::StepNameplateModel(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::step_nameplate_models::StepNameplateModel as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::StepStorageContainer(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::step_storage_containers::StepStorageContainer as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::StepToolModel(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::step_tool_models::StepToolModel as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Step(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::steps::Step as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::StorageContainer(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::storage_containers::StorageContainer as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Taxon(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::taxa::Taxon as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::TeamMember(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::team_members::TeamMember as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::TeamProject(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::team_projects::TeamProject as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::TeamState(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::team_states::TeamState as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Team(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::teams::Team as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ToolModel(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::tool_models::ToolModel as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::TrackableLocation(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::trackable_locations::TrackableLocation as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::TrackableState(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::trackable_states::TrackableState as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Trackable(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::trackables::Trackable as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Unit(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::units::Unit as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::UserEmail(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::user_emails::UserEmail as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::UserOrganization(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::user_organizations::UserOrganization as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::User(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::users::User as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::VolumetricProcessable(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::volumetric_processables::VolumetricProcessable as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::WeighingInstrumentModel(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::WeighingStepModel(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::weighing_step_models::WeighingStepModel as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::WeighingStep(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::weighing_steps::WeighingStep as web_common_traits::database::AsyncRead<
                        diesel_async::AsyncPgConnection,
                    >>::read_async(primary_key, conn)
                        .await?
                        .map(super::Row::from),
                )
            }
        }
    }
}

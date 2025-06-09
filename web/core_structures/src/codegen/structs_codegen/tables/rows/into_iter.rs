impl From<super::Rows> for Vec<crate::codegen::tables::row::Row> {
    fn from(rows: super::Rows) -> Self {
        match rows {
            super::Rows::Address(rows) => {
                rows.into_iter().map(crate::codegen::tables::row::Row::Address).collect::<Vec<_>>()
            }
            super::Rows::AliquotingInstrumentModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::AliquotingInstrumentModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::AliquotingProcedureModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::AliquotingProcedureModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::BallMillProcedureModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::BallMillProcedureModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::Brand(rows) => {
                rows.into_iter().map(crate::codegen::tables::row::Row::Brand).collect::<Vec<_>>()
            }
            super::Rows::CentrifugeProcedureModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::CentrifugeProcedureModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::City(rows) => {
                rows.into_iter().map(crate::codegen::tables::row::Row::City).collect::<Vec<_>>()
            }
            super::Rows::Color(rows) => {
                rows.into_iter().map(crate::codegen::tables::row::Row::Color).collect::<Vec<_>>()
            }
            super::Rows::CommercialProductLot(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::CommercialProductLot)
                    .collect::<Vec<_>>()
            }
            super::Rows::CommercialProduct(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::CommercialProduct)
                    .collect::<Vec<_>>()
            }
            super::Rows::CommercialReagent(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::CommercialReagent)
                    .collect::<Vec<_>>()
            }
            super::Rows::ContainerModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::ContainerModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::Container(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::Container)
                    .collect::<Vec<_>>()
            }
            super::Rows::Country(rows) => {
                rows.into_iter().map(crate::codegen::tables::row::Row::Country).collect::<Vec<_>>()
            }
            super::Rows::DisposalProcedureModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::DisposalProcedureModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::Document(rows) => {
                rows.into_iter().map(crate::codegen::tables::row::Row::Document).collect::<Vec<_>>()
            }
            super::Rows::EmailProvider(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::EmailProvider)
                    .collect::<Vec<_>>()
            }
            super::Rows::FractioningProcedureModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::FractioningProcedureModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::FreezeDryingProcedureModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::FreezeDryingProcedureModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::InstrumentModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::InstrumentModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::InstrumentState(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::InstrumentState)
                    .collect::<Vec<_>>()
            }
            super::Rows::Instrument(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::Instrument)
                    .collect::<Vec<_>>()
            }
            super::Rows::LoginProvider(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::LoginProvider)
                    .collect::<Vec<_>>()
            }
            super::Rows::Material(rows) => {
                rows.into_iter().map(crate::codegen::tables::row::Row::Material).collect::<Vec<_>>()
            }
            super::Rows::MixCountableProcedureModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::MixCountableProcedureModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::MixSolidProcedureModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::MixSolidProcedureModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::NextProcedureModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::NextProcedureModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::ObservationSubject(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::ObservationSubject)
                    .collect::<Vec<_>>()
            }
            super::Rows::OrganismObservation(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::OrganismObservation)
                    .collect::<Vec<_>>()
            }
            super::Rows::OrganismTaxon(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::OrganismTaxon)
                    .collect::<Vec<_>>()
            }
            super::Rows::Organism(rows) => {
                rows.into_iter().map(crate::codegen::tables::row::Row::Organism).collect::<Vec<_>>()
            }
            super::Rows::Organization(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::Organization)
                    .collect::<Vec<_>>()
            }
            super::Rows::PackagingModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::PackagingModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::PackagingProcedureModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::PackagingProcedureModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::ParentProcedureModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::ParentProcedureModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::PermanenceCategory(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::PermanenceCategory)
                    .collect::<Vec<_>>()
            }
            super::Rows::PouringProcedureModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::PouringProcedureModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::ProcedureModelTrackable(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::ProcedureModelTrackable)
                    .collect::<Vec<_>>()
            }
            super::Rows::ProcedureModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::ProcedureModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::ProcedureTrackable(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::ProcedureTrackable)
                    .collect::<Vec<_>>()
            }
            super::Rows::Procedure(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::Procedure)
                    .collect::<Vec<_>>()
            }
            super::Rows::Processable(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::Processable)
                    .collect::<Vec<_>>()
            }
            super::Rows::ProjectState(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::ProjectState)
                    .collect::<Vec<_>>()
            }
            super::Rows::Project(rows) => {
                rows.into_iter().map(crate::codegen::tables::row::Row::Project).collect::<Vec<_>>()
            }
            super::Rows::Rank(rows) => {
                rows.into_iter().map(crate::codegen::tables::row::Row::Rank).collect::<Vec<_>>()
            }
            super::Rows::Reagent(rows) => {
                rows.into_iter().map(crate::codegen::tables::row::Row::Reagent).collect::<Vec<_>>()
            }
            super::Rows::Role(rows) => {
                rows.into_iter().map(crate::codegen::tables::row::Row::Role).collect::<Vec<_>>()
            }
            super::Rows::Room(rows) => {
                rows.into_iter().map(crate::codegen::tables::row::Row::Room).collect::<Vec<_>>()
            }
            super::Rows::SampleState(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::SampleState)
                    .collect::<Vec<_>>()
            }
            super::Rows::SamplingProcedureModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::SamplingProcedureModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::ShakingProcedureModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::ShakingProcedureModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::SharedProcedureModelTrackable(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::SharedProcedureModelTrackable)
                    .collect::<Vec<_>>()
            }
            super::Rows::SpatialRefSy(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::SpatialRefSy)
                    .collect::<Vec<_>>()
            }
            super::Rows::Spectrum(rows) => {
                rows.into_iter().map(crate::codegen::tables::row::Row::Spectrum).collect::<Vec<_>>()
            }
            super::Rows::SpectraCollection(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::SpectraCollection)
                    .collect::<Vec<_>>()
            }
            super::Rows::Taxon(rows) => {
                rows.into_iter().map(crate::codegen::tables::row::Row::Taxon).collect::<Vec<_>>()
            }
            super::Rows::TeamMember(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::TeamMember)
                    .collect::<Vec<_>>()
            }
            super::Rows::TeamProject(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::TeamProject)
                    .collect::<Vec<_>>()
            }
            super::Rows::TeamState(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::TeamState)
                    .collect::<Vec<_>>()
            }
            super::Rows::Team(rows) => {
                rows.into_iter().map(crate::codegen::tables::row::Row::Team).collect::<Vec<_>>()
            }
            super::Rows::TemporaryUser(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::TemporaryUser)
                    .collect::<Vec<_>>()
            }
            super::Rows::TrackableLocation(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::TrackableLocation)
                    .collect::<Vec<_>>()
            }
            super::Rows::Trackable(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::Trackable)
                    .collect::<Vec<_>>()
            }
            super::Rows::Unit(rows) => {
                rows.into_iter().map(crate::codegen::tables::row::Row::Unit).collect::<Vec<_>>()
            }
            super::Rows::UserEmail(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::UserEmail)
                    .collect::<Vec<_>>()
            }
            super::Rows::UserOrganization(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::UserOrganization)
                    .collect::<Vec<_>>()
            }
            super::Rows::User(rows) => {
                rows.into_iter().map(crate::codegen::tables::row::Row::User).collect::<Vec<_>>()
            }
            super::Rows::VolumetricProcessable(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::VolumetricProcessable)
                    .collect::<Vec<_>>()
            }
            super::Rows::WeighingInstrumentModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::WeighingInstrumentModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::WeighingProcedureModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::WeighingProcedureModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::WeighingProcedure(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::WeighingProcedure)
                    .collect::<Vec<_>>()
            }
        }
    }
}
impl IntoIterator for super::Rows {
    type Item = crate::codegen::tables::row::Row;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        let row_vec: Vec<crate::codegen::tables::row::Row> = self.into();
        row_vec.into_iter()
    }
}

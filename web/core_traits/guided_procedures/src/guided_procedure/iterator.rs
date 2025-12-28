//! Submodule implementing the iterator for the `GuidedProcedureBuilder`.

#[cfg(feature = "backend")]
use std::fmt::Debug;

use core_structures::{
    ProcedureAsset, ProcedureTemplate, ProcedureTemplateAssetModel,
    tables::most_concrete_variants::{ProcedureBuilderDAG, ProcedureInsertErrorDAG},
};
#[cfg(feature = "backend")]
use web_common_traits::database::{
    DispatchableInsertVariantMetadata, DispatchableInsertableVariant, PrimaryKeyLike,
};
use web_common_traits::prelude::{MostConcreteVariant, ProcedureLike};

use crate::{
    GuidedProcedure,
    guided_procedure::error::{GuidedProcedureError, InternalGuidedProcedureError},
};

impl<'graph, C> Iterator for GuidedProcedure<'graph, C>
where
    ProcedureTemplate: MostConcreteVariant<C>,
    ProcedureBuilderDAG: DispatchableInsertableVariant<C>,
{
    type Item = Result<
        (Vec<&'graph ProcedureTemplate>, &'graph ProcedureTemplate, ProcedureBuilderDAG),
        InternalGuidedProcedureError<'graph>,
    >;

    fn next(&mut self) -> Option<Self::Item> {
        match self.visitor.next()? {
            Ok(output) => {
                match output {
                    None => self.next(),
                    Some(res) => Some(Ok(res)),
                }
            }
            Err(e) => Some(Err(e)),
        }
    }
}

#[cfg(feature = "backend")]
impl<'graph> GuidedProcedure<'graph, diesel::PgConnection> {
    /// Attempts to retrieve the next builder of the expected type from the
    /// iterator. If the next builder is not of the expected type, an error is
    /// returned. If there are no more builders, an error is also returned.
    ///
    /// # Arguments
    ///
    /// * `map` - A closure that takes the expected builder and a mutable
    ///   reference to the database connection, and returns a modified builder
    ///   or an error.
    ///
    /// # Errors
    ///
    /// * Returns `GuidedProcedureError::UnexpectedBuilder` if the next builder
    ///   is not of the expected type.
    /// * Returns `GuidedProcedureError::NoMoreBuilders` if there are no more
    ///   builders in the iterator.
    /// * Returns `GuidedProcedureError::ProcedureInsertErrorDAG` if the
    ///   insertion of the builder into the database fails.
    /// * Returns any error returned by the `map` closure.
    pub fn and_then<ExpectedProcedure, E>(
        mut self,
        map: impl FnOnce(
            ExpectedProcedure::Builder,
            &mut diesel::PgConnection,
        ) -> Result<ExpectedProcedure::Builder, E>,
    ) -> Result<Self, E>
    where
        ProcedureInsertErrorDAG:
            From<<ExpectedProcedure::Builder as DispatchableInsertVariantMetadata>::Error>,
        ExpectedProcedure: ProcedureLike<
                ProcedureAsset = ProcedureAsset,
                ProcedureTemplateAssetModel = ProcedureTemplateAssetModel,
            > + PrimaryKeyLike<PrimaryKey = ::rosetta_uuid::Uuid>,
        ExpectedProcedure::Builder: DispatchableInsertableVariant<diesel::PgConnection> + Debug,
        E: From<ProcedureInsertErrorDAG>,
        Option<ExpectedProcedure::Builder>: From<ProcedureBuilderDAG>,
    {
        let (parents, builder): (Vec<&'graph ProcedureTemplate>, ExpectedProcedure::Builder) =
            if let Some(result) = self.next() {
                let (parents, template, builder_dag) = result.unwrap();
                let found = builder_dag.type_name();
                if let Some(expected_builder) =
                    Option::<ExpectedProcedure::Builder>::from(builder_dag)
                {
                    (parents, expected_builder)
                } else {
                    // return Err(GuidedProcedureError::UnexpectedBuilder {
                    //     expected: std::any::type_name::<ExpectedProcedure::Builder>(),
                    //     found,
                    //     template,
                    // }.into());
                    unreachable!("Expected builder not found, but should have been checked earlier")
                }
            } else {
                unreachable!("No more builders available")
            };
        let builder = map(builder, self.visitor.listener_mut().connection())?;
        self.visitor
            .listener_mut()
            .insert(&parents, builder)
            .map_err(ProcedureInsertErrorDAG::from)?;
        Ok(self)
    }

    /// Finalizes the guided procedure by ensuring all builders have been
    /// processed and inserted into the database.
    ///
    /// # Errors
    ///
    /// * Returns `GuidedProcedureError::UnprocessedBuilder` if there are any
    ///   remaining builders that have not been processed.
    pub fn finish(mut self) -> Result<(), GuidedProcedureError> {
        if let Some(result) = self.next() {
            let (_parents, template, builder_dag) = result.unwrap();
            // Err(GuidedProcedureError::UnprocessedBuilder {
            //     found: builder_dag.type_name(),
            //     template,
            // })
            todo!()
        } else {
            Ok(())
        }
    }
}

//! Submodule defining an error that cannot occur during operations.

use std::{error::Error, fmt::Display, marker::PhantomData};

use web_common_traits::prelude::{Operation, OperationError};

#[common_traits::prelude::basic]
/// Error that cannot occur during operations.
pub struct InfallibleOperationError<OPS> {
    _phantom: PhantomData<OPS>,
}

impl<OPS: Operation> Display for InfallibleOperationError<OPS> {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unreachable!()
    }
}

impl<OPS: Operation> Error for InfallibleOperationError<OPS> {}

impl<OPS> OperationError for InfallibleOperationError<OPS>
where
    OPS: Operation<Error = Self>,
{
    type Operation = OPS;

    fn operation(&self) -> &Self::Operation {
        unreachable!()
    }
}

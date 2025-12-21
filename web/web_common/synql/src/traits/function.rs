//! Submodule defining and implementing the `FunctionSyn` trait, which provide
//! methods to facilitate the rust code generation starting from a SQL function
//! representation, building on top of the
//! [`FunctionLike`](sql_traits::traits::FunctionLike) trait.

use sql_traits::traits::FunctionLike;

use crate::structs::{ExternalFunctionRef, ExternalTypeRef, Workspace};

/// Trait implemented by types that represent SQL functions and can be used to
/// generate Rust code for them.
pub trait FunctionSynLike: FunctionLike {
    /// Returns the type ref curresponding to the argument types of the
    /// function, if any.
    ///
    /// # Arguments
    ///
    /// * `workspace` - The workspace where the column is defined.
    /// * `database` - The database where the column is defined.
    fn argument_types<'workspace, 'db>(
        &'db self,
        workspace: &'workspace Workspace,
        database: &'db Self::DB,
    ) -> impl Iterator<Item = Option<ExternalTypeRef<'workspace>>> {
        self.normalized_argument_type_names(database)
            .into_iter()
            .map(move |arg_type_name| workspace.external_postgres_type(arg_type_name))
    }

    /// Returns the type ref curresponding to the return type of the
    /// function, if any.
    ///
    /// # Arguments
    /// * `workspace` - The workspace where the column is defined.
    /// * `database` - The database where the column is defined.
    fn return_type<'workspace>(
        &self,
        workspace: &'workspace Workspace,
        database: &Self::DB,
    ) -> Option<ExternalTypeRef<'workspace>> {
        self.normalized_return_type_name(database)
            .and_then(|ret_type_name| workspace.external_postgres_type(ret_type_name))
    }

    /// Returns the external function reference for this function, if any.
    ///
    /// # Arguments
    /// * `workspace` - The workspace where the column is defined.
    fn external_function_ref<'workspace>(&self, workspace: &'workspace Workspace) -> Option<ExternalFunctionRef<'workspace>> {
        workspace.external_function(self.name())
    }
}

impl<T> FunctionSynLike for T where T: FunctionLike {}

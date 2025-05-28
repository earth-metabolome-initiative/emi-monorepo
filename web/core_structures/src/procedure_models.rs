//! Submodule implementing methods for the `ProcedureModel` struct.

use container_categories::ContainerCategory;
use diesel_async::AsyncPgConnection;
use nameplate_categories::NameplateCategory;
use tool_categories::ToolCategory;
use web_common_traits::{
    database::{InsertError, Insertable, InsertableVariant},
    prelude::Builder,
};

use crate::tables::insertables::{
    InsertableParentProcedureModelAttributes, InsertableProcedureModelContainerCategoryAttributes,
    InsertableProcedureModelNameplateCategoryAttributes,
    InsertableProcedureModelToolCategoryAttributes,
};

impl crate::ProcedureModel {
    #[cfg(feature = "postgres")]
    /// Creates a new tool category for the procedure model.
    ///
    /// # Arguments
    ///
    /// * `tool_category`: The category of the tool to be added.
    /// * `user`: The user who is creating the tool category.
    /// * `conn`: The database connection to use for the insertion.
    ///
    /// # Errors
    ///
    /// * If the insertion fails, an `InsertError` is returned.
    pub async fn tool(
        &self,
        tool_category: ToolCategory,
        user: &crate::User,
        conn: &mut AsyncPgConnection,
    ) -> Result<
        crate::ProcedureModelToolCategory,
        InsertError<InsertableProcedureModelToolCategoryAttributes>,
    > {
        crate::ProcedureModelToolCategory::new()
            .procedure_model_id(self.id)?
            .tool_category(tool_category)?
            .created_by(user.id)?
            .build()?
            .insert(&user.id, conn)
            .await
    }

    #[cfg(feature = "postgres")]
    /// Creates a new nameplate category for the procedure model.
    ///
    /// # Arguments
    ///
    /// * `nameplate_category`: The category of the nameplate to be added.
    /// * `user`: The user who is creating the tool category.
    /// * `conn`: The database connection to use for the insertion.
    ///
    /// # Errors
    ///
    /// * If the insertion fails, an `InsertError` is returned.
    pub async fn nameplate(
        &self,
        nameplate_category: NameplateCategory,
        user: &crate::User,
        conn: &mut AsyncPgConnection,
    ) -> Result<
        crate::ProcedureModelNameplateCategory,
        InsertError<InsertableProcedureModelNameplateCategoryAttributes>,
    > {
        crate::ProcedureModelNameplateCategory::new()
            .procedure_model_id(self.id)?
            .nameplate_category(nameplate_category)?
            .created_by(user.id)?
            .build()?
            .insert(&user.id, conn)
            .await
    }

    #[cfg(feature = "postgres")]
    /// Creates a new container category for the procedure model.
    ///
    /// # Arguments
    ///
    /// * `container_category`: The category of the container to be added.
    /// * `user`: The user who is creating the tool category.
    /// * `conn`: The database connection to use for the insertion.
    ///
    /// # Errors
    ///
    /// * If the insertion fails, an `InsertError` is returned.
    pub async fn container(
        &self,
        container_category: ContainerCategory,
        user: &crate::User,
        conn: &mut AsyncPgConnection,
    ) -> Result<
        crate::ProcedureModelContainerCategory,
        InsertError<InsertableProcedureModelContainerCategoryAttributes>,
    > {
        crate::ProcedureModelContainerCategory::new()
            .procedure_model_id(self.id)?
            .container_category(container_category)?
            .created_by(user.id)?
            .build()?
            .insert(&user.id, conn)
            .await
    }

    #[cfg(feature = "postgres")]
    /// Creates a new parent-child relationship for a procedure.
    ///
    /// # Arguments
    ///
    /// * `child_procedure`: The child procedure model to be added.
    /// * `user`: The user who is creating the tool category.
    /// * `conn`: The database connection to use for the insertion.
    ///
    /// # Errors
    ///
    /// * If the insertion fails, an `InsertError` is returned.
    pub async fn child(
        &self,
        child_procedure: &Self,
        user: &crate::User,
        conn: &mut AsyncPgConnection,
    ) -> Result<crate::ParentProcedureModel, InsertError<InsertableParentProcedureModelAttributes>>
    {
        crate::ParentProcedureModel::new()
            .parent_procedure_model_id(self.id)?
            .child_procedure_model_id(child_procedure.id)?
            .created_by(user.id)?
            .build()?
            .insert(&user.id, conn)
            .await
    }
}

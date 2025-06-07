#![cfg(feature = "postgres")]
//! Submodule providing a `ProcedureModelDot` trait for creating a DOT
//! representation of a procedure model.

use diesel::PgConnection;
use web_common_traits::{database::Read, prelude::ExtensionTable};

use crate::{
    NextProcedureModel, ParentProcedureModel, ProcedureModel, ProcedureModelTrackable,
    SharedProcedureModelTrackable,
};

pub trait ProcedureModelDot: ExtensionTable<ProcedureModel>
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{
    fn procedures_nodes(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<ProcedureModel>, diesel::result::Error> {
        use diesel::Identifiable;
        let child_procedures =
            ParentProcedureModel::from_parent_procedure_model_id(self.id(), conn)?
                .into_iter()
                .map(|p| p.child_procedure_model(conn))
                .collect::<Result<Vec<_>, diesel::result::Error>>()?;

        let mut procedures = child_procedures.clone();

        for procedure in child_procedures {
            procedures.extend(procedure.procedures_nodes(conn)?);
        }

        procedures.sort_unstable();
        procedures.dedup();

        Ok(procedures)
    }

    /// Returns all of the nodes associated with this procedure model.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection` for database
    ///
    /// # Errors
    ///
    /// * If an error occurs while retrieving the nodes, it returns a
    ///   `diesel::result::Error`.
    fn trackable_nodes(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<ProcedureModelTrackable>, diesel::result::Error> {
        use diesel::Identifiable;
        let child_procedures =
            ParentProcedureModel::from_parent_procedure_model_id(self.id(), conn)?
                .into_iter()
                .map(|p| p.child_procedure_model(conn))
                .collect::<Result<Vec<_>, diesel::result::Error>>()?;

        let mut trackables = ProcedureModelTrackable::from_procedure_model_id(self.id(), conn)?;

        for procedure in child_procedures {
            trackables.extend(procedure.trackable_nodes(conn)?);
        }

        trackables.sort_unstable();
        trackables.dedup();

        Ok(trackables)
    }

    fn parent_procedure_edges(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<ParentProcedureModel>, diesel::result::Error> {
        use diesel::Identifiable;

        let mut parent_procedures =
            ParentProcedureModel::from_parent_procedure_model_id(self.id(), conn)?;

        for child_procedure in self.procedures_nodes(conn)? {
            parent_procedures.extend(child_procedure.parent_procedure_edges(conn)?);
        }

        parent_procedures.sort_unstable();
        parent_procedures.dedup();

        Ok(parent_procedures)
    }

    fn next_procedure_edges(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<NextProcedureModel>, diesel::result::Error> {
        use diesel::Identifiable;

        let mut next_procedures = NextProcedureModel::from_parent_id(self.id(), conn)?;

        for child_procedure in self.procedures_nodes(conn)? {
            next_procedures.extend(child_procedure.next_procedure_edges(conn)?);
        }

        next_procedures.sort_unstable();
        next_procedures.dedup();

        Ok(next_procedures)
    }

    fn shared_trackable_edges(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<SharedProcedureModelTrackable>, diesel::result::Error> {
        use diesel::Identifiable;

        let mut shared_trackables = SharedProcedureModelTrackable::from_parent_id(self.id(), conn)?;
        for child_procedure in self.procedures_nodes(conn)? {
            shared_trackables.extend(child_procedure.shared_trackable_edges(conn)?);
        }

        shared_trackables.sort_unstable();
        shared_trackables.dedup();

        Ok(shared_trackables)
    }

    /// Generates a DOT representation of the procedure model.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection` for database
    ///   operations.
    ///
    /// # Errors
    ///
    /// * If an error occurs while generating the DOT representation, it returns
    ///   a `diesel::result::Error`.
    fn to_dot(&self, conn: &mut PgConnection) -> Result<String, diesel::result::Error> {
        use diesel::Identifiable;

        let mut dot = String::new();

        dot.push_str("digraph G {\n");

        let procedure_model = ProcedureModel::read(*self.id(), conn)?;

        // We display the child procedures as boxes
        for procedure in &self.procedures_nodes(conn)? {
            dot.push_str(&format!(
                "    P{} [label=\"{}\", shape=box, color=red];\n",
                procedure.id(),
                procedure.name
            ));
        }

        if let Some(procedure_model) = procedure_model.as_ref() {
            // We display the root procedure model as a diamond
            dot.push_str(&format!(
                "    P{} [label=\"{}\", shape=box, color=blue];\n",
                procedure_model.id(),
                procedure_model.name
            ));
        }

        // We display the trackables as circles
        for procedure_trackable in &self.trackable_nodes(conn)? {
            dot.push_str(&format!(
                "    T{} [label=\"{}\", shape=diamond, color=green];\n",
                procedure_trackable.id(),
                procedure_trackable.name
            ));
        }

        // We link parent procedures to child procedures
        for parent in &self.parent_procedure_edges(conn)? {
            dot.push_str(&format!(
                "    P{} -> P{} [style=dashed, color=blue, label=\"Parent of\"];\n",
                parent.parent_procedure_model_id, parent.child_procedure_model_id
            ));
        }

        // We link next procedures to child procedures
        for next in &self.next_procedure_edges(conn)? {
            dot.push_str(&format!(
                "    P{} -> P{} [color=red, label=\"And then\"];\n",
                next.current_id, next.successor_id
            ));
        }

        // We link the procedure model to its trackables
        for trackable in &self.trackable_nodes(conn)? {
            dot.push_str(&format!(
                "    P{} -> T{} [style=dashed, color=orange, label=\"Uses\"];\n",
                trackable.procedure_model_id,
                trackable.id()
            ));
        }

        // We link shared trackables to child procedures
        for shared in &self.shared_trackable_edges(conn)? {
            dot.push_str(&format!(
                "    T{} -> T{} [style=dashed, color=green, label=\"Same as\"];\n",
                shared.parent(conn)?.id(),
                shared.child(conn)?.id()
            ));
        }

        dot.push_str("}\n");

        Ok(dot)
    }
}

impl<T> ProcedureModelDot for T
where
    T: ExtensionTable<ProcedureModel>,
    for<'a> &'a T: diesel::Identifiable<Id = &'a i32>,
{
}

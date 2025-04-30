//! Webcommon traits generation
use std::path::Path;
mod attribute_traits;
mod deletable;
mod foreign;
mod insertables;
mod tabular;
mod updatables;
mod upsertables;

use diesel::PgConnection;
use proc_macro2::TokenStream;
use syn::Ident;
use time_requirements::prelude::{Task, TimeTracker};

use crate::{
    Codegen, Table,
    codegen::{
        CODEGEN_FOREIGN_PATH, CODEGEN_INSERTABLES_PATH, CODEGEN_TABULAR_PATH,
        CODEGEN_UPDATABLES_PATH, CODEGEN_UPSERTABLES_PATH,
    },
};

impl Codegen<'_> {
    /// Code relative to generating all of the diesel code.
    ///
    /// # Arguments
    ///
    /// * `root` - The root path for the generated code.
    /// * `tables` - The list of tables for which to generate the diesel code.
    /// * `conn` - A mutable reference to a `PgConnection`.
    pub(crate) fn generate_table_traits(
        &self,
        root: &Path,
        tables: &[Table],
        conn: &mut PgConnection,
    ) -> Result<TimeTracker, crate::errors::WebCodeGenError> {
        let submodule_file = root.with_extension("rs");
        let mut tracker = TimeTracker::new("Generate Table Traits");

        std::fs::create_dir_all(root)?;

        let mut submodule_file_content = TokenStream::new();

        if self.should_generate_crud() {
            let task = Task::new("Generate CRUD Traits");
            self.generate_tabular_impls(root.join(CODEGEN_TABULAR_PATH).as_path(), tables, conn)?;

            let tabular_module_ident =
                Ident::new(CODEGEN_TABULAR_PATH, proc_macro2::Span::call_site());

            submodule_file_content.extend(quote::quote! {
                mod #tabular_module_ident;
            });
            tracker.add_completed_task(task);
        }

        if self.enable_deletable_trait {
            let task = Task::new("Generate Deletable Traits");
            self.generate_deletable_impls(root.join("deletable").as_path(), tables, conn)?;

            let deletable_module_ident = Ident::new("deletable", proc_macro2::Span::call_site());

            submodule_file_content.extend(quote::quote! {
                mod #deletable_module_ident;
            });
            tracker.add_completed_task(task);
        }

        if self.enable_upsertable_trait {
            let task = Task::new("Generate Upsertable Traits");
            self.generate_upsertables_impls(
                root.join(CODEGEN_UPSERTABLES_PATH).as_path(),
                tables,
                conn,
            )?;

            let upsertable_module_ident =
                Ident::new(CODEGEN_UPSERTABLES_PATH, proc_macro2::Span::call_site());

            submodule_file_content.extend(quote::quote! {
                mod #upsertable_module_ident;
            });
            tracker.add_completed_task(task);
        }

        if self.enable_attribute_trait {
            let task = Task::new("Generate Attribute Traits");
            self.generate_attribute_impls(root.join("attributes").as_path(), tables, conn)?;

            let attribute_module_ident = Ident::new("attributes", proc_macro2::Span::call_site());

            submodule_file_content.extend(quote::quote! {
                mod #attribute_module_ident;
            });
            tracker.add_completed_task(task);
        }

        if self.enable_foreign_trait {
            let task = Task::new("Generate Foreign Traits");
            self.generate_foreign_impls(root.join(CODEGEN_FOREIGN_PATH).as_path(), tables, conn)?;

            let foreign_module_ident =
                Ident::new(CODEGEN_FOREIGN_PATH, proc_macro2::Span::call_site());

            submodule_file_content.extend(quote::quote! {
                mod #foreign_module_ident;
            });
            tracker.add_completed_task(task);
        }

        if self.enable_insertable_trait {
            let task = Task::new("Generate Insertable Traits");
            self.generate_insertables_impls(&root.join(CODEGEN_INSERTABLES_PATH), tables, conn)?;

            let insertable_module_ident =
                Ident::new(CODEGEN_INSERTABLES_PATH, proc_macro2::Span::call_site());

            submodule_file_content.extend(quote::quote! {
                mod #insertable_module_ident;
            });
            tracker.add_completed_task(task);
        }

        if self.enable_updatable_trait {
            let task = Task::new("Generate Updatable Traits");
            self.generate_updatables_impls(&root.join(CODEGEN_UPDATABLES_PATH), tables, conn)?;

            let updatable_module_ident =
                Ident::new(CODEGEN_UPDATABLES_PATH, proc_macro2::Span::call_site());

            submodule_file_content.extend(quote::quote! {
                mod #updatable_module_ident;
            });
            tracker.add_completed_task(task);
        }

        std::fs::write(&submodule_file, submodule_file_content.to_string())?;

        Ok(tracker)
    }
}

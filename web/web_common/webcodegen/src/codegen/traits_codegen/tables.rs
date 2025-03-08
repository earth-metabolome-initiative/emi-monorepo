//! Webcommon traits generation
use std::path::Path;
mod attribute_traits;
mod deletable;
mod foreign;
mod insertables;
mod loadable;
mod updatables;

use diesel::PgConnection;
use proc_macro2::TokenStream;
use syn::Ident;

use crate::{
    codegen::{CODEGEN_INSERTABLES_PATH, CODEGEN_UPDATABLES_PATH},
    Codegen, Table,
};

use time_requirements::prelude::{Task, TimeTracker};

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

        if self.enable_deletable_trait {
            let task = Task::new("Generate Deletable Traits");
            self.generate_deletable_impls(root.join("deletable").as_path(), tables, conn)?;

            let deletable_module_ident = Ident::new("deletable", proc_macro2::Span::call_site());

            submodule_file_content.extend(quote::quote! {
                mod #deletable_module_ident;
            });
            tracker.add_completed_task(task);
        }

        if self.enable_loadable_trait {
            let task = Task::new("Generate Loadable Traits");
            self.generate_loadable_impls(root.join("loadable").as_path(), tables, conn)?;

            let loadable_module_ident = Ident::new("loadable", proc_macro2::Span::call_site());

            submodule_file_content.extend(quote::quote! {
                mod #loadable_module_ident;
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
            self.generate_foreign_impls(root.join("foreign").as_path(), tables, conn)?;

            let foreign_module_ident = Ident::new("foreign", proc_macro2::Span::call_site());

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

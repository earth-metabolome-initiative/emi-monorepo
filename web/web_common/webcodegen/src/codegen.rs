//! Submodule providing the 'Codegen' struct for code generation.

use std::path::Path;

use diesel::PgConnection;
use prettyplease::unparse;
use proc_macro2::TokenStream;
use syn::File;
mod diesel_codegen;
mod structs_codegen;
mod syntaxes;
mod traits_codegen;

pub use syntaxes::Syntax;
use time_requirements::prelude::{Task, TimeTracker};

use crate::{
    PgExtension, PgType, Table, TableExtensionNetwork,
    errors::{CodeGenerationError, WebCodeGenError},
};

/// Constant defining the codegen directory name.
pub const CODEGEN_DIRECTORY: &str = "codegen";
/// Constant defining the diesel codegen module name.
pub const CODEGEN_DIESEL_MODULE: &str = "diesel_codegen";
/// Constant defining the structs codegen module name.
pub const CODEGEN_STRUCTS_MODULE: &str = "structs_codegen";
/// Constant defining the traits codegen module name.
pub const CODEGEN_TRAITS_MODULE: &str = "traits_codegen";
/// Constant defining the submodule for the tables schema.
pub const CODEGEN_TABLES_PATH: &str = "tables";
/// Constant defining the submodule for the SQL types.
pub const CODEGEN_TYPES_PATH: &str = "types";
/// Constant defining the submodule for the diesel joinable macro.
pub const CODEGEN_JOINABLE_PATH: &str = "joinable";
/// Constant defining the submodule for the Upsertable trait implementations.
pub const CODEGEN_UPSERTABLES_PATH: &str = "upsertables";
/// Constant defining the submodule for the Insertable-related trait
/// implementation.
pub const CODEGEN_INSERTABLES_PATH: &str = "insertables";
/// Constant defining the submodule for the MostConcreteVariant-related trait
/// implementation.
pub const CODEGEN_MOST_CONCRETE_VARIANTS_PATH: &str = "most_concrete_variants";
/// Constant defining the submodule for the Insertable trait implementations.
pub const CODEGEN_INSERTABLE_PATH: &str = "insertable";
/// Constant defining the submodule for the foreign-keys-related trait
/// implementations.
pub const CODEGEN_FOREIGN_KEYS_PATH: &str = "foreign_keys";
/// Constant defining the submodule for the Foreign trait implementations.
pub const CODEGEN_FOREIGN_PATH: &str = "foreign";
/// Constant defining the submodule for the Tabular trait implementations.
pub const CODEGEN_TABULAR_PATH: &str = "tabular";
/// Constant defining the submodule for the `InsertableVariant` trait
/// implementations.
pub const CODEGEN_INSERTABLE_VARIANT_PATH: &str = "insertable_variant";
/// Constant defining the submodule for the Updatable-related trait
/// implementations.
pub const CODEGEN_UPDATABLES_PATH: &str = "updatables";
/// Constant defining the submodule for the Updatable trait implementations.
pub const CODEGEN_UPDATABLE_PATH: &str = "updatable";

#[derive(Debug, Default)]
#[allow(clippy::struct_excessive_bools)]
/// Struct for code generation.
pub struct Codegen<'a> {
    /// The users table to refer.
    users_table: Option<&'a Table>,
    /// The projects table to refer.
    projects_table: Option<&'a Table>,
    /// The teams table to refer.
    teams_table: Option<&'a Table>,
    /// The team members table to refer.
    team_members_table: Option<&'a Table>,
    /// The team projects table to refer.
    team_projects_table: Option<&'a Table>,
    /// List of tables to ignore when generating code.
    tables_deny_list: Vec<&'a Table>,
    /// List of extensions to take into consideration
    /// when generating the CHECK constraints validations in
    /// the generated code.
    pub(crate) check_constraints_extensions: Vec<&'a PgExtension>,
    /// The output directory for the generated code.
    output_directory: Option<&'a Path>,
    /// Whether to make the code readable.
    beautify: bool,
    /// Whether to generate the diesel `joinables`.
    pub(super) enable_joinables: bool,
    /// Whether to generate the diesel `allow_tables_to_appear_in_same_query`.
    pub(super) enable_allow_tables_to_appear_in_same_query: bool,
    /// Whether to generate the SQL types.
    pub(super) enable_sql_types: bool,
    /// Whether to generate the tables schema.
    pub(super) enable_tables_schema: bool,
    /// Whether to enable the generation of the table structs.
    pub(super) enable_table_structs: bool,
    /// Whether to enable the generation of the type structs.
    pub(super) enable_type_structs: bool,
    /// Whether to enable the generation of the type traits implementations.
    pub(super) enable_type_impls: bool,
    /// Whether to enable the
    /// [`Deletable`](web_common_traits::database::Deletable) traits
    /// implementations.
    pub(super) enable_deletable_trait: bool,
    /// Whether to enable the attribute traits implementations.
    pub(super) enable_attribute_trait: bool,
    /// Whether to enable the [`Foreign`](web_common_traits::database::Foreign)
    /// traits implementations.
    pub(super) enable_foreign_trait: bool,
    /// Whether to enable the
    /// [`Insertable`](web_common_traits::database::Insertable) traits
    /// implementations.
    pub(super) enable_insertable_trait: bool,
    /// Whether to enable the
    /// [`MostConcreteVariant`](web_common_traits::database::MostConcreteVariant) traits
    /// implementations.
    pub(super) enable_most_concrete_variant_trait: bool,
    /// Whether to enable the
    /// [`Updatable`](web_common_traits::database::Updatable) traits
    /// implementations.
    pub(super) enable_updatable_trait: bool,
    /// Whether to enable the
    /// [`Upsertable`](web_common_traits::database::Upsertable) traits
    /// implementations.
    pub(super) enable_upsertable_trait: bool,
    /// Whether to enable the
    /// [`Read`](web_common_traits::crud::Read) traits
    /// implementations.
    pub(super) enable_read_trait: bool,
    /// Whether to derive traits relative to the `yew` framework.
    pub(super) enable_yew: bool,
    /// Graph representing the "extend" relationships between tables.
    table_extension_network: Option<TableExtensionNetwork>,
}

impl<'a> Codegen<'a> {
    #[must_use]
    /// Returns a reference to the table extension network.
    pub fn table_extension_network(&self) -> Option<&TableExtensionNetwork> {
        self.table_extension_network.as_ref()
    }

    #[must_use]
    /// Check wether traits should be generated for tables.
    pub fn should_generate_table_traits(&self) -> bool {
        self.enable_deletable_trait
            || self.enable_attribute_trait
            || self.enable_foreign_trait
            || self.enable_insertable_trait
            || self.enable_most_concrete_variant_trait
            || self.enable_updatable_trait
            || self.enable_upsertable_trait
            || self.enable_read_trait
    }

    #[must_use]
    /// Check wether the `CRUD`-related traits should be generated.
    pub fn should_generate_crud(&self) -> bool {
        self.enable_read_trait
    }

    #[must_use]
    /// Sets to generate the derive traits for the `yew` framework.
    pub fn enable_yew(mut self) -> Self {
        self.enable_yew = true;
        self
    }

    #[must_use]
    /// Sets the user table to refer to.
    pub fn users(mut self, users_table: &'a Table) -> Self {
        self.users_table = Some(users_table);
        self
    }

    #[must_use]
    /// Sets the projects table to refer to.
    pub fn projects(mut self, projects_table: &'a Table) -> Self {
        self.projects_table = Some(projects_table);
        self
    }

    #[must_use]
    /// Sets the teams table to refer to.
    pub fn teams(mut self, teams_table: &'a Table) -> Self {
        self.teams_table = Some(teams_table);
        self
    }

    #[must_use]
    /// Sets the team members table to refer to.
    pub fn team_members(mut self, team_members_table: &'a Table) -> Self {
        self.team_members_table = Some(team_members_table);
        self
    }

    #[must_use]
    /// Sets the team projects table to refer to.
    pub fn team_projects(mut self, team_projects_table: &'a Table) -> Self {
        self.team_projects_table = Some(team_projects_table);
        self
    }

    #[must_use]
    /// Adds a new table to the deny list.
    pub fn add_table_to_deny_list(mut self, table: &'a Table) -> Self {
        self.tables_deny_list.push(table);
        self
    }

    #[must_use]
    /// Adds a new extension to the list of extensions to consider
    /// when generating the CHECK constraints validations in the generated code.
    pub fn add_check_constraint_extension(mut self, extension: &'a PgExtension) -> Self {
        self.check_constraints_extensions.push(extension);
        self
    }

    #[must_use]
    /// Sets the output directory for the generated code.
    pub fn set_output_directory(mut self, output_directory: &'a Path) -> Self {
        self.output_directory = Some(output_directory);
        self
    }

    #[must_use]
    /// Whether to generate the diesel joinables.
    pub fn enable_joinables(mut self) -> Self {
        self.enable_joinables = true;
        self
    }

    #[must_use]
    /// Whether to generate the diesel `allow_tables_to_appear_in_same_query`.
    ///
    /// # Note
    /// Since to we need the tables before generating the
    /// `allow_tables_to_appear_in_same_query` we enable the generation of the
    /// tables schema.
    pub fn enable_allow_tables_to_appear_in_same_query(mut self) -> Self {
        self = self.enable_tables_schema();
        self.enable_allow_tables_to_appear_in_same_query = true;
        self
    }

    #[must_use]
    /// Whether to generate the SQL types.
    pub fn enable_sql_types(mut self) -> Self {
        self.enable_sql_types = true;
        self
    }

    #[must_use]
    /// Whether to generate the type structs.
    ///
    /// # Note
    ///
    /// Since the type structs require the SQL types, enabling the
    /// generation of the type structs automatically enables the generation
    /// of the SQL types.
    pub fn enable_type_structs(mut self) -> Self {
        self = self.enable_sql_types();
        self.enable_type_structs = true;
        self
    }

    #[must_use]
    /// Whether to generate the type traits implementations.
    ///
    /// # Note
    ///
    /// Since the type impls are defined on both SQL types and type structs,
    /// enabling the generation of the type impls automatically enables the
    /// generation of the SQL types and the type structs.
    pub fn enable_type_impls(mut self) -> Self {
        self = self.enable_sql_types();
        self = self.enable_type_structs();
        self.enable_type_impls = true;
        self
    }

    #[must_use]
    /// Whether to generate the tables schema.
    ///
    /// # Note
    ///
    /// Since the tables may require some custom types, enabling the
    /// generation of tables automatically enables the generation of SQL types.
    pub fn enable_tables_schema(mut self) -> Self {
        self = self.enable_sql_types();
        self.enable_tables_schema = true;
        self
    }

    #[must_use]
    /// Whether to enable the generation of the table structs.
    ///
    /// # Note
    ///
    /// Since the tables structs require the tables schema, enabling the
    /// generation of the table structs automatically enables the generation
    /// of the tables schema.
    pub fn enable_table_structs(mut self) -> Self {
        self = self.enable_tables_schema();
        self = self.enable_type_impls();
        self = self.enable_allow_tables_to_appear_in_same_query();
        self.enable_table_structs = true;
        self
    }

    #[must_use]
    /// Whether to enable the generation of the [`Deletable`] traits.
    ///
    /// # Note
    ///
    /// Since the [`Deletable`] traits require the tables structs, enabling the
    /// generation of the [`Deletable`] traits automatically enables the
    /// generation of the tables structs.
    pub fn enable_deletable_trait(mut self) -> Self {
        self = self.enable_updatable_trait();
        self.enable_deletable_trait = true;
        self
    }

    #[must_use]
    /// Whether to enable the generation of the Attribute traits.
    ///
    /// # Note
    ///
    /// Since the Attribute traits require the tables structs, enabling the
    /// generation of the Attribute traits automatically enables the generation
    /// of the tables structs.
    pub fn enable_attribute_trait(mut self) -> Self {
        self = self.enable_table_structs();
        self.enable_attribute_trait = true;
        self
    }

    #[must_use]
    /// Whether to enable the generation of the [`Foreign`] traits.
    ///
    /// # Note
    ///
    /// Since the [`Foreign`] traits require the tables structs, enabling the
    /// generation of the [`Foreign`] traits automatically enables the
    /// generation of the tables structs.
    pub fn enable_foreign_trait(mut self) -> Self {
        self = self.enable_table_structs();
        self.enable_foreign_trait = true;
        self
    }

    #[must_use]
    /// Whether to enable the generation of the
    /// [`Upsertable`](web_common_traits::database::Upsertable) traits.
    pub fn enable_upsertable_trait(mut self) -> Self {
        self = self.enable_table_structs();
        self.enable_upsertable_trait = true;
        self
    }

    #[must_use]
    /// Whether to enable the generation of the
    /// [`Insertable`](web_common_traits::database::Insertable) traits.
    ///
    /// # Note
    ///
    /// Since the [`Insertable`](web_common_traits::database::Insertable) traits
    /// require the tables structs, enabling the generation of the
    /// [`Insertable`](web_common_traits::database::Insertable) traits
    /// automatically enables the generation of the tables structs.
    pub fn enable_insertable_trait(mut self) -> Self {
        self = self.enable_updatable_trait();
        self.enable_insertable_trait = true;
        self
    }

    #[must_use]
    /// Whether to enable the generation of the
    /// [`MostConcreteVariant`](web_common_traits::database::MostConcreteVariant) traits.
    ///
    /// # Note
    ///
    /// Since the [`MostConcreteVariant`](web_common_traits::database::MostConcreteVariant) traits
    /// require the tables structs, enabling the generation of the
    /// [`MostConcreteVariant`](web_common_traits::database::MostConcreteVariant) traits
    /// automatically enables the generation of the tables structs.
    pub fn enable_most_concrete_variant_trait(mut self) -> Self {
        self = self.enable_table_structs();
        self.enable_most_concrete_variant_trait = true;
        self
    }

    #[must_use]
    /// Whether to enable the generation of the
    /// [`Updatable`](web_common_traits::database::Updatable) traits.
    ///
    /// # Note
    ///
    /// Since the [`Updatable`](web_common_traits::database::Updatable) traits
    /// require the tables structs, enabling the generation of the
    /// [`Updatable`](web_common_traits::database::Updatable) traits
    /// automatically enables the generation of the tables structs.
    pub fn enable_updatable_trait(mut self) -> Self {
        self = self.enable_foreign_trait();
        self.enable_updatable_trait = true;
        self
    }

    #[must_use]
    /// Whether to enable the generation of the
    /// [`Read`](web_common_traits::crud::Read) traits.
    pub fn enable_read_trait(mut self) -> Self {
        self = self.enable_table_structs();
        self.enable_read_trait = true;
        self
    }

    #[must_use]
    /// Whether to enable the generation of all the CRUD operations.
    pub fn enable_crud_operations(mut self) -> Self {
        self = self.enable_read_trait();
        self
    }

    #[must_use]
    /// Whether to make the code beautified after generation.
    ///
    /// # Note
    ///
    /// This should generally NOT be enabled for production code,
    /// as we do not care about the formatting of the generated code.
    /// It is primarily used for debugging purposes.
    pub fn beautify(mut self) -> Self {
        self.beautify = true;
        self
    }

    /// Dispatches beautification for the provided `TokenStream`, if requested.
    pub(crate) fn beautify_code(&self, code: &TokenStream) -> String {
        if !self.beautify {
            return code.to_string();
        }

        let code_string = code.to_string();

        // Parse the generated code string into a syn::Item
        let syntax_tree: File = syn::parse_str(&code_string).unwrap();

        // Use prettyplease to format the syntax tree
        unparse(&syntax_tree)
    }

    /// Returns the output directory.
    ///
    /// # Errors
    ///
    /// * Raises a `GenerationDirectoryNotProvided` when the output directory is
    ///   not provided.
    pub fn get_output_directory(&self) -> Result<&Path, CodeGenerationError> {
        self.output_directory.ok_or(CodeGenerationError::GenerationDirectoryNotProvided)
    }

    /// Returns the list of required types.
    ///
    /// # Arguments
    ///
    /// * `tables` - A slice of tables to check for custom types.
    /// * `conn` - A mutable reference to a `PgConnection`.
    ///
    /// # Errors
    ///
    /// * Returns an error if the connection to the database fails.
    pub(super) fn required_types(
        tables: &[Table],
        conn: &mut PgConnection,
    ) -> Result<Vec<PgType>, WebCodeGenError> {
        let mut types: Vec<PgType> = Vec::new();

        for table in tables {
            let mut custom_types = Vec::new();

            for column in table.columns(conn)?.iter() {
                if !column.has_custom_type() {
                    continue;
                }
                let column_type = PgType::from_name(column.data_type_str(conn)?, conn)?;
                if column_type.is_enum() || column_type.is_composite() {
                    custom_types.push(column_type);
                }
            }
            let mut additional_custom_types = custom_types.clone();
            for custom_type in custom_types {
                additional_custom_types.extend(custom_type.internal_user_defined_types(conn)?);
            }

            for pg_type in additional_custom_types {
                if pg_type.extension(conn)?.is_none() {
                    types.push(pg_type);
                }
            }
        }
        types.sort_unstable();
        types.dedup();

        Ok(types)
    }

    /// Writes all the tables syn version to a file.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`.
    /// * `table_catalog` - The name of the table catalog.
    ///
    /// # Errors
    ///
    /// * Returns an error if the output path is not provided.
    /// * Returns an error if the tables cannot be loaded.
    /// * Returns an error if the tables cannot be converted to schema.
    /// * Returns an error if the tables cannot be converted to syn.
    /// * Returns an error if the user defined types cannot be loaded.
    /// * Returns an error if the user defined types cannot be converted to syn.
    /// * Returns an error if the generated code cannot be written to the output
    ///   file.
    pub fn generate(
        &mut self,
        conn: &mut PgConnection,
        table_catalog: &str,
    ) -> Result<TimeTracker, WebCodeGenError> {
        let mut time_tracker = TimeTracker::new("Code generation");

        let task = Task::new("Retrieving tables");

        let mut tables = Table::load_all(conn, table_catalog, "public")?
            .iter()
            .filter(|table| !(table.is_temporary() || table.is_view()))
            .filter(|table| !self.tables_deny_list.contains(table))
            .cloned()
            .collect::<Vec<Table>>();

        tables.sort_unstable();

        time_tracker.add_completed_task(task);

        let task = Task::new("Creating table extension network");
        self.table_extension_network =
            Some(TableExtensionNetwork::from_tables(conn, tables.clone()));
        time_tracker.add_completed_task(task);

        let codegen_directory = self.get_output_directory()?.join(CODEGEN_DIRECTORY);
        std::fs::create_dir_all(&codegen_directory)?;
        let codegen_module = codegen_directory.with_extension("rs");

        time_tracker.extend(self.generate_diesel_code(
            codegen_directory.as_path().join(CODEGEN_DIESEL_MODULE).as_path(),
            &tables,
            conn,
        )?);

        time_tracker.extend(self.generate_structs_code(
            codegen_directory.as_path().join(CODEGEN_STRUCTS_MODULE).as_path(),
            &tables,
            conn,
        )?);

        time_tracker.extend(self.generate_web_common_traits_implementations(
            codegen_directory.as_path().join(CODEGEN_TRAITS_MODULE).as_path(),
            &tables,
            conn,
        )?);

        let diesel_codegen_ident =
            syn::Ident::new(CODEGEN_DIESEL_MODULE, proc_macro2::Span::call_site());
        let structs_codegen_ident =
            syn::Ident::new(CODEGEN_STRUCTS_MODULE, proc_macro2::Span::call_site());
        let traits_codegen_ident =
            syn::Ident::new(CODEGEN_TRAITS_MODULE, proc_macro2::Span::call_site());

        let codegen_module_impl = self.beautify_code(&quote::quote! {
            pub mod #diesel_codegen_ident;

            pub mod #structs_codegen_ident;
            pub use #structs_codegen_ident::*;

            mod #traits_codegen_ident;
        });

        std::fs::write(&codegen_module, codegen_module_impl)?;

        Ok(time_tracker)
    }
}

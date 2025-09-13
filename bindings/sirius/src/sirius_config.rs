use crate::prelude::*;
use crate::traits::Enablable;
use crate::semantic::{CoreParam, ConfigParam};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Action {
    Enable,
    Help,
    Version,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ToolKind {
    Formulas,
    Zodiac,
    Fingerprints,
    Structures,
    Classes,
    WriteSummaries,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ToolAction {
    kind: ToolKind,
    action: Action,
}

/// Struct providing the configuration for Sirius.
///
/// # Implementative details
/// This struct MUST be a private struct. It is only used by the [`SiriusBuilder`](crate::builder::SiriusBuilder) to
/// build the [`Sirius`](crate::sirius::Sirius) struct, and through the builder we can evaluate all of the provided
/// parameters. If we make this struct public, we would allow the user to create a [`Sirius`](crate::sirius::Sirius)
/// struct with invalid parameters. DO NOT MAKE THIS STRUCT PUBLIC.
pub(crate) struct SiriusConfig<V: Version> {
    // legacy version-specific params (kept for compatibility)
    core_parameters: Vec<V::Core>,
    config_parameters: Vec<V::Config>,
    // version-agnostic semantic params
    semantic_core: Vec<CoreParam>,
    semantic_config: Vec<ConfigParam>,
    tool_actions: Vec<ToolAction>,
    post_tool_args: Vec<String>,
}

impl<V: Version> Default for SiriusConfig<V> {
    fn default() -> Self {
        SiriusConfig {
            core_parameters: Vec::new(),
            config_parameters: Vec::new(),
            semantic_core: Vec::new(),
            semantic_config: Vec::new(),
            tool_actions: Vec::new(),
            post_tool_args: Vec::new(),
        }
    }
}

impl<V: Version> SiriusConfig<V> {
    pub(crate) fn add_semantic_core(&mut self, p: CoreParam) -> Result<(), String> {
        if let Some(existing) = self
            .semantic_core
            .iter()
            .find(|e| std::mem::discriminant(*e) == std::mem::discriminant(&p))
        {
            return Err(format!(
                "The core parameter {:?} cannot be added twice (existing: {:?}).",
                p, existing
            ));
        }
        self.semantic_core.push(p);
        Ok(())
    }

    pub(crate) fn add_semantic_config(&mut self, p: ConfigParam) -> Result<(), String> {
        if !matches!(p, ConfigParam::Raw(_)) {
            if let Some(existing) = self
                .semantic_config
                .iter()
                .find(|e| std::mem::discriminant(*e) == std::mem::discriminant(&p))
            {
                return Err(format!(
                    "The config parameter {:?} cannot be added twice (existing: {:?}).",
                    p, existing
                ));
            }
        }
        self.semantic_config.push(p);
        Ok(())
    }
    /// Add a parameter to the core configuration.
    ///
    /// # Arguments
    ///
    /// * `parameter` - The parameter to add.
    ///
    pub fn add_core_parameter(&mut self, parameter: V::Core) -> Result<(), String> {
        // We check if the parameter is already present in the vector
        // If it is, we return an error
        if let Some(existing_parameter) = self
            .core_parameters
            .iter()
            .find(|&p| std::mem::discriminant(p) == std::mem::discriminant(&parameter))
        {
            Err(format!(
                concat!(
                    "The core parameter {:?} cannot be added to the configuration. ",
                    "There is already an existing parameter which is {:?}. ",
                    "You cannot add it twice."
                ),
                parameter, existing_parameter
            ))
        } else {
            self.core_parameters.push(parameter);
            Ok(())
        }
    }

    /// Add a parameter to the config configuration.
    ///
    /// # Arguments
    ///
    /// * `parameter` - The parameter to add.
    ///
    pub fn add_config_parameter(&mut self, parameter: V::Config) -> Result<(), String> {
        // We check if the parameter is already present in the vector
        // If it is, we return an error
        if let Some(existing_parameter) = self
            .config_parameters
            .iter()
            .find(|&p| std::mem::discriminant(p) == std::mem::discriminant(&parameter))
        {
            Err(format!(
                concat!(
                    "The config parameter {:?} cannot be added to the configuration. ",
                    "There is already an existing parameter which is {:?}. ",
                    "You cannot add it twice."
                ),
                parameter, existing_parameter
            ))
        } else {
            if !parameter.is_enabler() {
                // If the current parameter is not an enabler, we make sure that the enabler variant
                // is present in the vector by trying to insert it without checking if it is already
                // present.
                let _ = self.add_config_parameter(V::Config::enabler());
            }
            self.config_parameters.push(parameter);
            Ok(())
        }
    }

    /// Add a parameter to the formula configuration.
    ///
    /// # Arguments
    ///
    /// * `parameter` - The parameter to add.
    ///
    pub fn add_formula_parameter(&mut self, parameter: V::Formula) -> Result<(), String> {
        let action = match parameter.to_string().as_str() {
            "--help" => ToolAction { kind: ToolKind::Formulas, action: Action::Help },
            "--version" => ToolAction { kind: ToolKind::Formulas, action: Action::Version },
            _ => ToolAction { kind: ToolKind::Formulas, action: Action::Enable },
        };
        if self.tool_actions.iter().any(|a| *a == action) {
            return Err(format!(
                "The formula parameter {:?} cannot be added twice.",
                parameter
            ));
        }
        self.tool_actions.push(action);
        Ok(())
    }

    /// Add a parameter to the zodiac configuration.
    ///
    /// # Arguments
    ///
    /// * `parameter` - The parameter to add.
    ///
    pub fn add_zodiac_parameter(&mut self, parameter: V::Zodiac) -> Result<(), String> {
        let action = match parameter.to_string().as_str() {
            "--help" => ToolAction { kind: ToolKind::Zodiac, action: Action::Help },
            "--version" => ToolAction { kind: ToolKind::Zodiac, action: Action::Version },
            _ => ToolAction { kind: ToolKind::Zodiac, action: Action::Enable },
        };
        if self.tool_actions.iter().any(|a| *a == action) {
            return Err(format!(
                "The zodiac parameter {:?} cannot be added twice.",
                parameter
            ));
        }
        self.tool_actions.push(action);
        Ok(())
    }

    /// Add a parameter to the fingerprint configuration.
    ///
    /// # Arguments
    ///
    /// * `parameter` - The parameter to add.
    ///
    pub fn add_fingerprint_parameter(&mut self, parameter: V::Fingerprint) -> Result<(), String> {
        let action = match parameter.to_string().as_str() {
            "--help" => ToolAction { kind: ToolKind::Fingerprints, action: Action::Help },
            "--version" => ToolAction { kind: ToolKind::Fingerprints, action: Action::Version },
            _ => ToolAction { kind: ToolKind::Fingerprints, action: Action::Enable },
        };
        if self.tool_actions.iter().any(|a| *a == action) {
            return Err(format!(
                "The fingerprint parameter {:?} cannot be added twice.",
                parameter
            ));
        }
        self.tool_actions.push(action);
        Ok(())
    }

    /// Add a parameter to the structure configuration.
    ///
    /// # Arguments
    ///
    /// * `parameter` - The parameter to add.
    ///
    pub fn add_structure_parameter(&mut self, parameter: V::Structure) -> Result<(), String> {
        let action = match parameter.to_string().as_str() {
            "--help" => ToolAction { kind: ToolKind::Structures, action: Action::Help },
            "--version" => ToolAction { kind: ToolKind::Structures, action: Action::Version },
            _ => ToolAction { kind: ToolKind::Structures, action: Action::Enable },
        };
        if self.tool_actions.iter().any(|a| *a == action) {
            return Err(format!(
                "The structure parameter {:?} cannot be added twice.",
                parameter
            ));
        }
        self.tool_actions.push(action);
        Ok(())
    }

    /// Add a parameter to the canopus configuration.
    ///
    /// # Arguments
    ///
    /// * `parameter` - The parameter to add.
    ///
    pub fn add_canopus_parameter(&mut self, parameter: V::Canopus) -> Result<(), String> {
        let action = match parameter.to_string().as_str() {
            "--help" => ToolAction { kind: ToolKind::Classes, action: Action::Help },
            "--version" => ToolAction { kind: ToolKind::Classes, action: Action::Version },
            _ => ToolAction { kind: ToolKind::Classes, action: Action::Enable },
        };
        if self.tool_actions.iter().any(|a| *a == action) {
            return Err(format!(
                "The canopus/classes parameter {:?} cannot be added twice.",
                parameter
            ));
        }
        self.tool_actions.push(action);
        Ok(())
    }

    /// Add a parameter to the write_summaries configuration.
    ///
    /// # Arguments
    ///
    /// * `parameter` - The parameter to add.
    ///
    pub fn add_write_summaries_parameter(
        &mut self,
        parameter: V::WriteSummaries,
    ) -> Result<(), String> {
        let action = match parameter.to_string().as_str() {
            "--help" => ToolAction { kind: ToolKind::WriteSummaries, action: Action::Help },
            "--version" => ToolAction { kind: ToolKind::WriteSummaries, action: Action::Version },
            _ => ToolAction { kind: ToolKind::WriteSummaries, action: Action::Enable },
        };
        if self.tool_actions.iter().any(|a| *a == action) {
            return Err(format!(
                "The write-summaries parameter {:?} cannot be added twice.",
                parameter
            ));
        }
        self.tool_actions.push(action);
        Ok(())
    }

    // Version-agnostic tool API (public within crate) to simplify usage/tests.
    pub(crate) fn enable_formulas(&mut self) -> Result<(), String> {
        self.push_tool(ToolKind::Formulas, Action::Enable)
    }

    pub(crate) fn enable_zodiac(&mut self) -> Result<(), String> {
        self.push_tool(ToolKind::Zodiac, Action::Enable)
    }

    pub(crate) fn enable_fingerprints(&mut self) -> Result<(), String> {
        self.push_tool(ToolKind::Fingerprints, Action::Enable)
    }

    pub(crate) fn enable_structures(&mut self) -> Result<(), String> {
        self.push_tool(ToolKind::Structures, Action::Enable)
    }

    pub(crate) fn enable_classes(&mut self) -> Result<(), String> {
        self.push_tool(ToolKind::Classes, Action::Enable)
    }

    pub(crate) fn enable_write_summaries(&mut self) -> Result<(), String> {
        self.push_tool(ToolKind::WriteSummaries, Action::Enable)
    }

    fn push_tool(&mut self, kind: ToolKind, action: Action) -> Result<(), String> {
        let ta = ToolAction { kind, action };
        if self.tool_actions.iter().any(|x| *x == ta) {
            return Err("Tool already enabled".to_string());
        }
        self.tool_actions.push(ta);
        Ok(())
    }

    pub fn args(&self) -> Vec<String> {
        let mut out: Vec<String> = Vec::new();
        // core flags first (legacy + semantic)
        for p in &self.core_parameters {
            out.push(p.to_string());
        }
        for p in &self.semantic_core {
            self.encode_core(V::VERSION, p, &mut out);
        }
        // config (legacy + semantic)
        let mut config_emitted = false;
        for p in &self.config_parameters {
            let s = p.to_string();
            if s == "config" { config_emitted = true; }
            out.push(s);
        }
        if !self.semantic_config.is_empty() && !config_emitted {
            out.push("config".to_string());
            config_emitted = true;
        }
        for p in &self.semantic_config {
            self.encode_config(V::VERSION, p, &mut out);
        }
        // tools in insertion order, version-specific names
        for t in &self.tool_actions {
            let tool_name = match (V::VERSION, t.kind) {
                (5, ToolKind::Formulas) => "formula",
                (5, ToolKind::Zodiac) => "zodiac",
                (5, ToolKind::Fingerprints) => "fingerprint",
                (5, ToolKind::Structures) => "structure",
                (5, ToolKind::Classes) => "canopus",
                (5, ToolKind::WriteSummaries) => "write-summaries",
                (_, ToolKind::Formulas) => "formulas",
                (_, ToolKind::Zodiac) => "zodiac",
                (_, ToolKind::Fingerprints) => "fingerprints",
                (_, ToolKind::Structures) => "structures",
                (_, ToolKind::Classes) => "classes",
                (_, ToolKind::WriteSummaries) => "write-summaries",
            };
            out.push(tool_name.to_string());
            match t.action {
                Action::Enable => {}
                Action::Help => out.push("--help".to_string()),
                Action::Version => out.push("--version".to_string()),
            }
        }
        // append raw post-tool args
        out.extend(self.post_tool_args.iter().cloned());
        out
    }

    fn encode_core(&self, version: usize, p: &CoreParam, out: &mut Vec<String>) {
        match (version, p) {
            (5, CoreParam::MaxMz(m)) => out.push(format!("--maxmz={}", m)),
            (_, CoreParam::MaxMz(m)) => out.push(format!("--mzmax={}", m)),
        }
    }

    fn encode_config(&self, version: usize, p: &ConfigParam, out: &mut Vec<String>) {
        match (version, p) {
            (_, ConfigParam::IsotopeFilter(b)) => out.push(format!("--IsotopeSettings.filter={}", b)),
            (_, ConfigParam::FormulaDb(db)) => out.push(format!("--FormulaSearchDB={}", db)),
            (_, ConfigParam::StructureDb(db)) => out.push(format!("--StructureSearchDB={}", db)),
            (_, ConfigParam::NumberOfCandidates(n)) => out.push(format!("--NumberOfCandidates={}", n)),
            (5, ConfigParam::NumberOfCandidatesPerIonization(n)) => out.push(format!("--NumberOfCandidatesPerIon={}", n)),
            (_, ConfigParam::NumberOfCandidatesPerIonization(n)) => out.push(format!("--NumberOfCandidatesPerIonization={}", n)),
            (_, ConfigParam::FormulaResultThreshold(b)) => out.push(format!("--FormulaResultThreshold={}", b)),
            (_, ConfigParam::MedianNoiseIntensity(v)) => out.push(format!("--MedianNoiseIntensity={}", v)),
            (5, ConfigParam::Ms1IsotopicIntensity { abs, min, rel }) => {
                if let Some(v) = abs { out.push(format!("--ms1.absoluteIntensityError={}", v)); }
                if let Some(v) = min { out.push(format!("--ms1.minimalIntensityToConsider={}", v)); }
                if let Some(v) = rel { out.push(format!("--ms1.relativeIntensityError={}", v)); }
            }
            (_, ConfigParam::Ms1IsotopicIntensity { abs, min, rel }) => {
                if let Some(v) = abs { out.push(format!("--IsotopicIntensitySettings.absoluteIntensityError={}", v)); }
                if let Some(v) = min { out.push(format!("--IsotopicIntensitySettings.minimalIntensityToConsider={}", v)); }
                if let Some(v) = rel { out.push(format!("--IsotopicIntensitySettings.relativeIntensityError={}", v)); }
            }
            (5, ConfigParam::Heuristic { use_above, only_above }) => {
                if let Some(v) = use_above { out.push(format!("--UseHeuristic.mzToUseHeuristic={}", v)); }
                if let Some(v) = only_above { out.push(format!("--UseHeuristic.mzToUseHeuristicOnly={}", v)); }
            }
            (_, ConfigParam::Heuristic { use_above, only_above }) => {
                if let Some(v) = use_above { out.push(format!("--UseHeuristic.useHeuristicAboveMz={}", v)); }
                if let Some(v) = only_above { out.push(format!("--UseHeuristic.useOnlyHeuristicAboveMz={}", v)); }
            }
            (_, ConfigParam::TimeoutPerTree(v)) => out.push(format!("--Timeout.secondsPerTree={}", v)),
            (_, ConfigParam::TimeoutPerInstance(v)) => out.push(format!("--Timeout.secondsPerInstance={}", v)),
            (_, ConfigParam::Ms1Allowed(d)) => out.push(format!("--MS1MassDeviation.allowedMassDeviation={}", d)),
            (_, ConfigParam::Ms2Allowed(d)) => out.push(format!("--MS2MassDeviation.allowedMassDeviation={}", d)),
            (_, ConfigParam::Raw(s)) => out.push(s.clone()),
            // new typed lists
            (_, ConfigParam::SpectralSearchDb(dbs)) => {
                if !dbs.is_empty() {
                    let list = dbs.iter().map(|d| d.to_string()).collect::<Vec<_>>().join(",");
                    out.push(format!("--SpectralSearchDB={}", list));
                }
            }
            (_, ConfigParam::FormulaDbList(dbs)) => {
                let list = dbs.iter().map(|d| d.to_string()).collect::<Vec<_>>().join(",");
                out.push(format!("--FormulaSearchDB={}", list));
            }
            (_, ConfigParam::StructureDbList(dbs)) => {
                let list = dbs.iter().map(|d| d.to_string()).collect::<Vec<_>>().join(",");
                out.push(format!("--StructureSearchDB={}", list));
            }
            (5, ConfigParam::IdentitySearchPrecursorDeviation(_)) => {}
            (_, ConfigParam::IdentitySearchPrecursorDeviation(dev)) => {
                out.push(format!("--IdentitySearchSettings.precursorDeviation={}", dev));
            }
            (5, ConfigParam::FormulaSearchPerformBottomUpAboveMz(_)) => {}
            (_, ConfigParam::FormulaSearchPerformBottomUpAboveMz(v)) => {
                out.push(format!("--FormulaSearchSettings.performBottomUpAboveMz={}", v));
            }
        }
    }

    pub(crate) fn push_post_tool_arg(&mut self, s: String) {
        self.post_tool_args.push(s);
    }
}

impl<V: Version> ToString for SiriusConfig<V> {
    fn to_string(&self) -> String {
        self.args().join(" ")
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_sirius_config() {
        let mut config: SiriusConfig<Version5> = SiriusConfig::default();
        config
            .add_config_parameter(ConfigV5::IsotopeSettingsFilter(true))
            .unwrap();
        config
            .add_config_parameter(ConfigV5::FormulaSearchDB(SearchDB::Bio))
            .unwrap();

        assert!(config
            .add_config_parameter(ConfigV5::IsotopeSettingsFilter(true))
            .is_err());

        assert!(config
            .add_config_parameter(ConfigV5::FormulaSearchDB(SearchDB::Bio))
            .is_err());
    }

    #[test]
    fn test_tool_names_v5() {
        let mut cfg: SiriusConfig<Version5> = SiriusConfig::default();
        cfg.enable_formulas().unwrap();
        cfg.enable_zodiac().unwrap();
        cfg.enable_fingerprints().unwrap();
        cfg.enable_structures().unwrap();
        cfg.enable_classes().unwrap();
        cfg.enable_write_summaries().unwrap();

        let args = cfg.args();
        // in V5 the tool names are singular except write-summaries
        assert_eq!(
            args,
            vec![
                "formula".to_string(),
                "zodiac".to_string(),
                "fingerprint".to_string(),
                "structure".to_string(),
                "canopus".to_string(),
                "write-summaries".to_string(),
            ]
        );
    }

    #[test]
    fn test_tool_names_v6() {
        let mut cfg: SiriusConfig<Version6> = SiriusConfig::default();
        cfg.enable_formulas().unwrap();
        cfg.enable_zodiac().unwrap();
        cfg.enable_fingerprints().unwrap();
        cfg.enable_structures().unwrap();
        cfg.enable_classes().unwrap();
        cfg.enable_write_summaries().unwrap();

        let args = cfg.args();
        // in V6 the tool names are plural (classes instead of canopus)
        assert_eq!(
            args,
            vec![
                "formulas".to_string(),
                "zodiac".to_string(),
                "fingerprints".to_string(),
                "structures".to_string(),
                "classes".to_string(),
                "write-summaries".to_string(),
            ]
        );
    }
}

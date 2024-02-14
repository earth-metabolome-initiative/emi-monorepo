# Sirius
[![Build status](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/sirius.yml/badge.svg)](https://github.com/earth-metabolome-initiative/emi-monorepo/actions)
[![Crates.io](https://img.shields.io/crates/v/sirius-bindings.svg)](https://crates.io/crates/sirius-bindings)
[![Documentation](https://docs.rs/sirius-bindings/badge.svg)](https://docs.rs/sirius-bindings)

SIRIUS is a computational mass spectrometry Java-based software framework for the analysis of LC-MS/MS metabolomics data sets and the annotation of metabolites and other small molecules of biological interest. SIRIUS integrates a collection of tools, including CSI:FingerID (with [COSMIC](https://bio.informatik.uni-jena.de/software/cosmic/), [ZODIAC](https://bio.informatik.uni-jena.de/software/zodiac/) and [CANOPUS](https://bio.informatik.uni-jena.de/software/canopus/).

For further reading please refer to the official [Sirius website](https://bio.informatik.uni-jena.de/software/sirius/). 

## Installation
Since version 5.7.0 SIRIUS is officially available via conda ([conda-forge](https://conda-forge.org/)) under the package name [sirius-ms](https://anaconda.org/conda-forge/sirius-ms). Native MacOS arm64 (Apple Silicon) builds are solely available via conda.

Additionally, you can install Sirius via their [GitHub repository](https://github.com/boecker-lab/sirius). 

# Sirius bindings
Here we present a binding for Sirius in [Rust](https://www.rust-lang.org/). This binding is a wrapper around the Sirius command line interface (CLI) and provides a more user-friendly interface for running Sirius. It also provides a safer way to run Sirius by using type safety and error handling before running Sirius executable.

## Usage
First you need to have Sirius installed in your system. Then you also need the following variables in your `.env` file: 
```bash
SIRIUS_PATH=/path/to/sirius_executable (on macOS it should be something like `/Applications/sirius.app/Contents/MacOS/sirius`)
SIRIUS_USERNAME=your_username
SIRIUS_PASSWORD=your_password
```

Then you can use the Sirius binding in your Rust project. To do so add this to your `Cargo.toml`:
```toml
[dependencies]
sirius = "0.1"
```
and this to your crate root:
```rust
use sirius_bindings::prelude::*;
```

## Examples
Here is an example of running Sirius with the default parameters:
```bash
sirius -i tests/data/input_sirius.mgf --output tests/data/output_sirius_default --maxmz=800.0 formula zodiac fingerprint structure canopus write-summaries
```

The equivalent Rust code is:
```rust
use sirius_bindings::prelude::*;
use std::path::Path;
let sirius = SiriusBuilder::<Version5>::default()
    .maximal_mz_default().unwrap()
    .max_cpus_default().unwrap()
    .enable_formula().unwrap()
    .enable_zodiac().unwrap()
    .enable_fingerprint().unwrap()
    .enable_structure().unwrap()
    .enable_canopus().unwrap()
    .enable_write_summaries().unwrap()
    .build();
let input_file_path = Path::new("tests/data/input_sirius.mgf");
let output_file_path = Path::new("tests/data/output_sirius_default");
// Check if the path exists before attempting to remove it
if output_file_path.exists() {
    let _ = std::fs::remove_dir_all(output_file_path);
}
sirius.run(input_file_path, output_file_path).unwrap();
```

You can also be more specific and add other parameters. The following example uses the parameters used for the [ENPKG pipeline](https://github.com/enpkg/enpkg_full/blob/c8e649290ee72f000c3385e7669b5da2215abad8/params/user.yml#L60):

```bash 
sirius -i tests/data/input_sirius.mgf --output tests/data/output_sirius --maxmz 800 \
config --IsotopeSettings.filter=true --FormulaSearchDB=BIO --Timeout.secondsPerTree=0 \
--FormulaSettings.enforced=H,C,N,O,P --Timeout.secondsPerInstance=0 \
--AdductSettings.detectable='[[M+H]+,[M-H4O2+H]+,[M+Na]+,[M+K]+,[M+H3N+H]+,[M-H2O+H]+]' \
--UseHeuristic.mzToUseHeuristicOnly=650 --AlgorithmProfile=orbitrap --IsotopeMs2Settings=IGNORE \
--MS2MassDeviation.allowedMassDeviation=5.0ppm --NumberOfCandidatesPerIon=1 \
--UseHeuristic.mzToUseHeuristic=300 --FormulaSettings.detectable=B,Cl,Br,Se,S \
--NumberOfCandidates=10 --ZodiacNumberOfConsideredCandidatesAt300Mz=10 \
--ZodiacRunInTwoSteps=true --ZodiacEdgeFilterThresholds.minLocalConnections=10 \
--ZodiacEdgeFilterThresholds.thresholdFilter=0.95 --ZodiacEpochs.burnInPeriod=2000 \
--ZodiacEpochs.numberOfMarkovChains=10 --ZodiacNumberOfConsideredCandidatesAt800Mz=50 \
--ZodiacEpochs.iterations=20000 --AdductSettings.enforced=, \
--AdductSettings.fallback='[[M+H]+,[M+Na]+,[M+K]+]' --FormulaResultThreshold=true \
--InjectElGordoCompounds=true --StructureSearchDB=BIO \
--RecomputeResults=false formula zodiac fingerprint structure canopus write-summaries
```

The equivalent Rust code is:
```rust
use sirius_bindings::prelude::*;
use std::path::Path;
let sirius = SiriusBuilder::default()
    .maximal_mz(800.0).unwrap()
    .max_cpus_default().unwrap()
    .isotope_settings_filter(true).unwrap()
    .formula_search_db(DBVector::from(vec![SearchDB::Bio])).unwrap()
    .timeout_seconds_per_tree(0).unwrap()
    .formula_settings_enforced(AtomVector::from(vec![
        Atoms::H,
        Atoms::C,
        Atoms::N,
        Atoms::O,
        Atoms::P,
    ])).unwrap()
    .timeout_seconds_per_instance(0).unwrap()
    .adduct_settings_detectable(AdductsVector::from(vec![
        Adducts::MplusHplus,
        Adducts::MplusHminusTwoH2Oplus,
        Adducts::MplusNaplus,
        Adducts::MplusKplus,
        Adducts::MplusH3NplusHplus,
        Adducts::MplusHminusH2Oplus,
    ])).unwrap()
    .use_heuristic_mz_to_use_heuristic_only(650).unwrap()
    .algorithm_profile(Instruments::Orbitrap).unwrap()
    .isotope_ms2_settings(IsotopeMS2Settings::Ignore).unwrap()
    .ms2_mass_deviation_allowed_mass_deviation(MassDeviation::Ppm(5.0)).unwrap()
    .number_of_candidates_per_ion(1).unwrap()
    .use_heuristic_mz_to_use_heuristic(300).unwrap()
    .formula_settings_detectable(AtomVector::from(vec![
        Atoms::B,
        Atoms::Cl,
        Atoms::Se,
        Atoms::S,
    ])).unwrap()
    .number_of_candidates(10).unwrap()
    .zodiac_number_of_considered_candidates_at_300_mz(10).unwrap()
    .zodiac_run_in_two_steps(true).unwrap()
    .zodiac_edge_filter_thresholds_min_local_connections(10).unwrap()
    .zodiac_edge_filter_thresholds_threshold_filter(0.95).unwrap()
    .zodiac_epochs_burn_in_period(2000).unwrap()
    .zodiac_epochs_number_of_markov_chains(10).unwrap()
    .zodiac_number_of_considered_candidates_at_800_mz(50).unwrap()
    .zodiac_epochs_iterations(20000).unwrap()
    .adduct_settings_enforced_default().unwrap()
    .adduct_settings_fallback(AdductsVector::from(vec![
        Adducts::MplusHplus,
        Adducts::MplusNaplus,
        Adducts::MplusKplus,
    ])).unwrap()
    .formula_result_threshold(true).unwrap()
    .inject_el_gordo_compounds(true).unwrap()
    .structure_search_db(DBVector::from(vec![SearchDB::Bio])).unwrap()
    .recompute_results(false).unwrap()
    .enable_formula().unwrap()
    .enable_zodiac().unwrap()
    .enable_fingerprint().unwrap()
    .enable_structure().unwrap()
    .enable_canopus().unwrap()
    .enable_write_summaries().unwrap()
    .build();

let input_file_path = Path::new("tests/data/input_sirius.mgf");
let output_file_path = Path::new("tests/data/output_sirius");
// Check if the path exists before attempting to remove it
if output_file_path.exists() {
    let _ = std::fs::remove_dir_all(output_file_path);
}
sirius.run(input_file_path, output_file_path).unwrap();
```

## Error cases
This binding also provides error handling before running the Sirius CLI. 

The following example will throw an error because the `maximal_mz` is added twice:
```should_panic
use sirius_bindings::prelude::*;
use std::path::Path;
let sirius = SiriusBuilder::<Version5>::default()
    .maximal_mz_default().unwrap()
    .maximal_mz(70.6).unwrap()
    .enable_formula().unwrap()
    .enable_zodiac().unwrap()
    .enable_fingerprint().unwrap()
    .enable_structure().unwrap()
    .enable_canopus().unwrap()
    .enable_write_summaries().unwrap()
    .build();
let input_file_path = Path::new("tests/data/input_sirius.mgf");
let output_file_path = Path::new("tests/data/output_sirius_default");
// Check if the path exists before attempting to remove it
if output_file_path.exists() {
    let _ = std::fs::remove_dir_all(output_file_path);
}
sirius.run(input_file_path, output_file_path).unwrap();
```

Will result in the following error:
```bash
Error: "The core parameter MaximalMz(70.6) cannot be added to the configuration. There is already an existing parameter which is MaximalMz(800.0). You cannot add it twice."
```

## Limitations
For now some *config* parameters are not fully implemented and only the default values are used. 

If you are interested in looking at the default values you can either run `sirius config --help`. Here we present is a non-exhaustive list of the parameters where only the default values can be used:
* **PossibleAdductsSwitches** default is `[M+Na]+:[M+H]+,[M+K]+:[M+H]+,[M+Cl]-:[M-H]-`
* **AdductSettingsEnforced** default is `,`
* **FormulaResultRankingScore** default is `AUTO`
* **IsotopeMS2Settings** default is `IGNORE`
* **NoiseThresholdSettingsBasePeak** default is `NOT_PRECURSOR`
* **Adducts** don't have default, but some adducts are probably not included in the enumeration.

In the future, we will add the possibility to add custom values for these parameters. In case you need to add custom values for these parameters, do not hesitate to open an issue or a pull request.

## Documentation
You can find the documentation for the Sirius binding by running the following command in the root of the repository:
```bash
cargo doc --open
```

## Fuzzing
Fuzzing is a technique for finding security vulnerabilities and bugs in software by providing random input to the code. It can be an effective way of uncovering issues that might not be discovered through other testing methods. In our library, we take fuzzing seriously, and we use the [cargo fuzz](https://github.com/rust-fuzz/cargo-fuzz) tool to ensure our code is robust and secure. cargo fuzz automates the process of generating and running randomized test inputs, and it can help identify obscure bugs that would be difficult to detect through traditional testing methods. We make sure that our fuzz targets are continuously updated and run against the latest versions of the library to ensure that any vulnerabilities or bugs are quickly identified and addressed. 

You can learn more about fuzzing [here](https://github.com/earth-metabolome-initiative/emi-monorepo/tree/sirius-bindings/bindings/sirius/fuzz). 

## Citing Sirius

```bib
Kai Dührkop, Markus Fleischauer, Marcus Ludwig, Alexander A. Aksenov, Alexey V. Melnik, Marvin Meusel, Pieter C. Dorrestein, Juho Rousu, and Sebastian Böcker, 
[SIRIUS 4: Turning tandem mass spectra into metabolite structure information.](https://doi.org/10.1038/s41592-019-0344-8)
*Nature Methods* 16, 299–302, 2019.
```
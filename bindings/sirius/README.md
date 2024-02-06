# Sirius
SIRIUS is a Java software for analyzing metabolites from tandem mass spectrometry data. 
It combines the analysis of isotope patterns in MS spectra with the analysis of fragmentation patterns in MS/MS spectra,
and uses CSI:FingerID as a web service to search in molecular structure databases.
Further it integrates CANOPUS for de novo compound class prediction.

SIRIUS requires **high mass accuracy data**. The mass deviation of your MS and MS/MS spectra should be within 20 ppm. Mass Spectrometry instruments such as TOF, Orbitrap and FT-ICR usually provide high mass accuracy data, as well as coupled instruments like Q-TOF, IT-TOF or IT-Orbitrap. Spectra measured with a quadrupole or linear trap do not provide the high mass accuracy that is required for our method.

SIRIUS expects MS and MS/MS spectra as input. It is possible to omit the MS data, but it will make the analysis more time consuming and might give you worse results. In this case, you should consider limiting the candidate molecular formulas to those found in PubChem.
# Sirius binding
## Usage
Add this to your `Cargo.toml`:
```toml
[dependencies]
sirius = "0.1"
```
and this to your crate root:
```rust
use sirius::prelude::*;
```

## Examples
In case you have an MGF file you can run Sirius as follows:
```rust
use sirius::prelude::*;
use std::path::Path;
let sirius = SiriusBuilder::<Version5>::default()
    .maximal_mz_default().unwrap()
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

Or with more options/parameters (the example below uses the parameters used for the ENPKG pipeline):
```rust
use sirius::prelude::*;
use std::path::Path;
let sirius = SiriusBuilder::default()
    .maximal_mz(800.0).unwrap()
    .isotope_settings_filter(true).unwrap()
    .formula_search_db(SearchDB::Bio).unwrap()
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
    .structure_search_db(SearchDB::Bio).unwrap()
    .recompute_results(false).unwrap()
    .enable_formula().unwrap()
    .enable_zodiac().unwrap()
    .enable_fingerprint().unwrap()
    .enable_structure().unwrap()
    .enable_canopus().unwrap()
    .build();

let input_file_path = Path::new("tests/data/input_sirius.mgf");
let output_file_path = Path::new("tests/data/output_sirius");
// Check if the path exists before attempting to remove it
if output_file_path.exists() {
    let _ = std::fs::remove_dir_all(output_file_path);
}
sirius.run(input_file_path, output_file_path).unwrap();
```

You can replace the `input_file_path` and `output_file_path` with your own paths.

<!--begin cite-->
# Citing Sirius

Kai Dührkop, Markus Fleischauer, Marcus Ludwig, Alexander A. Aksenov, Alexey V. Melnik, Marvin Meusel, Pieter C. Dorrestein, Juho Rousu, and Sebastian Böcker, 
[SIRIUS 4: Turning tandem mass spectra into metabolite structure information.](https://doi.org/10.1038/s41592-019-0344-8)
*Nature Methods* 16, 299–302, 2019.
<!--end cite-->

## Sirius config 

```bash
Usage: sirius config [-hV]
                     [COMMAND]
<CONFIGURATION> Override all possible default configurations of this toolbox
from the command line.
      --AdductSettings.detectable=[M+H]+,[M+K]+,[M+Na]+,[M+H-H2O]+,[M+H-H4O2]+,
        [M+NH4]+,[M-H]-,[M+Cl]-,[M-H2O-H]-,[M+Br]-
                           Detectable ion modes which are only considered if
                             there is an indication in the MS1 scan (e.g.
                           correct mass delta).
      --AdductSettings.enforced=,
                           Describes how to deal with Adducts:
                           Pos Examples:
                           [M+H]+,[M]+,[M+K]+,[M+Na]+,[M+H-H2O]+,[M+Na2-H]+,
                             [M+2K-H]+,[M+NH4]+,[M+H3O]+,[M+MeOH+H]+,[M+ACN+H]+,
                             [M+2ACN+H]+,[M+IPA+H]+,[M+ACN+Na]+,[M+DMSO+H]+
                           Neg Examples:
                           [M-H]-,[M]-,[M+K-2H]-,[M+Cl]-,[M-H2O-H]-,[M+Na-2H]-,
                             M+FA-H]-,[M+Br]-,[M+HAc-H]-,[M+TFA-H]-,[M+ACN-H]-
                           Enforced ion modes that are always considered.
      --AdductSettings.fallback=[M+H]+,[M-H]-,[M+Na]+,[M+K]+
                           Fallback ion modes which are considered if the auto
                             detection did not find any indication for
                           an ion mode.
      --AlgorithmProfile=default
                           Configuration profile to store instrument specific
                             algorithm properties.
                           Some of the default profiles are: 'qtof',
                             'orbitrap', 'fticr'.
      --CandidateFormulas=,
                           This configuration holds a set of user given
                             formulas to be used as candidates for SIRIUS
                           Note: This set might be merged with other sources
                             like formulas from databases
                           Set of Molecular Formulas to be used as candidates
                             for molecular formula estimation with
                           SIRIUS
      --CompoundQuality=UNKNOWN
                           Keywords that can be assigned to a input spectrum to
                             judge its quality. Available keywords
                           are: Good, LowIntensity, NoMS1Peak, FewPeaks,
                             Chimeric, NotMonoisotopicPeak,
                           PoorlyExplained
      --ForbidRecalibration=ALLOWED
                           Enable/Disable the hypothesen driven recalibration
                             of MS/MS spectra
                           Must be either 'ALLOWED' or FORBIDDEN'
      --FormulaResultRankingScore=AUTO
                           Allows the USER to Specify the ScoreType that is
                             used to rank the list of Molecular Formula
                           Identifications
                           before CSI:FingerID predictions are calculated. Auto
                             means that this ScoreType is
                           automatically set depending on the executed workflow.
      --FormulaResultThreshold=true
                           Specifies if the list of Molecular Formula
                             Identifications is filtered by a soft threshold
                           (calculateThreshold) before CSI:FingerID predictions
                             are calculated.
      --FormulaSearchDB=none

      --FormulaSettings.detectable=S,Br,Cl,B,Se
                           Detectable elements are added to the chemical
                             alphabet, if there are indications for them
                           (e.g. in isotope pattern)
      --FormulaSettings.enforced=C,H,N,O,P
                           These configurations hold the information how to
                             autodetect elements based on the given
                           formula constraints.
                           Note: If the compound is already assigned to a
                             specific molecular formula, this annotation is
                           ignored.
                           Enforced elements are always considered
      --FormulaSettings.fallback=S
                           Fallback elements are used, if the auto-detection
                             fails (e.g. no isotope pattern available)
  -h, --help               Show this help message and exit.
      --InjectElGordoCompounds=True
                           Candidates matching the lipid class estimated by El
                             Gordo will be tagged.
                           The lipid class will only be available if El Gordo
                             predicts that the MS/MS is a lipid spectrum.
                           If this parameter is set to 'false' El Gordo will
                             still be executed and e.g. improve the
                           fragmentation
                           tree, but the matching candidates will not be tagged
                             as lipid class.
      --IsotopeMs2Settings=IGNORE

      --IsotopeSettings.filter=True
                           This configurations define how to deal with isotope
                             patterns in MS1.
                           When filtering is enabled, molecular formulas are
                             excluded if their theoretical isotope
                           pattern does not match
                           the theoretical one, even if their MS/MS pattern has
                             high score.
      --IsotopeSettings.multiplier=1
                           multiplier for the isotope score. Set to 0 to
                             disable isotope scoring. Otherwise, the score
                           from isotope
                           pattern analysis is multiplied with this
                             coefficient. Set to a value larger than one if
                             your
                           isotope
                           pattern data is of much better quality than your
                             MS/MS data.
      --MedianNoiseIntensity=0.015

      --MotifDbFile=none
      --ms1.absoluteIntensityError=0.02
                           The average absolute deviation between theoretical
                             and measured intensity of isotope
                           peaks.
                           Do not change this parameter without a good reason!
      --ms1.minimalIntensityToConsider=0.01
                           Ignore isotope peaks below this intensity.
                           This value should reflect the smallest relative
                             intensive which is still above noise level.
                           Obviously, this is hard to judge without having
                             absolute values. Keeping this value around 1
                           percent is
                           fine for most settings. Set it to smaller values if
                             you trust your small intensities.
      --ms1.relativeIntensityError=0.08
                           The average relative deviation between theoretical
                             and measured intensity of isotope
                           peaks.
                           Do not change this parameter without a good reason!
      --MS1MassDeviation.allowedMassDeviation=10.0 ppm
                           Mass accuracy setting for MS1 spectra. Mass
                             accuracies are always written as "X ppm (Y Da)"
                           with X and Y
                           are numerical values. The ppm is a relative measure
                             (parts per million), Da is an absolute
                           measure. For each mass, the
                           maximum of relative and absolute is used.
      --MS1MassDeviation.massDifferenceDeviation=5.0 ppm

      --MS1MassDeviation.standardMassDeviation=10.0 ppm

      --MS2MassDeviation.allowedMassDeviation=10.0 ppm
                           Mass accuracy setting for MS2 spectra. Mass
                             Accuracies are always written as "X ppm (Y Da)"
                           with X and Y
                           are numerical values. The ppm is a relative measure
                             (parts per million), Da is an absolute
                           measure. For each mass, the
                           maximum of relative and absolute is used.
      --MS2MassDeviation.standardMassDeviation=10.0 ppm

      --NoiseThresholdSettings.absoluteThreshold=0

      --NoiseThresholdSettings.basePeak=NOT_PRECURSOR

      --NoiseThresholdSettings.intensityThreshold=0.005

      --NoiseThresholdSettings.maximalNumberOfPeaks=60

      --NumberOfCandidates=10

      --NumberOfCandidatesPerIon=1
                           use this parameter if you want to force to report at
                             least
                           numberOfResultsToKeepPerIonization results per
                             ionization.
                           if le 0, this parameter will have no effect and just
                             the top
                           numberOfResultsToKeep results will be reported.
      --NumberOfStructureCandidates=10000

      --PossibleAdductSwitches=[M+Na]+:[M+H]+,[M+K]+:[M+H]+,[M+Cl]-:[M-H]-
                           An adduct switch is a switch of the ionization mode
                             within a spectrum, e.g. an ion replaces an
                           sodium adduct
                           with a protonation during fragmentation. Such adduct
                             switches heavily increase the
                           complexity of the
                           analysis, but for certain adducts they might happen
                             regularly. Adduct switches are written
                           in the
                           form  {@literal a -> b, a -> c, d -> c} where a, b,
                             c, and d are adducts and  {@literal a -> b}
                             denotes an
                           allowed switch from
                           a to b within the MS/MS spectrum.
      --PrintCitations=True

      --RecomputeResults=False

      --StructurePredictors=CSI_FINGERID

      --StructureSearchDB=BIO

      --Timeout.secondsPerInstance=0
                           This configurations define a timeout for the tree
                             computation. As the underlying problem is
                           NP-hard, it might take
                           forever to compute trees for very challenging (e.g.
                             large mass) compounds. Setting an time
                           constraint allow the program
                           to continue with other instances and just skip the
                             challenging ones.
                           Note that, due to multithreading, this time
                             constraints are not absolutely accurate.
                           Set the maximum number of seconds for computing a
                             single compound. Set to 0 to disable the time
                           constraint.
      --Timeout.secondsPerTree=0
                           Set the maximum number of seconds for a single
                             molecular formula check. Set to 0 to disable the
                           time constraint
      --UseHeuristic.mzToUseHeuristic=300
                           Set minimum m/z to enable heuristic preprocessing.
                             The heuristic will be used to initially
                           rank the formula candidates. The Top
                             (NumberOfCandidates) candidates will then be
                           computed exactly by solving the ILP.
      --UseHeuristic.mzToUseHeuristicOnly=650
                           Set minimum m/z to only use heuristic tree
                             computation. No exact tree computation (ILP) will
                           be performed for this compounds.
  -V, --version            Print version information and exit.
      --ZodiacClusterCompounds=false
                           cluster compounds before running ZODIAC
      --ZodiacEdgeFilterThresholds.minLocalCandidates=1
                           Minimum number of candidates per compound which are
                             forced to have at least
                           [minLocalConnections] connections to other compounds.
                           E.g. 2 candidates per compound must have at least 10
                             connections to other compounds
      --ZodiacEdgeFilterThresholds.minLocalConnections=10
                           Minimum number of connections per candidate which
                             are forced for at least
                           [minLocalCandidates] candidates to other compounds.
                           E.g. 2 candidates per compound must have at least 10
                             connections to other compounds
      --ZodiacEdgeFilterThresholds.thresholdFilter=0.95
                           Defines the proportion of edges of the complete
                             network which will be ignored.
      --ZodiacEpochs.burnInPeriod=2000
                           Number of epochs considered as 'burn-in period'.
                           Samples from the beginning of a Markov chain do not
                             accurately represent the desired
                           distribution of candidates and are not used to
                             estimate the ZODIAC score.
      --ZodiacEpochs.iterations=20000
                           Number of epochs to run the Gibbs sampling. When
                             multiple Markov chains are computed, all
                           chains' iterations sum up to this value.
      --ZodiacEpochs.numberOfMarkovChains=10
                           Number of separate Gibbs sampling runs.
      --ZodiacLibraryScoring.lambda=1000
                           Lambda used in the scoring function of spectral
                             library hits. The higher this value the higher
                           are librar hits weighted in ZODIAC scoring.
      --ZodiacLibraryScoring.minCosine=0.5
                           Spectral library hits must have at least this cosine
                             or higher to be considered in scoring.
                           Value must be in [0,1].
      --ZodiacNumberOfConsideredCandidatesAt300Mz=10
                           Maximum number of candidate molecular formulas
                             (fragmentation trees computed by SIRIUS)
                           per compound which are considered by ZODIAC.
                           This is the threshold used for all compounds with mz
                             below 300 m/z and is used to interpolate the
                           number of candidates for larger compounds.
                           If lower than 0, all available candidates are
                             considered.
      --ZodiacNumberOfConsideredCandidatesAt800Mz=50
                           Maximum number of candidate molecular formulas
                             (fragmentation trees computed by SIRIUS)
                           per compound which are considered by ZODIAC.
                           This is the threshold used for all compounds with mz
                             above 800 m/z and is used to interpolate the
                           number of candidates for smaller compounds.
                           If lower than 0, all available candidates are
                             considered.
      --ZodiacRatioOfConsideredCandidatesPerIonization=0.2
                           Ratio of candidate molecular formulas (fragmentation
                             trees computed by SIRIUS) per
                           compound which are forced for each ionization to be
                             considered by ZODIAC.
                           This depends on the number of candidates ZODIAC
                             considers. E.g. if 50 candidates are
                           considered and a ratio of 0.2 is set, at least 10
                             candidates per ionization will be
                           considered, which might increase the number of
                             candidates above 50.
      --ZodiacRunInTwoSteps=true
                           As default ZODIAC runs a 2-step approach. First
                             running 'good quality compounds' only, and
                           afterwards including the remaining.
```

# Sirius options

--FormulaSearchDB=BIO,METACYC,CHEBI,COCONUT,ECOCYCMINE,GNPS,HMDB,HSDB,KEGG,KEGGMINE,KNAPSACK,MACONDA,MESH,NORMAN,UNDP,PLANTCYC,PUBCHEM,PUBMED,YMDB,YMDBMINE,ZINCBIO 



# Implementation details

## First we need to login 

```bash
sirius login --user-env SIRIUS_USERNAME --password-env SIRIUS_PASSWORD
``` 



Example of a minimal sirius command line 

```bash
sirius -i tests/data/input_sirius.mgf --output tests/data/output_sirius --maxmz=1000 config --IsotopeSettings.filter=False formula zodiac fingerprint structure canopus
```

Example of a typical sirius command line see https://github.com/enpkg/enpkg_full/blob/c8e649290ee72f000c3385e7669b5da2215abad8/params/user.yml#L60

```bash
sirius -i 'tests/data/input_sirius.mgf' --output 'tests/data/output_sirius' --maxmz 800 config --IsotopeSettings.filter=true --FormulaSearchDB=BIO --Timeout.secondsPerTree=0 --FormulaSettings.enforced=HCNOP --Timeout.secondsPerInstance=0 --AdductSettings.detectable=[[M+Na]+,[M+H3N+H]+,[M-H4O2+H]+,[M+K]+,[M+H]+,[M-H2O+H]+] --UseHeuristic.mzToUseHeuristicOnly=650 --AlgorithmProfile=qtof --IsotopeMs2Settings=IGNORE --MS2MassDeviation.allowedMassDeviation=10.0ppm --NumberOfCandidatesPerIon=1 --UseHeuristic.mzToUseHeuristic=300 --FormulaSettings.detectable=B,Cl,Br,Se,S --NumberOfCandidates=10 --ZodiacNumberOfConsideredCandidatesAt300Mz=10 --ZodiacRunInTwoSteps=true --ZodiacEdgeFilterThresholds.minLocalConnections=10 --ZodiacEdgeFilterThresholds.thresholdFilter=0.95 --ZodiacEpochs.burnInPeriod=2000 --ZodiacEpochs.numberOfMarkovChains=10 --ZodiacNumberOfConsideredCandidatesAt800Mz=50 --ZodiacEpochs.iterations=20000 --AdductSettings.enforced=, --AdductSettings.fallback=[[M+Na]+,[M-H+K+K]+,[M+K]+,[M+H]+,[M-H2O+H]+] --FormulaResultThreshold=true --InjectElGordoCompounds=true --StructureSearchDB=BIO --RecomputeResults=false formula zodiac fingerprint structure canopus write-summaries --output 'tests/data/output_sirius'
```

The order of the arguments is important.
This will work 

```bash
/Applications/sirius.app/Contents/MacOS/sirius -i tests/data/input_sirius.mgf --output tests/data/output_sirius --maxmz 800 config --IsotopeSettings.filter=true --FormulaSearchDB=BIO --Timeout.secondsPerTree=0 --FormulaSettings.enforced=HCNOP --Timeout.secondsPerInstance=0 --AdductSettings.detectable='[[M+H]+,[M-H4O2+H]+,[M+Na]+,[M+K]+,[M+H3N+H]+,[M-H2O+H]+]' --UseHeuristic.mzToUseHeuristicOnly=650 --AlgorithmProfile=orbitrap --IsotopeMs2Settings=IGNORE --MS2MassDeviation.allowedMassDeviation=5.0ppm --NumberOfCandidatesPerIon=1 --UseHeuristic.mzToUseHeuristic=300 --FormulaSettings.detectable=B,Cl,Br,Se,S --NumberOfCandidates=10 --ZodiacNumberOfConsideredCandidatesAt300Mz=10 --ZodiacRunInTwoSteps=true --ZodiacEdgeFilterThresholds.minLocalConnections=10 --ZodiacEdgeFilterThresholds.thresholdFilter=0.95 --ZodiacEpochs.burnInPeriod=2000 --ZodiacEpochs.numberOfMarkovChains=10 --ZodiacNumberOfConsideredCandidatesAt800Mz=50 --ZodiacEpochs.iterations=20000 --AdductSettings.enforced=, --AdductSettings.fallback='[[M+H]+,[M+Na]+,[M+K]+]' --FormulaResultThreshold=true --InjectElGordoCompounds=true --StructureSearchDB=BIO --RecomputeResults=false formula zodiac fingerprint structure canopus write-summaries
```


This wil not 

```bash

/Applications/sirius.app/Contents/MacOS/sirius -i tests/data/input_sirius.mgf --output tests/data/output_sirius --maxmz 800 formula zodiac fingerprint structure canopus write-summaries config --IsotopeSettings.filter=true --FormulaSearchDB=BIO --Timeout.secondsPerTree=0 --FormulaSettings.enforced=HCNOP --Timeout.secondsPerInstance=0 --AdductSettings.detectable='[[M+H]+,[M-H4O2+H]+,[M+Na]+,[M+K]+,[M+H3N+H]+,[M-H2O+H]+]' --UseHeuristic.mzToUseHeuristicOnly=650 --AlgorithmProfile=orbitrap --IsotopeMs2Settings=IGNORE --MS2MassDeviation.allowedMassDeviation=5.0ppm --NumberOfCandidatesPerIon=1 --UseHeuristic.mzToUseHeuristic=300 --FormulaSettings.detectable=B,Cl,Br,Se,S --NumberOfCandidates=10 --ZodiacNumberOfConsideredCandidatesAt300Mz=10 --ZodiacRunInTwoSteps=true --ZodiacEdgeFilterThresholds.minLocalConnections=10 --ZodiacEdgeFilterThresholds.thresholdFilter=0.95 --ZodiacEpochs.burnInPeriod=2000 --ZodiacEpochs.numberOfMarkovChains=10 --ZodiacNumberOfConsideredCandidatesAt800Mz=50 --ZodiacEpochs.iterations=20000 --AdductSettings.enforced=, --AdductSettings.fallback='[[M+H]+,[M+Na]+,[M+K]+]' --FormulaResultThreshold=true --InjectElGordoCompounds=true --StructureSearchDB=BIO --RecomputeResults=false
```

```bash

```bash
```


Sirius takes nested command line arguments.

For example :


## Sirius Core Params

These are the sirius core params

```bash
sirius [-hV] [--noCite] [--recompute] [--cores=<numOfCores>]
              [--instance-buffer=<initialInstanceBuffer>] [--log=<logLevel>]
              [--maxmz=<maxMz>] [--workspace=<workspace>]
              [[-o=<outputProjectLocation>] [--no-compression]
              [--update-fingerprint-version]
              [--naming-convention=<projectSpaceFilenameFormatter>]]
              [[[--ignore-formula] [--allow-ms1-only] -i=<inputPath>[,
              <inputPath>...] [-i=<inputPath>[,<inputPath>...]]...]
              [-z=<parentMz> [--adduct=<ionType>] [-f=<formula>] [-1=<ms1File>[,
              <ms1File>...]] -2=<ms2File>[,<ms2File>...]]...] [COMMAND]
```


They are detailed below (also available with `sirius --help`)

```bash

-h, --help                Show this help message and exit.
-V, --version             Print version information and exit.
--log, --loglevel=<logLevel>
                      Set logging level of the Jobs SIRIUS will execute.
                        Valid values: SEVERE, WARNING, INFO, FINER, ALL
                        Default: WARNING
--cores, --processors=<numOfCores>
                      Number of cpu cores to use. If not specified Sirius
                        uses all available cores.
--instance-buffer, --compound-buffer,
  --initial-compound-buffer=<initialInstanceBuffer>
                      Number of compounds that will be loaded into the
                        Memory. A larger buffer ensures that there are
                        enough compounds available to use all cores
                        efficiently during computation. A smaller buffer
                        saves Memory. To load all compounds immediately
                        set it to -1. Default (numeric value 0): 3 x
                        --cores. Note that for <DATASET_TOOLS> the
                        compound buffer may have no effect because this
                        tools may have to load compounds simultaneously
                        into the memory.
                        Default: 0
--workspace=<workspace>
                      Specify sirius workspace location. This is the
                        directory for storing Property files, logs,
                        databases and caches.  This is NOT for the
                        project-space that stores the results! Default is
                        $USER_HOME/.sirius-<MINOR_VERSION>
--recompute           Recompute results of ALL tools where results are
                        already present. Per default already present
                        results will be preserved and the instance will
                        be skipped for the corresponding Task/Tool
--maxmz=<maxMz>       Only considers compounds with a precursor m/z lower
                        or equal [--maxmz]. All other compounds in the
                        input will be skipped.
                        Default: Infinity
--noCite, --noCitations, --no-citations
                      Do not write summary files to the project-space

Specify OUTPUT Project-Space:
-o, --output, --project=<outputProjectLocation>
                            Specify the project-space to write into. If no
                              [--input] is specified it is also used as input.
                              For compression use the File ending .zip or .
                              sirius.
--naming-convention=<projectSpaceFilenameFormatter>
                      Specify a naming scheme for the  compound
                        directories ins the project-space. Default %
                        index_%filename_%compoundname
--no-compression      Does not use compressed project-space format (not
                        recommended) when creating the project-space. If
                        an existing project-space is opened this
                        parameter has no effect.
--update-fingerprint-version
                      Updates Fingerprint versions of the input project
                        to the one used by this SIRIUS version.
                      WARNING: All Fingerprint related results (CSI:
                        FingerID, CANOPUS) will be lost!

Specify multi-compound inputs (.ms, .mgf, .mzML/.mzXml, .sirius):
  -i, --input=<inputPath>[,<inputPath>...]
                            Specify the input in multi-compound input formats:
                              Preprocessed mass spectra in .ms or .mgf file
                              format, LC/MS runs in .mzML/.mzXml format or
                              already existing SIRIUS project-spaces
                              (uncompressed/compressed) but also any other file
                              type e.g. to provide input for STANDALONE tools.
      --ignore-formula      ignore given molecular formula if present in .ms or
                              .mgf input files.
      --allow-ms1-only      Allow MS1 only data to be imported.
Specify generic inputs (CSV) on per compound level:
  -1, --ms1=<ms1File>[,<ms1File>...]
                            MS1 spectra files
  -2, --ms2=<ms2File>[,<ms2File>...]
                            MS2 spectra files
  -z, --mz, --precursor, --parentmass=<parentMz>
                            The mass of the parent ion for the specified ms2
                              spectra
      --adduct, --ionization=<ionType>
                            Specify the adduct for this compound
                              Default: [M+?]+
  -f, --formula=<formula>   Specify the neutralized formula of this compound.
                              This will be used for tree computation. If given
                              no mass decomposition will be performed.
                              
```


## Sirius Config Params

The sirius config params are typicall specified after the sirius core params.

```bash
sirius config [-hV] [--AdductSettings.detectable=[M+H]+,[M+K]+,[M+Na]+,
                     [M+H-H2O]+,[M+H-H4O2]+,[M+NH4]+,[M-H]-,[M+Cl]-,[M-H2O-H]-,
                     [M+Br]-] [--AdductSettings.enforced=,] [--AdductSettings.
                     fallback=[M+H]+,[M-H]-,[M+Na]+,[M+K]+]
                     [--AlgorithmProfile=default] [--CandidateFormulas=,]
                     [--CompoundQuality=UNKNOWN]
                     [--ForbidRecalibration=ALLOWED]
                     [--FormulaResultRankingScore=AUTO]
                     [--FormulaResultThreshold=true] [--FormulaSearchDB=none]
                     [--FormulaSettings.detectable=S,Br,Cl,B,Se]
                     [--FormulaSettings.enforced=C,H,N,O,P] [--FormulaSettings.
                     fallback=S] [--InjectElGordoCompounds=True]
                     [--IsotopeMs2Settings=IGNORE] [--IsotopeSettings.
                     filter=True] [--IsotopeSettings.multiplier=1]
                     [--MedianNoiseIntensity=0.015] [--MotifDbFile=none] [--ms1.
                     absoluteIntensityError=0.02] [--ms1.
                     minimalIntensityToConsider=0.01] [--ms1.
                     relativeIntensityError=0.08] [--MS1MassDeviation.
                     allowedMassDeviation=10.0 ppm] [--MS1MassDeviation.
                     massDifferenceDeviation=5.0 ppm] [--MS1MassDeviation.
                     standardMassDeviation=10.0 ppm] [--MS2MassDeviation.
                     allowedMassDeviation=10.0 ppm] [--MS2MassDeviation.
                     standardMassDeviation=10.0 ppm] [--NoiseThresholdSettings.
                     absoluteThreshold=0] [--NoiseThresholdSettings.
                     basePeak=NOT_PRECURSOR] [--NoiseThresholdSettings.
                     intensityThreshold=0.005] [--NoiseThresholdSettings.
                     maximalNumberOfPeaks=60] [--NumberOfCandidates=10]
                     [--NumberOfCandidatesPerIon=1]
                     [--NumberOfStructureCandidates=10000]
                     [--PossibleAdductSwitches=[M+Na]+:[M+H]+,[M+K]+:[M+H]+,
                     [M+Cl]-:[M-H]-] [--PrintCitations=True]
                     [--RecomputeResults=False]
                     [--StructurePredictors=CSI_FINGERID]
                     [--StructureSearchDB=BIO] [--Timeout.secondsPerInstance=0]
                     [--Timeout.secondsPerTree=0] [--UseHeuristic.
                     mzToUseHeuristic=300] [--UseHeuristic.
                     mzToUseHeuristicOnly=650] [--ZodiacClusterCompounds=false]
                     [--ZodiacEdgeFilterThresholds.minLocalCandidates=1]
                     [--ZodiacEdgeFilterThresholds.minLocalConnections=10]
                     [--ZodiacEdgeFilterThresholds.thresholdFilter=0.95]
                     [--ZodiacEpochs.burnInPeriod=2000] [--ZodiacEpochs.
                     iterations=20000] [--ZodiacEpochs.numberOfMarkovChains=10]
                     [--ZodiacLibraryScoring.lambda=1000]
                     [--ZodiacLibraryScoring.minCosine=0.5]
                     [--ZodiacNumberOfConsideredCandidatesAt300Mz=10]
                     [--ZodiacNumberOfConsideredCandidatesAt800Mz=50]
                     [--ZodiacRatioOfConsideredCandidatesPerIonization=0.2]
                     [--ZodiacRunInTwoSteps=true] [COMMAND]
<CONFIGURATION> Override all possible default configurations of this toolbox
from the command line.
```

They are detailed below (also available with `sirius config --help`)

```bash
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
                           Must be either 'ALLOWED' or 'FORBIDDEN'
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
                           chains iterations sum up to this value.
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
#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use rand::rngs::SmallRng;
use rand::Rng;
use rand::SeedableRng;
use sirius::prelude::*; // to change

#[derive(Arbitrary, Debug)]
struct FuzzCase {
    random_state: u64,
    maximal_mz: f64,
    formula_search_db: FormulaSearchDB,
    isotope_settings_filter: bool,
    structure_search_db: FormulaSearchDB,
    timeout_seconds_per_tree: u32,
    number_of_candidates: u32,
    number_of_candidates_per_ion: u32,
    number_of_structure_candidates: u32,
    recompute_results: bool,
    print_citations: bool,
    timeout_seconds_per_instance: u32,
}

fuzz_target!(|params: FuzzCase| {
    let rng = SmallRng::seed_from_u64(params.random_state);
    let builder = SiriusBuilder::default();
    let _ = router(rng, builder, &params);
});

const NUMBER_OF_METHODS: usize = 8;

fn router(
    mut random_state: SmallRng,
    builder: SiriusBuilder<Version5>,
    params: &FuzzCase,
) -> Result<(), String> {
    // we sample a random integer to decide which method to call
    let state = random_state.gen_range(0..NUMBER_OF_METHODS + 1);
    match state {
        0 => {
            router(random_state, builder.maximal_mz_default()?, params)?;
        }
        1 => {
            router(random_state, builder.maximal_mz(params.maximal_mz)?, params)?;
        }
        2 => {
            router(
                random_state,
                builder.formula_search_db(params.formula_search_db)?,
                params,
            )?;
        }
        3 => {
            router(random_state, builder.formula_search_db_default()?, params)?;
        }
        4 => {
            router(
                random_state,
                builder.isotope_settings_filter(params.isotope_settings_filter)?,
                params,
            )?;
        }
        5 => {
            router(
                random_state,
                builder.isotope_settings_filter_default()?,
                params,
            )?;
        }
        6 => {
            router(
                random_state,
                builder.structure_search_db(params.structure_search_db)?,
                params,
            )?;
        }
        7 => {
            router(random_state, builder.structure_search_db_default()?, params)?;
        }
        8 => {
            router(
                random_state,
                builder.timeout_seconds_per_tree(params.timeout_seconds_per_tree)?,
                params,
            )?;
        }
        9 => {
            router(
                random_state,
                builder.timeout_seconds_per_tree_default()?,
                params,
            )?;
        }
        _ => {
            builder.build();
        }
    }
    Ok(())
}

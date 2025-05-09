//! Submodule to test whether the most abundant isotope method works as expected

use elements::{
    Element, Isotope,
    isotopes::{CarbonIsotope, HydrogenIsotope, NitrogenIsotope, OxygenIsotope, SodiumIsotope},
};

const MOST_ABUNDANT_ISOTOPES: &[(Element, Isotope)] = &[
    (Element::H, Isotope::H(HydrogenIsotope::H1)),
    (Element::C, Isotope::C(CarbonIsotope::C12)),
    (Element::O, Isotope::O(OxygenIsotope::O16)),
    (Element::N, Isotope::N(NitrogenIsotope::N14)),
    (Element::Na, Isotope::Na(SodiumIsotope::Na23)),
];

#[test]
fn test_most_abundant_isotope() {
    for (element, isotope) in MOST_ABUNDANT_ISOTOPES {
        let most_abundant_isotope = element.most_abundant_isotope();
        assert_eq!(
            most_abundant_isotope, *isotope,
            "Expected most abundant isotope of {} to be {}",
            element, isotope
        );
    }
}

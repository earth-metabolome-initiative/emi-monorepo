//! Tests of the parser module for several corner cases.

use smiles_parser::parser::token_iter::TokenIter;
use smiles_parser::prelude::Smiles;
const SMILES_STR: &[&str] = &[
    "C1=CC=CC=C1",
    "[OH2]",
    "[Ti+4]",
    "[Co+3]",
    "CCO",
    "C#N",
    "[Ga+]$[As-]",
    "[Na+].[Cl-]",
    "C1CCCC2C1CCCC2",
    "C0CCCCC0C0CCCCC0",
    "C1:C:C:C:C:C1",
    "c1ccccc1",
    "c1ccccc1c2ccccc2",
    "COc(c1)cccc1C#N",
    "FC(Br)(Cl)F",
    "CC1CCC/C(C)=C1/C=C/C(C)=C/C=C/C(C)=C/C=C/C=C(C)/C=C/C=C(C)/C=C/C2=C(C)/CCCC2(C)C",
    "N[C@@H](C)C(=O)O",
    "N[CH](C)C(=O)O",
    "NC(C)C(=O)O",
    "[14cH]1ccccc1",
    "[2H]C(Cl)(Cl)Cl",
    "[C@@H](C)(N)C(=O)O",
    "C[C@H](N)C(=O)O",
    "OC(=O)[C@@H](N)C",
    "[K+].C=C.Cl[Pt-](Cl)Cl.O",
    "[Ti++++]",
];

#[test]
fn test_tokenizer() {
    for &s in SMILES_STR {
        let _tokens = TokenIter::from(s)
            .collect::<Result<Vec<_>, _>>()
            .expect(&format!("Failed to parse {s}"));
    }
}

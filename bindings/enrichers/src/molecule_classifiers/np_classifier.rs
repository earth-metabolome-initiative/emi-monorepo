// create a function that takes a request and returns a response
// https://npclassifier.gnps2.org/classify?smiles=Cn1c(=O)c2c(ncn2C)n(C)c1=O
// retrun {"class_results": ["Purine alkaloids"], "superclass_results": ["Pseudoalkaloids"], "pathway_results": ["Alkaloids"], "isglycoside": false}

use reqwest::{self};

use crate::molecule_classifiers::ChemicalIdentifier;
use serde_json::Value;

const URL: &str = "https://npclassifier.gnps2.org/classify?";

async fn get_np_classifier_response(
    molecule: &str,
    chemical_identifier: ChemicalIdentifier,
) -> Result<String, reqwest::Error> {
    // convert the molecule to a url encoded string
    let encoded_molecule: String =
        url::form_urlencoded::byte_serialize(molecule.as_bytes()).collect();

    // create the url, if it is a smiles
    let url = match chemical_identifier {
        ChemicalIdentifier::Smiles => format!("{}smiles={}", URL, encoded_molecule),
        ChemicalIdentifier::Inchi => {
            panic!("NP Classifier can't have Inchi. Only SMILES can be used !")
        }
    };
    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;

    // once we have the response, we check if it was an error
    // the error_for_status_ref() method will turn a response into an error if the server returned an error.
    match response.error_for_status_ref() {
        Ok(_) => return Ok(response.text().await?),
        Err(e) => {
            return Err(e);
        }
    }
}

fn read_json(raw_json: &str) -> Result<Value, serde_json::Error> {
    let parsed: Value = serde_json::from_str(raw_json)?;
    Ok(parsed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    #[should_panic]
    async fn test_get_np_classifier_response_fail() {
        let molecule = "InChI=1S/C37H67NO13/c1-14-25-37(10,45)30(41)20(4)27(39)18(2)16-35(8,44)32(51-34-28(40)24(38(11)12)15-19(3)47-34)21(5)29(22(6)33(43)49-25)50-26-17-36(9,46-13)31(42)23(7)48-26/h18-26,28-32,34,40-42,44-45H,14-17H2,1-13H3/t18-,19-,20+,21+,22-,23+,24+,25-,26+,28-,29+,30-,31+,32-,34+,35-,36-,37-/m1/s1";
        let result = get_np_classifier_response(molecule, ChemicalIdentifier::Inchi).await;
        let inchi = read_json(&result.unwrap()).unwrap();
        assert_eq!(inchi["inchikey"], "InChIKey=ULGZDMOVFRHVEP-RWJQBGPGSA-N");
    }

    #[tokio::test]
    async fn test_get_np_classifier_response() {
        let molecule = "Brc1ccc2c(-c3ncc(-c4c[nH]c5ccccc45)[nH]3)c[nH]c2c1";
        let res = get_np_classifier_response(molecule, ChemicalIdentifier::Smiles).await;
        assert!(res.is_ok())
    }
}

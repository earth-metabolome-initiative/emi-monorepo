use crate::prelude::Molecule;
use reqwest;

const URL: &str = "https://npclassifier.gnps2.org/classify?";

pub async fn get_np_classifier_response(molecule: Molecule) -> Result<String, reqwest::Error> {
    // convert the molecule to a url encoded string
    let encoded_molecule: String =
        url::form_urlencoded::byte_serialize(molecule.smiles.as_bytes()).collect();

    // create the url, if it is a smiles
    let url = format!("{}smiles={}", URL, encoded_molecule);

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

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_get_np_classifier_response_fail() {
        let molecule = Molecule {
            inchi: "InChI=1S/C37H67NO13/c1-14-25-37(10,45)30(41)20(4)27(39)18(2)16-35(8,44)32(51-34-28(40)24(38(11)12)15-19(3)47-34)21(5)29(22(6)33(43)49-25)50-26-17-36(9,46-13)31(42)23(7)48-26/h18-26,28-32,34,40-42,44-45H,14-17H2,1-13H3/t18-,19-,20+,21+,22-,23+,24+,25-,26+,28-,29+,30-,31+,32-,34+,35-,36-,37-/m1/s1".to_string(),
            smiles: "TEst".to_string(),
        };
        let result = get_np_classifier_response(molecule).await;
        assert!(result.is_err())
    }

    #[tokio::test]
    async fn test_np_classifier() {
        let mol = Molecule {
            inchi: "InChI=1S/C37H67NO13/c1-14-25-37(10,45)30(41)20(4)27(39)18(2)16-35(8,44)32(51-34-28(40)24(38(11)12)15-19(3)47-34)21(5)29(22(6)33(43)49-25)50-26-17-36(9,46-13)31(42)23(7)48-26/h18-26,28-32,34,40-42,44-45H,14-17H2,1-13H3/t18-,19-,20+,21+,22-,23+,24+,25-,26+,28-,29+,30-,31+,32-,34+,35-,36-,37-/m1/s1".to_string(),
            smiles: "CCC1C(C(C(C(=O)C(CC(C(C(C(C(C(=O)O1)C)OC2CC(C(C(O2)C)O)(C)OC)C)OC3C(C(CC(O3)C)N(C)C)O)(C)O)C)C)O)(C)O".to_string() 
        };
        let res = get_np_classifier_response(mol).await;
        assert!(res.is_ok())
    }
}

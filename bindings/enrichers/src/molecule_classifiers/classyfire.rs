use crate::prelude::Molecule;
use reqwest;
use serde_json::Value;

const URL: &str = "https://structure.gnps2.org/classyfire?";

pub async fn get_classyfire_smiles_response(molecule: Molecule) -> Result<String, reqwest::Error> {
    // convert the molecule to a url encoded string
    let encoded_molecule: String =
        url::form_urlencoded::byte_serialize(molecule.smiles.as_bytes()).collect();

    // create the url
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

pub async fn get_classyfire_inchi_response(molecule: Molecule) -> Result<String, reqwest::Error> {
    // convert the molecule to a url encoded string
    let encoded_molecule: String =
        url::form_urlencoded::byte_serialize(molecule.inchi.as_bytes()).collect();

    // create the url
    let url = format!("{}inchi={}", URL, encoded_molecule);

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

/// Reads a string that is supposes to be a json and returns it as json.
///
/// # Arguments
/// * `raw_json` - The string that should be converted
///
/// # Example
/// ```
/// use enrichers::prelude::*;
/// let my_string = "{\"class_results\": [\"Corynanthe type\"], \"superclass_results\": [\"Tryptophan alkaloids\"], \"pathway_results\": [\"Alkaloids\"], \"isglycoside\": false}";
/// let out = read_json(my_string).unwrap();
/// assert_eq!(out["class_results"][0], "Corynanthe type");
/// ````
pub fn read_json(raw_json: &str) -> Result<Value, serde_json::Error> {
    let parsed: Value = serde_json::from_str(raw_json)?;
    Ok(parsed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_get_classyfire_response() {
        let molecule = Molecule{
            inchi: "InChI=1S/C37H67NO13/c1-14-25-37(10,45)30(41)20(4)27(39)18(2)16-35(8,44)32(51-34-28(40)24(38(11)12)15-19(3)47-34)21(5)29(22(6)33(43)49-25)50-26-17-36(9,46-13)31(42)23(7)48-26/h18-26,28-32,34,40-42,44-45H,14-17H2,1-13H3/t18-,19-,20+,21+,22-,23+,24+,25-,26+,28-,29+,30-,31+,32-,34+,35-,36-,37-/m1/s1".to_string(),
            smiles: "HEY".to_string(),
        };
        let result = get_classyfire_inchi_response(molecule).await;
        let inchi = read_json(&result.unwrap()).unwrap();
        assert_eq!(inchi["inchikey"], "InChIKey=ULGZDMOVFRHVEP-RWJQBGPGSA-N");
    }

    #[tokio::test]
    async fn test_nested_json() {
        let molecule = Molecule{
            inchi: "InChI=1S/C37H67NO13/c1-14-25-37(10,45)30(41)20(4)27(39)18(2)16-35(8,44)32(51-34-28(40)24(38(11)12)15-19(3)47-34)21(5)29(22(6)33(43)49-25)50-26-17-36(9,46-13)31(42)23(7)48-26/h18-26,28-32,34,40-42,44-45H,14-17H2,1-13H3/t18-,19-,20+,21+,22-,23+,24+,25-,26+,28-,29+,30-,31+,32-,34+,35-,36-,37-/m1/s1".to_string(),
            smiles: "HEY".to_string(),
        };
        let result = get_classyfire_inchi_response(molecule).await;
        let url = read_json(&result.unwrap()).unwrap();
        assert_eq!(
            url["kingdom"]["url"],
            "http://classyfire.wishartlab.com/tax_nodes/C0000000"
        );
    }

    #[tokio::test]
    async fn test_with_incorrect_molecule() {
        let molecule = Molecule{
            inchi: "InChI=1S/C37H67NO13/c1-14-25-37(10,45)30(41)20(4)27(39)18(2)16-35(8,44)32(51-34-28(40)24(38(11)12)15-19(3)47-34)21(5)29(22(6)33(43)49-25)50-26-17-36(9,46-13)31(42)23(7)48-26/h18-26,28-32,34,40-42,44-45H,14-17H2,1-13H3/t18-,19-,20+,21+,22-,23+,24+,25-,26+,28-,29+,30-,31+,32-,34+,35-,36-,37-/m1/s1".to_string(),
            smiles: "HEY".to_string(),
        };
        let result = get_classyfire_smiles_response(molecule).await;
        assert!(result.is_err());
    }
}

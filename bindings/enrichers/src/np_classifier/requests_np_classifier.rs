// create a function that takes a request and returns a response
// https://npclassifier.gnps2.org/classify?smiles=Cn1c(=O)c2c(ncn2C)n(C)c1=O
// retrun {"class_results": ["Purine alkaloids"], "superclass_results": ["Pseudoalkaloids"], "pathway_results": ["Alkaloids"], "isglycoside": false}
use reqwest;
use serde::Deserialize;

fn np_classifier_url(smiles: &str) -> String {
    format!("https://npclassifier.gnps2.org/classify?smiles={}", smiles)
}

async fn get_np_classifier_response(smiles: &str) -> Result<String, reqwest::Error> {
    let url = np_classifier_url(smiles);
    let body = reqwest::get(&url).await?.text().await?;
    Ok(body)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_get_np_classifier_response() {
        let smiles = "Cn1c(=O)c2c(ncn2C)n(C)c1=O";
        let result = get_np_classifier_response(smiles).await;
        println!("{:?}", result);
        assert_eq!(result.is_ok(), true);
    }
}
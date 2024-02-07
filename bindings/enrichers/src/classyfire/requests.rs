use reqwest;
use serde::Deserialize;

const URL: &str = "http://classyfire.wishartlab.com";

#[derive(Deserialize)]
struct Ip {
    origin: String,
}

async fn structure_query(compound: &str, label: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let json_body = serde_json::json!({
        "label": label.to_string(),
        "query_input": compound.to_string(),
        "query_type": "STRUCTURE",
    });

    let response = client
        .post(&format!("{}/queries.json", URL))
        .json(&json_body)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .send()
        .await?;

    response.error_for_status_ref()?;

    let id: Ip = response.json::<Ip>().await?;
    Ok(id.origin)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_structure_query() {
        let compound = "CCC";
        let label = "pyclassyfire";
        let result = structure_query(compound, label).await;
        println!("{:?}", result);
        assert_eq!(result.is_ok(), true);
    }
}

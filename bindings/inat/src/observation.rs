use inaturalist::apis::configuration;
use inaturalist::apis::observations_api::observations_get;
use inaturalist::apis::observations_api::observations_id_get;
use inaturalist::apis::observations_api::ObservationsGetError;
use inaturalist::apis::observations_api::ObservationsGetParams;
use inaturalist::apis::observations_api::ObservationsIdGetError;
use inaturalist::apis::observations_api::ObservationsIdGetParams;
use inaturalist::models::observations_show_response::ObservationsShowResponse;
use inaturalist::models::ObservationsResponse;

/// Get the observation given a structure of parameters
/// # Arguments
/// * `input` - A structure of parameters to filter the observations
/// # Returns
/// * A structure of observations of type `ObservationsResponse`
pub async fn get_observation_filtered(
    input: ObservationsGetParams,
) -> Result<ObservationsResponse, inaturalist::apis::Error<ObservationsGetError>> {
    let mut config = configuration::Configuration::new();
    config.base_path = "https://api.inaturalist.org/v1".to_owned();

    let result = observations_get(&config, input).await;
    match result {
        Ok(response) => Ok(response),
        Err(error) => Err(error),
    }
}

/// Get the observation given an id
/// This answers the GiHub task in [issue #3](https://github.com/earth-metabolome-initiative/emi-monorepo/issues/3)
pub async fn get_observation(
    id: i32,
) -> Result<ObservationsShowResponse, inaturalist::apis::Error<ObservationsIdGetError>> {
    let mut config = configuration::Configuration::new();
    config.base_path = "https://api.inaturalist.org/v1".to_owned();

    let params = ObservationsIdGetParams { id: vec![id] };

    let result = observations_id_get(&config, params).await;
    match result {
        Ok(response) => Ok(response),
        Err(error) => Err(error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_get_observation() {
        let out = get_observation(130644).await;
        println!("{:?}", out);
        assert!(out.is_ok());
    }

    #[tokio::test]
    async fn test_get_observations_from_whole_project() {
        let params = ObservationsGetParams {
            project_id: Some(vec!["130644".to_string()]),
            ..Default::default()
        };
        let out = get_observation_filtered(params).await;
        println!("{:?}", out);
        assert!(out.is_ok());
    }

    #[tokio::test]
    async fn test_get_observation_from_project_and_restricted_taxa() {
        let params = ObservationsGetParams {
            //hrank: Some("parvorder".to_string()),
            taxon_id: Some(vec!["919182".to_string()]),
            project_id: Some(vec!["130644".to_string()]),
            ..Default::default()
        };

        let out = get_observation_filtered(params).await;
        println!("{:?}", out);
        assert!(out.is_ok());
    }
}

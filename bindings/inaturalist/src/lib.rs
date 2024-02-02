use inaturalist::apis::configuration;
use inaturalist::apis::observations_api::observations_id_get;
use inaturalist::apis::observations_api::ObservationsIdGetError;
use inaturalist::apis::observations_api::ObservationsIdGetParams;
use inaturalist::models::observations_show_response::ObservationsShowResponse;
use tokio;

fn main() {
    let out = get_observation(1573655).unwrap();
    println!("{:#?}", out);
}

#[tokio::main]
async fn get_observation(
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

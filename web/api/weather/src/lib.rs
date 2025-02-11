#[tokio::test]
async fn weather_from_greenland() {
    let client = open_meteo_rs::Client::new();
    let mut opts = open_meteo_rs::forecast::Options::default();
    opts.cell_selection = Some(open_meteo_rs::forecast::CellSelection::Nearest);

    // Dates
    let start_date = chrono::Utc::now().naive_local().date() - chrono::Duration::days(10);
    opts.start_date = Some(start_date);
    opts.end_date = Some(start_date + chrono::Duration::days(5));
    opts.daily.push("temperature_2m_max".into());

    // Location
    opts.location = open_meteo_rs::Location {
        lat: 67.843665,
        lng: -43.400017,
    };

    let res = client.archive(opts).await;
    assert!(res.is_ok());
}

#[tokio::test]
async fn weather_from_china() {
    let client: open_meteo_rs::Client = open_meteo_rs::Client::new();
    let mut opts = open_meteo_rs::forecast::Options::default();
    opts.cell_selection = Some(open_meteo_rs::forecast::CellSelection::Nearest);

    // Dates
    let start_date = chrono::Utc::now().naive_local().date() - chrono::Duration::days(10);
    opts.start_date = Some(start_date);
    opts.end_date = Some(start_date + chrono::Duration::days(5));
    // opts.daily.push("temperature_2m_max".into());
    // opts.daily.push("weather_code".into());
    opts.daily.push("snowfall_sum".into());

    // Location
    opts.location = open_meteo_rs::Location {
        lat: 36.829891,
        lng: 95.555054,
    };

    let res = client.archive(opts).await;
    assert!(res.is_ok());
}

#[tokio::test]
async fn weather_from_western_sahara() {
    let client: open_meteo_rs::Client = open_meteo_rs::Client::new();
    let mut opts = open_meteo_rs::forecast::Options::default();
    opts.cell_selection = Some(open_meteo_rs::forecast::CellSelection::Nearest);

    // Dates
    let start_date = chrono::Utc::now().naive_local().date() - chrono::Duration::days(10);
    opts.start_date = Some(start_date);
    opts.end_date = Some(start_date + chrono::Duration::days(5));
    opts.daily.push("temperature_2m_max".into());
    opts.daily.push("daylight_duration".into());

    // Location
    opts.location = open_meteo_rs::Location {
        lat: 19.380580,
        lng: -14.345692,
    };

    let res = client.archive(opts).await;
    assert!(res.is_ok());
}

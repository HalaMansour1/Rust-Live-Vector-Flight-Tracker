use crate::aircraft::Aircraft;
use crate::geo::Location;
use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::Deserialize;
use std::time::Duration;
use tracing::{debug, info};

const OPENSKY_BASE_URL: &str = "https://opensky-network.org/api";

#[derive(Debug, Clone)]
pub struct OpenSkyApi {
    client: Client,
    username: Option<String>,
    password: Option<String>,
}

impl OpenSkyApi {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .unwrap_or_default(),
            username: None,
            password: None,
        }
    }

    pub fn with_credentials(username: String, password: String) -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .unwrap_or_default(),
            username: Some(username),
            password: Some(password),
        }
    }

    pub async fn get_aircraft_in_radius(
        &self,
        location: &Location,
        radius_km: f64,
    ) -> Result<Vec<Aircraft>> {
        let url = format!("{}/states/all", OPENSKY_BASE_URL);
        
        let mut request = self.client.get(&url);
        
        // Add authentication if available
        if let (Some(username), Some(password)) = (&self.username, &self.password) {
            request = request.basic_auth(username, Some(password));
        }

        let response = request
            .query(&[
                ("lamin", &(location.lat - radius_km / 111.0).to_string()),
                ("lamax", &(location.lat + radius_km / 111.0).to_string()),
                ("lomin", &(location.lon - radius_km / (111.0 * location.lat.cos())).to_string()),
                ("lomax", &(location.lon + radius_km / (111.0 * location.lat.cos())).to_string()),
            ])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow!(
                "OpenSky API request failed with status: {}",
                response.status()
            ));
        }

        let states_response: StatesResponse = response.json().await?;
        
        if let Some(states) = states_response.states {
            let aircraft: Vec<Aircraft> = states
                .into_iter()
                .map(|state| state.into())
                .collect();
            
            info!("Retrieved {} aircraft from OpenSky API", aircraft.len());
            Ok(aircraft)
        } else {
            debug!("No aircraft data received from OpenSky API");
            Ok(Vec::new())
        }
    }

    pub async fn get_aircraft_by_icao24(&self, icao24: &str) -> Result<Option<Aircraft>> {
        let url = format!("{}/states/all", OPENSKY_BASE_URL);
        
        let mut request = self.client.get(&url);
        
        if let (Some(username), Some(password)) = (&self.username, &self.password) {
            request = request.basic_auth(username, Some(password));
        }

        let response = request
            .query(&[("icao24", icao24)])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow!(
                "OpenSky API request failed with status: {}",
                response.status()
            ));
        }

        let states_response: StatesResponse = response.json().await?;
        
        Ok(states_response
            .states
            .and_then(|states| states.into_iter().next())
            .map(|state| state.into()))
    }
}

#[derive(Debug, Deserialize)]
struct StatesResponse {
    time: Option<i64>,
    states: Option<Vec<StateData>>,
}

#[derive(Debug, Deserialize)]
struct StateData(
    String,    // icao24
    String,    // callsign
    String,    // origin_country
    i64,       // time_position
    i64,       // time_velocity
    f64,       // longitude
    f64,       // latitude
    f64,       // altitude
    f64,       // velocity
    f64,       // true_track
    f64,       // vertical_rate
    Vec<String>, // sensors
    f64,       // geo_altitude
    String,    // squawk
    bool,      // spi
    i32,       // position_source
    i32,       // category
);

impl From<StateData> for Aircraft {
    fn from(state: StateData) -> Self {
        Self {
            icao24: state.0,
            callsign: if state.1.is_empty() { None } else { Some(state.1) },
            origin_country: if state.2.is_empty() { None } else { Some(state.2) },
            time_position: if state.3 > 0 {
                Some(DateTime::from_timestamp(state.3, 0).unwrap_or_else(|| Utc::now()))
            } else {
                None
            },
            time_velocity: if state.4 > 0 {
                Some(DateTime::from_timestamp(state.4, 0).unwrap_or_else(|| Utc::now()))
            } else {
                None
            },
            longitude: if state.5 != 0.0 { Some(state.5) } else { None },
            latitude: if state.6 != 0.0 { Some(state.6) } else { None },
            altitude: if state.7 != 0.0 { Some(state.7) } else { None },
            velocity: if state.8 != 0.0 { Some(state.8) } else { None },
            true_track: if state.9 != 0.0 { Some(state.9) } else { None },
            vertical_rate: if state.10 != 0.0 { Some(state.10) } else { None },
            sensors: if state.11.is_empty() { None } else { Some(state.11) },
            geo_altitude: if state.12 != 0.0 { Some(state.12) } else { None },
            squawk: if state.13.is_empty() { None } else { Some(state.13) },
            spi: Some(state.14),
            position_source: Some(state.15),
            category: Some(state.16),
        }
    }
}

// Mock API for testing when no internet connection is available
#[derive(Debug, Clone)]
pub struct MockApi;

impl MockApi {
    pub fn new() -> Self {
        Self
    }

    pub async fn get_aircraft_in_radius(
        &self,
        _location: &Location,
        _radius_km: f64,
    ) -> Result<Vec<Aircraft>> {
        // Generate some mock aircraft data
        let mock_aircraft = vec![
            Aircraft {
                icao24: "a12345".to_string(),
                callsign: Some("TEST123".to_string()),
                origin_country: Some("United States".to_string()),
                time_position: Some(Utc::now()),
                time_velocity: Some(Utc::now()),
                longitude: Some(-122.4194),
                latitude: Some(37.7749),
                altitude: Some(35000.0),
                velocity: Some(450.0),
                true_track: Some(90.0),
                vertical_rate: Some(0.0),
                sensors: Some(vec!["ADSB".to_string()]),
                geo_altitude: Some(35000.0),
                squawk: Some("1234".to_string()),
                spi: Some(false),
                position_source: Some(0),
                category: Some(0),
            },
            Aircraft {
                icao24: "b67890".to_string(),
                callsign: Some("DEMO456".to_string()),
                origin_country: Some("Canada".to_string()),
                time_position: Some(Utc::now()),
                time_velocity: Some(Utc::now()),
                longitude: Some(-122.4000),
                latitude: Some(37.7800),
                altitude: Some(28000.0),
                velocity: Some(380.0),
                true_track: Some(180.0),
                vertical_rate: Some(-500.0),
                sensors: Some(vec!["ADSB".to_string()]),
                geo_altitude: Some(28000.0),
                squawk: Some("5678".to_string()),
                spi: Some(false),
                position_source: Some(0),
                category: Some(0),
            },
        ];

        info!("Generated {} mock aircraft", mock_aircraft.len());
        Ok(mock_aircraft)
    }
} 
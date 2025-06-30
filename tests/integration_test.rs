use skyradar::aircraft::{Aircraft, AltitudeBand};
use skyradar::config::AppConfig;
use skyradar::geo::Location;

#[test]
fn test_aircraft_altitude_bands() {
    let mut aircraft = Aircraft::new("test123".to_string());
    
    // Test low altitude
    aircraft.altitude = Some(500.0);
    assert_eq!(aircraft.altitude_band(), AltitudeBand::Low);
    
    // Test medium altitude
    aircraft.altitude = Some(5000.0);
    assert_eq!(aircraft.altitude_band(), AltitudeBand::Medium);
    
    // Test high altitude
    aircraft.altitude = Some(15000.0);
    assert_eq!(aircraft.altitude_band(), AltitudeBand::High);
    
    // Test very high altitude
    aircraft.altitude = Some(35000.0);
    assert_eq!(aircraft.altitude_band(), AltitudeBand::VeryHigh);
    
    // Test unknown altitude
    aircraft.altitude = None;
    assert_eq!(aircraft.altitude_band(), AltitudeBand::Unknown);
}

#[test]
fn test_location_distance_calculation() {
    let sf = Location::san_francisco();
    let ny = Location::new_york();
    
    let distance = sf.distance_to(&ny);
    
    // Distance should be roughly 4000-5000 km
    assert!(distance > 4000.0 && distance < 5000.0);
}

#[test]
fn test_config_defaults() {
    let config = AppConfig::default();
    
    assert_eq!(config.refresh_interval_seconds, 30);
    assert_eq!(config.radar_radius_km, 8.0);
    assert!(config.show_trails);
    assert_eq!(config.trail_length, 10);
    assert_eq!(config.theme, skyradar::config::Theme::Dark);
}

#[test]
fn test_aircraft_display_name() {
    let mut aircraft = Aircraft::new("test123".to_string());
    
    // Test with callsign
    aircraft.callsign = Some("TEST123".to_string());
    assert_eq!(aircraft.display_name(), "TEST123");
    
    // Test without callsign (should use ICAO24)
    aircraft.callsign = None;
    assert_eq!(aircraft.display_name(), "test123");
    
    // Test with empty callsign
    aircraft.callsign = Some("".to_string());
    assert_eq!(aircraft.display_name(), "test123");
}

#[test]
fn test_aircraft_position() {
    let mut aircraft = Aircraft::new("test123".to_string());
    
    // Test without position
    assert!(aircraft.position().is_none());
    
    // Test with position
    aircraft.longitude = Some(-122.4194);
    aircraft.latitude = Some(37.7749);
    
    let position = aircraft.position().unwrap();
    assert_eq!(position.x(), -122.4194);
    assert_eq!(position.y(), 37.7749);
}

#[test]
fn test_aircraft_activity() {
    let mut aircraft = Aircraft::new("test123".to_string());
    
    // Test without timestamp
    assert!(!aircraft.is_active());
    
    // Test with recent timestamp
    aircraft.time_position = Some(chrono::Utc::now());
    assert!(aircraft.is_active());
    
    // Test with old timestamp
    aircraft.time_position = Some(chrono::Utc::now() - chrono::Duration::minutes(10));
    assert!(!aircraft.is_active());
} 
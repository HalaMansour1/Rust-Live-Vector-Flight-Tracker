use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use geo_types::Point;
use egui::Color32;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Aircraft {
    pub icao24: String,
    pub callsign: Option<String>,
    pub origin_country: Option<String>,
    pub time_position: Option<DateTime<Utc>>,
    pub time_velocity: Option<DateTime<Utc>>,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    pub altitude: Option<f64>,
    pub velocity: Option<f64>,
    pub true_track: Option<f64>,
    pub vertical_rate: Option<f64>,
    pub sensors: Option<Vec<String>>,
    pub geo_altitude: Option<f64>,
    pub squawk: Option<String>,
    pub spi: Option<bool>,
    pub position_source: Option<i32>,
    pub category: Option<i32>,
}

impl Aircraft {
    pub fn new(icao24: String) -> Self {
        Self {
            icao24,
            callsign: None,
            origin_country: None,
            time_position: None,
            time_velocity: None,
            longitude: None,
            latitude: None,
            altitude: None,
            velocity: None,
            true_track: None,
            vertical_rate: None,
            sensors: None,
            geo_altitude: None,
            squawk: None,
            spi: None,
            position_source: None,
            category: None,
        }
    }

    pub fn has_position(&self) -> bool {
        self.longitude.is_some() && self.latitude.is_some()
    }

    pub fn position(&self) -> Option<Point<f64>> {
        match (self.longitude, self.latitude) {
            (Some(lon), Some(lat)) => Some(Point::new(lon, lat)),
            _ => None,
        }
    }

    pub fn altitude_band(&self) -> AltitudeBand {
        match self.altitude {
            Some(alt) if alt < 1000.0 => AltitudeBand::Low,
            Some(alt) if alt < 10000.0 => AltitudeBand::Medium,
            Some(alt) if alt < 25000.0 => AltitudeBand::High,
            Some(_) => AltitudeBand::VeryHigh,
            None => AltitudeBand::Unknown,
        }
    }

    pub fn display_name(&self) -> String {
        self.callsign
            .as_ref()
            .map(|c| c.trim().to_string())
            .unwrap_or_else(|| self.icao24.clone())
    }

    pub fn is_active(&self) -> bool {
        if let Some(time) = self.time_position {
            let now = Utc::now();
            (now - time).num_seconds() < 300 // 5 minutes
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AltitudeBand {
    Low,      // 0-1000 ft
    Medium,   // 1000-10000 ft
    High,     // 10000-25000 ft
    VeryHigh, // 25000+ ft
    Unknown,
}

impl AltitudeBand {
    pub fn color(&self) -> Color32 {
        match self {
            AltitudeBand::Low => Color32::from_rgb(255, 255, 0),      // Yellow
            AltitudeBand::Medium => Color32::from_rgb(0, 255, 0),     // Green
            AltitudeBand::High => Color32::from_rgb(0, 255, 255),     // Cyan
            AltitudeBand::VeryHigh => Color32::from_rgb(255, 0, 255), // Magenta
            AltitudeBand::Unknown => Color32::from_rgb(128, 128, 128), // Gray
        }
    }
}

#[derive(Debug, Clone)]
pub struct AircraftTrail {
    pub icao24: String,
    pub positions: Vec<(Point<f64>, DateTime<Utc>)>,
    pub max_points: usize,
}

impl AircraftTrail {
    pub fn new(icao24: String, max_points: usize) -> Self {
        Self {
            icao24,
            positions: Vec::new(),
            max_points,
        }
    }

    pub fn add_position(&mut self, position: Point<f64>, timestamp: DateTime<Utc>) {
        self.positions.push((position, timestamp));
        if self.positions.len() > self.max_points {
            self.positions.remove(0);
        }
    }

    pub fn clear(&mut self) {
        self.positions.clear();
    }
} 
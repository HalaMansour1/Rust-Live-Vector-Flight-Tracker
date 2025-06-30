use geo::Point;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub lat: f64,
    pub lon: f64,
    pub name: Option<String>,
}

impl Location {
    pub fn new(lat: f64, lon: f64) -> Self {
        Self {
            lat,
            lon,
            name: None,
        }
    }

    pub fn with_name(lat: f64, lon: f64, name: String) -> Self {
        Self {
            lat,
            lon,
            name: Some(name),
        }
    }

    pub fn distance_to(&self, other: &Location) -> f64 {
        let lat1 = self.lat.to_radians();
        let lon1 = self.lon.to_radians();
        let lat2 = other.lat.to_radians();
        let lon2 = other.lon.to_radians();

        let dlat = lat2 - lat1;
        let dlon = lon2 - lon1;

        let a = (dlat / 2.0).sin().powi(2)
            + lat1.cos() * lat2.cos() * (dlon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().asin();

        // Earth's radius in kilometers
        6371.0 * c
    }

    pub fn bearing_to(&self, other: &Location) -> f64 {
        let lat1 = self.lat.to_radians();
        let lon1 = self.lon.to_radians();
        let lat2 = other.lat.to_radians();
        let lon2 = other.lon.to_radians();

        let dlon = lon2 - lon1;

        let y = dlon.sin() * lat2.cos();
        let x = lat1.cos() * lat2.sin() - lat1.sin() * lat2.cos() * dlon.cos();

        let bearing = y.atan2(x).to_degrees();
        (bearing + 360.0) % 360.0
    }

    pub fn point_at_distance(&self, distance_km: f64, bearing_degrees: f64) -> Location {
        let lat1 = self.lat.to_radians();
        let lon1 = self.lon.to_radians();
        let bearing = bearing_degrees.to_radians();
        let angular_distance = distance_km / 6371.0; // Earth's radius in km

        let lat2 = (lat1.sin() * angular_distance.cos()
            + lat1.cos() * angular_distance.sin() * bearing.cos())
        .asin();

        let lon2 = lon1
            + (angular_distance.sin() * bearing.sin() * lat1.cos()).atan2(
                angular_distance.cos() - lat1.sin() * lat2.sin(),
            );

        Location::new(lat2.to_degrees(), lon2.to_degrees())
    }

    pub fn to_point(&self) -> Point<f64> {
        Point::new(self.lon, self.lat)
    }

    pub fn from_point(point: Point<f64>) -> Self {
        Self::new(point.y(), point.x())
    }
}

impl From<Point<f64>> for Location {
    fn from(point: Point<f64>) -> Self {
        Self::from_point(point)
    }
}

impl From<Location> for Point<f64> {
    fn from(location: Location) -> Self {
        location.to_point()
    }
}

// Common locations
impl Location {
    pub fn san_francisco() -> Self {
        Self::with_name(37.7749, -122.4194, "San Francisco".to_string())
    }

    pub fn new_york() -> Self {
        Self::with_name(40.7128, -74.0060, "New York".to_string())
    }

    pub fn london() -> Self {
        Self::with_name(51.5074, -0.1278, "London".to_string())
    }

    pub fn tokyo() -> Self {
        Self::with_name(35.6762, 139.6503, "Tokyo".to_string())
    }

    pub fn sydney() -> Self {
        Self::with_name(-33.8688, 151.2093, "Sydney".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_calculation() {
        let sf = Location::san_francisco();
        let ny = Location::new_york();
        let distance = sf.distance_to(&ny);
        
        // Distance should be roughly 4000-5000 km
        assert!(distance > 4000.0 && distance < 5000.0);
    }

    #[test]
    fn test_bearing_calculation() {
        let sf = Location::san_francisco();
        let ny = Location::new_york();
        let bearing = sf.bearing_to(&ny);
        
        // Bearing should be roughly east (90 degrees)
        assert!(bearing > 60.0 && bearing < 120.0);
    }

    #[test]
    fn test_point_at_distance() {
        let sf = Location::san_francisco();
        let point = sf.point_at_distance(100.0, 90.0); // 100km east
        
        // Should be east of San Francisco
        assert!(point.lon > sf.lon);
        assert!((point.lat - sf.lat).abs() < 1.0); // Should be roughly same latitude
    }
} 
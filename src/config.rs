use crate::geo::Location;
use anyhow::Result;
use config::{Config, Environment, File};
use dirs::config_dir;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub location: Location,
    pub refresh_interval_seconds: u64,
    pub radar_radius_km: f64,
    pub show_trails: bool,
    pub trail_length: usize,
    pub theme: Theme,
    pub api_credentials: Option<ApiCredentials>,
    pub window_size: Option<WindowSize>,
    pub auto_refresh: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiCredentials {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowSize {
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Theme {
    Dark,
    Light,
    Auto,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            location: Location::san_francisco(),
            refresh_interval_seconds: 30,
            radar_radius_km: 8.0, // 5 miles ≈ 8 km
            show_trails: true,
            trail_length: 10,
            theme: Theme::Dark,
            api_credentials: None,
            window_size: None,
            auto_refresh: true,
        }
    }
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_file_path();
        
        let config = Config::builder()
            .add_source(File::from(config_path.as_path()).required(false))
            .add_source(Environment::with_prefix("SKYRADAR"))
            .build()?;

        let mut app_config: AppConfig = config.try_deserialize()?;
        
        // Ensure reasonable defaults
        if app_config.refresh_interval_seconds < 10 {
            app_config.refresh_interval_seconds = 30;
        }
        if app_config.radar_radius_km < 1.0 || app_config.radar_radius_km > 100.0 {
            app_config.radar_radius_km = 8.0;
        }
        if app_config.trail_length > 50 {
            app_config.trail_length = 10;
        }

        Ok(app_config)
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_file_path();
        
        // Ensure config directory exists
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let config_str = toml::to_string_pretty(self)?;
        std::fs::write(config_path, config_str)?;
        
        Ok(())
    }

    pub fn config_file_path() -> PathBuf {
        config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("skyradar")
            .join("config.toml")
    }

    pub fn set_location(&mut self, location: Location) {
        self.location = location;
    }

    pub fn set_refresh_interval(&mut self, seconds: u64) {
        self.refresh_interval_seconds = seconds.max(10);
    }

    pub fn set_radar_radius(&mut self, radius_km: f64) {
        self.radar_radius_km = radius_km.clamp(1.0, 100.0);
    }

    pub fn set_theme(&mut self, theme: Theme) {
        self.theme = theme;
    }

    pub fn set_api_credentials(&mut self, username: String, password: String) {
        self.api_credentials = Some(ApiCredentials { username, password });
    }

    pub fn clear_api_credentials(&mut self) {
        self.api_credentials = None;
    }

    pub fn set_window_size(&mut self, width: f32, height: f32) {
        self.window_size = Some(WindowSize { width, height });
    }

    pub fn toggle_trails(&mut self) {
        self.show_trails = !self.show_trails;
    }

    pub fn toggle_auto_refresh(&mut self) {
        self.auto_refresh = !self.auto_refresh;
    }
}

impl Theme {
    pub fn is_dark(&self) -> bool {
        match self {
            Theme::Dark => true,
            Theme::Light => false,
            Theme::Auto => {
                // Simple heuristic - could be improved with system detection
                true
            }
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Theme::Dark => "Dark",
            Theme::Light => "Light",
            Theme::Auto => "Auto",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert_eq!(config.refresh_interval_seconds, 30);
        assert_eq!(config.radar_radius_km, 8.0);
        assert!(config.show_trails);
        assert_eq!(config.trail_length, 10);
        assert_eq!(config.theme, Theme::Dark);
    }

    #[test]
    fn test_theme_is_dark() {
        assert!(Theme::Dark.is_dark());
        assert!(!Theme::Light.is_dark());
        assert!(Theme::Auto.is_dark()); // Our simple heuristic
    }

    #[test]
    fn test_config_validation() {
        let mut config = AppConfig::default();
        
        // Test refresh interval validation
        config.set_refresh_interval(5); // Should be clamped to 10
        assert_eq!(config.refresh_interval_seconds, 10);
        
        // Test radar radius validation
        config.set_radar_radius(150.0); // Should be clamped to 100
        assert_eq!(config.radar_radius_km, 100.0);
        
        config.set_radar_radius(0.5); // Should be clamped to 1
        assert_eq!(config.radar_radius_km, 1.0);
    }
} 
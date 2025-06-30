use crate::aircraft::Aircraft;
use crate::api::{MockApi, OpenSkyApi};
use crate::config::{AppConfig, Theme};
use crate::geo::Location;
use crate::radar_view::RadarView;
use crate::theme::apply_theme;
use anyhow::Result;
use chrono::{DateTime, Utc};
use eframe::egui;
use std::time::{Duration, Instant};
use tokio::runtime::Runtime;
use tracing::{error, info, warn};

pub struct SkyRadarApp {
    config: AppConfig,
    aircraft: Vec<Aircraft>,
    radar_view: RadarView,
    last_update: Option<DateTime<Utc>>,
    last_refresh: Instant,
    selected_aircraft: Option<String>,
    show_settings: bool,
    show_aircraft_list: bool,
    runtime: Runtime,
    api: Box<dyn AircraftApi>,
    refresh_timer: f32,
    status_message: String,
    is_loading: bool,
}

trait AircraftApi: Send + Sync {
    fn get_aircraft_in_radius(
        &self,
        location: &Location,
        radius_km: f64,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Vec<Aircraft>>> + Send>>;
}

impl AircraftApi for OpenSkyApi {
    fn get_aircraft_in_radius(
        &self,
        location: &Location,
        radius_km: f64,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Vec<Aircraft>>> + Send>> {
        let api = self.clone();
        let location = location.clone();
        Box::pin(async move { api.get_aircraft_in_radius(&location, radius_km).await })
    }
}

impl AircraftApi for MockApi {
    fn get_aircraft_in_radius(
        &self,
        location: &Location,
        radius_km: f64,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Vec<Aircraft>>> + Send>> {
        let api = self.clone();
        let location = location.clone();
        Box::pin(async move { api.get_aircraft_in_radius(&location, radius_km).await })
    }
}

impl SkyRadarApp {
    pub fn new() -> Self {
        let runtime = Runtime::new().expect("Failed to create async runtime");
        
        // Load configuration
        let config = AppConfig::load().unwrap_or_else(|e| {
            warn!("Failed to load config: {}, using defaults", e);
            AppConfig::default()
        });

        // Initialize API
        let api: Box<dyn AircraftApi> = if let Some(creds) = &config.api_credentials {
            Box::new(OpenSkyApi::with_credentials(
                creds.username.clone(),
                creds.password.clone(),
            ))
        } else {
            info!("No API credentials found, using mock data");
            Box::new(MockApi::new())
        };

        let radar_view = RadarView::new(egui::Rect::NOTHING); // Will be updated in update()

        Self {
            config,
            aircraft: Vec::new(),
            radar_view,
            last_update: None,
            last_refresh: Instant::now(),
            selected_aircraft: None,
            show_settings: false,
            show_aircraft_list: true,
            runtime,
            api,
            refresh_timer: 0.0,
            status_message: "Initializing...".to_string(),
            is_loading: false,
        }
    }

    fn update(&mut self, ctx: &egui::Context) {
        // Update radar view with current window size
        let available_rect = ctx.available_rect();
        self.radar_view = RadarView::new(available_rect);

        // Apply theme
        ctx.style_mut(|style| {
            apply_theme(&mut style.visuals, self.config.theme.is_dark());
        });

        // Handle auto-refresh
        if self.config.auto_refresh {
            let elapsed = self.last_refresh.elapsed();
            if elapsed >= Duration::from_secs(self.config.refresh_interval_seconds) {
                self.refresh_aircraft_data();
                self.last_refresh = Instant::now();
            }
        }

        // Update refresh timer for UI
        let elapsed = self.last_refresh.elapsed();
        self.refresh_timer = elapsed.as_secs_f32();
    }

    fn refresh_aircraft_data(&mut self) {
        if self.is_loading {
            return;
        }

        self.is_loading = true;
        self.status_message = "Fetching aircraft data...".to_string();

        // For now, we'll use mock data
        self.aircraft = vec![
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
        ];

        self.last_update = Some(Utc::now());
        self.status_message = format!("Last updated: {}", self.last_update.unwrap().format("%H:%M:%S"));
        self.is_loading = false;
    }

    fn draw_main_window(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("SkyRadar - Live Aircraft Tracker");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("⚙️ Settings").clicked() {
                        self.show_settings = !self.show_settings;
                    }
                    if ui.button("📋 Aircraft").clicked() {
                        self.show_aircraft_list = !self.show_aircraft_list;
                    }
                    if ui.button("🔄 Refresh").clicked() {
                        self.refresh_aircraft_data();
                    }
                });
            });

            ui.separator();

            // Status bar
            ui.horizontal(|ui| {
                ui.label(&self.status_message);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if self.config.auto_refresh {
                        let remaining = self.config.refresh_interval_seconds as f32 - self.refresh_timer;
                        ui.label(format!("Auto-refresh in {:.0}s", remaining.max(0.0)));
                    }
                });
            });

            ui.separator();

            // Main content area
            ui.horizontal(|ui| {
                // Radar view (takes most space)
                ui.vertical(|ui| {
                    ui.label("Radar View");
                    self.radar_view.draw(ui, &self.aircraft, &self.config, &self.config.location);
                });

                // Side panels
                if self.show_aircraft_list {
                    ui.vertical(|ui| {
                        ui.label("Aircraft List");
                        self.draw_aircraft_list(ui);
                    });
                }
            });
        });
    }

    fn draw_aircraft_list(&mut self, ui: &mut egui::Ui) {
        ui.set_enabled(!self.is_loading);

        egui::ScrollArea::vertical().show(ui, |ui| {
            if self.aircraft.is_empty() {
                ui.label("No aircraft detected");
                return;
            }

            for aircraft in &self.aircraft {
                ui.horizontal(|ui| {
                    let is_selected = self.selected_aircraft.as_ref() == Some(&aircraft.icao24);
                    
                    if ui.selectable_label(is_selected, &aircraft.display_name()).clicked() {
                        self.selected_aircraft = Some(aircraft.icao24.clone());
                    }

                    if let Some(altitude) = aircraft.altitude {
                        ui.label(format!("{:.0}ft", altitude));
                    }
                });

                if let Some(selected) = &self.selected_aircraft {
                    if selected == &aircraft.icao24 {
                        ui.indent("details", |ui| {
                            if let Some(callsign) = &aircraft.callsign {
                                ui.label(format!("Callsign: {}", callsign));
                            }
                            if let Some(country) = &aircraft.origin_country {
                                ui.label(format!("Country: {}", country));
                            }
                            if let Some(speed) = aircraft.velocity {
                                ui.label(format!("Speed: {:.0} km/h", speed));
                            }
                            if let Some(heading) = aircraft.true_track {
                                ui.label(format!("Heading: {:.0}°", heading));
                            }
                            if let Some(squawk) = &aircraft.squawk {
                                ui.label(format!("Squawk: {}", squawk));
                            }
                        });
                    }
                }
            }
        });
    }

    fn draw_settings_window(&mut self, ctx: &egui::Context) {
        if !self.show_settings {
            return;
        }

        egui::Window::new("Settings")
            .open(&mut self.show_settings)
            .show(ctx, |ui| {
                ui.label("Location");
                ui.horizontal(|ui| {
                    ui.label("Latitude:");
                    let mut lat = self.config.location.lat;
                    if ui.add(egui::DragValue::new(&mut lat).speed(0.1)).changed() {
                        self.config.location.lat = lat;
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("Longitude:");
                    let mut lon = self.config.location.lon;
                    if ui.add(egui::DragValue::new(&mut lon).speed(0.1)).changed() {
                        self.config.location.lon = lon;
                    }
                });

                ui.separator();

                ui.label("Radar Settings");
                ui.horizontal(|ui| {
                    ui.label("Radius (km):");
                    let mut radius = self.config.radar_radius_km;
                    if ui.add(egui::DragValue::new(&mut radius).speed(0.5).clamp_range(1.0..=100.0)).changed() {
                        self.config.set_radar_radius(radius);
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Refresh interval (seconds):");
                    let mut interval = self.config.refresh_interval_seconds;
                    if ui.add(egui::DragValue::new(&mut interval).speed(1.0).clamp_range(10..=300)).changed() {
                        self.config.set_refresh_interval(interval);
                    }
                });

                ui.checkbox(&mut self.config.auto_refresh, "Auto-refresh");
                ui.checkbox(&mut self.config.show_trails, "Show aircraft trails");

                ui.separator();

                ui.label("Theme");
                egui::ComboBox::from_label("Theme")
                    .selected_text(self.config.theme.name())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.config.theme, Theme::Dark, "Dark");
                        ui.selectable_value(&mut self.config.theme, Theme::Light, "Light");
                        ui.selectable_value(&mut self.config.theme, Theme::Auto, "Auto");
                    });

                ui.separator();

                ui.horizontal(|ui| {
                    if ui.button("Save Settings").clicked() {
                        if let Err(e) = self.config.save() {
                            error!("Failed to save settings: {}", e);
                        } else {
                            info!("Settings saved successfully");
                        }
                    }
                    if ui.button("Reset to Defaults").clicked() {
                        self.config = AppConfig::default();
                    }
                });
            });
    }
}

impl eframe::App for SkyRadarApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.update(ctx);
        self.draw_main_window(ctx);
        self.draw_settings_window(ctx);
    }
} 
use crate::aircraft::{Aircraft, AircraftTrail};
use crate::config::AppConfig;
use crate::geo::Location;
use egui::{Color32, Painter, Pos2, Rect, Sense, Shape, Stroke, Ui, Vec2};
use geo::Point;
use std::collections::HashMap;
use std::f64::consts::PI;

pub struct RadarView {
    center: Pos2,
    radius: f32,
    scale: f32,
    aircraft_trails: HashMap<String, AircraftTrail>,
}

impl RadarView {
    pub fn new(rect: Rect) -> Self {
        let center = rect.center();
        let radius = rect.width().min(rect.height()) * 0.4;
        
        Self {
            center,
            radius,
            scale: 1.0,
            aircraft_trails: HashMap::new(),
        }
    }

    pub fn update_trails(&mut self, aircraft: &[Aircraft], config: &AppConfig) {
        for aircraft in aircraft {
            if let Some(position) = aircraft.position() {
                let trail = self.aircraft_trails
                    .entry(aircraft.icao24.clone())
                    .or_insert_with(|| AircraftTrail::new(aircraft.icao24.clone(), config.trail_length));
                
                trail.add_position(position, aircraft.time_position.unwrap_or_else(chrono::Utc::now));
            }
        }
    }

    pub fn draw(&self, ui: &mut Ui, aircraft: &[Aircraft], config: &AppConfig, user_location: &Location) -> egui::Response {
        let (response, painter) = ui.allocate_painter(
            Vec2::new(ui.available_width(), ui.available_height()),
            Sense::click_and_drag(),
        );

        self.draw_radar_background(&painter, config);
        self.draw_range_rings(&painter, config);
        self.draw_compass_rose(&painter);
        
        if config.show_trails {
            self.draw_aircraft_trails(&painter, user_location);
        }
        
        self.draw_aircraft(&painter, aircraft, user_location);
        self.draw_center_marker(&painter, user_location);
        
        response
    }

    fn draw_radar_background(&self, painter: &Painter, config: &AppConfig) {
        let background_color = if config.theme.is_dark() {
            Color32::from_rgb(20, 20, 30)
        } else {
            Color32::from_rgb(240, 240, 250)
        };

        painter.rect_filled(
            Rect::from_center_size(self.center, Vec2::splat(self.radius * 2.0)),
            self.radius,
            background_color,
        );
    }

    fn draw_range_rings(&self, painter: &Painter, config: &AppConfig) {
        let ring_color = if config.theme.is_dark() {
            Color32::from_rgb(60, 60, 80)
        } else {
            Color32::from_rgb(200, 200, 220)
        };

        let stroke = Stroke::new(1.0, ring_color);

        // Draw range rings at 1, 2, 3, 4, and 5 miles
        for i in 1..=5 {
            let ring_radius = self.radius * (i as f32 / 5.0);
            painter.circle_stroke(self.center, ring_radius, stroke);
            
            // Add range labels
            let label_pos = Pos2::new(
                self.center.x + ring_radius * 0.7,
                self.center.y - ring_radius * 0.7,
            );
            painter.text(
                label_pos,
                egui::Align2::CENTER_CENTER,
                &format!("{} mi", i),
                egui::FontId::proportional(12.0),
                ring_color,
            );
        }
    }

    fn draw_compass_rose(&self, painter: &Painter) {
        let directions = ["N", "E", "S", "W"];
        let angles = [0.0, 90.0, 180.0, 270.0];
        
        for (direction, angle) in directions.iter().zip(angles.iter()) {
            let angle_rad = angle * PI / 180.0;
            let pos = Pos2::new(
                self.center.x + (self.radius + 20.0) * angle_rad.sin() as f32,
                self.center.y - (self.radius + 20.0) * angle_rad.cos() as f32,
            );
            
            painter.text(
                pos,
                egui::Align2::CENTER_CENTER,
                direction,
                egui::FontId::proportional(14.0),
                Color32::WHITE,
            );
        }
    }

    fn draw_aircraft_trails(&self, painter: &Painter, user_location: &Location) {
        for trail in self.aircraft_trails.values() {
            if trail.positions.len() < 2 {
                continue;
            }

            let mut points = Vec::new();
            for (position, _) in &trail.positions {
                if let Some(screen_pos) = self.geo_to_screen(position, user_location) {
                    points.push(screen_pos);
                }
            }

            if points.len() >= 2 {
                let trail_color = Color32::from_rgba_premultiplied(100, 100, 255, 100);
                let stroke = Stroke::new(2.0, trail_color);
                painter.add(Shape::line(points, stroke));
            }
        }
    }

    fn draw_aircraft(&self, painter: &Painter, aircraft: &[Aircraft], user_location: &Location) {
        for aircraft in aircraft {
            if let Some(position) = aircraft.position() {
                if let Some(screen_pos) = self.geo_to_screen(&position, user_location) {
                    self.draw_aircraft_icon(painter, aircraft, screen_pos);
                }
            }
        }
    }

    fn draw_aircraft_icon(&self, painter: &Painter, aircraft: &Aircraft, pos: Pos2) {
        let color = aircraft.altitude_band().color();
        let size = 8.0;
        
        // Draw aircraft as a triangle pointing in the direction of travel
        if let Some(heading) = aircraft.true_track {
            let heading_rad = heading * PI / 180.0;
            let points = vec![
                Pos2::new(
                    pos.x + size * heading_rad.cos() as f32,
                    pos.y - size * heading_rad.sin() as f32,
                ),
                Pos2::new(
                    pos.x + size * 0.5 * (heading_rad + 2.5).cos() as f32,
                    pos.y - size * 0.5 * (heading_rad + 2.5).sin() as f32,
                ),
                Pos2::new(
                    pos.x + size * 0.5 * (heading_rad - 2.5).cos() as f32,
                    pos.y - size * 0.5 * (heading_rad - 2.5).sin() as f32,
                ),
            ];
            
            painter.add(Shape::convex_polygon(
                points,
                color,
                Stroke::new(1.0, Color32::BLACK),
            ));
        } else {
            // Draw as a circle if no heading available
            painter.circle_filled(pos, size as f32, color);
        }

        // Draw aircraft label
        let label = aircraft.display_name();
        if !label.is_empty() {
            let label_pos = Pos2::new(pos.x, pos.y - 15.0);
            painter.text(
                label_pos,
                egui::Align2::CENTER_CENTER,
                &label,
                egui::FontId::proportional(10.0),
                Color32::WHITE,
            );
        }
    }

    fn draw_center_marker(&self, painter: &Painter, user_location: &Location) {
        // Draw center cross
        let cross_size = 10.0;
        let stroke = Stroke::new(2.0, Color32::WHITE);
        
        painter.line_segment(
            [Pos2::new(self.center.x - cross_size, self.center.y), 
             Pos2::new(self.center.x + cross_size, self.center.y)],
            stroke,
        );
        painter.line_segment(
            [Pos2::new(self.center.x, self.center.y - cross_size), 
             Pos2::new(self.center.x, self.center.y + cross_size)],
            stroke,
        );

        // Draw location label
        let location_name = user_location.name.as_deref().unwrap_or("Your Location");
        painter.text(
            Pos2::new(self.center.x, self.center.y + 25.0),
            egui::Align2::CENTER_CENTER,
            location_name,
            egui::FontId::proportional(12.0),
            Color32::WHITE,
        );
    }

    fn geo_to_screen(&self, geo_point: &Point<f64>, user_location: &Location) -> Option<Pos2> {
        // Simple projection - could be improved with proper map projection
        let lat_diff = geo_point.y() - user_location.lat;
        let lon_diff = geo_point.x() - user_location.lon;
        
        // Convert to screen coordinates (rough approximation)
        let x = self.center.x + (lon_diff * 1000000.0) as f32 * self.scale;
        let y = self.center.y - (lat_diff * 1000000.0) as f32 * self.scale;
        
        // Check if within radar range
        let distance = ((x - self.center.x).powi(2) + (y - self.center.y).powi(2)).sqrt();
        if distance <= self.radius {
            Some(Pos2::new(x, y))
        } else {
            None
        }
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale.clamp(0.1, 5.0);
    }

    pub fn zoom_in(&mut self) {
        self.set_scale(self.scale * 1.2);
    }

    pub fn zoom_out(&mut self) {
        self.set_scale(self.scale / 1.2);
    }

    pub fn clear_trails(&mut self) {
        self.aircraft_trails.clear();
    }
} 
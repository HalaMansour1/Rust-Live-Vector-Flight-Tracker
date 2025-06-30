use egui::{Color32, Stroke, Visuals};

pub fn apply_theme(visuals: &mut Visuals, is_dark: bool) {
    if is_dark {
        apply_dark_theme(visuals);
    } else {
        apply_light_theme(visuals);
    }
}

fn apply_dark_theme(visuals: &mut Visuals) {
    visuals.dark_mode = true;
    
    // Background colors
    visuals.panel_fill = Color32::from_rgb(30, 30, 40);
    visuals.window_fill = Color32::from_rgb(25, 25, 35);
    visuals.faint_bg_color = Color32::from_rgb(40, 40, 50);
    
    // Widget colors
    visuals.widgets.noninteractive.bg_fill = Color32::from_rgb(45, 45, 55);
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, Color32::from_rgb(60, 60, 70));
    
    visuals.widgets.inactive.bg_fill = Color32::from_rgb(50, 50, 60);
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, Color32::from_rgb(70, 70, 80));
    
    visuals.widgets.hovered.bg_fill = Color32::from_rgb(60, 60, 70);
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, Color32::from_rgb(80, 80, 90));
    
    visuals.widgets.active.bg_fill = Color32::from_rgb(70, 70, 80);
    visuals.widgets.active.bg_stroke = Stroke::new(1.0, Color32::from_rgb(90, 90, 100));
    
    // Selection colors
    visuals.selection.bg_fill = Color32::from_rgb(100, 100, 200);
    visuals.selection.stroke = Stroke::new(1.0, Color32::from_rgb(120, 120, 220));
    
    // Hyperlink colors
    visuals.hyperlink_color = Color32::from_rgb(100, 150, 255);
    
    // Code colors
    visuals.code_bg_color = Color32::from_rgb(20, 20, 30);
}

fn apply_light_theme(visuals: &mut Visuals) {
    visuals.dark_mode = false;
    
    // Background colors
    visuals.panel_fill = Color32::from_rgb(240, 240, 250);
    visuals.window_fill = Color32::from_rgb(250, 250, 255);
    visuals.faint_bg_color = Color32::from_rgb(230, 230, 240);
    
    // Widget colors
    visuals.widgets.noninteractive.bg_fill = Color32::from_rgb(220, 220, 230);
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, Color32::from_rgb(200, 200, 210));
    
    visuals.widgets.inactive.bg_fill = Color32::from_rgb(210, 210, 220);
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, Color32::from_rgb(190, 190, 200));
    
    visuals.widgets.hovered.bg_fill = Color32::from_rgb(200, 200, 210);
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, Color32::from_rgb(180, 180, 190));
    
    visuals.widgets.active.bg_fill = Color32::from_rgb(190, 190, 200);
    visuals.widgets.active.bg_stroke = Stroke::new(1.0, Color32::from_rgb(170, 170, 180));
    
    // Selection colors
    visuals.selection.bg_fill = Color32::from_rgb(150, 150, 255);
    visuals.selection.stroke = Stroke::new(1.0, Color32::from_rgb(130, 130, 235));
    
    // Hyperlink colors
    visuals.hyperlink_color = Color32::from_rgb(50, 100, 200);
    
    // Code colors
    visuals.code_bg_color = Color32::from_rgb(245, 245, 250);
}

// Aircraft-specific colors
pub struct AircraftColors {
    pub low_altitude: Color32,
    pub medium_altitude: Color32,
    pub high_altitude: Color32,
    pub very_high_altitude: Color32,
    pub unknown_altitude: Color32,
    pub trail: Color32,
    pub selected: Color32,
}

impl AircraftColors {
    pub fn new(is_dark: bool) -> Self {
        if is_dark {
            Self {
                low_altitude: Color32::from_rgb(255, 255, 0),      // Yellow
                medium_altitude: Color32::from_rgb(0, 255, 0),     // Green
                high_altitude: Color32::from_rgb(0, 255, 255),     // Cyan
                very_high_altitude: Color32::from_rgb(255, 0, 255), // Magenta
                unknown_altitude: Color32::from_rgb(128, 128, 128), // Gray
                trail: Color32::from_rgba_premultiplied(100, 100, 255, 100),
                selected: Color32::from_rgb(255, 255, 0),
            }
        } else {
            Self {
                low_altitude: Color32::from_rgb(200, 200, 0),      // Darker Yellow
                medium_altitude: Color32::from_rgb(0, 150, 0),     // Darker Green
                high_altitude: Color32::from_rgb(0, 150, 150),     // Darker Cyan
                very_high_altitude: Color32::from_rgb(150, 0, 150), // Darker Magenta
                unknown_altitude: Color32::from_rgb(100, 100, 100), // Darker Gray
                trail: Color32::from_rgba_premultiplied(50, 50, 200, 150),
                selected: Color32::from_rgb(200, 200, 0),
            }
        }
    }
}

// Radar-specific colors
pub struct RadarColors {
    pub background: Color32,
    pub range_rings: Color32,
    pub compass_rose: Color32,
    pub center_marker: Color32,
    pub grid_lines: Color32,
}

impl RadarColors {
    pub fn new(is_dark: bool) -> Self {
        if is_dark {
            Self {
                background: Color32::from_rgb(20, 20, 30),
                range_rings: Color32::from_rgb(60, 60, 80),
                compass_rose: Color32::from_rgb(200, 200, 200),
                center_marker: Color32::from_rgb(255, 255, 255),
                grid_lines: Color32::from_rgb(40, 40, 50),
            }
        } else {
            Self {
                background: Color32::from_rgb(240, 240, 250),
                range_rings: Color32::from_rgb(200, 200, 220),
                compass_rose: Color32::from_rgb(50, 50, 50),
                center_marker: Color32::from_rgb(30, 30, 30),
                grid_lines: Color32::from_rgb(220, 220, 230),
            }
        }
    }
} 
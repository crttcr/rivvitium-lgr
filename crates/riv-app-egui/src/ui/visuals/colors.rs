
use eframe::epaint::Stroke;
use egui::Color32;
use rand::Rng;

#[derive(Clone, Copy)]
pub enum ColorTheme {
	ForestCanopy,
	MinimalMono,
	Neutral,
	OceanBreeze,
	SunsetGlow,
}

impl ColorTheme {
	pub fn default() -> Self {
		ColorTheme::ForestCanopy
	}

	pub fn random() -> Self {
        // A const array means zero heap allocation.
        const ALL: &[ColorTheme] = &[
            ColorTheme::ForestCanopy,
            ColorTheme::MinimalMono,
            ColorTheme::Neutral,
            ColorTheme::OceanBreeze,
            ColorTheme::SunsetGlow,
        ];
        let mut rng = rand::rng();
        let i       = rng.random_range(0..ALL.len());
        ALL[i].clone()
    }
}


// Ocean Breeze ─ crisp blues

pub const OCEAN_BG:        Color32 = Color32::from_rgb(235, 238, 245);
pub const OCEAN_BUTTON:    Color32 = Color32::from_rgb( 28, 120, 210);
pub const OCEAN_TEXT:      Color32 = Color32::from_rgb(255, 255, 255);
pub const OCEAN_HIGHLIGHT: Color32 = Color32::from_rgb( 60, 180, 240);
pub const OCEAN_BORDER:    Color32 = Color32::from_rgb( 16,  80, 160);

// Sunset Glow ─ warm oranges

pub const SUNSET_BG:        Color32 = Color32::from_rgb(255, 244, 230);
pub const SUNSET_BUTTON:    Color32 = Color32::from_rgb(234,  92,  68);
pub const SUNSET_TEXT:      Color32 = Color32::from_rgb(255, 255, 255);
pub const SUNSET_HIGHLIGHT: Color32 = Color32::from_rgb(255, 142,  49);
pub const SUNSET_BORDER:    Color32 = Color32::from_rgb(196,  72,  52);

// Forest Canopy ─ calming greens

pub const FOREST_BG:        Color32 = Color32::from_rgb(236, 242, 235);
pub const FOREST_BUTTON:    Color32 = Color32::from_rgb( 34, 121,  71);
pub const FOREST_TEXT:      Color32 = Color32::from_rgb(255, 255, 255);
pub const FOREST_HIGHLIGHT: Color32 = Color32::from_rgb( 48, 159,  98);
pub const FOREST_BORDER:    Color32 = Color32::from_rgb( 24,  90,  54);

// Minimal Mono ─ neutral greys with a blue accent

pub const MONO_BG:        Color32 = Color32::from_rgb(245, 245, 245);
pub const MONO_BUTTON:    Color32 = Color32::from_rgb(224, 224, 224);
pub const MONO_TEXT:      Color32 = Color32::from_rgb( 20,  20,  20);
pub const MONO_HIGHLIGHT: Color32 = Color32::from_rgb( 96, 160, 255);
pub const MONO_BORDER:    Color32 = Color32::from_rgb(176, 176, 176);


/// BOGUS Color pallette
///
pub const ACCENT:         Color32 = Color32::from_rgb(128,   0, 192); // purple “Run” button
pub const ACCENT_BORDER:  Color32 = Color32::from_rgb( 90,   0, 140);
#[allow(dead_code)]
pub const ACCENT_HOVER:   Color32 = Color32::from_rgb(148,  30, 212);
#[allow(dead_code)]
pub const ACCENT_ACTIVE:  Color32 = Color32::from_rgb(108,   0, 162);
pub const TEXT_ON_ACCENT: Color32 = Color32::WHITE;

// Usage example:
/*
use my_theme::OCEAN_BUTTON;
ui.add(egui::Button::new("Run").fill(OCEAN_BUTTON));
*/



pub fn apply_color_theme(ui: &mut egui::Ui, theme: ColorTheme) {
	let v = &mut ui.visuals_mut().widgets;
	match theme {
		ColorTheme::ForestCanopy => {
			let text    = Stroke::new(1.0, FOREST_TEXT);
			let outline = Stroke::new(1.0, FOREST_BORDER);
			v.active.bg_fill              = FOREST_BUTTON;
			v.active.bg_stroke            = outline;
			v.active.fg_stroke            = text;
			v.hovered.bg_fill             = FOREST_HIGHLIGHT;
			v.inactive.bg_fill            = FOREST_BG;
			v.noninteractive.bg_fill      = FOREST_BG;
			v.active.weak_bg_fill         = FOREST_BUTTON;
			v.hovered.weak_bg_fill        = FOREST_HIGHLIGHT;
			v.inactive.weak_bg_fill       = FOREST_BG;
			v.noninteractive.weak_bg_fill = FOREST_BG;
		},
		ColorTheme::MinimalMono => {
			let text    = Stroke::new(1.0, MONO_TEXT);
			let outline = Stroke::new(1.0, MONO_BORDER);
			v.active.bg_fill        = MONO_BUTTON;
 			v.active.bg_stroke      = outline;
 			v.active.fg_stroke      = text;
			v.inactive.bg_fill      = MONO_BG;
			v.hovered.bg_fill       = MONO_HIGHLIGHT;
			v.inactive.weak_bg_fill = MONO_BG;
			v.hovered.weak_bg_fill  = MONO_HIGHLIGHT;
			v.active.weak_bg_fill   = MONO_BUTTON;
		},
		ColorTheme::Neutral => {
			let text    = Stroke::new(1.0, MONO_TEXT);
			let outline = Stroke::new(1.0, MONO_BORDER);
			v.active.bg_fill        = MONO_BUTTON;
 			v.active.bg_stroke      = outline;
 			v.active.fg_stroke      = text;
			v.inactive.bg_fill      = MONO_BG;
			v.hovered.bg_fill       = MONO_HIGHLIGHT;
			v.inactive.weak_bg_fill = MONO_BG;
			v.hovered.weak_bg_fill  = MONO_HIGHLIGHT;
			v.active.weak_bg_fill   = MONO_BUTTON;
		},
		ColorTheme::OceanBreeze => {
			let text    = Stroke::new(1.0, OCEAN_TEXT);
			let outline = Stroke::new(1.0, OCEAN_BORDER);
			v.active.bg_fill        = OCEAN_BUTTON;
			v.active.bg_stroke      = outline;
			v.active.fg_stroke      = text;
			v.inactive.bg_fill      = OCEAN_BG;
			v.hovered.bg_fill       = OCEAN_HIGHLIGHT;
			v.inactive.weak_bg_fill = OCEAN_BG;
			v.hovered.weak_bg_fill  = OCEAN_HIGHLIGHT;
			v.active.weak_bg_fill   = OCEAN_BUTTON;
		},
		ColorTheme::SunsetGlow => {
			let text    = Stroke::new(1.0, SUNSET_TEXT);
			let outline = Stroke::new(1.0, SUNSET_BORDER);
			v.active.bg_fill        = SUNSET_BUTTON;
			v.active.bg_stroke      = outline;
			v.active.fg_stroke      = text;
			v.inactive.bg_fill      = SUNSET_BG;
			v.hovered.bg_fill       = SUNSET_HIGHLIGHT;
			v.inactive.weak_bg_fill = SUNSET_BG;
			v.hovered.weak_bg_fill  = SUNSET_HIGHLIGHT;
			v.active.weak_bg_fill   = SUNSET_BUTTON;
		},
	}
}

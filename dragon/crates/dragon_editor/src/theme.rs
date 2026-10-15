//! Tema del editor: único origen de colores, tamaños y tipografías (principio VI).
//!
//! Ningún otro módulo define colores literales; todos usan estas constantes.

use bevy::feathers::constants::fonts;
use bevy::prelude::*;
use bevy::text::{FontSize, FontSource};

// Superficies, de la más profunda a la más elevada.
pub const WINDOW_BG: Color = Color::srgb(0.106, 0.106, 0.122);
pub const PANEL_BG: Color = Color::srgb(0.149, 0.149, 0.165);
pub const PANEL_HEADER_BG: Color = Color::srgb(0.122, 0.122, 0.141);
pub const BAR_BG: Color = Color::srgb(0.122, 0.122, 0.141);
pub const ROW_HOVER_BG: Color = Color::srgb(0.196, 0.196, 0.216);
pub const BORDER: Color = Color::srgb(0.204, 0.204, 0.220);

// Acento único.
pub const ACCENT: Color = Color::srgb(0.126, 0.431, 0.784);

// Texto.
pub const TEXT_MAIN: Color = Color::srgb(0.878, 0.878, 0.890);
pub const TEXT_DIM: Color = Color::srgb(0.553, 0.553, 0.580);
pub const TEXT_ON_ACCENT: Color = Color::WHITE;

// Colores de los ejes, como en la mayoría de editores 3D.
pub const AXIS_X: Color = Color::srgb(0.886, 0.322, 0.322);
pub const AXIS_Y: Color = Color::srgb(0.443, 0.757, 0.341);
pub const AXIS_Z: Color = Color::srgb(0.353, 0.529, 0.937);

// Medidas.
pub const TOP_BAR_HEIGHT: f32 = 32.0;
pub const STATUS_BAR_HEIGHT: f32 = 24.0;
pub const SIDE_PANEL_WIDTH: f32 = 260.0;
pub const PANEL_HEADER_HEIGHT: f32 = 28.0;
pub const ROW_HEIGHT: f32 = 22.0;
pub const INDENT: f32 = 14.0;
pub const GAP: f32 = 1.0;
pub const PADDING: f32 = 8.0;
pub const RADIUS: f32 = 3.0;
pub const MIN_VIEWPORT_WIDTH: f32 = 120.0;

// Tamaños de fuente.
pub const FONT_SIZE: f32 = 13.0;
pub const FONT_SIZE_SMALL: f32 = 12.0;
pub const FONT_SIZE_TITLE: f32 = 14.0;

/// Fuentes del editor (las embebidas por `bevy_feathers`).
#[derive(Resource, Clone)]
pub struct EditorFonts {
    pub regular: Handle<Font>,
    pub bold: Handle<Font>,
    pub mono: Handle<Font>,
}

impl EditorFonts {
    pub fn regular(&self, size: f32) -> TextFont {
        text_font(&self.regular, size)
    }

    pub fn bold(&self, size: f32) -> TextFont {
        text_font(&self.bold, size)
    }

    pub fn mono(&self, size: f32) -> TextFont {
        text_font(&self.mono, size)
    }
}

fn text_font(font: &Handle<Font>, size: f32) -> TextFont {
    TextFont {
        font: FontSource::Handle(font.clone()),
        font_size: FontSize::Px(size),
        ..default()
    }
}

pub struct ThemePlugin;

impl Plugin for ThemePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(WINDOW_BG));
        let assets = app.world().resource::<AssetServer>();
        let editor_fonts = EditorFonts {
            regular: assets.load(fonts::REGULAR),
            bold: assets.load(fonts::BOLD),
            mono: assets.load(fonts::MONO),
        };
        app.insert_resource(editor_fonts);
    }
}

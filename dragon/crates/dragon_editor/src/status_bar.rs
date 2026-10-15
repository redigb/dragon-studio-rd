//! Barra de estado: versiones y FPS (FR-006).

use std::time::Duration;

use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;

use crate::shell::StatusBar;
use crate::theme::{self, EditorFonts};
use crate::{EditorStartup, BEVY_VERSION, DRAGON_VERSION};

/// Texto con los FPS actuales.
#[derive(Component, Debug, Default)]
pub struct FpsText;

/// FR-006 pide al menos dos actualizaciones por segundo.
const FPS_REFRESH: Duration = Duration::from_millis(250);

pub struct StatusBarPlugin;

impl Plugin for StatusBarPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<FrameTimeDiagnosticsPlugin>() {
            app.add_plugins(FrameTimeDiagnosticsPlugin::default());
        }
        app.add_systems(Startup, spawn_status_bar.in_set(EditorStartup::Panels))
            .add_systems(Update, update_fps.run_if(on_timer(FPS_REFRESH)));
    }
}

fn spawn_status_bar(
    mut commands: Commands,
    fonts: Res<EditorFonts>,
    bars: Query<Entity, With<StatusBar>>,
) {
    let Ok(bar) = bars.single() else {
        return;
    };
    commands.entity(bar).with_children(|bar| {
        bar.spawn((
            Text::new(format!(
                "Dragon Studio RD v{DRAGON_VERSION}  ·  Bevy {BEVY_VERSION}"
            )),
            fonts.regular(theme::FONT_SIZE_SMALL),
            TextColor(theme::TEXT_DIM),
        ));
        bar.spawn((
            FpsText,
            Text::new("FPS: —"),
            fonts.mono(theme::FONT_SIZE_SMALL),
            TextColor(theme::TEXT_DIM),
        ));
    });
}

fn update_fps(diagnostics: Res<DiagnosticsStore>, mut texts: Query<&mut Text, With<FpsText>>) {
    let Some(fps) = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.smoothed())
    else {
        return;
    };
    for mut text in &mut texts {
        text.0 = format!("FPS: {fps:.0}");
    }
}

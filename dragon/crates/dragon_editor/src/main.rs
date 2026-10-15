use bevy::feathers::FeathersPlugins;
use bevy::prelude::*;
use dragon_editor::DragonEditorPlugins;

fn main() {
    let mut window = Window {
        title: "Dragon Studio RD".into(),
        resolution: (1280, 720).into(),
        position: WindowPosition::Centered(MonitorSelection::Primary),
        ..default()
    };
    // Maximizada: un tamaño fijo puede exceder la pantalla con el escalado de Windows.
    window.set_maximized(true);

    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(window),
                ..default()
            }),
            FeathersPlugins,
            DragonEditorPlugins,
        ))
        .run();
}

//! Editor visual de Dragon Studio RD.
//!
//! Cada zona del editor es un [`Plugin`] independiente (principio VII de la constitución) y
//! todos se agrupan en [`DragonEditorPlugins`]. Ver `specs/001-editor-base/` para la spec, el plan
//! y el contrato de esta API.

use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

pub mod demo_scene;
pub mod hierarchy;
pub mod inspector;
pub mod selection;
pub mod shell;
pub mod status_bar;
pub mod theme;
pub mod viewport;

pub use selection::Selection;
pub use viewport::EditorCamera;

/// Versión de Dragon Studio RD.
pub const DRAGON_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Versión de Bevy sobre la que se construye Dragon. Se actualiza como paso de
/// `dragon/UPSTREAM.md` (Bevy no la expone como constante pública).
pub const BEVY_VERSION: &str = "0.20.0";

/// Marca una entidad como propia del editor: nunca forma parte de la escena del usuario
/// y no aparece en la Jerarquía.
#[derive(Component, Debug, Default, Clone, Copy)]
pub struct EditorOnly;

/// Filtro de los objetos de la escena: entidades con `Transform` que no son del editor.
pub type SceneObjectFilter = (With<Transform>, Without<EditorOnly>);

/// Orden de construcción de la interfaz al arrancar.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum EditorStartup {
    /// Se crean la raíz de la UI y las zonas vacías.
    Shell,
    /// Cada panel se monta dentro de su zona.
    Panels,
}

/// Todos los plugins del editor, en orden.
pub struct DragonEditorPlugins;

impl PluginGroup for DragonEditorPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(EditorCorePlugin)
            .add(theme::ThemePlugin)
            .add(selection::SelectionPlugin)
            .add(shell::ShellPlugin)
            .add(viewport::ViewportPlugin)
            .add(hierarchy::HierarchyPlugin)
            .add(inspector::InspectorPlugin)
            .add(status_bar::StatusBarPlugin)
            .add(demo_scene::DemoScenePlugin)
    }
}

/// Configuración compartida por todos los plugins del editor.
pub struct EditorCorePlugin;

impl Plugin for EditorCorePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Startup,
            (EditorStartup::Shell, EditorStartup::Panels).chain(),
        );
    }
}

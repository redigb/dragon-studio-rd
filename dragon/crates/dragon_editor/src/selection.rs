//! Selección del editor: como máximo un objeto de la escena a la vez (FR-010, FR-013).

use bevy::prelude::*;

use crate::SceneObjectFilter;

/// Objeto de la escena seleccionado, si hay alguno.
#[derive(Resource, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Selection {
    pub entity: Option<Entity>,
}

pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Selection>()
            .add_systems(PreUpdate, clear_invalid_selection);
    }
}

/// Limpia la selección si el objeto dejó de existir o dejó de ser parte de la escena.
fn clear_invalid_selection(
    mut selection: ResMut<Selection>,
    scene_objects: Query<(), SceneObjectFilter>,
) {
    if let Some(entity) = selection.entity
        && !scene_objects.contains(entity)
    {
        selection.entity = None;
    }
}

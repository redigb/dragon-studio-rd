//! Panel Jerarquía: árbol de objetos de la escena con selección (FR-008 … FR-011).

use bevy::picking::events::PointerClick;
use bevy::picking::hover::Hovered;
use bevy::prelude::*;
use bevy::ui_widgets::ScrollArea;

use crate::selection::Selection;
use crate::shell::{panel_header, HierarchyPanel};
use crate::theme::{self, EditorFonts};
use crate::{EditorStartup, SceneObjectFilter};

/// Contenedor desplazable con las filas.
#[derive(Component, Debug, Default)]
pub struct HierarchyList;

/// Fila que representa un objeto de la escena.
#[derive(Component, Debug, Clone, Copy)]
pub struct HierarchyRow {
    pub target: Entity,
}

/// Datos mínimos de un objeto de escena para construir el árbol.
#[derive(Debug, Clone, PartialEq)]
pub struct SceneNode {
    pub entity: Entity,
    pub parent: Option<Entity>,
    pub label: String,
}

/// Una fila del árbol ya ordenada.
#[derive(Debug, Clone, PartialEq)]
pub struct TreeRow {
    pub entity: Entity,
    pub depth: usize,
    pub label: String,
}

/// Ordena los nodos como árbol (padres antes que hijos, hijos indentados), conservando el orden
/// de entrada entre hermanos. Un nodo cuyo padre no está en la lista se trata como raíz.
pub fn build_tree(nodes: &[SceneNode]) -> Vec<TreeRow> {
    let is_root = |node: &SceneNode| {
        node.parent
            .is_none_or(|parent| !nodes.iter().any(|other| other.entity == parent))
    };

    let mut rows = Vec::with_capacity(nodes.len());
    let mut stack: Vec<(usize, usize)> = nodes
        .iter()
        .enumerate()
        .rev()
        .filter(|(_, node)| is_root(node))
        .map(|(index, _)| (index, 0))
        .collect();

    while let Some((index, depth)) = stack.pop() {
        let node = &nodes[index];
        rows.push(TreeRow {
            entity: node.entity,
            depth,
            label: node.label.clone(),
        });
        stack.extend(
            nodes
                .iter()
                .enumerate()
                .rev()
                .filter(|(_, child)| child.parent == Some(node.entity))
                .map(|(child_index, _)| (child_index, depth + 1)),
        );
    }
    rows
}

/// Etiqueta visible de un objeto: su nombre o, si no tiene, su identificador.
pub fn label_for(entity: Entity, name: Option<&Name>) -> String {
    match name {
        Some(name) if !name.as_str().is_empty() => name.as_str().to_owned(),
        _ => format!("Entidad {entity}"),
    }
}

pub struct HierarchyPlugin;

impl Plugin for HierarchyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_hierarchy_panel.in_set(EditorStartup::Panels))
            .add_systems(Update, (refresh_hierarchy, highlight_rows).chain());
    }
}

fn spawn_hierarchy_panel(
    mut commands: Commands,
    fonts: Res<EditorFonts>,
    panels: Query<Entity, With<HierarchyPanel>>,
) {
    let Ok(panel) = panels.single() else {
        return;
    };
    commands.entity(panel).with_children(|panel| {
        panel.spawn(panel_header(&fonts, "Jerarquía"));
        panel.spawn((
            HierarchyList,
            ScrollArea,
            Node {
                flex_grow: 1.0,
                min_height: px(0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::vertical(px(4)),
                overflow: Overflow::scroll_y(),
                ..default()
            },
        ));
    });
}

/// Reconstruye las filas solo cuando cambia el árbol (research R-07).
fn refresh_hierarchy(
    mut commands: Commands,
    mut current: Local<Option<Vec<TreeRow>>>,
    fonts: Res<EditorFonts>,
    lists: Query<Entity, With<HierarchyList>>,
    objects: Query<(Entity, Option<&Name>, Option<&ChildOf>), SceneObjectFilter>,
) {
    let Ok(list) = lists.single() else {
        return;
    };

    let mut nodes: Vec<SceneNode> = objects
        .iter()
        .map(|(entity, name, child_of)| SceneNode {
            entity,
            parent: child_of.map(ChildOf::parent),
            label: label_for(entity, name),
        })
        .collect();
    nodes.sort_by_key(|node| node.entity);
    let rows = build_tree(&nodes);

    if current.as_ref() == Some(&rows) {
        return;
    }

    commands.entity(list).despawn_related::<Children>();
    commands.entity(list).with_children(|list| {
        if rows.is_empty() {
            list.spawn((
                Text::new("La escena está vacía"),
                fonts.regular(theme::FONT_SIZE),
                TextColor(theme::TEXT_DIM),
                Node {
                    margin: UiRect::all(px(theme::PADDING)),
                    ..default()
                },
            ));
        }
        for row in &rows {
            let target = row.entity;
            list.spawn((
                HierarchyRow { target },
                Hovered::default(),
                Node {
                    height: px(theme::ROW_HEIGHT),
                    flex_shrink: 0.0,
                    align_items: AlignItems::Center,
                    padding: UiRect::left(px(theme::PADDING + row.depth as f32 * theme::INDENT)),
                    margin: UiRect::horizontal(px(4)),
                    border_radius: BorderRadius::all(px(theme::RADIUS)),
                    ..default()
                },
                BackgroundColor(Color::NONE),
                children![(
                    Text::new(row.label.clone()),
                    fonts.regular(theme::FONT_SIZE),
                    TextColor(theme::TEXT_MAIN),
                )],
            ))
            .observe(
                move |_: On<PointerClick>, mut selection: ResMut<Selection>| {
                    selection.entity = Some(target);
                },
            );
        }
    });

    *current = Some(rows);
}

/// Resalta la fila seleccionada con el acento y la fila bajo el puntero (FR-011).
fn highlight_rows(
    selection: Res<Selection>,
    mut rows: Query<(&HierarchyRow, &Hovered, &mut BackgroundColor, &Children)>,
    mut texts: Query<&mut TextColor>,
) {
    for (row, hovered, mut background, children) in &mut rows {
        let selected = selection.entity == Some(row.target);
        let (fill, text) = match (selected, hovered.get()) {
            (true, _) => (theme::ACCENT, theme::TEXT_ON_ACCENT),
            (false, true) => (theme::ROW_HOVER_BG, theme::TEXT_MAIN),
            (false, false) => (Color::NONE, theme::TEXT_MAIN),
        };
        background.set_if_neq(BackgroundColor(fill));
        for child in children {
            if let Ok(mut color) = texts.get_mut(*child) {
                color.set_if_neq(TextColor(text));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entity(index: u32) -> Entity {
        Entity::from_raw_u32(index).unwrap()
    }

    fn node(index: u32, parent: Option<u32>, label: &str) -> SceneNode {
        SceneNode {
            entity: entity(index),
            parent: parent.map(entity),
            label: label.to_owned(),
        }
    }

    #[test]
    fn children_follow_their_parent_indented() {
        let nodes = [
            node(1, None, "Suelo"),
            node(2, None, "Planeta"),
            node(3, Some(2), "Luna"),
            node(4, None, "Sol"),
        ];
        let rows = build_tree(&nodes);
        let summary: Vec<(&str, usize)> = rows
            .iter()
            .map(|row| (row.label.as_str(), row.depth))
            .collect();
        assert_eq!(
            summary,
            [("Suelo", 0), ("Planeta", 0), ("Luna", 1), ("Sol", 0)]
        );
    }

    #[test]
    fn orphan_with_missing_parent_is_a_root() {
        // El padre (99) no es un objeto de escena, por ejemplo porque es del editor.
        let rows = build_tree(&[node(5, Some(99), "Huérfano")]);
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].depth, 0);
    }

    #[test]
    fn empty_scene_has_no_rows() {
        assert!(build_tree(&[]).is_empty());
    }

    #[test]
    fn unnamed_entities_get_an_id_label() {
        assert_eq!(label_for(entity(7), Some(&Name::new("Cubo"))), "Cubo");
        assert!(label_for(entity(7), None).starts_with("Entidad "));
    }
}

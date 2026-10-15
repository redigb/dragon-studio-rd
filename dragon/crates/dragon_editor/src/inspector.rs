//! Panel Inspector de solo lectura: nombre y `Transform` del objeto seleccionado (FR-012).

use bevy::prelude::*;

use crate::hierarchy::label_for;
use crate::selection::Selection;
use crate::shell::{panel_header, InspectorPanel};
use crate::theme::{self, EditorFonts};
use crate::EditorStartup;

/// Mensaje que se muestra sin selección.
#[derive(Component, Debug, Default)]
pub struct InspectorPlaceholder;

/// Contenedor con los datos del objeto seleccionado.
#[derive(Component, Debug, Default)]
pub struct InspectorDetails;

/// Texto con el nombre del objeto.
#[derive(Component, Debug, Default)]
pub struct InspectorName;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransformProperty {
    Translation,
    Rotation,
    Scale,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

/// Texto que muestra un valor del `Transform`.
#[derive(Component, Debug, Clone, Copy)]
pub struct InspectorField {
    pub property: TransformProperty,
    pub axis: Axis,
}

const PROPERTIES: [(TransformProperty, &str); 3] = [
    (TransformProperty::Translation, "Posición"),
    (TransformProperty::Rotation, "Rotación"),
    (TransformProperty::Scale, "Escala"),
];

const AXES: [(Axis, &str, Color); 3] = [
    (Axis::X, "X", theme::AXIS_X),
    (Axis::Y, "Y", theme::AXIS_Y),
    (Axis::Z, "Z", theme::AXIS_Z),
];

/// Rotación como ángulos de Euler en grados (orden Y-X-Z), devueltos como (X, Y, Z).
pub fn rotation_degrees(rotation: Quat) -> Vec3 {
    let (y, x, z) = rotation.to_euler(EulerRot::YXZ);
    Vec3::new(x.to_degrees(), y.to_degrees(), z.to_degrees())
}

/// Formato de un valor con dos decimales, sin mostrar "-0.00".
pub fn format_value(value: f32) -> String {
    let text = format!("{value:.2}");
    if text == "-0.00" {
        "0.00".to_owned()
    } else {
        text
    }
}

fn property_value(transform: &Transform, property: TransformProperty) -> Vec3 {
    match property {
        TransformProperty::Translation => transform.translation,
        TransformProperty::Rotation => rotation_degrees(transform.rotation),
        TransformProperty::Scale => transform.scale,
    }
}

fn axis_value(value: Vec3, axis: Axis) -> f32 {
    match axis {
        Axis::X => value.x,
        Axis::Y => value.y,
        Axis::Z => value.z,
    }
}

pub struct InspectorPlugin;

impl Plugin for InspectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_inspector_panel.in_set(EditorStartup::Panels))
            .add_systems(Update, update_inspector);
    }
}

fn spawn_inspector_panel(
    mut commands: Commands,
    fonts: Res<EditorFonts>,
    panels: Query<Entity, With<InspectorPanel>>,
) {
    let Ok(panel) = panels.single() else {
        return;
    };
    commands.entity(panel).with_children(|panel| {
        panel.spawn(panel_header(&fonts, "Inspector"));
        panel
            .spawn(Node {
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(px(theme::PADDING)),
                row_gap: px(theme::PADDING),
                ..default()
            })
            .with_children(|body| {
                body.spawn((
                    InspectorPlaceholder,
                    Text::new("Ningún objeto seleccionado"),
                    fonts.regular(theme::FONT_SIZE),
                    TextColor(theme::TEXT_DIM),
                ));
                body.spawn((
                    InspectorDetails,
                    Node {
                        display: Display::None,
                        flex_direction: FlexDirection::Column,
                        row_gap: px(6),
                        ..default()
                    },
                ))
                .with_children(|details| {
                    details.spawn((
                        InspectorName,
                        Text::new(""),
                        fonts.bold(theme::FONT_SIZE_TITLE),
                        TextColor(theme::TEXT_MAIN),
                    ));
                    details.spawn((
                        Text::new("Transform"),
                        fonts.bold(theme::FONT_SIZE_SMALL),
                        TextColor(theme::TEXT_DIM),
                        Node {
                            margin: UiRect::top(px(4)),
                            ..default()
                        },
                    ));
                    for (property, label) in PROPERTIES {
                        spawn_property_row(details, &fonts, property, label);
                    }
                });
            });
    });
}

fn spawn_property_row(
    parent: &mut ChildSpawnerCommands,
    fonts: &EditorFonts,
    property: TransformProperty,
    label: &str,
) {
    parent
        .spawn(Node {
            align_items: AlignItems::Center,
            column_gap: px(4),
            ..default()
        })
        .with_children(|row| {
            row.spawn((
                Text::new(label),
                fonts.regular(theme::FONT_SIZE_SMALL),
                TextColor(theme::TEXT_DIM),
                Node {
                    width: px(64),
                    flex_shrink: 0.0,
                    ..default()
                },
            ));
            for (axis, axis_label, axis_color) in AXES {
                row.spawn((
                    Node {
                        flex_grow: 1.0,
                        flex_basis: px(0),
                        align_items: AlignItems::Center,
                        column_gap: px(4),
                        padding: UiRect::axes(px(4), px(2)),
                        border_radius: BorderRadius::all(px(theme::RADIUS)),
                        ..default()
                    },
                    BackgroundColor(theme::PANEL_HEADER_BG),
                    children![
                        (
                            Text::new(axis_label),
                            fonts.bold(theme::FONT_SIZE_SMALL),
                            TextColor(axis_color),
                        ),
                        (
                            InspectorField { property, axis },
                            Text::new(""),
                            fonts.mono(theme::FONT_SIZE_SMALL),
                            TextColor(theme::TEXT_MAIN),
                        ),
                    ],
                ));
            }
        });
}

fn update_inspector(
    selection: Res<Selection>,
    objects: Query<(Option<&Name>, &Transform)>,
    mut placeholder: Query<&mut Node, (With<InspectorPlaceholder>, Without<InspectorDetails>)>,
    mut details: Query<&mut Node, (With<InspectorDetails>, Without<InspectorPlaceholder>)>,
    mut name_text: Query<&mut Text, (With<InspectorName>, Without<InspectorField>)>,
    mut fields: Query<(&InspectorField, &mut Text), Without<InspectorName>>,
) {
    let selected = selection
        .entity
        .and_then(|entity| objects.get(entity).ok().map(|object| (entity, object)));

    let (show_placeholder, show_details) = match selected {
        Some(_) => (Display::None, Display::Flex),
        None => (Display::Flex, Display::None),
    };
    for mut node in &mut placeholder {
        if node.display != show_placeholder {
            node.display = show_placeholder;
        }
    }
    for mut node in &mut details {
        if node.display != show_details {
            node.display = show_details;
        }
    }

    let Some((entity, (name, transform))) = selected else {
        return;
    };

    let label = label_for(entity, name);
    for mut text in &mut name_text {
        if text.0 != label {
            text.0.clone_from(&label);
        }
    }
    for (field, mut text) in &mut fields {
        let value = axis_value(property_value(transform, field.property), field.axis);
        let formatted = format_value(value);
        if text.0 != formatted {
            text.0 = formatted;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn values_have_two_decimals_and_no_negative_zero() {
        assert_eq!(format_value(1.0), "1.00");
        assert_eq!(format_value(-2.345), "-2.35");
        assert_eq!(format_value(-0.001), "0.00");
    }

    #[test]
    fn rotation_is_reported_in_degrees_per_axis() {
        let rotation = Quat::from_rotation_y(90f32.to_radians());
        let degrees = rotation_degrees(rotation);
        assert!((degrees.y - 90.0).abs() < 1e-3);
        assert!(degrees.x.abs() < 1e-3);
        assert!(degrees.z.abs() < 1e-3);

        let rotation = Quat::from_rotation_x(30f32.to_radians());
        assert!((rotation_degrees(rotation).x - 30.0).abs() < 1e-3);
    }
}

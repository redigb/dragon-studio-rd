//! Armazón del editor: raíz de la UI y las cinco zonas (FR-001, FR-002, FR-003).
//!
//! Este plugin solo crea las zonas vacías, marcadas con un componente. Cada panel se monta en la
//! suya desde su propio plugin, en [`EditorStartup::Panels`].

use bevy::picking::hover::Hovered;
use bevy::prelude::*;

use crate::theme::{self, EditorFonts};
use crate::{EditorOnly, EditorStartup};

/// Barra superior.
#[derive(Component, Debug, Default)]
pub struct TopBar;

/// Zona izquierda: Jerarquía.
#[derive(Component, Debug, Default)]
pub struct HierarchyPanel;

/// Zona central: panel de la Escena (encabezado + vista).
#[derive(Component, Debug, Default)]
pub struct ScenePanel;

/// Nodo donde se dibuja la Escena 3D.
#[derive(Component, Debug, Default)]
pub struct SceneViewport;

/// Zona derecha: Inspector.
#[derive(Component, Debug, Default)]
pub struct InspectorPanel;

/// Barra de estado inferior.
#[derive(Component, Debug, Default)]
pub struct StatusBar;

/// Elemento que se ilumina al pasar el puntero (menús de la barra superior).
#[derive(Component, Debug, Default)]
pub struct HoverHighlight;

const MENUS: [&str; 4] = ["Archivo", "Editar", "Ver", "Ayuda"];

pub struct ShellPlugin;

impl Plugin for ShellPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_shell.in_set(EditorStartup::Shell))
            .add_systems(Update, update_hover_highlight);
    }
}

fn spawn_shell(mut commands: Commands, fonts: Res<EditorFonts>) {
    // Cámara que dibuja la interfaz en la ventana.
    commands.spawn((Name::new("Cámara de la UI"), Camera2d, EditorOnly));

    let top_bar = commands
        .spawn((
            Name::new("Barra superior"),
            TopBar,
            Node {
                height: px(theme::TOP_BAR_HEIGHT),
                flex_shrink: 0.0,
                align_items: AlignItems::Center,
                padding: UiRect::horizontal(px(theme::PADDING)),
                column_gap: px(2),
                border: UiRect::bottom(px(1)),
                ..default()
            },
            BackgroundColor(theme::BAR_BG),
            BorderColor::all(theme::BORDER),
        ))
        .with_children(|bar| {
            bar.spawn((
                Text::new("Dragon Studio RD"),
                fonts.bold(theme::FONT_SIZE_TITLE),
                TextColor(theme::ACCENT),
                Node {
                    margin: UiRect::right(px(theme::PADDING * 2.0)),
                    ..default()
                },
            ));
            for menu in MENUS {
                bar.spawn((
                    HoverHighlight,
                    Hovered::default(),
                    Node {
                        padding: UiRect::axes(px(theme::PADDING), px(4)),
                        border_radius: BorderRadius::all(px(theme::RADIUS)),
                        ..default()
                    },
                    BackgroundColor(Color::NONE),
                    children![(
                        Text::new(menu),
                        fonts.regular(theme::FONT_SIZE),
                        TextColor(theme::TEXT_MAIN),
                    )],
                ));
            }
        })
        .id();

    let middle = commands
        .spawn((
            Name::new("Zona central"),
            Node {
                flex_grow: 1.0,
                min_height: px(0),
                column_gap: px(theme::GAP),
                ..default()
            },
        ))
        .with_children(|middle| {
            middle.spawn((
                Name::new("Panel Jerarquía"),
                HierarchyPanel,
                side_panel(),
                BackgroundColor(theme::PANEL_BG),
            ));
            middle
                .spawn((
                    Name::new("Panel Escena"),
                    ScenePanel,
                    Node {
                        flex_grow: 1.0,
                        min_width: px(theme::MIN_VIEWPORT_WIDTH),
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    BackgroundColor(theme::PANEL_BG),
                ))
                .with_children(|scene| {
                    scene.spawn(panel_header(&fonts, "Escena"));
                    scene.spawn((
                        Name::new("Vista de la Escena"),
                        SceneViewport,
                        Node {
                            flex_grow: 1.0,
                            min_height: px(0),
                            ..default()
                        },
                    ));
                });
            middle.spawn((
                Name::new("Panel Inspector"),
                InspectorPanel,
                side_panel(),
                BackgroundColor(theme::PANEL_BG),
            ));
        })
        .id();

    let status_bar = commands
        .spawn((
            Name::new("Barra de estado"),
            StatusBar,
            Node {
                height: px(theme::STATUS_BAR_HEIGHT),
                flex_shrink: 0.0,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::horizontal(px(theme::PADDING)),
                border: UiRect::top(px(1)),
                ..default()
            },
            BackgroundColor(theme::BAR_BG),
            BorderColor::all(theme::BORDER),
        ))
        .id();

    commands
        .spawn((
            Name::new("Editor"),
            EditorOnly,
            Node {
                width: percent(100),
                height: percent(100),
                flex_direction: FlexDirection::Column,
                row_gap: px(theme::GAP),
                ..default()
            },
            BackgroundColor(theme::WINDOW_BG),
        ))
        .add_children(&[top_bar, middle, status_bar]);
}

/// Nodo base de los paneles laterales (ancho fijo, contenido en columna).
fn side_panel() -> Node {
    Node {
        width: px(theme::SIDE_PANEL_WIDTH),
        flex_shrink: 0.0,
        flex_direction: FlexDirection::Column,
        min_height: px(0),
        ..default()
    }
}

/// Encabezado estándar de un panel.
pub fn panel_header(fonts: &EditorFonts, title: &str) -> impl Bundle {
    (
        Node {
            height: px(theme::PANEL_HEADER_HEIGHT),
            flex_shrink: 0.0,
            align_items: AlignItems::Center,
            padding: UiRect::horizontal(px(theme::PADDING)),
            border: UiRect::bottom(px(1)),
            ..default()
        },
        BackgroundColor(theme::PANEL_HEADER_BG),
        BorderColor::all(theme::BORDER),
        children![(
            Text::new(title),
            fonts.bold(theme::FONT_SIZE),
            TextColor(theme::TEXT_MAIN),
        )],
    )
}

/// Elementos resaltables cuyo estado de hover cambió este cuadro.
type HoverChanged = (With<HoverHighlight>, Changed<Hovered>);

fn update_hover_highlight(mut items: Query<(&Hovered, &mut BackgroundColor), HoverChanged>) {
    for (hovered, mut background) in &mut items {
        background.0 = if hovered.get() {
            theme::ROW_HOVER_BG
        } else {
            Color::NONE
        };
    }
}

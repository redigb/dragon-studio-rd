//! Escena 3D dentro de su panel y cámara orbital del editor (FR-004, FR-014, FR-015).
//!
//! La cámara del editor renderiza a una textura que un [`ViewportNode`] muestra y redimensiona
//! al tamaño del panel (research R-04). La navegación solo responde con el cursor sobre la Escena
//! (research R-05).

use std::f32::consts::FRAC_PI_2;

use bevy::camera::RenderTarget;
use bevy::input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll, MouseScrollUnit};
use bevy::math::ops;
use bevy::prelude::*;
use bevy::render::render_resource::TextureFormat;
use bevy::ui::RelativeCursorPosition;

use crate::shell::SceneViewport;
use crate::{EditorOnly, EditorStartup};

pub const MIN_DISTANCE: f32 = 1.0;
pub const MAX_DISTANCE: f32 = 200.0;
const MAX_PITCH: f32 = FRAC_PI_2 - 0.01;
const ORBIT_SPEED: f32 = 0.006;
const PAN_SPEED: f32 = 0.0015;
const ZOOM_STEP: f32 = 0.12;
const PIXELS_PER_LINE: f32 = 40.0;

/// Cámara orbital del editor. Su `Transform` se deriva siempre de estos valores.
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct EditorCamera {
    pub focus: Vec3,
    pub distance: f32,
    pub yaw: f32,
    pub pitch: f32,
}

impl Default for EditorCamera {
    fn default() -> Self {
        Self {
            focus: Vec3::new(0.0, 0.5, 0.0),
            distance: 12.0,
            yaw: 0.6,
            pitch: -0.45,
        }
    }
}

impl EditorCamera {
    /// Gira alrededor del punto focal. `delta` en píxeles de ratón.
    pub fn orbit(&mut self, delta: Vec2) {
        self.yaw -= delta.x * ORBIT_SPEED;
        self.pitch = (self.pitch - delta.y * ORBIT_SPEED).clamp(-MAX_PITCH, MAX_PITCH);
    }

    /// Desplaza el punto focal en el plano de la pantalla. `delta` en píxeles de ratón.
    pub fn pan(&mut self, delta: Vec2) {
        let rotation = self.rotation();
        let right = rotation * Vec3::X;
        let up = rotation * Vec3::Y;
        let scale = self.distance * PAN_SPEED;
        self.focus += (-right * delta.x + up * delta.y) * scale;
    }

    /// Acerca (positivo) o aleja (negativo) la cámara; `steps` en líneas de rueda.
    pub fn zoom(&mut self, steps: f32) {
        // `ops::powf` en lugar de `f32::powf`: resultado determinista en todas las plataformas.
        let factor = ops::powf(1.0 - ZOOM_STEP, steps);
        self.distance = (self.distance * factor).clamp(MIN_DISTANCE, MAX_DISTANCE);
    }

    pub fn rotation(&self) -> Quat {
        Quat::from_euler(EulerRot::YXZ, self.yaw, self.pitch, 0.0)
    }

    pub fn transform(&self) -> Transform {
        let rotation = self.rotation();
        let translation = self.focus + rotation * Vec3::new(0.0, 0.0, self.distance);
        Transform {
            translation,
            rotation,
            ..default()
        }
    }
}

pub struct ViewportPlugin;

impl Plugin for ViewportPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_editor_camera.in_set(EditorStartup::Panels))
            .add_systems(Update, navigate_camera);
    }
}

fn spawn_editor_camera(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    viewports: Query<Entity, With<SceneViewport>>,
) {
    let Ok(viewport) = viewports.single() else {
        return;
    };

    // La textura empieza vacía; `ViewportNode` le da el tamaño del panel.
    let image = Image::new_target_texture(0, 0, TextureFormat::Bgra8UnormSrgb, None);
    let image = images.add(image);

    let editor_camera = EditorCamera::default();
    let camera = commands
        .spawn((
            Name::new("Cámara del editor"),
            EditorOnly,
            editor_camera,
            editor_camera.transform(),
            Camera3d::default(),
            Camera {
                // Se dibuja antes que la UI.
                order: -1,
                ..default()
            },
            RenderTarget::Image(image.into()),
        ))
        .id();

    commands
        .entity(viewport)
        .insert((ViewportNode::new(camera), RelativeCursorPosition::default()));
}

/// Botón con el que empezó el arrastre actual, si lo hay.
#[derive(Default)]
struct ActiveDrag(Option<MouseButton>);

fn navigate_camera(
    mut drag: Local<ActiveDrag>,
    buttons: Res<ButtonInput<MouseButton>>,
    motion: Res<AccumulatedMouseMotion>,
    scroll: Res<AccumulatedMouseScroll>,
    viewports: Query<&RelativeCursorPosition, With<SceneViewport>>,
    mut cameras: Query<(&mut EditorCamera, &mut Transform)>,
) {
    let Ok(cursor) = viewports.single() else {
        return;
    };
    let Ok((mut camera, mut transform)) = cameras.single_mut() else {
        return;
    };
    let over_viewport = cursor.cursor_over();

    // Un arrastre solo empieza sobre la Escena, pero sigue aunque el cursor salga de ella.
    if let Some(button) = drag.0
        && !buttons.pressed(button)
    {
        drag.0 = None;
    }
    if drag.0.is_none() && over_viewport {
        drag.0 = [MouseButton::Right, MouseButton::Middle]
            .into_iter()
            .find(|button| buttons.just_pressed(*button));
    }

    let before = *camera;
    match drag.0 {
        Some(MouseButton::Right) => camera.orbit(motion.delta),
        Some(MouseButton::Middle) => camera.pan(motion.delta),
        _ => {}
    }

    if over_viewport && scroll.delta.y != 0.0 {
        let steps = match scroll.unit {
            MouseScrollUnit::Line => scroll.delta.y,
            MouseScrollUnit::Pixel => scroll.delta.y / PIXELS_PER_LINE,
        };
        camera.zoom(steps);
    }

    if *camera != before {
        *transform = camera.transform();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zoom_stays_within_bounds() {
        let mut camera = EditorCamera::default();
        camera.zoom(1000.0);
        assert_eq!(camera.distance, MIN_DISTANCE);
        camera.zoom(-1000.0);
        assert_eq!(camera.distance, MAX_DISTANCE);
    }

    #[test]
    fn pitch_never_flips() {
        let mut camera = EditorCamera::default();
        camera.orbit(Vec2::new(0.0, -100_000.0));
        assert!(camera.pitch <= MAX_PITCH);
        camera.orbit(Vec2::new(0.0, 100_000.0));
        assert!(camera.pitch >= -MAX_PITCH);
    }

    #[test]
    fn transform_keeps_distance_and_looks_at_focus() {
        let camera = EditorCamera::default();
        let transform = camera.transform();
        let distance = transform.translation.distance(camera.focus);
        assert!((distance - camera.distance).abs() < 1e-4);
        let to_focus = (camera.focus - transform.translation).normalize();
        assert!(transform.forward().dot(to_focus) > 0.9999);
    }

    #[test]
    fn pan_moves_focus_but_not_distance() {
        let mut camera = EditorCamera::default();
        let focus = camera.focus;
        camera.pan(Vec2::new(30.0, -10.0));
        assert_ne!(camera.focus, focus);
        assert_eq!(camera.distance, EditorCamera::default().distance);
    }
}

# Implementation Plan: Editor base de Dragon Studio RD

**Branch**: `001-editor-base` (trabajo en `main`, D-05) | **Date**: 2026-10-14 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `/specs/001-editor-base/spec.md`

## Summary

Crear el primer ejecutable de Dragon Studio RD: una ventana de editor con barra superior,
Jerarquía, Escena 3D, Inspector y barra de estado, construida solo con la pila de UI de Bevy
0.20 (`bevy_ui` + `bevy_feathers` + `bevy_ui_widgets`), con navegación orbital de cámara,
selección desde la Jerarquía, Inspector de solo lectura y una configuración de compilación rápida
(enlazado dinámico + `rust-lld`). Todo vive en un workspace nuevo, `dragon/`, que consume el Bevy
de la raíz sin modificarlo.

## Technical Context

**Language/Version**: Rust estable 1.99.0 (mínimo 1.97.1, exigido por Bevy 0.20)

**Primary Dependencies**: Bevy v0.20.0 del propio repositorio (features por defecto +
`bevy_feathers`; en desarrollo además `dynamic_linking` y `file_watcher`)

**Storage**: N/A (sin persistencia en esta feature)

**Testing**: `cargo test` para la lógica pura (cámara orbital, firma de la Jerarquía, formato de
valores); validación manual de las historias según `quickstart.md`

**Target Platform**: editor de escritorio, Windows 10/11 (referencia) y Linux

**Project Type**: aplicación de escritorio (editor) + biblioteca de plugins

**Performance Goals**: ≥ 60 FPS con la escena de demostración; recompilación incremental ≤ 3 s
(meta) / ≤ 10 s (máximo); interfaz visible ≤ 3 s tras el lanzamiento

**Constraints**: cero cambios a archivos de Bevy; sin dependencias de terceros fuera de Bevy;
colores solo desde el módulo de tema

**Scale/Scope**: un crate, unos 8 módulos, escenas de decenas de objetos

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principio | Cumplimiento en esta feature | Estado |
|-----------|------------------------------|--------|
| I. Bevy único upstream | Workspace `dragon/` aparte con `[patch.crates-io]`; ningún archivo de Bevy modificado; sin código de terceros | ✅ |
| II. 100 % Rust, UI nativa | `bevy_ui` + `bevy_feathers` + `bevy_ui_widgets`; sin egui ni web | ✅ |
| III. Velocidad | Feature `dev` con `dynamic_linking`, `rust-lld`, perfiles de optimización, métricas en `quickstart.md` | ✅ |
| IV. Alcance Hammer | Solo explorar e inspeccionar la escena; nada de IDE ni modelado | ✅ |
| V. Móvil primera clase | No aplica al editor (escritorio). No se crea código de runtime en esta feature, así que nada impide Android | ✅ (N/A) |
| VI. Bonito por defecto | Módulo `theme` único; fuentes de feathers | ✅ |
| VII. Todo es un plugin | Un `Plugin` por zona; marcador `EditorOnly`; el juego no depende del editor | ✅ |
| VIII. Honestidad del estado | `ROADMAP.md` y `tasks.md` reflejan lo verificado | ✅ |

**Re-check post-diseño (Fase 1)**: sin violaciones. La decisión de un solo crate (R-12) es
compatible con el principio III y queda justificada en `research.md`.

## Project Structure

### Documentation (this feature)

```text
specs/001-editor-base/
├── plan.md              # Este archivo
├── research.md          # Fase 0: decisiones R-01 … R-12
├── data-model.md        # Fase 1: recursos y componentes
├── quickstart.md        # Fase 1: cómo ejecutar, validar y medir
├── contracts/
│   └── editor-plugin-api.md
├── checklists/
│   └── requirements.md
└── tasks.md             # Fase 2 (/speckit-tasks)
```

### Source Code (repository root)

```text
dragon/
├── Cargo.toml                 # workspace + [patch.crates-io] bevy = { path = ".." } + perfiles
├── .cargo/config.toml         # rust-lld en Windows, alias `dev` y `distrib`
├── .gitignore                 # reincluye Cargo.lock
└── crates/
    └── dragon_editor/
        ├── Cargo.toml         # feature `dev`
        └── src/
            ├── main.rs        # App: DefaultPlugins + FeathersPlugins + DragonEditorPlugins
            ├── lib.rs         # DragonEditorPlugins, EditorStartup, EditorOnly, versiones
            ├── theme.rs       # colores, tamaños, fuentes (único origen de estilo)
            ├── shell.rs       # raíz de la UI y las cinco zonas; barra superior
            ├── viewport.rs    # cámara del editor, textura, ViewportNode, navegación orbital
            ├── selection.rs   # recurso Selection y limpieza
            ├── hierarchy.rs   # panel Jerarquía
            ├── inspector.rs   # panel Inspector
            ├── status_bar.rs  # versiones y FPS
            └── demo_scene.rs  # escena de demostración
```

**Structure Decision**: un workspace `dragon/` con un único crate `dragon_editor` (biblioteca +
binario) organizado en un módulo y un plugin por zona del editor (R-12). Los futuros crates
(`dragon_runtime`, `dragon_cli`, `dragon_export_android`) se añadirán en `dragon/crates/`.

## Complexity Tracking

Sin violaciones de la constitución que justificar.

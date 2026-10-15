---

description: "Lista de tareas de la feature 001-editor-base"
---

# Tasks: Editor base de Dragon Studio RD

**Input**: Design documents from `/specs/001-editor-base/`

**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: la spec no pide TDD. Se incluyen pruebas unitarias solo para la lógica pura (cámara,
formato de valores, árbol de la Jerarquía); las historias se validan con `quickstart.md`.

**Organization**: tareas agrupadas por historia de usuario.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: se puede hacer en paralelo (archivos distintos, sin dependencias)
- **[Story]**: historia a la que pertenece (US1 … US4)
- Rutas relativas a la raíz del repositorio

## Phase 1: Setup (Shared Infrastructure)

- [ ] T001 Crear el workspace en `dragon/Cargo.toml` con `[workspace.dependencies] bevy = "0.20"` (+ `bevy_feathers`), `[patch.crates-io] bevy = { path = ".." }` y perfiles `dev` (opt-level 1 / dependencias 3)
- [ ] T002 [P] Crear `dragon/.cargo/config.toml` con `linker = "rust-lld.exe"` para `x86_64-pc-windows-msvc` y los alias `dev` y `distrib`
- [ ] T003 [P] Crear `dragon/.gitignore` que reincluya `Cargo.lock`
- [ ] T004 Crear `dragon/crates/dragon_editor/Cargo.toml` (lib + bin, feature `dev` = `bevy/dynamic_linking` + `bevy/file_watcher`)

---

## Phase 2: Foundational (Blocking Prerequisites)

- [ ] T005 Definir `EditorOnly`, `EditorStartup`, `DRAGON_VERSION`, `BEVY_VERSION` y `DragonEditorPlugins` en `dragon/crates/dragon_editor/src/lib.rs`
- [ ] T006 [P] Implementar el tema (colores, tamaños, recurso `EditorFonts`) y `ThemePlugin` en `dragon/crates/dragon_editor/src/theme.rs`
- [ ] T007 [P] Implementar `Selection` y `SelectionPlugin` (limpieza cuando la entidad desaparece, FR-013) en `dragon/crates/dragon_editor/src/selection.rs`
- [ ] T008 Crear `dragon/crates/dragon_editor/src/main.rs` con `DefaultPlugins` (ventana "Dragon Studio RD"), `FeathersPlugins` y `DragonEditorPlugins`

**Checkpoint**: el workspace compila y abre una ventana vacía.

---

## Phase 3: User Story 1 - Abrir el editor y ver el espacio de trabajo (Priority: P1) 🎯 MVP

**Goal**: ventana con las cinco zonas, escena 3D en el centro y barra de estado con versiones y FPS.

**Independent Test**: `cargo dev`; ver las cinco zonas; redimensionar sin deformación.

- [ ] T009 [US1] Implementar `ShellPlugin` (raíz de UI, barra superior con nombre y menús Archivo/Editar/Ver/Ayuda, zonas marcadas) en `dragon/crates/dragon_editor/src/shell.rs`
- [ ] T010 [US1] Implementar la cámara del editor con render a textura y el nodo `ViewportNode` dentro de `SceneViewport` en `dragon/crates/dragon_editor/src/viewport.rs`
- [ ] T011 [P] [US1] Implementar `DemoScenePlugin` (Suelo, Cubo, Torre, Planeta en rotación con hija Luna, Sol) en `dragon/crates/dragon_editor/src/demo_scene.rs`
- [ ] T012 [P] [US1] Implementar `StatusBarPlugin` (versión de Dragon, versión de Bevy, FPS cada 0,25 s) en `dragon/crates/dragon_editor/src/status_bar.rs`
- [ ] T013 [US1] Montar paneles vacíos de Jerarquía e Inspector con su encabezado para completar el layout (`hierarchy.rs`, `inspector.rs`)

**Checkpoint**: historia 1 verificable de forma independiente.

---

## Phase 4: User Story 2 - Iterar rápido sobre el editor (Priority: P2)

**Goal**: recompilación incremental dentro de la meta y comandos documentados.

**Independent Test**: cambiar una línea, `cargo dev`, cronometrar.

- [ ] T014 [US2] Verificar que `cargo dev` usa `dynamic_linking` y `rust-lld` (compilación completa exitosa)
- [ ] T015 [US2] Medir la compilación inicial y la recompilación incremental y anotarlas en `specs/001-editor-base/quickstart.md`
- [ ] T016 [US2] Verificar que `cargo distrib` compila sin `dynamic_linking`

---

## Phase 5: User Story 3 - Explorar la escena desde la Jerarquía y el Inspector (Priority: P3)

**Goal**: Jerarquía en árbol con selección y un Inspector que muestra el `Transform` en vivo.

**Independent Test**: clic en cada fila; la Luna en movimiento actualiza sus valores.

- [ ] T017 [P] [US3] Prueba unitaria del cálculo del árbol (orden, profundidad, etiquetas, exclusión de `EditorOnly`) en `dragon/crates/dragon_editor/src/hierarchy.rs`
- [ ] T018 [US3] Implementar la firma del árbol, la reconstrucción de filas, el área desplazable y los mensajes de escena vacía en `dragon/crates/dragon_editor/src/hierarchy.rs`
- [ ] T019 [US3] Implementar el clic en filas (`PointerClick` → `Selection`) y el resaltado de la fila seleccionada en `dragon/crates/dragon_editor/src/hierarchy.rs`
- [ ] T020 [P] [US3] Prueba unitaria del formato de valores y la conversión de rotación a grados en `dragon/crates/dragon_editor/src/inspector.rs`
- [ ] T021 [US3] Implementar el Inspector (nombre, posición, rotación, escala; mensaje "Ningún objeto seleccionado"; actualización en vivo) en `dragon/crates/dragon_editor/src/inspector.rs`

---

## Phase 6: User Story 4 - Mover la cámara del editor en la Escena (Priority: P4)

**Goal**: órbita, paneo y zoom acotado solo sobre la Escena.

**Independent Test**: arrastres y rueda sobre la Escena y sobre los paneles.

- [ ] T022 [P] [US4] Prueba unitaria de `EditorCamera` (límites de distancia y de inclinación, transform derivada) en `dragon/crates/dragon_editor/src/viewport.rs`
- [ ] T023 [US4] Implementar la navegación (arrastre iniciado sobre la Escena, rueda solo encima) en `dragon/crates/dragon_editor/src/viewport.rs`

---

## Phase 7: Polish & Cross-Cutting Concerns

- [ ] T024 `cargo fmt --check` y `cargo clippy --workspace -- -D warnings` sin errores en `dragon/`
- [ ] T025 `cargo test --workspace` en `dragon/`
- [ ] T026 Ejecutar el editor y validar las cuatro historias según `quickstart.md`; anotar FPS y tiempo de arranque
- [ ] T027 Actualizar `dragon/ROADMAP.md` y el estado de `spec.md`

---

## Dependencies & Execution Order

- Setup (T001–T004) → Foundational (T005–T008) → US1 (T009–T013).
- US2 depende de US1 (necesita algo que compilar y medir).
- US3 y US4 dependen de US1 y son independientes entre sí.
- Polish al final.

### Parallel Opportunities

- T002 y T003 en paralelo tras T001.
- T006 y T007 en paralelo.
- T011 y T012 en paralelo con T010.
- US3 (T017–T021) y US4 (T022–T023) en paralelo tras US1.

## Implementation Strategy

1. MVP = Setup + Foundational + US1: ya hay un editor que mostrar.
2. US2 asegura que todo lo siguiente se itera rápido.
3. US3 y US4 convierten la vista en una herramienta de exploración.
4. Cada checkpoint se valida antes de seguir.

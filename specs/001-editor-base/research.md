# Research: Editor base de Dragon Studio RD

Fase 0 del plan. Cada decisión se verificó contra el código de Bevy v0.20.0 presente en este
repositorio (rutas indicadas) y contra la constitución v1.0.0.

## R-01 · Cómo consume `dragon/` el Bevy de la raíz

- **Decision**: `dragon/` es un workspace propio. Declara `bevy = "0.20"` en
  `[workspace.dependencies]` y lo redirige con `[patch.crates-io] bevy = { path = ".." }`.
- **Rationale**: el principio I exige no tocar los archivos de Bevy (incluido su `Cargo.toml`
  raíz). Un workspace anidado que no figura en los `members` de Bevy es independiente. El `patch`
  garantiza que cualquier crate de terceros que pida `bevy` reciba nuestro fork y no una segunda
  copia de crates.io.
- **Alternatives considered**: añadir `dragon/*` a los `members` del workspace de Bevy (modifica
  un archivo de Bevy y provoca conflictos en cada rebase); dependencia directa por ruta sin `patch`
  (funciona hoy, pero duplica Bevy cuando entren crates de terceros).
- **Nota**: el `.gitignore` de Bevy ignora `Cargo.lock` en todo el repo. Como el editor es una
  aplicación, `dragon/.gitignore` lo reincluye (`!Cargo.lock`) para builds reproducibles.

## R-02 · Pila de UI

- **Decision**: nodos de `bevy_ui` construidos desde Rust, con el plugin `FeathersPlugins`
  activo (fuentes embebidas, tema, controles) y los widgets de `bevy_ui_widgets` que vienen en
  `DefaultPlugins` (`ScrollArea` para la Jerarquía).
- **Rationale**: principio II. `crates/bevy_feathers/src/lib.rs` declara que feathers está
  pensado para editores e inspectores. `crates/bevy_internal/src/default_plugins.rs` ya incluye
  `UiWidgetsPlugins` (con `ScrollAreaPlugin`) y `DefaultPickingPlugins`.
- **Alternatives considered**: escribir todo el layout con `bsn!` (el macro es nuevo en 0.20 y su
  sintaxis aún cambia; se adoptará de forma progresiva cuando un panel lo justifique); egui
  (prohibido por el principio II).

## R-03 · Tipografía

- **Decision**: Fira Sans (regular y bold) y Fira Mono embebidas por feathers, cargadas desde
  `bevy::feathers::constants::fonts::{REGULAR, BOLD, MONO}` y guardadas en un recurso del tema.
- **Rationale**: no añade archivos de fuentes al repo, coincide con el aspecto de los widgets de
  feathers y el texto de valores numéricos usa una fuente monoespaciada que no "baila".
- **Alternatives considered**: Inter (habría que añadir y mantener el archivo de fuente; se puede
  cambiar luego desde el tema sin tocar los paneles).

## R-04 · Escena dentro de un panel

- **Decision**: una cámara 3D del editor que renderiza a una textura (`RenderTarget::Image`) y un
  nodo de UI con `ViewportNode::new(camera)` que la muestra y la redimensiona al tamaño del panel.
- **Rationale**: es el patrón oficial de `examples/ui/widgets/viewport_node.rs`. Cumple FR-004 y
  el escenario 3 de la historia 1 (sin deformación al redimensionar), porque `ViewportNode`
  ajusta el tamaño de la textura y el aspecto de la cámara al del nodo.
- **Alternatives considered**: ajustar `Camera::viewport` en píxeles de la ventana (hay que
  recalcularlo a mano en cada cambio de layout y escala; frágil).

## R-05 · Navegación de la cámara solo sobre la Escena

- **Decision**: el nodo de la Escena lleva `RelativeCursorPosition`. Un sistema lee
  `AccumulatedMouseMotion`, `AccumulatedMouseScroll` y `ButtonInput<MouseButton>`. Un arrastre
  solo comienza si el botón se pulsó con el cursor sobre la Escena y continúa hasta soltarlo, aunque
  el cursor salga del panel. La rueda solo actúa con el cursor encima.
- **Rationale**: cumple FR-014 y el escenario 4 de la historia 4. Los eventos de picking
  (`PointerDrag`) no sirven aquí: `ViewportNode` reenvía el picking a las mallas de la escena,
  cuyos eventos no se propagan al nodo de UI.
- **Alternatives considered**: `bevy_camera_controller` (cámara libre en primera persona, no
  orbital); observar `PointerDrag` en el nodo (falla al arrastrar sobre un objeto).

## R-06 · Distinguir entidades del editor y de la escena

- **Decision**: componente marcador `EditorOnly` en toda entidad raíz creada por el editor (raíz
  de la UI, cámaras). Un objeto de escena es toda entidad con `Transform` y sin `EditorOnly`.
- **Rationale**: principio VII y FR-008. En Bevy 0.20 los nodos de UI usan `UiTransform`, no
  `Transform`, así que la UI nunca se cuela en la Jerarquía; las cámaras sí tienen `Transform` y
  por eso llevan el marcador.
- **Alternatives considered**: mundos o escenas separados (más correcto a largo plazo, cuando el
  juego corra en su propio proceso, decisión D-12; prematuro ahora).

## R-07 · Actualización de la Jerarquía

- **Decision**: cada cuadro se calcula una "firma" del árbol (lista ordenada de entidad,
  profundidad y etiqueta). Solo si cambia se reconstruyen las filas. El resaltado de selección se
  actualiza aparte, sin reconstruir.
- **Rationale**: detecta altas, bajas, renombres y cambios de padre sin depender de muchas
  consultas de cambios. El costo es despreciable para escenas de decenas o cientos de objetos.
- **Alternatives considered**: detección por `Added`/`Changed`/`RemovedComponents` (más eficiente
  pero con muchos casos borde); reconstruir siempre (parpadeo y trabajo innecesario).

## R-08 · Clic en filas

- **Decision**: cada fila observa `PointerClick` (eventos de `bevy_picking`, que en 0.20 son tipos
  concretos `PointerClick`, `PointerDrag`, `PointerScroll`). El clic en el texto se propaga a la
  fila.
- **Rationale**: el backend de picking de UI está en `DefaultPickingPlugins`; `PointerClick`
  propaga a los padres (`auto_propagate`).

## R-09 · Velocidad de compilación

- **Decision**:
  - Feature `dev` en `dragon_editor` que activa `bevy/dynamic_linking` y `bevy/file_watcher`.
  - `dragon/.cargo/config.toml` con `linker = "rust-lld.exe"` para `x86_64-pc-windows-msvc`.
  - `[profile.dev] opt-level = 1` y `[profile.dev.package."*"] opt-level = 3`.
  - Alias `cargo dev` (ejecutar en desarrollo) y `cargo distrib` (build de distribución).
- **Rationale**: principio III y decisión D-11; es la configuración recomendada por Bevy para
  iterar rápido. La feature de distribución no usa `dynamic_linking`.
- **Alternatives considered**: backend Cranelift (requiere nightly; se evaluará en una feature
  propia); hotpatching (feature futura de la hoja de ruta).

## R-10 · Versión de Bevy visible en la barra de estado

- **Decision**: constante `BEVY_VERSION = "0.20.0"` en el crate del editor, actualizada como paso
  explícito de `dragon/UPSTREAM.md`. La versión de Dragon sale de `CARGO_PKG_VERSION`.
- **Rationale**: Bevy no expone su versión como constante pública; leerla del lockfile en tiempo de
  compilación añadiría un build script por un solo texto.

## R-11 · FPS

- **Decision**: `FrameTimeDiagnosticsPlugin` y lectura del valor suavizado de FPS desde
  `DiagnosticsStore`, refrescando el texto cada 0,25 s (cumple FR-006: al menos dos veces por
  segundo).

## R-12 · Organización en crates

- **Decision**: un solo crate, `dragon_editor` (biblioteca + binario), con un módulo y un `Plugin`
  por zona del editor.
- **Rationale**: con una sola feature y unas pocas centenas de líneas, dividir en varios crates no
  acelera la compilación y añade ceremonia. El principio III ("crates pequeños") se cumple: el
  crate es pequeño y Bevy ya se enlaza dinámicamente. Cuando aparezca el runtime de juego
  (feature 003 de la hoja de ruta) se separará `dragon_runtime`, que debe compilar para Android
  (principio V) y no puede depender del editor.

## R-13 · Reglas de `clippy` heredadas de Bevy (hallazgo de la implementación)

- **Decision**: `dragon/` no define su propio `clippy.toml`, así que Cargo usa el de la raíz de
  Bevy. Ese archivo prohíbe los métodos de `f32` no deterministas (`powf`, `sin`, `cos`, …); se usan
  sus equivalentes de `bevy::math::ops`.
- **Rationale**: los resultados matemáticos son idénticos en todas las plataformas (importante para
  Android y para una futura red multijugador), y Dragon sigue las mismas reglas que Bevy.
- **Medido**: build de distribución (`cargo distrib`) completa en 12 min 23 s; `clippy` en modo
  desarrollo, 4 min 23 s la primera vez.

# Dragon Studio RD Constitution

Esta constitución gobierna todo el trabajo en Dragon Studio RD: un motor de videojuegos y editor
visual hecho 100 % en Rust, construido sobre un fork de Bevy. Cada especificación, plan, lista de
tareas e implementación se valida contra estos principios. La visión pública (el "por qué") vive en
`dragon/VISION.md`; este documento fija las reglas verificables (el "cómo se decide").

## Core Principles

### I. Bevy es la única base y el único upstream

- Dragon Studio RD es un fork de `bevyengine/bevy`. Bevy es la **única** dependencia de motor: no se
  incorpora código de otros editores o motores de terceros (por ejemplo Jackdaw). El editor se
  construye desde cero sobre Bevy.
- Solo se siguen **versiones estables** de Bevy (tags `vX.Y.Z`). Nunca se trabaja sobre el `main` de
  Bevy. La versión actual es **v0.20.0**.
- Los archivos de Bevy (todo lo que está fuera de `dragon/`, `specs/`, `.specify/` y `.claude/`)
  MUST NOT modificarse, salvo dos excepciones:
  1. El `README.md` de la raíz, que es el de Dragon.
  2. Cambios imprescindibles al motor, en commits pequeños y aislados con el prefijo `[bevy]`,
     justificados en la spec que los necesita.
- Todo lo que pueda resolverse como plugin MUST vivir en `dragon/`.
- `dragon/` es un workspace de Cargo independiente que consume el Bevy de la raíz mediante
  `[patch.crates-io]`, de modo que cualquier dependencia que pida `bevy` reciba el nuestro.

**Razón:** cada línea cambiada dentro de Bevy es un conflicto potencial en cada actualización.
Aislar lo nuestro en carpetas nuevas hace que el rebase sobre una versión nueva de Bevy sea casi
automático. Depender de otro editor de terceros añadiría un segundo upstream que no controlamos.

### II. 100 % Rust, con la UI nativa de Bevy

- Todo el código de Dragon, incluida la interfaz del editor, MUST estar escrito en Rust.
- La interfaz del editor MUST construirse con la pila de UI de Bevy: `bevy_ui` (layout y nodos),
  `bevy_ui_widgets` (comportamiento sin estilo), `bevy_feathers` (widgets con estilo para editores)
  y `bsn!` (escenas declarativas).
- Quedan prohibidos: egui, webviews o HTML, toolkits de UI en C/C++ y lenguajes de scripting
  embebidos para la lógica del editor.

**Razón:** un solo lenguaje y un solo motor; quien sabe extender su juego sabe extender el editor.
`bevy_feathers` es el kit con el que Bevy construye su editor oficial, así que alinearse con él
reduce migraciones futuras. Al inicio se recomendó egui por velocidad de desarrollo; esa
recomendación se revirtió al comprobar que, en Bevy 0.20, `bevy_feathers` ya permite editores con
aspecto profesional (paneles, menús, inspectores, campos numéricos y de color).

### III. La velocidad es una funcionalidad

- El ciclo *editar → compilar → ver* es el producto. Una regresión de velocidad se trata como bug.
- Las builds de desarrollo MUST usar: enlazado dinámico de Bevy (`dynamic_linking`), un linker
  rápido (`rust-lld` en Windows, `mold` o `lld` en Linux), `opt-level = 1` para nuestro código y
  `opt-level = 3` para las dependencias.
- Los crates de Dragon MUST mantenerse pequeños y con responsabilidades claras, para que un cambio
  recompile lo mínimo.
- Metas de referencia (se miden y registran en cada feature que las afecte):
  - Recompilación incremental tras cambiar una línea del editor: **≤ 3 s** (meta), **≤ 10 s**
    (máximo aceptable).
  - Desde lanzar el editor ya compilado hasta ver la interfaz completa: **≤ 3 s**.
  - Interfaz del editor fluida: **≥ 60 FPS** con la escena de demostración.
- Las builds de distribución MUST NOT usar `dynamic_linking`.

**Razón:** el mayor punto débil de Rust frente a Godot o Unity es la iteración. Si Dragon no
resuelve eso, nada más importa.

### IV. Alcance de Hammer, sensación de Godot

- El editor NO reemplaza a Rust, a Blender, a Krita ni a un DAW. No incluye IDE, ni modelador 3D,
  ni lenguaje de scripting propio. Los assets se crean en sus aplicaciones y se importan.
- El editor SÍ ofrece: gestión de proyecto, escena editable visualmente, inspector de componentes,
  botón Play/Stop dentro del editor, depuración y exportación con un clic.
- Cada funcionalidad MUST poder justificarse con un usuario concreto: artista, diseñador o
  programador. Si solo sirve para "parecerse a Unity", no entra.

**Razón:** es la visión oficial del editor de Bevy ("Hammer, no Unity"), complementada con la
accesibilidad de Godot que pide Dragon: un artista debe poder cambiar un material y probarlo sin
escribir código.

### V. Móvil como ciudadano de primera clase

- Android es una plataforma de destino oficial desde el diseño, no un añadido posterior.
- El editor es una aplicación de escritorio (Windows y Linux). Los crates de **runtime** (lo que se
  distribuye con el juego) MUST compilar para `aarch64-linux-android` y MUST NOT depender de APIs
  exclusivas de escritorio.
- La exportación a Android MUST automatizarse con `cargo-ndk` y un proyecto Gradle con
  `GameActivity` (el camino del ejemplo `examples/mobile` de Bevy). `cargo-apk` queda descartado
  por estar abandonado.

**Razón:** Bevy soporta Android, pero deja la configuración al usuario. Resolverla es una ventaja
directa sobre Bevy puro y una necesidad declarada del proyecto.

### VI. Bonito por defecto

- Todos los colores, tamaños, espaciados y tipografías del editor MUST venir de un único módulo de
  tema (tokens), apoyado en el sistema de temas de `bevy_feathers`. Quedan prohibidos los colores
  literales fuera de ese módulo.
- La referencia visual es el diseño oficial (Figma) del editor de Bevy: fondo oscuro neutro, un solo
  color de acento, bordes sutiles, esquinas pequeñas y una tipografía legible.

**Razón:** una herramienta agradable se usa más. Centralizar los tokens permite cambiar el aspecto
completo sin tocar cada panel.

### VII. Todo es un plugin

- Cada parte del editor (paneles, viewport, inspector, barras) MUST implementarse como un `Plugin`
  de Bevy, registrable y removible de forma independiente.
- El editor MUST distinguir sus propias entidades (UI, cámaras del editor) de las entidades de la
  escena del usuario mediante un componente marcador, y nunca mostrar ni guardar las suyas como
  parte de la escena.
- El inspector MUST basarse en `Reflect` y no en código específico por componente, salvo que una
  spec justifique un editor visual especializado.
- Un juego MUST poder ejecutarse sin el editor.

**Razón:** la extensibilidad por equipo (espacios de trabajo por rol, herramientas propias de cada
juego) solo es posible si el propio editor está hecho de piezas intercambiables.

### VIII. Honestidad del estado

- `dragon/ROADMAP.md` MUST reflejar en todo momento el estado real de cada feature (pendiente, en
  curso, terminada) y enlazar su carpeta en `specs/`.
- Una tarea solo se marca como completada si fue verificada (compila, se ejecutó o se midió). Lo no
  verificado se declara como tal.

**Razón:** el proyecto se valida por specs; un estado inflado invalida todo el método.

## Restricciones técnicas y registro de decisiones validadas

**Stack:** Rust estable (≥ la `rust-version` que exija Bevy; hoy 1.97.1, instalado 1.99.0), Bevy
v0.20.0 del propio repositorio, Cargo workspaces. Plataformas del editor: Windows (principal) y
Linux. Plataformas de juego: Windows, Linux y Android.

**Idioma:** documentación, specs, comentarios y textos visibles de la interfaz en español.
Identificadores de código (tipos, funciones, módulos, crates) en inglés, siguiendo la convención
del ecosistema Rust y de Bevy. Los encabezados de las plantillas de spec-kit se conservan en inglés
para compatibilidad con sus comandos.

**Registro de decisiones** (cada una validada con el responsable del proyecto, RD):

| ID | Decisión | Argumento | Alternativas descartadas |
|----|----------|-----------|--------------------------|
| D-01 | Construir sobre Bevy y no un motor desde cero | ECS rápido, renderer, assets, Android y Reflect ya resueltos | Motor propio: años de trabajo sin diferenciación |
| D-02 | Hacer fork de Bevy (`redigb/dragon-studio-rd`) | Control total del motor y capacidad de parchearlo si hace falta | Depender solo de crates.io: no permite parches propios |
| D-03 | El fork se mantiene mínimo; lo propio vive en `dragon/` | Rebase casi sin conflictos en cada versión de Bevy | Modificar Bevy libremente: cada actualización se vuelve una fusión manual |
| D-04 | Seguir solo tags estables de Bevy (hoy v0.20.0) | Estabilidad y guías de migración oficiales | Seguir `main` de Bevy: roturas constantes |
| D-05 | La rama de trabajo es `main`; las versiones de Bevy llegan por el remoto `upstream` | Es lo que espera quien visita el repo; un espejo de Bevy en el fork no aporta nada | Mantener `main` como espejo y trabajar en otra rama: confuso |
| D-06 | No usar código de Jackdaw ni de otros editores de terceros | Un solo upstream (Bevy); el editor es propio y se diseña a nuestra medida | Usar o forkear Jackdaw: segundo upstream, más de 50 crates que migrar y dependencias en forks personales |
| D-07 | UI con `bevy_ui` + `bevy_feathers` + `bsn!` | Ver principio II | egui: rápido de empezar, pero no nativo, estética limitada y fuera del camino oficial |
| D-08 | Alcance "Hammer, no Unity" con sensación de Godot | Ver principio IV | Clonar Unity: alcance inalcanzable |
| D-09 | No hacer renderer, física ni lenguaje de scripting propios | Se usa Bevy y su ecosistema; el diferenciador es Rust con iteración rápida | Escribirlos: costo enorme sin ventaja |
| D-10 | Android con `cargo-ndk` + Gradle + `GameActivity` | Es el camino vigente del ejemplo móvil de Bevy | `cargo-apk`: abandonado |
| D-11 | Iteración rápida: `dynamic_linking`, `rust-lld`, crates pequeños; hotpatching como feature futura | Ver principio III | Cargar el juego como `.dll` (hot reload de librerías): frágil en Rust por la falta de ABI estable |
| D-12 | El editor convive con el juego como plugin en desarrollo; editor y juego en procesos separados (Bevy Remote Protocol) es una meta posterior | Empezar simple y robusto | Procesos separados desde el día uno: complejidad prematura |
| D-13 | Desarrollo guiado por especificaciones con spec-kit | Estado y pasos visibles y validables; nada se pierde entre conversaciones | Desarrollo sin specs: decisiones dispersas |
| D-14 | Nombre del proyecto: Dragon Studio RD; crates con prefijo `dragon_` | Identidad del proyecto definida por RD | — |

**Propuestas pendientes de ratificar** (no son reglas hasta que RD las apruebe):

- P-01: Editor controlable desde fuera (Bevy Remote Protocol + MCP) para que un asistente de IA
  pueda operar el editor con el mismo historial de deshacer que una persona.

## Flujo de desarrollo y puertas de calidad

1. **Flujo por feature (spec-kit):** `/speckit-specify` → (`/speckit-clarify` si hay ambigüedad)
   → `/speckit-plan` → `/speckit-tasks` → (`/speckit-analyze`) → `/speckit-implement`. Cada
   feature vive en `specs/NNN-nombre/`.
2. **Puertas antes de dar una feature por terminada:**
   - `cargo fmt --check` sin diferencias en `dragon/`.
   - `cargo clippy --workspace -- -D warnings` sin advertencias en `dragon/`.
   - `cargo build` del workspace `dragon/` exitoso.
   - La historia de usuario se verificó ejecutando el editor, y las métricas del principio III que
     la feature afecte quedaron medidas y anotadas en su `quickstart.md`.
   - `dragon/ROADMAP.md` actualizado.
3. **Commits:** prefijo `[dragon]` para trabajo propio, `[bevy]` para cambios al motor y `[spec]`
   para documentación de specs.
4. **Actualizar Bevy:** se sigue `dragon/UPSTREAM.md`. Una actualización es una feature más, con su
   propia spec.

## Governance

- Esta constitución prevalece sobre cualquier otra práctica del proyecto. Ante un conflicto entre
  una spec y la constitución, se corrige la spec o se enmienda la constitución; nunca se ignora.
- **Enmiendas:** se proponen por escrito, con el argumento y las alternativas descartadas; las
  aprueba RD; se registran en la tabla de decisiones y se actualiza la versión.
- **Versionado (semver):** MAJOR al eliminar o redefinir un principio; MINOR al añadir un principio,
  una sección o una decisión; PATCH en aclaraciones de redacción.
- **Cumplimiento:** cada `plan.md` incluye un *Constitution Check* que revisa estos principios antes
  y después del diseño. Toda violación se justifica en su sección *Complexity Tracking* o se
  rechaza.

**Version**: 1.0.0 | **Ratified**: 2026-10-14 | **Last Amended**: 2026-10-14

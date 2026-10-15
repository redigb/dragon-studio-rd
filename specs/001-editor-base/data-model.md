# Data Model: Editor base de Dragon Studio RD

Todo el estado vive en el ECS de Bevy (componentes y recursos). No hay persistencia en esta
feature.

## Recursos

### `Selection`
| Campo | Tipo | Regla |
|-------|------|-------|
| `entity` | `Option<Entity>` | Como máximo una entidad seleccionada (FR-010). Siempre es un objeto de escena o `None`. |

**Transiciones**
- `None` → `Some(e)`: clic en la fila de `e` en la Jerarquía.
- `Some(a)` → `Some(b)`: clic en otra fila.
- `Some(e)` → `None`: `e` deja de existir o deja de ser objeto de escena (FR-013).

### `EditorTheme` (módulo `theme`)
Fuente única de colores, tamaños y tipografías (principio VI, FR-007). Las constantes de color y
tamaño son `const`; las fuentes son un recurso con `Handle<Font>` para `regular`, `bold` y `mono`.

## Componentes

### `EditorOnly` (marcador)
Toda entidad raíz creada por el editor (raíz de la UI, cámara de la Escena, cámara de la UI).
Las entidades con este marcador nunca son objetos de escena (FR-008).

### Objeto de escena (no es un componente; es una consulta)
Entidad con `Transform` y sin `EditorOnly`. Atributos que lee el editor:
- `Name` (opcional) → etiqueta; si falta, se muestra `Entidad <índice>v<generación>`.
- `ChildOf` (opcional) → padre; define la indentación en la Jerarquía.
- `Transform` → mostrado por el Inspector (posición, rotación en grados con orden Y-X-Z, escala).

### `EditorCamera`
| Campo | Tipo | Regla |
|-------|------|-------|
| `focus` | `Vec3` | Punto alrededor del cual se orbita. Se desplaza con el paneo. |
| `distance` | `f32` | Acotada a `[MIN_DISTANCE, MAX_DISTANCE]` (FR-015). |
| `yaw` | `f32` (radianes) | Ángulo horizontal; libre. |
| `pitch` | `f32` (radianes) | Ángulo vertical; acotado a ±(π/2 − 0,01) para no voltear la cámara. |

La `Transform` de la cámara se deriva siempre de estos cuatro valores.

### Marcadores de zonas de la UI
| Componente | Zona |
|------------|------|
| `TopBar` | Barra superior |
| `HierarchyPanel` / `HierarchyList` | Panel Jerarquía / contenedor desplazable de filas |
| `SceneViewport` | Nodo que muestra la Escena (`ViewportNode` + `RelativeCursorPosition`) |
| `InspectorPanel` / `InspectorBody` | Panel Inspector / contenido |
| `StatusBar` / `FpsText` | Barra de estado / texto de FPS |

### `HierarchyRow`
| Campo | Tipo | Regla |
|-------|------|-------|
| `target` | `Entity` | Objeto de escena que representa la fila. |

### Campos del Inspector
`InspectorField { property: TransformProperty, axis: Axis }` en cada texto de valor, donde
`TransformProperty ∈ {Translation, Rotation, Scale}` y `Axis ∈ {X, Y, Z}`.

## Relaciones

```text
Selection.entity ──► objeto de escena ◄── HierarchyRow.target
                              │
                              └── leído por los InspectorField
EditorCamera ──(renderiza a)──► Image ◄──(muestra)── SceneViewport
```

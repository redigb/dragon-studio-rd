# Contract: API pública del crate `dragon_editor`

El editor no expone red ni CLI en esta feature; su contrato es la API de Rust que otras partes
(futuras features, el juego del usuario) pueden usar.

## Plugins

| Tipo | Contenido | Garantía |
|------|-----------|----------|
| `DragonEditorPlugins` (`PluginGroup`) | Todos los plugins de abajo, en orden | Añadirlo a una `App` con `DefaultPlugins` y `FeathersPlugins` produce el editor completo. |
| `ThemePlugin` | Recurso de fuentes y color de fondo | Debe estar antes que cualquier panel. |
| `ShellPlugin` | Raíz de la UI y las cinco zonas vacías | Crea las entidades marcadoras en `EditorStartup::Shell`. |
| `ViewportPlugin` | Cámara del editor, textura y navegación | Rellena `SceneViewport` en `EditorStartup::Panels`. |
| `HierarchyPlugin` | Panel Jerarquía | Rellena `HierarchyPanel` en `EditorStartup::Panels`. |
| `InspectorPlugin` | Panel Inspector | Rellena `InspectorPanel` en `EditorStartup::Panels`. |
| `StatusBarPlugin` | Barra de estado con versiones y FPS | Rellena `StatusBar` en `EditorStartup::Panels`. |
| `SelectionPlugin` | Recurso `Selection` y su limpieza | Independiente del resto. |
| `DemoScenePlugin` | Escena de demostración | Opcional; quitarlo deja una escena vacía sin errores. |

Cada plugin MUST poder deshabilitarse con
`DragonEditorPlugins.build().disable::<X>()` sin que el resto entre en pánico (FR-016). Los
paneles que no encuentran su contenedor no hacen nada.

## Conjuntos de sistemas

```rust
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum EditorStartup {
    Shell,  // crea las zonas vacías
    Panels, // cada panel se monta en su zona (después de Shell)
}
```

## Tipos públicos

- `EditorOnly`: marcador para excluir una entidad de la escena (ver data-model).
- `Selection { entity: Option<Entity> }`: recurso de lectura y escritura.
- `EditorCamera`: componente de la cámara de la Escena.
- `DRAGON_VERSION: &str`, `BEVY_VERSION: &str`.

## Reglas de compatibilidad

- Añadir campos o plugins es un cambio compatible. Renombrar o quitar uno de los tipos anteriores
  es incompatible y requiere nota en el ROADMAP.

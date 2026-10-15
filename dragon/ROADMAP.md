# Hoja de ruta de Dragon Studio RD

Estado real del desarrollo (principio VIII de la [constitución](../.specify/memory/constitution.md)).
Cada feature tiene su carpeta en [`specs/`](../specs) con spec, plan y tareas.

Leyenda: ✅ terminada · 🔨 en curso · ⏳ pendiente

| # | Feature | Qué entrega | Estado |
|---|---------|-------------|--------|
| 001 | [Editor base](../specs/001-editor-base/spec.md) | Ventana del editor (Jerarquía, Escena 3D, Inspector, barras), cámara orbital, selección, compilación rápida | ✅ implementada · falta probar a mano la cámara (historia 4) |
| 002 | Edición en el Inspector + deshacer/rehacer | Editar valores del `Transform` y de cualquier componente vía `Reflect`; historial de cambios | ⏳ |
| 003 | Escenas y proyecto | Crear/abrir proyecto, guardar y cargar escenas, separar `dragon_runtime` del editor | ⏳ |
| 004 | Herramientas en la Escena | Seleccionar con clic en el 3D, gizmos de mover/rotar/escalar, rejilla | ⏳ |
| 005 | Play / Stop en el editor | Ejecutar el juego dentro del editor y restaurar la escena al detenerlo | ⏳ |
| 006 | Exportar a Android | `dragon export android`: verificación de SDK/NDK, `cargo-ndk`, Gradle con `GameActivity`, APK/AAB | ⏳ |
| 007 | Navegador de assets | Explorar, previsualizar e importar assets con recarga en caliente | ⏳ |
| 008 | Iteración de código en caliente | Hotpatching de sistemas Rust sin reiniciar | ⏳ |
| 009 | Paneles acoplables y espacios de trabajo | Reorganizar paneles con pestañas; espacios por rol | ⏳ |
| 010 | Editor y juego en procesos separados | Comunicación por Bevy Remote Protocol (D-12) | ⏳ |

El orden puede cambiar; cualquier cambio se registra aquí con su motivo.

## Historial

- 2026-10-14 — Constitución v1.0.0 ratificada. Inicio de la feature 001.
- 2026-10-14 — Feature 001 implementada: compila en desarrollo y distribución, `clippy` y 10 pruebas
  en verde, historias 1–3 validadas con capturas. Recompilación de la biblioteca: 3,67 s (meta 3 s).
  Siguiente paso: probar la cámara a mano y empezar la 002.

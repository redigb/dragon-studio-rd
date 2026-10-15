# Dragon Studio RD

Motor de videojuegos y editor visual basado en [Bevy](https://bevy.org), 100 % Rust.

Lee primero la [visión del proyecto](VISION.md).

## Organización del repositorio

Este repositorio es un fork de Bevy:

| Ruta | Qué es | ¿Se modifica? |
|---|---|---|
| `/` (raíz, `crates/`, `examples/`…) | Bevy, tal cual viene de upstream | Lo mínimo posible |
| `dragon/` | Todo lo propio de Dragon: editor, runtime, CLI, exportación | Aquí trabajamos |

`dragon/` es un workspace de Cargo independiente que usa el Bevy de la raíz mediante `[patch.crates-io]`.

## Ramas

- `main`: espejo limpio de `bevyengine/bevy` (`main`). No se trabaja aquí.
- `dragon`: nuestra rama de trabajo, basada en la última versión estable de Bevy.

Para actualizar Bevy, sigue [UPSTREAM.md](UPSTREAM.md).

## Licencia

MIT o Apache-2.0, igual que Bevy. Partes del editor derivan de
[Jackdaw](https://github.com/jbuehler23/jackdaw) (MIT/Apache-2.0).

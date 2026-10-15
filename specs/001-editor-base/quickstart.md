# Quickstart: Editor base de Dragon Studio RD

## Requisitos

- Rust estable ≥ 1.97.1 (`rustup update stable`).
- Windows 10/11 con toolchain MSVC (Visual Studio Build Tools), o Linux con las dependencias de
  Bevy instaladas.

## Ejecutar

Todos los comandos se ejecutan dentro de `dragon/`:

```sh
cd dragon
cargo dev        # modo desarrollo: enlazado dinámico, recompilación rápida
cargo distrib    # build de distribución (sin enlazado dinámico), en target/release
```

La primera compilación compila Bevy completo y tarda varios minutos. Las siguientes son
incrementales.

## Validación de las historias

1. **Historia 1**: `cargo dev`. Deben verse la barra superior (Dragon Studio RD + Archivo, Editar,
   Ver, Ayuda), Jerarquía, Escena 3D, Inspector y barra de estado con versiones y FPS.
   Redimensionar la ventana: la Escena se adapta y no se deforma.
2. **Historia 2**: con el editor compilado, cambiar el texto `"Jerarquía"` en
   `dragon/crates/dragon_editor/src/hierarchy.rs`, ejecutar `cargo dev` y cronometrar.
3. **Historia 3**: hacer clic en "Planeta": el Inspector muestra sus valores y su rotación Y cambia
   continuamente (el Planeta gira). Hacer clic en "Luna": muestra su posición local (2,20, 0, 0), que
   no cambia porque la Luna se mueve al girar su padre.
4. **Historia 4**: sobre la Escena, arrastrar con botón derecho (orbitar), central (desplazar) y
   usar la rueda (zoom). Sobre los paneles, la cámara no debe moverse.

## Mediciones (principio III)

Registradas en el equipo de referencia de RD:

| Métrica | Meta | Resultado |
|---------|------|-----------|
| Compilación inicial completa (`cargo dev`, sin caché) | — | 21 min 58 s |
| Recompilación tras cambiar `main.rs` (binario) | ≤ 3 s (máx. 10 s) | ✅ 2,06 s |
| Recompilación tras cambiar un módulo de la biblioteca | ≤ 3 s (máx. 10 s) | ⚠️ 3,67 s (sobre la meta, dentro del máximo) |
| Lanzamiento hasta crear la ventana | ≤ 3 s | ✅ 1,5 s (según el log de Bevy) |
| FPS con la escena de demostración | ≥ 60 | ✅ 64 |

| Build de distribución completa (`cargo distrib`, sin caché) | — | 12 min 23 s |

Equipo de referencia: Windows 10, NVIDIA GeForce GTX 1650 (Vulkan), pantalla 2560×1440 al 150 %,
Rust 1.99.0. Medido el 2026-10-14.

## Resultado de la validación (2026-10-14)

| Historia | Verificado | Cómo |
|----------|------------|------|
| 1 — Espacio de trabajo | ✅ | Captura del editor maximizado: barra superior, Jerarquía, Escena 3D con sombras, Inspector y barra de estado ("Dragon Studio RD v0.1.0 · Bevy 0.20.0", "FPS: 64"). |
| 2 — Iteración rápida | ✅ con observación | Ver tabla de mediciones: el cambio en un módulo de la biblioteca supera la meta de 3 s (3,67 s) pero queda dentro del máximo de 10 s. |
| 3 — Jerarquía e Inspector | ✅ parcial | Árbol correcto (Luna indentada bajo Planeta, sin entidades del editor); clic simulado en "Planeta" lo selecciona y lo resalta; el Inspector mostró nombre, posición, rotación (Y = −144,05°) y escala; el hover resalta en gris. **No verificado visualmente**: dos capturas consecutivas con el Planeta seleccionado para ver cambiar la rotación (la persona usuaria tomó el control del editor). El código actualiza los valores en cada cuadro. |
| 4 — Cámara | ⚠️ no verificado por el agente | La lógica está cubierta por pruebas unitarias (límites de zoom e inclinación, paneo, orientación). Falta probar a mano los arrastres y la rueda sobre la Escena y sobre los paneles. |

Hallazgo corregido durante la validación: con una ventana fija de 1600×900 y el escalado de
Windows al 150 %, la ventana excedía la pantalla y ocultaba el Inspector. Ahora el editor arranca
maximizado.

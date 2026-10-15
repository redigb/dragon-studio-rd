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
3. **Historia 3**: hacer clic en "Planeta" y luego en "Luna" (que orbita): el Inspector muestra sus
   valores y la posición de la Luna cambia continuamente.
4. **Historia 4**: sobre la Escena, arrastrar con botón derecho (orbitar), central (desplazar) y
   usar la rueda (zoom). Sobre los paneles, la cámara no debe moverse.

## Mediciones (principio III)

Registradas en el equipo de referencia de RD:

| Métrica | Meta | Resultado |
|---------|------|-----------|
| Compilación inicial completa (`cargo dev`, sin caché) | — | _pendiente_ |
| Recompilación tras cambiar una línea del editor | ≤ 3 s (máx. 10 s) | _pendiente_ |
| Lanzamiento hasta interfaz completa | ≤ 3 s | _pendiente_ |
| FPS con la escena de demostración | ≥ 60 | _pendiente_ |

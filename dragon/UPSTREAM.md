# Actualizar Bevy en Dragon Studio RD

Seguimos solo versiones estables de Bevy (tags `vX.Y.Z`), nunca el `main` de Bevy.

```sh
# una sola vez
git remote add upstream https://github.com/bevyengine/bevy.git

# con cada versión nueva de Bevy (ejemplo: v0.21.0)
git fetch upstream tag v0.21.0 --no-tags
git checkout main
git rebase --onto v0.21.0 v0.20.0 main   # re-aplica nuestros commits encima de la nueva versión
```

Después:

1. Lee la *migration guide* oficial de la versión.
2. Actualiza `bevy = "0.21"` en `dragon/Cargo.toml`.
3. Ejecuta `cargo check` en `dragon/` y corrige crate por crate.
4. Actualiza la línea "Versión de Bevy actual" de abajo.

## Reglas

- Los cambios a archivos de Bevy (fuera de `dragon/`) van en commits pequeños con el prefijo `[bevy]`. Cuantos menos, más fácil el rebase.
- Todo lo que pueda ser un plugin vive en `dragon/`.
- Única excepción: el `README.md` de la raíz es nuestro. Si choca en el rebase, conserva el nuestro (`git checkout --theirs README.md`; durante un rebase, "theirs" son nuestros commits).

Versión de Bevy actual: **v0.20.0**

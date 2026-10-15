# Dragon Studio RD — Visión

> **En una frase:** el poder de Bevy y Rust, con la sensación de un estudio visual.
> Piensa en el editor Hammer de Valve, hecho en Rust, que abre en segundos y exporta a tu móvil con un clic.

---

## De dónde venimos

Dragon Studio RD es un fork de [Bevy](https://github.com/bevyengine/bevy). Compartimos la visión oficial del editor de Bevy y la tomamos como base:

- **El editor no reemplaza a Rust.** El código del juego se escribe en Rust, en tu IDE favorito.
- **El editor no reemplaza a Blender, Krita o tu DAW.** Los assets se crean en sus aplicaciones y se *importan*.
- **El editor sí es** una plataforma para gestionar proyectos, una interfaz potente para artistas y diseñadores, la base para las herramientas propias de cada juego y el lugar para depurar y probar.

Bevy ya es fuerte en tres cosas: una arquitectura ECS rápida y agradable, una flexibilidad total (quitas lo que no usas y añades lo que quieras del ecosistema) y Rust de principio a fin. Lo que le falta es **que los no programadores puedan trabajar sin depender del programador** y **que iterar sea rápido**. Esa brecha es la razón de existir de Dragon.

---

## Lo que nos hace distintos (nuestros pilares)

### 1. 100 % Rust, incluida la interfaz
Ni egui, ni web, ni C++. El editor está construido con la misma UI nativa de Bevy (`bevy_ui` + `bevy_feathers`) con la que se hacen los juegos. Un solo lenguaje, un solo motor y una sola forma de pensar. Si sabes extender tu juego, sabes extender el editor.

### 2. La velocidad es una funcionalidad
- **Abrir el editor en segundos**, no en minutos.
- **Recompilar en 1 a 3 segundos** en desarrollo (enlazado dinámico, linker rápido, crates pequeños).
- **Ver cambios sin reiniciar** (hot reload de assets y hotpatching de código Rust).
- Cada cambio que haga más lento el ciclo *editar → probar* se considera un bug.

### 3. Visual y accesible, sin perder el alma de Bevy
Mantenemos el alcance de Hammer (no vamos a ser Unity), pero con la *sensación* de Godot o Unity: botón **Play/Stop** dentro del editor, inspector de todo, arrastrar y soltar, paneles que se acomodan a cada rol. Un artista debe poder abrir un proyecto, cambiar un material y probarlo **sin escribir una línea de código**.

### 4. Móvil como ciudadano de primera clase
Exportar a **Android** debe ser tan fácil como exportar a Windows: un botón (o `dragon export android`) que verifique el SDK, compile y entregue el APK/AAB. Lo que Bevy deja como "configúralo tú", Dragon lo resuelve.

### 5. Bonito por defecto
Seguimos el diseño oficial de Figma del editor de Bevy, con un sistema de *tokens* (colores, medidas y tipografía centralizados). Una herramienta agradable de usar es una herramienta que se usa.

### 6. Extensible por cada equipo
Cada juego necesita herramientas propias. Las extensiones, los paneles personalizados y los espacios de trabajo por rol (nivel, animación, UI, depuración) son parte del diseño desde el día uno, no un añadido.

### 7. (Propuesta) Preparado para IA
El editor se puede controlar desde fuera (Bevy Remote Protocol + MCP). Esto permite que un asistente de IA cree niveles, ajuste valores o ejecute pruebas, y que todo quede en el mismo historial de deshacer que usa una persona.

---

## Lo que NO vamos a hacer

- ❌ Un renderer propio: usamos el de Bevy.
- ❌ Un motor de física propio: usamos el ecosistema (Avian).
- ❌ Un lenguaje de scripting propio: el diferenciador es Rust con iteración rápida.
- ❌ Un IDE integrado, ni un Blender dentro del editor.
- ❌ Divergir de Bevy sin necesidad (ver abajo).

---

## Cómo nos relacionamos con Bevy

- **Bevy es nuestra única base y nuestro único upstream.** Seguimos sus versiones estables (`v0.20.0`, `v0.21.0`, …). El código de Bevy en este repo se modifica lo mínimo posible; todo lo nuestro vive en `dragon/`.
- **El editor es nuestro, construido desde cero sobre Bevy.** No partimos del código de ningún otro editor de terceros. Cuando Bevy cambia, nosotros migramos.
- Si algo que construimos sirve a toda la comunidad, intentamos devolverlo a Bevy.

El estado del desarrollo y los pasos siguientes están en [ROADMAP.md](ROADMAP.md). Las reglas que gobiernan el proyecto están en la [constitución](../.specify/memory/constitution.md).

---

## Cómo sabremos que vamos bien

1. Un artista abre un proyecto, cambia un material y lo prueba sin pedir ayuda a nadie.
2. Desde guardar un cambio de código hasta verlo en pantalla pasan menos de 3 segundos.
3. Un proyecto nuevo llega a un teléfono Android en menos de 5 minutos.
4. Actualizar Dragon a una nueva versión de Bevy toma días, no meses.
5. Nosotros mismos terminamos un juego pequeño hecho con Dragon.

---

*Dragon Studio RD — desarrollado por RD, sobre los hombros de Bevy.*

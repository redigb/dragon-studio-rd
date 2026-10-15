# Feature Specification: Editor base de Dragon Studio RD

**Feature Branch**: `001-editor-base` (se trabaja directamente en `main`, decisión D-05)

**Created**: 2026-10-14

**Status**: Implementada — pendiente la validación manual de la historia 4 (ver `quickstart.md`)

**Input**: User description: "Primera implementación de Dragon Studio RD, solo sobre Bevy (sin
Jackdaw): una ventana de editor con aspecto profesional donde se vea una escena, su jerarquía y un
inspector, con compilación rápida para iterar desde el primer día. Base para el editor visual tipo
Godot/Unity con alcance de Hammer."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Abrir el editor y ver el espacio de trabajo (Priority: P1)

Una persona del equipo (programador, artista o diseñador) abre Dragon Studio RD y ve una ventana
de editor completa: una barra superior con el nombre del proyecto y los menús principales, un
panel de **Jerarquía** a la izquierda, la **Escena** en el centro mostrando un mundo 3D de
demostración, un panel **Inspector** a la derecha y una **barra de estado** abajo. Todo usa el
mismo estilo visual oscuro y coherente.

**Why this priority**: es el esqueleto sobre el que se monta cualquier funcionalidad futura. Sin
él no hay editor; con él ya hay algo que mostrar y sobre lo que iterar.

**Independent Test**: lanzar el editor y comprobar que aparecen las cinco zonas, que la escena 3D
se ve en el centro y que redimensionar la ventana reacomoda los paneles sin romperse.

**Acceptance Scenarios**:

1. **Given** el proyecto compilado, **When** se lanza el editor, **Then** se abre una ventana
   titulada "Dragon Studio RD" con barra superior, panel Jerarquía, Escena, panel Inspector y barra
   de estado.
2. **Given** el editor abierto, **When** se mira la zona central, **Then** se ve una escena 3D de
   demostración (suelo, varios objetos y una luz) renderizada dentro del panel Escena, sin invadir
   los paneles laterales.
3. **Given** el editor abierto, **When** se redimensiona la ventana, **Then** los paneles laterales
   mantienen su ancho, la Escena ocupa el espacio restante y la imagen 3D no se deforma.
4. **Given** el editor abierto, **When** se mira la barra de estado, **Then** muestra la versión de
   Dragon, la versión de Bevy y los cuadros por segundo actuales.

---

### User Story 2 - Iterar rápido sobre el editor (Priority: P2)

Un programador cambia una línea del código del editor (por ejemplo, el texto de un panel), vuelve a
ejecutar y ve el cambio en segundos, no en minutos.

**Why this priority**: la velocidad es una funcionalidad (principio III). Si iterar sobre el propio
editor es lento desde el día uno, todo el desarrollo posterior se frena.

**Independent Test**: tras una compilación inicial, cambiar un texto visible del editor, ejecutar
el comando de desarrollo y cronometrar desde que se lanza el comando hasta que se ve la ventana con
el cambio.

**Acceptance Scenarios**:

1. **Given** una compilación previa en modo desarrollo, **When** se cambia una línea del editor y se
   ejecuta el comando de desarrollo, **Then** la recompilación termina dentro de la meta del
   principio III (≤ 3 s meta, ≤ 10 s máximo aceptable) y el cambio es visible.
2. **Given** el repositorio recién clonado, **When** se sigue `quickstart.md`, **Then** existe un
   único comando documentado para ejecutar el editor en modo desarrollo y otro para compilarlo en
   modo distribución.

---

### User Story 3 - Explorar la escena desde la Jerarquía y el Inspector (Priority: P3)

Un diseñador ve en el panel Jerarquía la lista de objetos de la escena (no los del propio editor),
con su anidamiento. Al hacer clic en uno, queda seleccionado y resaltado, y el Inspector muestra
su nombre y su transformación (posición, rotación y escala), que se actualizan si el objeto se
mueve.

**Why this priority**: es el primer paso real de "editor visual": entender qué hay en la escena
sin leer código. Depende de la historia 1.

**Independent Test**: con la escena de demostración, hacer clic en cada entrada de la Jerarquía y
comprobar que el Inspector muestra los datos correctos de ese objeto, incluido uno que se mueve
solo.

**Acceptance Scenarios**:

1. **Given** el editor abierto, **When** se mira la Jerarquía, **Then** aparecen todos los objetos
   con nombre de la escena de demostración, los hijos indentados bajo su padre, y ninguna entidad
   interna del editor (cámaras, paneles).
2. **Given** ningún objeto seleccionado, **When** se mira el Inspector, **Then** muestra el mensaje
   "Ningún objeto seleccionado".
3. **Given** el editor abierto, **When** se hace clic en un objeto de la Jerarquía, **Then** esa
   fila queda resaltada y el Inspector muestra su nombre, posición, rotación (en grados) y escala.
4. **Given** un objeto seleccionado que se mueve en la escena, **When** pasa el tiempo, **Then** los
   valores del Inspector se actualizan para reflejar su transformación actual.
5. **Given** un objeto seleccionado, **When** se hace clic en otro, **Then** la selección cambia y
   solo la nueva fila queda resaltada.

---

### User Story 4 - Mover la cámara del editor en la Escena (Priority: P4)

Una persona orbita, desplaza y acerca la vista de la escena con el ratón para inspeccionarla desde
cualquier ángulo, sin afectar a los objetos de la escena.

**Why this priority**: sin navegar la vista, la escena es solo una imagen. Es la base para futuras
herramientas de edición en el viewport.

**Independent Test**: con el puntero sobre la Escena, arrastrar con botón derecho, arrastrar con
botón central y usar la rueda; comprobar que la vista cambia y que los paneles no reaccionan.

**Acceptance Scenarios**:

1. **Given** el puntero sobre la Escena, **When** se arrastra con el botón derecho, **Then** la
   cámara orbita alrededor de su punto focal.
2. **Given** el puntero sobre la Escena, **When** se arrastra con el botón central, **Then** la
   cámara se desplaza lateralmente (paneo).
3. **Given** el puntero sobre la Escena, **When** se gira la rueda, **Then** la cámara se acerca o
   se aleja, sin atravesar el punto focal ni alejarse indefinidamente.
4. **Given** el puntero sobre un panel lateral, **When** se gira la rueda o se arrastra, **Then** la
   cámara de la Escena no se mueve.

### Edge Cases

- **Ventana muy pequeña:** si el ancho no alcanza para los paneles, la Escena se reduce hasta un
  mínimo y nunca provoca un error; la imagen 3D no se renderiza con tamaño cero de forma inválida.
- **Ventana minimizada:** al minimizar y restaurar, el editor vuelve a mostrarse sin errores.
- **Objeto seleccionado eliminado:** si el objeto seleccionado deja de existir, la selección se
  limpia y el Inspector vuelve a "Ningún objeto seleccionado".
- **Objetos sin nombre:** las entidades de la escena sin nombre se muestran como "Entidad <id>"
  para que nada quede oculto.
- **Escena vacía:** la Jerarquía muestra "La escena está vacía".
- **Muchos objetos:** con más objetos de los que caben, la Jerarquía permite desplazarse.
- **Zoom extremo:** la distancia de la cámara queda limitada entre un mínimo y un máximo.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: El editor MUST abrirse en una ventana titulada "Dragon Studio RD".
- **FR-002**: El espacio de trabajo MUST tener cinco zonas: barra superior, Jerarquía (izquierda),
  Escena (centro), Inspector (derecha) y barra de estado (abajo).
- **FR-003**: La barra superior MUST mostrar el nombre del producto y los menús Archivo, Editar,
  Ver y Ayuda (en esta feature los menús son solo visuales; sus acciones llegan en features
  posteriores).
- **FR-004**: La Escena MUST mostrar el mundo 3D renderizado dentro de su panel, ajustado al
  tamaño del panel en todo momento.
- **FR-005**: El editor MUST incluir una escena de demostración con suelo, al menos tres objetos
  con nombre (al menos uno con un hijo y uno en movimiento continuo) y una luz.
- **FR-006**: La barra de estado MUST mostrar la versión de Dragon, la versión de Bevy y los FPS
  actualizados al menos dos veces por segundo.
- **FR-007**: Todos los colores, tamaños y tipografías del editor MUST provenir de un único módulo
  de tema (principio VI).
- **FR-008**: Las entidades propias del editor MUST estar marcadas y excluidas de la Jerarquía
  (principio VII).
- **FR-009**: La Jerarquía MUST listar las entidades de la escena respetando el anidamiento
  padre-hijo, y MUST actualizarse cuando se crean o eliminan objetos.
- **FR-010**: Hacer clic en una fila de la Jerarquía MUST seleccionar ese objeto; MUST existir como
  máximo un objeto seleccionado a la vez.
- **FR-011**: La fila del objeto seleccionado MUST verse resaltada con el color de acento.
- **FR-012**: El Inspector MUST mostrar el nombre y la transformación (posición, rotación en
  grados y escala) del objeto seleccionado, y actualizarse cuando esta cambie.
- **FR-013**: Si el objeto seleccionado desaparece, la selección MUST limpiarse.
- **FR-014**: La cámara de la Escena MUST orbitar (botón derecho), desplazarse (botón central) y
  hacer zoom (rueda) solo cuando el puntero está sobre la Escena.
- **FR-015**: La distancia de zoom MUST estar acotada entre un mínimo y un máximo.
- **FR-016**: Cada zona del editor MUST implementarse como una pieza independiente que pueda
  añadirse o quitarse por separado (principio VII).
- **FR-017**: Debe existir un modo de desarrollo con compilación rápida y un modo de distribución
  sin enlazado dinámico (principio III), cada uno con un comando documentado.
- **FR-018**: Ningún archivo de Bevy fuera de las excepciones del principio I MUST modificarse.

### Key Entities

- **Selección**: el objeto de la escena actualmente elegido (o ninguno). Única en todo el editor.
- **Objeto de escena**: cualquier entidad del mundo del usuario. Tiene opcionalmente un nombre, una
  transformación y un padre.
- **Entidad del editor**: cualquier entidad creada por el editor para sí mismo (paneles, cámaras).
  Nunca forma parte de la escena.
- **Cámara del editor**: la vista de la Escena; tiene un punto focal, una distancia y dos ángulos
  (horizontal y vertical).
- **Tema**: el conjunto central de colores, tamaños y tipografías del editor.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Una persona que nunca usó el editor identifica las cinco zonas y selecciona un objeto
  en menos de 30 segundos sin instrucciones.
- **SC-002**: Con el editor ya compilado, la interfaz completa aparece en ≤ 3 segundos desde el
  lanzamiento.
- **SC-003**: Tras cambiar una línea del editor, la recompilación en modo desarrollo termina en
  ≤ 10 segundos (meta: ≤ 3 s).
- **SC-004**: El editor se mantiene a ≥ 60 FPS con la escena de demostración en el equipo de
  desarrollo.
- **SC-005**: El 100 % de los objetos con nombre de la escena aparece en la Jerarquía y el 0 % de
  las entidades del editor.
- **SC-006**: Al seleccionar un objeto en movimiento, el Inspector refleja su posición con un
  retraso máximo de un cuadro.

## Assumptions

- El editor se ejecuta en un equipo de escritorio con GPU compatible con Vulkan, DirectX 12 o Metal;
  el equipo de referencia es el de RD (Windows 10).
- En esta feature el Inspector es de solo lectura; editar valores llega en una feature posterior.
- El Inspector muestra la transformación **local** (relativa al padre), como Unity y Godot. Un hijo
  que se mueve porque gira su padre (la Luna) conserva sus valores locales; el objeto cuyos valores
  cambian es el padre (el Planeta). *(Aclaración surgida al validar la historia 3.)*
- Los menús de la barra superior son solo visuales en esta feature.
- La escena de demostración es temporal; cargar y guardar escenas reales es una feature posterior.
- La exportación a Android no forma parte de esta feature (ver ROADMAP); la feature no debe
  introducir nada que la impida en los crates de runtime.
- La compilación inicial completa de Bevy puede tardar varios minutos; las metas de velocidad se
  refieren a recompilaciones incrementales.

# gradebook-rs 🦀

Sistema de gestión de calificaciones para estudiantes de nivel no universitario, hecho en Rust como proyecto de práctica dentro de mi camino de aprendizaje de dicho lenguaje.

## Funcionalidades

El programa corre en consola con un menú interactivo:

1. **Agregar estudiante/s** — registra uno o varios estudiantes, cada uno con una clave única autoincremental.
2. **Agregar curso** — asigna uno o varios cursos a un estudiante existente, cada curso con sus 4 unidades (I, II, III, IV) inicializadas en 0.
3. **Agregar/actualizar nota** — permite ingresar (o sobreescribir) las 4 notas de un curso específico de un estudiante, con validación completa antes de guardarlas.
4. **Ver boleta** — muestra todos los cursos de un estudiante, el detalle de nota por unidad, el total acumulado, y si ganó o perdió cada curso.
5. **Salir**

## Cómo ejecutarlo

```bash
git clone https://github.com/<tu-usuario>/gradebook-rs.git
cd gradebook-rs
cargo run
```

## Planes futuros

- **Persistencia con JSON (Usando `serde`)**: actualmente todos los datos viven en memoria y se pierden al cerrar el programa. La idea es serializar `Vec<Estudiantes>` a un archivo `.json` al salir, y deserializarlo al iniciar, para que el progreso se mantenga entre sesiones (Simulando cómo funcionaría en la vida real, donde las notas se cargan por periodos, no todas de una vez).

## Sobre el proyecto

Hecho como parte de mi aprendizaje de Rust. Cualquier sugerencia o corrección es bienvenida — Sigo aprendiendo. 🦀

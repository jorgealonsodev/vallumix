# Contribuir

¡Gracias por tu interés en contribuir a Vallumix! Este documento proporciona directrices para comenzar.

## Configuración de Desarrollo

1. Clona el repositorio:
   ```bash
   git clone https://github.com/vallumix/vallumix.git
   cd vallumix
   ```

2. Compila el workspace:
   ```bash
   cargo build --workspace
   ```

3. Ejecuta los tests:
   ```bash
   cargo test --workspace
   ```

4. Ejecuta clippy:
   ```bash
   cargo clippy --workspace -- -D warnings
   ```

## Estilo de Código

- Sigue las Rust API Guidelines.
- Ejecuta `cargo fmt` antes de hacer commit.
- Todos los elementos públicos deben tener comentarios de documentación.
- Usa nombres significativos para variables y funciones.

## Añadir Nuevos Controles

1. Implementa el trait `Control` en `crates/vallumix-controls/src/`.
2. Añade el control al archivo TOML de perfil apropiado.
3. Escribe tests unitarios para la lógica del control.
4. Actualiza la documentación de mapeo de controles CIS.

## Testing

- Escribe tests unitarios para funciones puras y lógica de controles.
- Usa tests de integración en `crates/vallumix-cli/tests/cli.rs` para comportamiento CLI.
- Se recomiendan tests de snapshot usando `insta` para la salida de reporters.

## Enviar Cambios

1. Crea una rama de feature desde `develop`.
2. Realiza tus cambios con mensajes de commit claros.
3. Asegúrate de que todos los tests pasen.
4. Abre un pull request contra la rama `develop`.

## Reportar Issues

Por favor usa el issue tracker de GitHub para reportar bugs o solicitar features. Incluye:
- Tu sistema operativo y versión
- La versión de Vallumix
- Pasos para reproducir
- Comportamiento esperado y actual

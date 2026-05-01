# Introducción

<p align="center">
  <img src="/vallumix/img/valumix.png" alt="Vallumix" width="640">
</p>

Vallumix es un motor modular de endurecimiento para sistemas Linux, escrito en Rust y alineado con los benchmarks del Center for Internet Security (CIS). Automatiza la aplicación de controles de seguridad en servidores, eliminando la inconsistencia del hardening manual y generando reportes de cumplimiento trazables para auditorías.

## ¿Qué hace Vallumix?

En esencia, Vallumix evalúa la postura de seguridad de un servidor Linux y, cuando se le indica, aplica remediaciones automáticas basadas en el CIS Benchmark. Cada control —desde deshabilitar filesystems innecesarios hasta endurecer la configuración de SSH— se implementa como una unidad independiente, testeable y reversible. El resultado es una herramienta que transforma el hardening de horas de trabajo manual en un proceso de minutos, reproducible y documentado.

### Capacidades principales

- **70 controles CIS** distribuidos en seis categorías: configuración inicial, servicios, red, logging y auditoría, acceso y autenticación, y mantenimiento del sistema.
- **Tres perfiles preconfigurados** que adaptan la selección de controles al rol del servidor: web, base de datos o bastión.
- **Cinco formatos de reporte**: HTML autocontenido, JSON parseable, JUnit XML para pipelines, texto plano con colores, y salida estructurada en JSON Lines para integración con SIEMs.
- **Modo dry-run** que previsualiza todos los cambios sin modificarlos, ideal para validación previa en producción.
- **Rollback granular** con respaldos versionados en `/var/backups/vallumix`, permitiendo revertir un control específico o una sesión completa.
- **Binario único estático** compilado con `musl`, sin dependencias de runtime: lo copias, lo ejecutas, funciona.

## ¿Para quién es?

Vallumix está diseñado para tres perfiles profesionales:

- **Administradores de sistemas** en PYMEs que necesitan endurecer servidores rápidamente y demostrar cumplimiento ante auditorías ISO 27001 o PCI-DSS sin invertir en herramientas comerciales.
- **Ingenieros DevOps y DevSecOps** que integran seguridad en pipelines de CI/CD con Terraform, Packer o Ansible, y requieren artefactos parseables y códigos de salida coherentes.
- **Consultores de seguridad freelance** que auditan clientes con una herramienta portable, de un solo binario, que genera reportes presentables y permite remediación controlada.

```tip
¿Es tu primera vez con Vallumix? Ve directamente a la [guía de instalación](installation/README.md) y luego ejecuta `vallumix audit --profile web --report html` para ver cómo funciona sin modificar nada. El modo audit es la forma más segura de familiarizarte con la herramienta.
```

## Por qué Rust

La elección de Rust no es accidental. Una herramienta que se ejecuta como root y modifica archivos de sistema críticos debe ser intrínsecamente segura. Rust garantiza ausencia de errores de memoria en código safe, obliga al manejo explícito de errores mediante `Result<T, E>`, y permite compilar un binario estático que no depende de versiones de Python, intérpretes de Bash ni paquetes del sistema. Esto diferencia a Vallumix de soluciones basadas en scripts que fallan silenciosamente o rompen dependencias entre distribuciones.

## Estado del proyecto

Vallumix es un proyecto de código abierto en evolución activa. La versión 1.0 soporta Debian 12, Ubuntu 22.04/24.04 LTS, RHEL 9 y derivadas (Rocky Linux, AlmaLinux). La documentación está disponible en español e inglés. Si encuentras un problema o tienes una sugerencia, consulta la guía de contribución en el repositorio.

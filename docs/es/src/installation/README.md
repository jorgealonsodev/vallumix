# Instalación

Vallumix se distribuye como un binario único sin dependencias de runtime. Puedes instalarlo mediante tres métodos, dependiendo de tu entorno y necesidades.

## Métodos de instalación

| Método | Ideal para | Requiere privilegios | Tiempo estimado |
|---|---|---|---|
| **Paquete `.deb`** | Debian 12, Ubuntu 22.04/24.04 | Root | < 1 min |
| **Paquete `.rpm`** | RHEL 9, Rocky, AlmaLinux | Root | < 1 min |
| **Desde código fuente** | Desarrolladores, arquitecturas no soportadas, parches | Root (solo para instalar) | 5-10 min |

## Decisión rápida

```
¿Usas Debian o Ubuntu?     →  .deb  (ver página siguiente)
¿Usas RHEL, Rocky o Alma?  →  .rpm  (ver página siguiente)
¿Necesitas modificar el código o compilar para ARM64?  →  Desde fuente
```

## Requisitos previos comunes

Independientemente del método:

- Acceso root o capacidad de ejecutar `sudo`.
- Sistema operativo soportado (consulta la sección de compatibilidad).
- Espacio libre en disco: al menos 50 MB para el binario y los respaldos iniciales.

```warning
Vallumix debe ejecutarse como root para aplicar controles de hardening. Sin embargo, la instalación del binario en sí puede hacerse como usuario normal si lo colocas en `~/.local/bin`. Solo las operaciones `apply`, `audit` y `rollback` requieren elevación de privilegios.
```

## Verificación post-instalación

Tras instalar Vallumix, verifica que funciona correctamente:

```bash
vallumix --version
vallumix list --profile web
```

Si ambos comandos devuelven salida sin errores, la instalación es correcta y puedes continuar con la [guía de primeros pasos](getting-started.md).

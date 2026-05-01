# Informe de Texto para Terminal

El reporter de texto produce un informe amigable para terminal, diseñado para operadores que desean retroalimentación inmediata durante una ejecución de `audit` o `apply`. Usa iconos Unicode y códigos de color ANSI por defecto, con retroceso automático a texto plano cuando la salida se canaliza o cuando `NO_COLOR` está configurado.

## Cuándo Usar Texto

- **Triaje de operadores** — detecta rápidamente controles fallidos después de ejecutar `apply`.
- **Paneles de terminal** — muestra el estado de cumplimiento en una consola o monitor de pared.
- **Streaming de logs** — transmite salida a un multiplexor de terminal (tmux, screen) durante la automatización.
- **Filtrado por canalización** — canaliza a `grep` o `less` para aislar controles específicos.

## Generar un Informe de Texto

```bash
vallumix audit --profile web --report text --output /var/reports/vallumix/audit
```

Cuando se usa `--report text` sin `--output`, el informe se imprime en stdout.

## Formato de Salida

```text
Vallumix Compliance Report — srv01 (rocky/9)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Total:  45   Pass:  38   Fail:   5   Skip:   2
  Compliance Rate: 84.4%

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
[LOW]  ✓ Compliant 1.1.1.1 — Disable cramfs
      → not present
[HIGH] ✗ Non-Compliant 5.2.4 — Disable root login
      → PermitRootLogin yes
[MED]  ⚠ Skipped 3.1.1 — Disable IP forwarding
      → dry-run
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## Elementos Visuales

| Elemento | Modo de Color | Modo Plano (`NO_COLOR`) |
|----------|---------------|--------------------------|
| Compliant | `✓ Compliant` (verde) | `OK` |
| NonCompliant | `✗ Non-Compliant` (rojo) | `FAIL` |
| Skipped | `⚠ Skipped` (amarillo) | `SKIP` |
| Severidad alta | `[HIGH]` (rojo, negrita) | `[HIGH]` |
| Severidad media | `[MED]` (amarillo) | `[MED]` |
| Severidad baja | `[LOW]` (verde) | `[LOW]` |

## Soporte para NO_COLOR

Vallumix respeta el estándar [NO_COLOR](https://no-color.org/). Si la variable de entorno `NO_COLOR` está configurada con cualquier valor no vacío, el reporter de texto elimina todas las secuencias de escape ANSI:

```bash
NO_COLOR=1 vallumix audit --profile web --report text
```

Esto es útil cuando:

- Rediriges la salida a un archivo que será leído por humanos.
- Ejecutas dentro de sistemas CI que no renderizan códigos ANSI.
- Canalizas hacia herramientas que no procesan secuencias de escape.

## Canalización y Filtrado

Dado que el formato de texto está orientado a líneas, funciona bien con herramientas Unix estándar:

```bash
# Mostrar solo controles fallidos
vallumix audit --profile web --report text | grep 'FAIL'

# Contar fallos de alta severidad
vallumix audit --profile web --report text | grep -c '\[HIGH\].*FAIL'

# Transmitir a un archivo de log con marcas temporales
vallumix audit --profile web --report text | ts '[%Y-%m-%d %H:%M:%S]' > /var/log/vallumix/audit.log
```

```tip
En un terminal con fondo oscuro, el esquema de colores predeterminado (aprobado en verde, fallido en rojo, omitido en amarillo) proporciona alto contraste. Si tu tema de terminal usa colores inusuales, el modo de texto plano (`NO_COLOR=1`) puede ser más legible.
```

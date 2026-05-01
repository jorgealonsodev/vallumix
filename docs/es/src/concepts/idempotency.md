# Idempotencia

La idempotencia es una de las propiedades fundamentales de Vallumix. Significa que ejecutar la herramienta una vez, dos veces o cien veces sobre el mismo servidor produce el mismo estado final, sin efectos secundarios acumulativos ni duplicación de cambios.

## ¿Por qué importa?

En entornos de producción, el hardening no es un evento único. Los servidores se reprovisionan, las configuraciones se ajustan, y las políticas de seguridad evolucionan. Si una herramienta de hardening no es idempotente, cada ejecución adicional puede:

- Acumular reglas duplicadas en archivos de configuración.
- Agregar múltiples entradas idénticas en cron o systemd.
- Sobreescribir respaldos anteriores con versiones posteriores, perdiendo la trazabilidad.
- Generar reportes inconsistentes entre ejecuciones.

Vallumix evita todos estos problemas diseñando cada control para que sea inherentemente idempotente.

## Cómo garantiza la idempotencia Vallumix

### Pre-check antes de aplicar

Antes de modificar cualquier archivo, cada control ejecuta una fase `pre_check` que evalúa si el sistema ya cumple con la recomendación. Si el control determina que el estado actual es compliant, salta directamente al siguiente sin tocar nada.

```rust
fn apply(&self, ctx: &Context) -> Result<ApplyResult, ControlError> {
    match self.check(ctx)? {
        CheckResult::Compliant => {
            return Ok(ApplyResult::SkippedAlreadyCompliant);
        }
        CheckResult::NonCompliant => {
            // Procede al backup y aplicación
        }
    }
}
```

### Backup condicional

Los respaldos solo se crean cuando un control realmente va a modificar algo. Si todos los controles de una ejecución ya están compliant, no se genera ningún backup nuevo vacío.

### Aplicaciones atómicas

Cuando un control modifica un archivo, lo hace de forma atómica: escribe el contenido nuevo en un archivo temporal y luego usa `rename` para reemplazar el original. Esto garantiza que el archivo nunca quede en un estado intermedio corrupto, incluso si el proceso se interrumpe.

### Post-check de verificación

Después de aplicar un cambio, el control ejecuta `post_check` para confirmar que el estado del sistema coincide con lo esperado. Si no coincide, el resultado se marca como fallido y el reporte documenta la discrepancia.

## Dry-run como validación de idempotencia

El modo `--dry-run` es la herramienta más efectiva para verificar la idempotencia sin riesgo:

```bash
sudo vallumix apply --profile web --dry-run --verbose
```

En este modo, Vallumix ejecuta `pre_check` para todos los controles e informa qué cambios *se harían*, pero no ejecuta `backup` ni `apply`. Si ejecutas `--dry-run` inmediatamente después de un `apply` exitoso, deberías ver que todos los controles aparecen como `Compliant` o `SkippedAlreadyCompliant`.

```tip
Incorpora `vallumix apply --profile web --dry-run` en tus playbooks de Ansible o scripts de aprovisionamiento como paso de validación antes de marcar un servidor como listo. Si el dry-run reporta cambios pendientes, significa que algo en tu proceso anterior no fue idempotente.
```

## Idempotencia y reportes

Los reportes reflejan la idempotencia mediante los estados:

- **Compliant:** el control ya cumplía antes de la ejecución.
- **Remediated:** el control no cumplía y se aplicó con éxito.
- **SkippedAlreadyCompliant:** el control se saltó porque el `pre_check` indicó cumplimiento.
- **Failed:** el control no cumplía, se intentó remediar, pero el `post_check` falló.

Un reporte idempotente sobre un servidor ya endurecido debería mostrar predominantemente `Compliant` y `SkippedAlreadyCompliant`, con cero entradas `Remediated`.

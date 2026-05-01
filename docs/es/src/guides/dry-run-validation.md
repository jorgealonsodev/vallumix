# Validación en Dry-Run (CU-03)

El modo dry-run permite revisar exactamente qué cambios realizaría Vallumix antes de aplicarlos. Es una práctica obligatoria en entornos de producción donde cualquier modificación no planificada puede causar interrupciones de servicio.

## Escenario

- **Entorno:** Pre-producción que replica fielmente el entorno productivo.
- **Rol del servidor:** Bastión SSH en Rocky Linux 9.
- **Objetivo:** Validar que el perfil `bastion` no rompe el acceso SSH administrativo antes de aplicarlo en producción.
- **Preocupación:** El perfil bastión deshabilita autenticación por contraseña y podría bloquear acceso si las claves no están correctamente distribuidas.

## Paso 1: Verificar acceso por clave SSH

Antes de cualquier dry-run del perfil bastión, confirma que puedes acceder por clave pública:

```bash
# Desde tu estación de trabajo
ssh -o PasswordAuthentication=no admin@bastion-preprod
# Si falla, NO apliques el perfil bastión hasta resolverlo
```

```danger
El perfil bastión deshabilita explícitamente `PasswordAuthentication` en `sshd_config`. Si accedes por contraseña y aplicas este perfil, te bloquearás. Verifica el acceso por clave antes de continuar.
```

## Paso 2: Ejecutar dry-run

```bash
sudo vallumix apply --profile bastion --dry-run --verbose --report json --output /tmp/dry-run-bastion
```

### Qué hace exactamente el dry-run

- Ejecuta `check` en todos los controles del perfil, igual que `audit`.
- Para controles `NonCompliant`, genera una entrada en el reporte que describe **qué cambio se haría**, incluyendo:
  - Archivo que se modificaría.
  - Valor actual vs valor objetivo.
  - Justificación CIS.
- **No ejecuta `backup` ni `apply`**. El sistema permanece intacto.

### Salida esperada (extracto)

```text
[Dry-run] 5.2.4  Ensure SSH root login is disabled
  → Would set: PermitRootLogin no
  → In file:   /etc/ssh/sshd_config
  → Current:   PermitRootLogin yes
  → Justification: CIS 5.2.4 — Root login via SSH increases attack surface

[Dry-run] 5.2.8  Ensure SSH password authentication is disabled
  → Would set: PasswordAuthentication no
  → In file:   /etc/ssh/sshd_config
  → Current:   PasswordAuthentication yes
  → Justification: CIS 5.2.8 — Passwords are vulnerable to brute-force attacks

[Dry-run] 3.4.2  Ensure default deny firewall policy
  → Would execute: nftables rule set
  → Current:       no active firewall rules
  → Impact:        Port 22 will be explicitly allowed; all others dropped
```

## Paso 3: Revisar controles de alto riesgo

En el reporte JSON, filtra los controles que podrían impactar operaciones críticas:

```bash
jq '.controls[] | select(.impact == "High")' /tmp/dry-run-bastion.json
```

Presta especial atención a:

- Controles de SSH que cambian métodos de autenticación.
- Controles de firewall que restringen puertos.
- Controles PAM que afectan políticas de contraseñas o bloqueo de cuentas.
- Controles de sudo que limitan privilegios administrativos.

## Paso 4: Ajustar el entorno si es necesario

Si el dry-run revela que un control rompería algo esencial, tienes dos opciones antes de aplicar:

1. **Modificar el entorno:** por ejemplo, distribuir claves SSH a todos los administradores antes de deshabilitar contraseñas.
2. **Omitir el control específico:** edita el perfil TOML y comenta el control problemático. Esta opción está documentada en la referencia de perfiles.

## Paso 5: Aplicar tras validación exitosa

Una vez que el dry-run no muestra sorpresas:

```bash
sudo vallumix apply --profile bastion --report html,json --output /tmp/bastion-applied
```

```tip
Automatiza el dry-run en tu pipeline de CI: ejecuta `vallumix apply --profile $PROFILE --dry-run --report json` en la etapa de validación. Si el reporte contiene controles con `impact: "High"` que no están en una lista de excepciones aprobada, falla el pipeline y requiere revisión manual.
```

## Paso 6: Comparar dry-run vs ejecución real

Para auditoría interna, compara los reportes:

```bash
# Controles que el dry-run dijo que remediaría
jq '.controls[] | select(.dry_run_action == "WouldRemediate") | .id' /tmp/dry-run-bastion.json | sort > /tmp/expected.txt

# Controles que realmente se remediaron
jq '.controls[] | select(.status == "Remediated") | .id' /tmp/bastion-applied.json | sort > /tmp/actual.txt

# Deben ser idénticos
diff /tmp/expected.txt /tmp/actual.txt
```

Si hay diferencias, investiga: podría indicar que el estado del sistema cambió entre el dry-run y la ejecución real, o que un control falló silenciosamente.

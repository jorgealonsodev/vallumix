# Preguntas Frecuentes

## 1. ¿Vallumix sustituye a OpenSCAP o Lynis?

No. Vallumix se posiciona en un segmento diferente: es una herramienta moderna, autónoma y escrita en Rust que aplica cambios y permite rollback. OpenSCAP es más completo en cobertura de benchmarks pero requiere infraestructura de contenido SCAP. Lynis audita pero no aplica remediaciones automáticas. Usa la herramienta que mejor se adapte a tu flujo de trabajo.

## 2. ¿Es seguro ejecutar Vallumix en producción?

Sí, si sigues las precauciones recomendadas: ejecuta `--dry-run` primero, verifica el reporte, asegúrate de tener acceso a la consola física o VNC en caso de bloqueo SSH, y utiliza el perfil adecuado para el rol del servidor. El sistema de rollback te permite revertir cambios si algo falla.

## 3. ¿Puedo omitir controles específicos?

Sí. Edita el archivo de perfil TOML (`/etc/vallumix/profiles/web.toml` o una copia local) y comenta o elimina el control que deseas omitir. Luego usa tu perfil personalizado con `--profile /ruta/a/mi-perfil.toml`.

## 4. ¿Cuánto espacio en disco necesito?

Vallumix requiere al menos 100 MB libres en `/var/backups/vallumix` para la sesión de respaldo inicial. El binario ocupa menos de 8 MB. Los reportes HTML son típicamente de 50-200 KB.

## 5. ¿Puedo usar Vallumix en contenedores Docker?

No es el caso de uso previsto. Los controles CIS están diseñados para sistemas operativos completos, no para contenedores. El hardening de contenedores requiere el CIS Docker Benchmark, que es un enfoque distinto. Vallumix abortará en la mayoría de contenedores porque no detectará una distribución soportada completa.

## 6. ¿Cómo actualizo Vallumix?

Descarga la nueva versión del release e instálala sobre la anterior. Los respaldos en `/var/backups/vallumix` se conservan. Tras actualizar, ejecuta `vallumix --version` para confirmar.

```bash
wget https://github.com/jorgealonsodev/vallumix/releases/download/v1.1.0/vallumix_1.1.0_amd64.deb
sudo dpkg -i vallumix_1.1.0_amd64.deb
```

## 7. ¿Los respaldos caducan?

No. Vallumix no elimina respaldos automáticamente. Configura una tarea cron para limpiar sesiones antiguas si el espacio en disco es limitado.

## 8. ¿Puedo aplicar múltiples perfiles en el mismo servidor?

No está recomendado. Los perfiles pueden tener controles superpuestos con configuraciones conflictivas. Si necesitas características de varios perfiles, crea un perfil personalizado que combine los controles deseados sin duplicaciones.

## 9. ¿Qué pasa si un control falla?

Un control en estado `Failed` significa que la remediación no pudo aplicarse o el post-check no confirmó el cambio. El servidor sigue funcionando; Vallumix no aborta la ejecución por un control fallido. Revisa el reporte detallado para entender la causa y corrige manualmente si es necesario.

## 10. ¿Vallumix modifica contraseñas de usuario?

No. Vallumix no gestiona contraseñas de usuarios ni crea ni elimina cuentas. Los controles de autenticación se limitan a configuraciones del sistema como PAM, SSH y sudoers.

## 11. ¿Cómo reporto un bug o solicito una función?

Abre un issue en el repositorio de GitHub. Para bugs, incluye la distribución, la versión de Vallumix, el comando ejecutado y el mensaje de error completo. Para solicitudes de función, describe el caso de uso y por qué la funcionalidad actual no lo cubre.

## 12. ¿Vallumix garantiza la seguridad de mi servidor?

No. Vallumix automatiza controles CIS que reducen la superficie de ataque, pero la seguridad es un proceso continuo, no un producto. Sigue aplicando parches, monitorea logs, realiza auditorías periódicas y complementa con herramientas de detección de intrusiones. Vallumix es una pieza del rompecabezas, no el rompecabezas completo.

```tip
Si tu pregunta no está aquí, revisa la [sección de solución de problemas](troubleshooting.md) o busca en los issues abiertos del repositorio. Es probable que alguien ya haya encontrado el mismo problema.
```

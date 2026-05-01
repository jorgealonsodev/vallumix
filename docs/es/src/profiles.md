# Guía de Perfiles

Los perfiles son conjuntos predefinidos de controles CIS adaptados a roles específicos de servidor.

## Perfiles Integrados

### Perfil de Servidor Web (`web`)

Optimizado para servidores HTTP/HTTPS que ejecutan Nginx, Apache o similares.

Controles clave:
- Deshabilitar servicios innecesarios
- Configurar reglas de firewall para HTTP/HTTPS
- Establecer permisos de archivos seguros para directorios web
- Habilitar logging de auditoría para tráfico web

### Perfil de Servidor de Base de Datos (`database`)

Diseñado para PostgreSQL, MariaDB, MongoDB y servidores de bases de datos similares.

Controles clave:
- Restringir acceso de red a puertos de base de datos
- Exigir autenticación fuerte
- Configurar logging de auditoría para operaciones de base de datos
- Establecer parámetros de kernel apropiados para cargas de trabajo de base de datos

### Perfil de Host Bastión (`bastion`)

Endurecimiento para jump hosts y bastiones SSH.

Controles clave:
- Configuración estricta de SSH (solo autenticación por clave, sin login root)
- Tiempo de espera de sesión y desconexión por inactividad
- Logging y monitoreo mejorados
- Paquetes instalados mínimos

## Perfiles Personalizados

Puedes crear perfiles personalizados añadiendo archivos TOML al directorio de perfiles. Consulta la documentación de mapeo de controles para los IDs de control disponibles y sus opciones de configuración.

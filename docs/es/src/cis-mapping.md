# Mapeo de Controles CIS

Vallumix implementa controles alineados con los benchmarks CIS para distribuciones Linux. Este capítulo proporciona una referencia de mapeo entre los controles de Vallumix y las recomendaciones CIS.

## Categorías de Controles

### 1.x Configuración Inicial

Controles relacionados con la configuración del sistema de archivos, actualizaciones de software y configuración inicial del sistema.

### 2.x Servicios

Controles para configurar y asegurar servicios del sistema, incluyendo la deshabilitación de servicios innecesarios y la configuración de sincronización de tiempo.

### 3.x Configuración de Red

Controles para la configuración del firewall, parámetros de red y endurecimiento de la pila TCP/IP.

### 4.x Logging y Auditoría

Controles para configurar auditd, rsyslog y asegurar niveles de logging apropiados para eventos de seguridad.

### 5.x Acceso, Autenticación y Autorización

Controles para la gestión de cuentas de usuario, configuración PAM, endurecimiento SSH y políticas de contraseñas.

### 6.x Mantenimiento del Sistema

Controles para permisos de archivos, verificación de integridad del sistema y asegurar que solo software autorizado esté instalado.

## Estado de los Controles

Cada control puede reportar uno de los siguientes estados durante una auditoría:

- **Compliant**: El sistema cumple con la recomendación CIS.
- **Non-Compliant**: El sistema no cumple con la recomendación.
- **Not Applicable**: El control no aplica a la distribución o configuración actual.

## Niveles de Severidad

Los controles se clasifican por severidad:

- **High**: Problemas de seguridad críticos que deben abordarse inmediatamente.
- **Medium**: Problemas importantes que deben abordarse en el corto plazo.
- **Low**: Mejoras recomendadas con menor riesgo inmediato.

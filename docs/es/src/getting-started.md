# Primeros Pasos

Vallumix está diseñado para hacer que el endurecimiento de servidores Linux sea directo y reproducible. Ya sea que gestiones un único servidor web o una flota de instancias de bases de datos, Vallumix proporciona las herramientas necesarias para hacer cumplir las líneas base de seguridad.

## ¿Qué es Vallumix?

Vallumix es una herramienta de línea de comandos que aplica controles de benchmarks CIS a tus sistemas Linux. Soporta múltiples distribuciones y proporciona capacidades de rollback, haciendo que sea seguro experimentar con diferentes configuraciones de endurecimiento.

## Conceptos Clave

- **Perfiles**: Conjuntos predefinidos de controles adaptados a roles específicos de servidor (web, base de datos, bastión).
- **Controles**: Comprobaciones de seguridad individuales y acciones de remediación alineadas con benchmarks CIS.
- **Sesiones**: Una instantánea de los cambios aplicados que puede revertirse si es necesario.
- **Informes**: Salida detallada de cumplimiento en HTML, JSON, JUnit XML o texto plano.

## Antes de Comenzar

Asegúrate de tener acceso root o sudo en el sistema objetivo, ya que muchos controles de endurecimiento requieren privilegios administrativos para modificar archivos de configuración del sistema.

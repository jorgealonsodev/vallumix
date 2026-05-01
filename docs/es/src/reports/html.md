# Informe HTML

El reporter HTML produce un informe autónomo en un único archivo, adecuado para auditores, oficiales de cumplimiento y presentaciones de gestión. Todo el CSS está incrustado en el documento; no se requieren solicitudes de red externas ni hojas de estilo.

## Cuándo Usar HTML

- **Auditorías de cumplimiento** — envía un único archivo `.html` a un auditor externo.
- **Paneles de gestión** — ábrelo en un navegador para un resumen visual de la postura de seguridad del servidor.
- **Revisión sin conexión** — funciona sin acceso a Internet porque todos los estilos están en línea.
- **Adjuntos de correo electrónico** — un único archivo lo contiene todo.

## Generar un Informe HTML

```bash
vallumix audit --profile web --report html --output /var/reports/vallumix/audit
```

El archivo resultante es `/var/reports/vallumix/audit.html`.

## Estructura del Informe

El informe HTML incluye las siguientes secciones:

1. **Encabezado** — logo de Vallumix, marca temporal de generación, nombre del host y distribución.
2. **Tarjeta de Resumen** — gráfico circular o de barras que muestra los totales de aprobados, fallidos, omitidos y el porcentaje de tasa de cumplimiento.
3. **Tabla de Controles** — tabla ordenable con columnas para ID CIS, Descripción, Severidad, Estado, Evidencia y Mensaje. El estado está codificado por colores: verde para Compliant, rojo para NonCompliant, amarillo para Skipped.
4. **Pie de Página** — versión de Vallumix, nombre del perfil y una nota sobre verificación manual.

## Descripción de la Captura de Pantalla

Al abrirlo en un navegador, el informe presenta un diseño limpio y profesional:

- La barra superior muestra el nombre del host (`web01`) y la distribución (`debian/12`) a la izquierda, y la marca temporal de generación a la derecha.
- Una gran cifra de tasa de cumplimiento (por ejemplo, **87.3%**) se muestra prominentemente debajo de la barra.
- Tres cajas de resumen muestran **Pass: 38**, **Fail: 5**, **Skip: 2** con fondos verde, rojo y ámbar respectivamente.
- La tabla de controles alterna colores de fila para legibilidad. Cada fila tiene una insignia de estado (`Compliant`, `Non-Compliant`, `Skipped`).
- Al pasar el cursor sobre una insignia de severidad (`High`, `Medium`, `Low`) se muestra una descripción emergente con la definición de severidad del CIS.
- Las filas con mensajes (por ejemplo, `should be no`) muestran una flecha de detalles expandible.

```tip
El reporter HTML usa el motor de plantillas Askama. La plantilla `report.html` se compila en el binario, por lo que la generación de informes no requiere archivos de plantilla en tiempo de ejecución.
```

## Fragmento de Ejemplo de Salida

```html
<!DOCTYPE html>
<html lang="es">
<head>
  <meta charset="UTF-8">
  <title>Informe de Cumplimiento Vallumix — web01</title>
  <style>
    body { font-family: system-ui, sans-serif; margin: 2rem; }
    .summary { display: flex; gap: 1rem; }
    .badge-pass { background: #22c55e; color: #fff; padding: .25rem .5rem; border-radius: .25rem; }
    .badge-fail { background: #ef4444; color: #fff; padding: .25rem .5rem; border-radius: .25rem; }
  </style>
</head>
<body>
  <h1>Informe de Cumplimiento Vallumix</h1>
  <p>Host: <strong>web01</strong> | Distro: <strong>debian/12</strong> | Generado: 2024-06-01 14:32:00 +0000</p>
  <div class="summary">
    <div>Total: 45</div>
    <div class="badge-pass">Pass: 38</div>
    <div class="badge-fail">Fail: 5</div>
    <div>Skip: 2</div>
  </div>
  <p>Tasa de Cumplimiento: <strong>84.4%</strong></p>
  <!-- Tabla de controles a continuación -->
</body>
</html>
```

## Personalización

Dado que la plantilla se compila en el binario, personalizar la salida HTML requiere modificar `crates/vallumix-reporters/templates/report.html` y recompilar Vallumix. Las versiones futuras pueden admitir plantillas proporcionadas por el usuario a través del directorio `--output`.

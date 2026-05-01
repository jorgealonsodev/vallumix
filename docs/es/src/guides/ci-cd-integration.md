# Integración CI/CD (CU-05)

Esta guía describe cómo integrar Vallumix en pipelines de aprovisionamiento automatizado usando Packer, Terraform y plataformas de CI como Jenkins, GitLab CI o GitHub Actions.

## Escenario

- **Pipeline:** Construcción de imágenes base con Packer, despliegue con Terraform.
- **Requisito:** La imagen base debe cumplir al menos el 95% de controles CIS del perfil web.
- **Objetivo:** Fallar automáticamente la construcción si la tasa de cumplimiento está por debajo del umbral.
- **Reporte:** JUnit XML consumible por Jenkins para mostrar resultados en el dashboard de build.

## Paso 1: Integración con Packer

En tu template Packer (por ejemplo, `base-image.pkr.hcl`), añade un provisioner `shell` que ejecute Vallumix después de la instalación base:

```hcl
provisioner "shell" {
  inline = [
    "wget -q https://github.com/tu-org/vallumix/releases/download/v1.0.0/vallumix_1.0.0_amd64.deb",
    "dpkg -i vallumix_1.0.0_amd64.deb",
    "vallumix apply --profile web --threshold 95 --report junit --output /tmp/vallumix-results.xml",
  ]
  expect_disconnect = false
}

provisioner "file" {
  source      = "/tmp/vallumix-results.xml"
  destination = "vallumix-results.xml"
  direction   = "download"
}
```

```tip
Usa `--threshold 95` para que Vallumix devuelva código de salida `1` si la tasa de cumplimiento es inferior al 95%. Packer interpreta un código de salida distinto de cero como fallo de provisioner y aborta la construcción de la imagen.
```

## Paso 2: Configurar threshold y códigos de salida

Vallumix devuelve códigos de salida explícitos que las herramientas de CI interpretan nativamente:

| Código | Significado | Acción en CI |
|---|---|---|
| `0` | Cumplimiento ≥ umbral (o no se configuró umbral) | Build pasa |
| `1` | Cumplimiento < umbral | Build falla — política de seguridad no cumplida |
| `2` | Error de configuración (perfil inválido, distro no soportada) | Build falla — revisar configuración |
| `3` | Error de privilegios (no se ejecutó como root) | Build falla — revisar provisioner |

## Paso 3: Integración con GitHub Actions

```yaml
name: Hardening Base Image

on:
  push:
    branches: [main]

jobs:
  hardening:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Build image with Packer
        run: packer build base-image.pkr.hcl

      - name: Upload JUnit report
        uses: actions/upload-artifact@v4
        with:
          name: vallumix-report
          path: vallumix-results.xml

      - name: Publish test results
        uses: dorny/test-reporter@v1
        with:
          name: Vallumix CIS Compliance
          path: vallumix-results.xml
          reporter: java-junit
```

El paso `Publish test results` muestra cada control CIS como un caso de test en la interfaz de GitHub Actions, con controles fallidos resaltados en rojo.

## Paso 4: Integración con GitLab CI

```yaml
stages:
  - build
  - compliance

build-image:
  stage: build
  script:
    - packer build base-image.pkr.hcl
  artifacts:
    paths:
      - vallumix-results.xml
    reports:
      junit: vallumix-results.xml
    expire_in: 30 days
  allow_failure: false
```

GitLab CI consume automáticamente el JUnit XML y muestra los resultados en la pestaña "Tests" del pipeline, con gráficos de tendencia entre ejecuciones.

## Paso 5: Integración con Jenkins

En tu `Jenkinsfile`:

```groovy
stage('Hardening') {
    steps {
        sh 'packer build base-image.pkr.hcl'
    }
    post {
        always {
            junit 'vallumix-results.xml'
        }
    }
}
```

Jenkins parsea el JUnit XML y muestra controles fallidos como tests fallidos en el dashboard de build. Puedes configurar políticas de "unstable build" si hay controles con severidad media o baja fallidos, y "failed build" si hay controles críticos fallidos.

## Paso 6: Terraform post-deployment validation

Si usas Terraform para desplegar instancias, añade un `local-exec` provisioner o un `null_resource` que ejecute Vallumix tras el despliegue:

```hcl
resource "null_resource" "hardening_validation" {
  triggers = {
    instance_id = aws_instance.web.id
  }

  provisioner "remote-exec" {
    connection {
      type        = "ssh"
      host        = aws_instance.web.public_ip
      user        = "ubuntu"
      private_key = file(var.private_key_path)
    }

    inline = [
      "sudo vallumix audit --profile web --threshold 95 --report junit --output /tmp/vallumix.xml",
      "cat /tmp/vallumix.xml",
    ]
  }
}
```

```warning
No ejecutes `vallumix apply` directamente desde Terraform `remote-exec` a menos que tengas un mecanismo de rollback automatizado y un proceso de aprobación de cambios. La aplicación de hardening en producción debe ser una operación deliberada, no un side effect del despliegue de infraestructura.
```

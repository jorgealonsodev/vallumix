# Prompt completo para generar la documentación de Vallumix

Aquí tienes el prompt íntegro con todos los datos del repo `jorgealonsodev/vallumix` ya incorporados. Cópialo entero y pégalo en una nueva conversación con Claude (idealmente en Claude Code dentro del directorio del repo). Al final, sustituye el placeholder por tu PRD.

---

## PROMPT

Necesito que generes la estructura completa de documentación profesional para **Vallumix**, un proyecto open source de portafolio alojado en `https://github.com/jorgealonsodev/vallumix`. Vallumix es una herramienta CLI escrita en Rust que automatiza el endurecimiento (hardening) de sistemas Linux aplicando controles alineados con los CIS Benchmarks. Soporta perfiles especializados (web, base de datos, bastión), genera reportes de cumplimiento (HTML, JSON, JUnit XML), opera en Debian 12, Ubuntu 22.04/24.04 LTS, RHEL 9 y derivadas, y se distribuye como binario único estático compilado con musl. Es un workspace Cargo con los crates `vallumix-core`, `vallumix-controls`, `vallumix-reporters`, `vallumix-backup` y `vallumix-cli`. La licencia es dual MIT OR Apache-2.0, MSRV 1.75, edición 2021.

### Objetivo

Publicar documentación bilingüe (español e inglés) en **GitHub Pages** combinando **mdBook** (guías de usuario) con **`cargo doc`** (referencia de API), todo desplegado mediante **GitHub Actions** usando el método nativo de Pages (no la rama `gh-pages` clásica). El sitio se servirá en:

- `https://jorgealonsodev.github.io/vallumix/` — español (por defecto)
- `https://jorgealonsodev.github.io/vallumix/en/` — inglés
- `https://jorgealonsodev.github.io/vallumix/api/` — referencia de API rustdoc

No se configurará dominio personalizado. No incluyas fichero `CNAME` ni instrucciones DNS.

### Lo que debes generar

**1. Ficheros raíz del repositorio**

Crea los siguientes ficheros con contenido completo y profesional, no plantillas vacías:

- `README.md` con: título, badges apuntando a:
  - CI: `https://github.com/jorgealonsodev/vallumix/actions/workflows/ci.yml/badge.svg`
  - Docs: `https://github.com/jorgealonsodev/vallumix/actions/workflows/docs.yml/badge.svg`
  - Crates.io: `https://img.shields.io/crates/v/vallumix.svg`
  - Docs.rs: `https://docs.rs/vallumix/badge.svg`
  - License: `https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg`
  - MSRV: `https://img.shields.io/badge/MSRV-1.75-blue.svg`
  
  Continúa con: descripción ejecutiva en una pantalla, propuesta de valor en bullets, instalación rápida (cargo install, paquetes .deb/.rpm, binario directo), ejemplo mínimo de los tres comandos clave (`apply`, `audit`, `rollback`), tabla comparativa breve con OpenSCAP/Lynis/Ansible Lockdown, enlaces al sitio de docs (`https://jorgealonsodev.github.io/vallumix/`) y a la documentación de API (`https://jorgealonsodev.github.io/vallumix/api/`), sección de licencia dual y enlace a CONTRIBUTING.

- `CONTRIBUTING.md` con: cómo configurar el entorno de desarrollo, cómo añadir un nuevo control CIS (con plantilla de módulo Rust), cómo correr tests (`cargo test`, `cargo tarpaulin`, `assert_cmd`, `insta`), políticas de estilo (rustfmt, clippy pedantic, prohibición de `.unwrap()` y `.expect()` en código de producción), proceso de PR con enlace a `https://github.com/jorgealonsodev/vallumix/pulls`, formato de commits (Conventional Commits), workflow de releases con `cargo-release`, y enlace al formulario de issues en `https://github.com/jorgealonsodev/vallumix/issues/new/choose`.

- `CODE_OF_CONDUCT.md` basado en Contributor Covenant 2.1.

- `SECURITY.md` muy cuidado: explica que Vallumix es una herramienta de seguridad, política de divulgación responsable de vulnerabilidades, indica como **canal preferente** el formulario privado de GitHub Security Advisories en `https://github.com/jorgealonsodev/vallumix/security/advisories/new`, y como canal alternativo el correo `security@vallumix.example` (placeholder a sustituir por uno real). Incluye tiempos de respuesta esperados, política de soporte de versiones, y mención al firmado de releases con cosign y attestations SLSA.

- `CHANGELOG.md` siguiendo Keep a Changelog 1.1.0 con secciones [Unreleased], [0.0.1] y un placeholder para futuras versiones.

- `LICENSE-MIT` y `LICENSE-APACHE` con el texto completo estándar. Para la atribución usa `Tu Nombre <correo@example.com>` como placeholder.

**2. Estructura mdBook bilingüe en `/docs`**

Crea la estructura siguiente:

```
docs/
├── theme/
│   └── (placeholders para custom CSS si aplica)
├── es/
│   ├── book.toml
│   └── src/
│       ├── SUMMARY.md
│       ├── introduction.md
│       ├── installation/
│       │   ├── README.md
│       │   ├── debian-ubuntu.md
│       │   ├── rhel-derivatives.md
│       │   └── from-source.md
│       ├── concepts/
│       │   ├── README.md
│       │   ├── profiles.md
│       │   ├── controls.md
│       │   ├── idempotency.md
│       │   └── rollback.md
│       ├── guides/
│       │   ├── README.md
│       │   ├── hardening-web-server.md       (basada en CU-01)
│       │   ├── compliance-audit.md           (CU-02)
│       │   ├── dry-run-validation.md         (CU-03)
│       │   ├── rollback-after-incident.md    (CU-04)
│       │   └── ci-cd-integration.md          (CU-05)
│       ├── reference/
│       │   ├── README.md
│       │   ├── cli.md
│       │   ├── apply.md
│       │   ├── audit.md
│       │   ├── rollback.md
│       │   ├── list.md
│       │   └── completion.md
│       ├── controls/
│       │   ├── README.md   (catálogo con tabla de los 60+ controles, mapeo CIS/NIST 800-53/ISO 27001/PCI-DSS)
│       │   ├── filesystem.md
│       │   ├── services.md
│       │   ├── network.md
│       │   ├── logging-audit.md
│       │   ├── access-auth.md
│       │   └── system-maintenance.md
│       ├── reports/
│       │   ├── README.md
│       │   ├── html.md
│       │   ├── json.md
│       │   ├── junit.md
│       │   └── text.md
│       ├── troubleshooting.md
│       ├── faq.md
│       └── glossary.md
└── en/
    └── (idéntica estructura, traducción al inglés)
```

Para los ficheros markdown:
- Genera `SUMMARY.md` completo y válido para mdBook con la jerarquía exacta de arriba.
- Para los ficheros de contenido, genera contenido real útil (no lorem ipsum), basándote en la información del PRD que te proporciono al final. Mínimo 200 palabras por página principal; las páginas de control individuales pueden ser más cortas.
- Usa bloques de admonición de `mdbook-admonish` (`note`, `warning`, `danger`, `tip`) cuando aporten claridad, especialmente en avisos sobre operaciones destructivas, requisitos de root y consideraciones de seguridad.
- Incluye al menos un diagrama Mermaid en `concepts/README.md` mostrando el flujo de ejecución (las 10 fases descritas en sección 7.3 del PRD).
- En `controls/README.md` incluye una tabla con columnas: ID CIS, Descripción, Severidad, Perfiles aplicables, Distros, Mapeo NIST 800-53, Mapeo ISO 27001 Annex A. Incluye al menos 15 controles de ejemplo bien rellenados.
- En cada página de la sección `controls/`, sigue una plantilla consistente: descripción del dominio, lista de controles con su ID CIS, qué hace cada control en `apply`, cómo verificar manualmente, y justificación de seguridad.
- El enlace "Edit this page" se generará automáticamente vía `edit-url-template` en `book.toml`.

**3. Configuración mdBook**

Genera dos `book.toml` (uno por idioma) con esta configuración para el español (adapta para el inglés cambiando `language` y `site-url`):

```toml
[book]
title = "Vallumix"
authors = ["Tu Nombre <correo@example.com>"]
language = "es"
src = "src"

[output.html]
default-theme = "navy"
preferred-dark-theme = "navy"
git-repository-url = "https://github.com/jorgealonsodev/vallumix"
edit-url-template = "https://github.com/jorgealonsodev/vallumix/edit/main/docs/es/src/{path}"
site-url = "/vallumix/"

[output.html.search]
enable = true
limit-results = 30

[output.html.fold]
enable = true
level = 1

[preprocessor.mermaid]
command = "mdbook-mermaid"

[preprocessor.admonish]
command = "mdbook-admonish"

[output.linkcheck]
follow-web-links = false
```

Para inglés: `language = "en"`, `site-url = "/vallumix/en/"`, `edit-url-template = "https://github.com/jorgealonsodev/vallumix/edit/main/docs/en/src/{path}"`.

**4. GitHub Actions workflow `.github/workflows/docs.yml`**

Genera el workflow completo que:
- Se dispara en push a `main` y manualmente con `workflow_dispatch`.
- Tiene los `permissions` correctos (`contents: read`, `pages: write`, `id-token: write`).
- Configura `concurrency` con `group: pages` y `cancel-in-progress: false`.
- Cachea cargo con `Swatinem/rust-cache@v2`.
- Instala `mdbook`, `mdbook-mermaid`, `mdbook-admonish` y `mdbook-linkcheck`.
- Construye el book en español a la raíz del directorio de salida (`site/`).
- Construye el book en inglés a `site/en/`.
- Ejecuta `cargo doc --workspace --no-deps --all-features` y mueve el resultado a `site/api/` con un `index.html` que redirige a `vallumix_core/index.html`.
- Sube el artifact con `actions/upload-pages-artifact@v3`.
- Despliega con `actions/deploy-pages@v4` en un job separado con `environment: github-pages`.

**5. Metadatos `Cargo.toml` para docs.rs**

Genera el bloque `[package.metadata.docs.rs]` que debe ir en cada `Cargo.toml` de los crates publicables (`vallumix-core`, `vallumix-controls`, `vallumix-reporters`, `vallumix-backup`, `vallumix-cli`):

```toml
[package.metadata.docs.rs]
all-features = true
rustdoc-args = ["--cfg", "docsrs"]
```

Indica claramente que va en cada uno de esos cinco crates.

**6. Plantillas `.github/`**

- `.github/ISSUE_TEMPLATE/bug_report.yml` (formulario estructurado).
- `.github/ISSUE_TEMPLATE/feature_request.yml`.
- `.github/ISSUE_TEMPLATE/new_control.yml` (específica para proponer un nuevo control CIS, con campos para ID CIS, severidad, distros aplicables, perfiles destino, justificación de seguridad).
- `.github/ISSUE_TEMPLATE/config.yml` con `blank_issues_enabled: false` y `contact_links` apuntando al formulario de Security Advisories.
- `.github/PULL_REQUEST_TEMPLATE.md` con checklist (tests añadidos, clippy pedantic limpio, CHANGELOG actualizado, docs actualizadas en español e inglés, control mapeado en `controls/README.md` si aplica, sin `.unwrap()` ni `.expect()` en código de producción).

**7. Pasos de activación final**

Al final de tu respuesta, incluye esta lista numerada exacta de pasos manuales:

1. Clonar el repo y crear una rama de trabajo:
   ```bash
   git clone https://github.com/jorgealonsodev/vallumix.git
   cd vallumix
   git checkout -b docs/initial-setup
   ```
2. Aplicar todos los ficheros generados respetando las rutas indicadas.
3. Commit y push:
   ```bash
   git add .
   git commit -m "docs: initial documentation structure with mdBook + cargo doc"
   git push origin docs/initial-setup
   ```
4. Abrir PR en `https://github.com/jorgealonsodev/vallumix/compare/docs/initial-setup` y mergear a `main`.
5. Activar GitHub Pages: ir a `https://github.com/jorgealonsodev/vallumix/settings/pages`, en *Source* seleccionar **GitHub Actions** (no "Deploy from a branch").
6. Verificar que el workflow `docs.yml` se ejecuta correctamente en `https://github.com/jorgealonsodev/vallumix/actions`.
7. Una vez completado el deploy, el sitio estará disponible en `https://jorgealonsodev.github.io/vallumix/`.
8. Añadir la URL del sitio en la sección "About" del repo (rueda dentada arriba a la derecha en la página principal) y marcar la casilla "Use your GitHub Pages website".
9. Habilitar el formulario privado de Security Advisories: `Settings → Code security and analysis → Private vulnerability reporting → Enable`.
10. (Opcional pero recomendado) Activar Dependabot para `cargo` y `github-actions` desde `Settings → Code security and analysis`.

### Restricciones y estilo

- Usa **placeholders claros** donde proceda: `Tu Nombre <correo@example.com>` para autoría, `security@vallumix.example` para correo de seguridad alternativo.
- Toda la documentación debe ser **coherente con el PRD** que pego abajo: mismos nombres de crates, mismos comandos, mismos perfiles, mismos casos de uso, mismas fases.
- Estilo profesional, técnico, sin marketing vacío. Tono directo y útil.
- Markdown válido para mdBook (no GFM-only). Bloques de código con lenguaje declarado.
- Para los ficheros largos, entrega su contenido íntegro, no resúmenes ni "...continúa igual".
- Si algún fichero supera lo razonable para una sola respuesta, entrégalo completo igualmente; prioriza completitud sobre brevedad. Si hace falta, divide la entrega en varias respuestas pero no abrevies contenido.

### Orden de entrega sugerido

1. Estructura de directorios completa como árbol.
2. Ficheros raíz (README, CONTRIBUTING, CODE_OF_CONDUCT, SECURITY, CHANGELOG, LICENSE-MIT, LICENSE-APACHE).
3. `book.toml` (ambos idiomas).
4. `SUMMARY.md` (ambos idiomas).
5. Contenido de las páginas mdBook por secciones, primero español y luego inglés.
6. Workflow `.github/workflows/docs.yml`.
7. Plantillas `.github/ISSUE_TEMPLATE/` y `.github/PULL_REQUEST_TEMPLATE.md`.
8. Bloque `Cargo.toml` para docs.rs.
9. Pasos de activación final.

### Contexto del proyecto (PRD completo)

# Vallumix — Product Requirements Document

**Sistema Modular de Hardening para Linux con Perfiles CIS, escrito en Rust**

| | |
|---|---|
| **Versión del documento** | 1.1 |
| **Estado** | Borrador inicial |
| **Tipo de proyecto** | Portafolio · Open Source |
| **Categoría** | Ciberseguridad · Hardening · DevSecOps · Systems Programming |
| **Lenguaje principal** | Rust (edición 2021, MSRV 1.75) |
| **Licencia propuesta** | MIT OR Apache-2.0 (dual, estándar en el ecosistema Rust) |
| **Última actualización** | 30 de abril de 2026 |

---

## 1. Resumen Ejecutivo

Vallumix es una herramienta CLI escrita en Rust que automatiza el endurecimiento de sistemas Linux aplicando controles alineados con los CIS Benchmarks. Permite seleccionar perfiles especializados según el rol del servidor —web, base de datos o bastión— y genera reportes detallados de cumplimiento que documentan el estado de cada control evaluado, los cambios aplicados y las desviaciones detectadas.

El proyecto resuelve un problema recurrente en producción: el hardening manual es tedioso, propenso a errores y rara vez se documenta de forma trazable. Vallumix ofrece una solución idempotente, auditable, reversible y distribuida como un único binario estático sin dependencias de runtime, ideal para integrarse en pipelines de CI/CD y procesos de aprovisionamiento.

Como proyecto de portafolio, Vallumix demuestra competencias en seguridad de sistemas, programación de sistemas en Rust, diseño de software modular con arquitectura basada en traits, observancia de estándares industriales (CIS, NIST 800-53) y desarrollo de herramientas DevSecOps modernas. La elección de Rust es deliberada: garantiza seguridad de memoria, manejo robusto de errores y un binario único multiplataforma, características que diferencian al proyecto de la mayoría de soluciones existentes basadas en Bash o Ansible.

### 1.1 Propuesta de valor

- Binario único estático compilado con `musl`, sin dependencias de runtime ni intérpretes.
- Reducción del tiempo de hardening de horas a minutos por servidor.
- Reportes de cumplimiento listos para auditorías de seguridad (HTML, JSON, JUnit XML).
- Modo dry-run que muestra los cambios sin aplicarlos, ideal para validación previa.
- Mecanismo de rollback que restaura configuraciones originales mediante respaldos versionados.
- Perfiles preconfigurados que respetan las particularidades operativas de cada tipo de servidor.
- Garantías de Rust: ausencia de errores de memoria, manejo explícito de errores, concurrencia segura cuando sea necesaria.

### 1.2 Por qué Rust

La elección del lenguaje es una decisión de producto, no solo técnica:

- **Seguridad de memoria garantizada en compilación:** una herramienta de hardening que se ejecuta como root debe ser intrínsecamente segura.
- **Distribución simplificada:** un binario estático elimina problemas de versiones de Python o dependencias de Bash entre distribuciones.
- **Manejo de errores explícito:** el sistema de tipos `Result<T, E>` obliga a tratar todos los modos de fallo, crítico cuando se modifican archivos del sistema.
- **Rendimiento:** ejecución paralela segura de checks independientes mediante `rayon` reduce el tiempo de auditoría en sistemas con muchos controles.
- **Diferenciación en el ecosistema:** la mayoría de herramientas de hardening están en Bash, Python o Ansible; una solución en Rust se posiciona junto a herramientas modernas como `ripgrep`, `bat` o `trivy`.

### 1.3 Alcance del proyecto

La versión 1.0 cubre las distribuciones Debian 12, Ubuntu 22.04/24.04 LTS, RHEL 9 y derivadas (Rocky, AlmaLinux). Implementa entre 60 y 80 controles seleccionados de CIS Benchmark Level 1 y un subconjunto de Level 2. Las distribuciones embebidas, contenedores y entornos no convencionales quedan fuera del alcance inicial.

---

## 2. Contexto y Justificación

### 2.1 Problema

El endurecimiento de servidores Linux en producción enfrenta cuatro obstáculos persistentes:

1. **Aplicación inconsistente:** distintos administradores aplican criterios diferentes, generando flotas heterogéneas en términos de postura de seguridad.
2. **Falta de trazabilidad:** los cambios manuales raramente quedan documentados, dificultando auditorías y respuesta a incidentes.
3. **Riesgo operativo:** hardening agresivo puede romper servicios. Sin perfiles diferenciados por rol, la única alternativa es ser conservador y dejar superficie de ataque expuesta.
4. **Curva de aprendizaje del CIS Benchmark:** el documento oficial supera las 1000 páginas y requiere interpretación experta para cada control.

### 2.2 Soluciones existentes y diferenciadores

Existen herramientas como OpenSCAP, Lynis, CIS-CAT, y roles de Ansible (DevSec, ansible-lockdown). La elección de Rust posiciona a Vallumix en un segmento prácticamente inexplorado:

| Característica | Vallumix | OpenSCAP | Lynis | Ansible Lockdown |
|---|---|---|---|---|
| **Lenguaje** | Rust | C + XML | Bash | YAML + Python |
| **Distribución** | Binario único estático | Paquetes + SCAP content | Script Bash | Roles + controlador |
| **Aplica cambios** | Sí | Limitado | No (solo audita) | Sí |
| **Perfiles por rol** | Sí, integrados | Manual | No | Parcial |
| **Sin dependencias runtime** | Sí | No | Bash | Requiere Ansible |
| **Rollback automático** | Sí | No | N/A | Manual |
| **Reporte HTML** | Sí | Sí | Texto | No |
| **Seguridad de memoria** | Garantizada | C (manual) | N/A | N/A |
| **Ejecución paralela** | Sí (rayon) | No | No | Limitada |

> Vallumix se posiciona como una herramienta moderna, autónoma y verificable. Está pensada para administradores y equipos DevSecOps que valoran herramientas confiables, auditables y fáciles de desplegar sin infraestructura adicional.

---

## 3. Objetivos del Proyecto

### 3.1 Objetivos primarios

- Implementar al menos 60 controles del CIS Benchmark Level 1 distribuidos entre los tres perfiles.
- Garantizar idempotencia: ejecutar la herramienta múltiples veces produce el mismo estado final sin efectos secundarios.
- Generar reportes de cumplimiento en formato HTML, JSON y JUnit XML con identificación clara de controles aprobados, fallidos y omitidos.
- Soportar modo dry-run para previsualizar cambios antes de aplicarlos.
- Implementar respaldos automáticos previos a cada modificación, con función de rollback granular.
- Distribuir como binario único estático compilado con `musl`, ejecutable sin dependencias en cualquier distribución soportada.

### 3.2 Objetivos secundarios

- Lograr un tiempo de ejecución completo inferior a 90 segundos en un servidor de referencia (4 vCPU, 8 GB RAM), aprovechando paralelismo cuando sea seguro.
- Mantener una cobertura de tests superior al 80% medida con `cargo-tarpaulin`.
- Documentar el proyecto en español e inglés, incluyendo guías de uso, contribución, mapeo CIS-control y documentación de API generada con `cargo doc`.
- Publicar en `crates.io` y mantener paquetes `.deb` y `.rpm` generados con `cargo-deb` y `cargo-generate-rpm`.
- Ofrecer imágenes Docker en GitHub Container Registry para validación rápida.

### 3.3 Métricas de éxito (KPIs)

| Métrica | Objetivo v1.0 | Método de medición |
|---|---|---|
| Controles CIS implementados | ≥ 60 | Conteo en code review |
| Tasa de cumplimiento post-ejecución | ≥ 90% | Reporte generado por la herramienta |
| Tiempo de ejecución (perfil web) | < 90 s | Benchmark con `hyperfine` |
| Tamaño del binario (musl, stripped) | < 8 MB | `ls -lh target/release/` |
| Cobertura de tests | ≥ 80% | `cargo-tarpaulin` |
| Falsos positivos en detección | < 5% | Validación manual sobre VMs |
| Warnings de `clippy` (pedantic) | 0 | CI con `cargo clippy -- -D warnings` |
| Stars en GitHub a 6 meses | ≥ 75 | Métrica pública del repositorio |
| Descargas en `crates.io` a 6 meses | ≥ 500 | Estadísticas de crates.io |

---

## 4. Usuarios y Casos de Uso

### 4.1 Personas objetivo

#### Persona 1: Administrador de sistemas en PYME

María, 32 años, gestiona una flota de 15 servidores Linux para una empresa mediana. No tiene equipo dedicado de seguridad ni presupuesto para herramientas comerciales. Necesita aplicar buenas prácticas de hardening rápidamente y poder demostrar cumplimiento ante una auditoría ISO 27001 que se aproxima. Valora simplicidad, documentación clara y reportes presentables. Aprecia que sea un binario único que no requiera instalar dependencias.

#### Persona 2: Ingeniero DevOps integrando seguridad en CI/CD

Carlos, 28 años, trabaja en una empresa SaaS. Quiere integrar hardening automatizado en su pipeline de aprovisionamiento con Terraform y Packer. Necesita una herramienta que se ejecute sin interactividad, devuelva códigos de salida coherentes y produzca artefactos parseables (JSON, JUnit XML) para alimentar dashboards en Grafana y reportes de pipeline.

#### Persona 3: Consultor de seguridad freelance

Ana, 41 años, audita clientes pequeños y medianos. Necesita una herramienta portable de un solo binario que pueda copiar a sistemas de clientes sin instalar nada, generar reportes de evaluación inicial y aplicar remediación cuando el cliente lo autoriza. Valora la trazabilidad y el rollback.

### 4.2 Casos de uso principales

#### CU-01: Hardening inicial de un servidor web nuevo

Un administrador aprovisiona un servidor Ubuntu 24.04 destinado a alojar una aplicación web Nginx. Después de la instalación base, ejecuta Vallumix con el perfil web para aplicar controles relevantes (deshabilitar servicios innecesarios, configurar firewall, endurecer SSH, asegurar permisos de `/var/www`, configurar fail2ban) sin romper la funcionalidad del servidor web.

```bash
sudo vallumix apply --profile web --report html
```

#### CU-02: Auditoría de cumplimiento sin modificar el sistema

Un consultor recibe acceso temporal a un servidor de un cliente para evaluar su postura de seguridad. Ejecuta Vallumix en modo audit para generar un reporte que identifique brechas sin alterar nada.

```bash
sudo vallumix audit --profile database --report html,json
```

#### CU-03: Validación previa con dry-run

Antes de aplicar hardening en un servidor de producción, el operador ejecuta la herramienta en modo dry-run para revisar exactamente qué cambios se realizarían. El reporte detalla cada modificación planeada con justificación CIS.

```bash
sudo vallumix apply --profile bastion --dry-run --verbose
```

#### CU-04: Rollback tras incidente operativo

Después de aplicar hardening, un servicio crítico deja de funcionar. El operador identifica el control responsable a partir del reporte y revierte únicamente esa configuración usando el ID del control.

```bash
sudo vallumix rollback --control-id 5.2.4
```

#### CU-05: Integración en pipeline CI/CD

Un equipo DevOps incluye Vallumix en su flujo de aprovisionamiento Packer. La construcción de la imagen base falla automáticamente si la tasa de cumplimiento es inferior al umbral configurado.

```bash
vallumix apply --profile web --threshold 95 --report junit --output /tmp/results.xml
```

---

## 5. Requisitos Funcionales

### 5.1 Arquitectura modular

El sistema se organiza como un workspace de Cargo con varios crates que pueden evolucionar de forma independiente:

- **`vallumix-core`:** lógica común, definición de tipos, traits centrales (`Control`, `Reporter`, `Profile`), gestión de errores con `thiserror`.
- **`vallumix-controls`:** implementaciones concretas de cada control CIS, una por módulo. Cada control implementa el trait `Control`.
- **`vallumix-reporters`:** generadores de reportes (HTML, JSON, JUnit XML, texto), todos implementan el trait `Reporter`.
- **`vallumix-cli`:** binario principal que orquesta la ejecución, parsea argumentos con `clap` y compone los demás crates.
- **`vallumix-backup`:** gestión de respaldos versionados y rollback.

Esta separación permite que terceros consuman `vallumix-core` y `vallumix-controls` como librerías para construir herramientas derivadas sin depender del binario CLI completo.

### 5.2 Diseño basado en traits

El núcleo del sistema se modela con un trait `Control` que abstrae el ciclo de vida de cada verificación:

```rust
pub trait Control: Send + Sync {
    fn id(&self) -> &str;                          // p. ej. "5.2.4"
    fn description(&self) -> &str;
    fn severity(&self) -> Severity;
    fn applicable_distros(&self) -> &[Distro];

    fn check(&self, ctx: &Context) -> Result<CheckResult, ControlError>;
    fn apply(&self, ctx: &Context) -> Result<ApplyResult, ControlError>;
    fn rollback(&self, ctx: &Context, backup: &Backup) -> Result<(), ControlError>;
}
```

Esta abstracción permite que cada control sea testeable de forma aislada con mocks del contexto, y que el motor principal itere sobre `Box<dyn Control>` sin acoplarse a implementaciones específicas.

### 5.3 Perfiles preconfigurados

#### Perfil servidor web

Optimizado para hosts que sirven aplicaciones HTTP/HTTPS (Nginx, Apache). Aplica controles de filtrado de red para puertos 80/443, configura límites de recursos para procesos web, endurece TLS, asegura permisos en directorios de despliegue y aplica políticas de logging para análisis forense.

#### Perfil base de datos

Para servidores PostgreSQL, MariaDB o MongoDB. Restringe el acceso de red a redes internas, aplica controles más estrictos sobre montajes de filesystem (especialmente `/tmp` y `/var`), endurece configuración del kernel para cargas de trabajo de E/S intensivas y desactiva binarios SUID/SGID no esenciales.

#### Perfil bastión

Para hosts saltadores (jump hosts) cuyo único propósito es alojar conexiones SSH entrantes. Aplica el conjunto más agresivo de controles: solo SSH expuesto, autenticación por clave obligatoria, autenticación multifactor (MFA) opcional con Google Authenticator, registro exhaustivo de sesiones con auditd y restricciones extensas de comandos.

### 5.4 Subcomandos de operación

Vallumix sigue el patrón de subcomandos popular en el ecosistema Rust (estilo `cargo`, `git`):

| Subcomando | Comportamiento | Modifica sistema |
|---|---|---|
| `vallumix apply` | Aplica todos los controles del perfil seleccionado, generando respaldos previos y reporte final. | Sí |
| `vallumix audit` | Evalúa el estado actual y genera reporte de cumplimiento sin proponer ni aplicar cambios. | No |
| `vallumix rollback` | Restaura una ejecución previa completa o un control específico desde los respaldos versionados. | Sí (revierte) |
| `vallumix list` | Imprime el catálogo de controles disponibles con su mapeo CIS y perfiles asociados. | No |
| `vallumix completion` | Genera scripts de autocompletado para `bash`, `zsh`, `fish` y `nushell`. | No |

Flags globales relevantes: `--dry-run`, `--verbose`, `--quiet`, `--no-color`, `--threshold N`, `--profile <web|database|bastion>`.

### 5.5 Sistema de reportes

Cada ejecución produce un reporte estructurado mediante el trait `Reporter`. La estructura serializable común se define con `serde` e incluye:

- **Identificación del host:** hostname, distribución, kernel, fecha y duración de la ejecución.
- **Resumen ejecutivo:** tasa de cumplimiento, controles aprobados/fallidos/omitidos, comparación con ejecución previa si existe.
- **Detalle por control:** ID CIS, descripción, severidad, estado, evidencia (comando ejecutado y salida), justificación de fallo si aplica.
- **Mapeo a estándares cruzados:** NIST 800-53, ISO 27001 Annex A, PCI-DSS donde corresponda.
- **Recomendaciones de remediación manual** para controles que no pueden automatizarse.

Formatos de salida implementados:

- **HTML** (con `askama` para templating compilado): autocontenido con CSS embebido, sin dependencias externas.
- **JSON** (con `serde_json`): validable contra un JSON Schema versionado y publicado.
- **JUnit XML** (con `quick-xml`): consumible directamente por Jenkins, GitLab CI y GitHub Actions.
- **Texto plano**: para terminales, con coloreado opcional vía `owo-colors`.

### 5.6 Categorías de controles cubiertos

La selección de controles se distribuye en seis dominios funcionales del CIS Benchmark, con énfasis y umbrales que varían por perfil:

1. **Configuración inicial:** deshabilitación de filesystems no usados, configuración de actualizaciones automáticas, integridad del sistema de paquetes.
2. **Servicios:** identificación y desactivación de servicios innecesarios (avahi, cups, telnet, rsh, etc.).
3. **Configuración de red:** parámetros del kernel para TCP/IP, IPv6, ICMP, redirecciones y filtrado por host (TCP wrappers, nftables/firewalld).
4. **Logging y auditoría:** rsyslog, journald y auditd con reglas mínimas según CIS.
5. **Acceso, autenticación y autorización:** PAM, política de contraseñas, SSH, sudo, restricciones de cron.
6. **Mantenimiento del sistema:** permisos de archivos críticos, integridad de `/etc/passwd` y `/etc/shadow`, configuración de umask global.

---

## 6. Requisitos No Funcionales

### 6.1 Compatibilidad

La herramienta debe operar correctamente en las siguientes plataformas, detectando la distribución automáticamente para invocar la lógica adecuada:

- Debian 12 (Bookworm).
- Ubuntu 22.04 LTS (Jammy) y 24.04 LTS (Noble).
- Red Hat Enterprise Linux 9, Rocky Linux 9, AlmaLinux 9.

Arquitecturas soportadas mediante compilación cruzada con `cross`:

- `x86_64-unknown-linux-musl` (principal).
- `aarch64-unknown-linux-musl` (ARM64, para servidores AWS Graviton, Ampere, Raspberry Pi 4/5).

> La detección se realiza leyendo `/etc/os-release` mediante un módulo dedicado en `vallumix-core::distro`. Si la distribución no es soportada, la herramienta aborta con un error tipado y un mensaje claro.

### 6.2 Seguridad de la herramienta

- **Garantías del lenguaje:** ausencia de errores de memoria, sin punteros colgantes ni race conditions en código `safe`.
- **Manejo explícito de errores:** uso sistemático de `Result<T, E>` con tipos de error específicos definidos con `thiserror`. Prohibición de `.unwrap()` y `.expect()` en código de producción (verificado con `clippy`).
- **Validación estricta de entradas:** uso de `clap` con validadores tipados; los paths se validan con `std::path::Path::canonicalize` antes de ser usados.
- **Verificación de privilegios** al inicio mediante `nix::unistd::Uid::effective()`; aborta si no se ejecuta como root para operaciones que lo requieren.
- **Auditoría de dependencias** automatizada con `cargo-audit` y `cargo-deny` en CI.
- **SBOM** (Software Bill of Materials) generado en cada release con `cargo-sbom`.
- **Firmado de releases** con `cosign` y publicación de attestations SLSA Level 3.
- **Análisis estático** con `clippy` en modo `pedantic` y `cargo-geiger` para detectar uso de `unsafe`.

### 6.3 Rendimiento

- Ejecución completa en menos de 90 segundos para el perfil de mayor cobertura sobre hardware de referencia.
- Paralelización de checks independientes mediante `rayon` (controles que solo leen estado pueden ejecutarse concurrentemente).
- Consumo de memoria pico inferior a 50 MB durante toda la ejecución.
- Tamaño del binario stripped y comprimido con UPX inferior a 8 MB.
- Tiempo de arranque (cold start) inferior a 50 ms.

### 6.4 Mantenibilidad y observabilidad

- Logging estructurado con `tracing` y `tracing-subscriber`, con soporte para JSON Lines en CI.
- Niveles configurables (`TRACE`, `DEBUG`, `INFO`, `WARN`, `ERROR`) vía variable de entorno `RUST_LOG` y flag `--log-level`.
- Cada control en su propio módulo, no excede 200 líneas de código efectivo.
- Documentación inline obligatoria en items públicos, verificada con `cargo doc --no-deps -- -D warnings`.
- `CHANGELOG.md` siguiendo Keep a Changelog y versionado semántico.
- Gestión de versiones automatizada con `release-please` o `cargo-release`.

### 6.5 Usabilidad

- Mensajes de salida con `owo-colors` y prefijos icónicos (✓ ✗ ⚠ ℹ) en terminales compatibles.
- Detección automática de soporte de color y respeto a la variable `NO_COLOR` (estándar de facto).
- Barra de progreso con `indicatif` durante ejecuciones largas.
- Página man generada automáticamente con `clap_mangen` durante el build.
- Autocompletado para Bash, Zsh, Fish y Nushell generado por el subcomando `vallumix completion`.
- Mensajes de error con sugerencias de remediación cuando sea aplicable, usando `miette` para formateo enriquecido.

---

## 7. Arquitectura Técnica

### 7.1 Stack tecnológico

| Componente | Tecnología | Crate |
|---|---|---|
| **Lenguaje** | Rust 2021, MSRV 1.75 | — |
| **Parsing CLI** | Argumentos y subcomandos | `clap` v4 (con feature `derive`) |
| **Serialización** | JSON, YAML, TOML | `serde`, `serde_json`, `serde_yaml`, `toml` |
| **Manejo de errores** | Tipos de error de aplicación | `thiserror` |
| **Errores de runtime** | Captura y contexto | `anyhow` (solo en `main`) |
| **Errores enriquecidos** | Para CLI con sugerencias | `miette` |
| **Logging** | Tracing estructurado | `tracing`, `tracing-subscriber` |
| **Templating HTML** | Compilado en build time | `askama` |
| **XML (JUnit)** | Generación XML | `quick-xml` |
| **Llamadas al sistema** | wrappers de libc | `nix` |
| **Operaciones filesystem** | Operaciones avanzadas | `walkdir`, `tempfile` |
| **Paralelismo** | Iteradores paralelos | `rayon` |
| **Coloreado terminal** | Colores condicionales | `owo-colors` |
| **Barra de progreso** | UI en terminal | `indicatif` |
| **Detección de distro** | Parser de `/etc/os-release` | `os-release` o implementación propia |
| **Pruebas** | Framework integrado + extras | `cargo test`, `insta` (snapshots), `assert_cmd`, `predicates` |
| **Cobertura** | Análisis de cobertura | `cargo-tarpaulin` |
| **Análisis estático** | Linter idiomático | `clippy` |
| **Auditoría** | CVEs en dependencias | `cargo-audit`, `cargo-deny` |
| **Empaquetado .deb** | Generación Debian | `cargo-deb` |
| **Empaquetado .rpm** | Generación RPM | `cargo-generate-rpm` |
| **CI/CD** | Pipelines | GitHub Actions con matriz multi-distro y multi-arch |
| **Documentación** | Sitio estático | `cargo doc` + `mdBook` para guías |
| **Compilación cruzada** | Builds multi-target | `cross` |

### 7.2 Estructura del workspace

```
vallumix/
├── Cargo.toml                    # Workspace root
├── Cargo.lock
├── rust-toolchain.toml           # Pin de toolchain
├── deny.toml                     # Configuración cargo-deny
├── crates/
│   ├── vallumix-core/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── control.rs        # Trait Control y tipos asociados
│   │       ├── context.rs        # Context de ejecución
│   │       ├── distro.rs         # Detección de distribución
│   │       ├── error.rs          # Tipos de error con thiserror
│   │       └── profile.rs        # Modelo de perfiles
│   ├── vallumix-controls/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── filesystem/
│   │       │   ├── mod.rs
│   │       │   └── disable_cramfs.rs   # Control CIS 1.1.1.1
│   │       ├── ssh/
│   │       │   ├── mod.rs
│   │       │   └── disable_root_login.rs   # Control CIS 5.2.4
│   │       └── ...               # Un módulo por dominio CIS
│   ├── vallumix-reporters/
│   │   ├── Cargo.toml
│   │   ├── templates/            # Templates askama
│   │   │   └── report.html
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── html.rs
│   │       ├── json.rs
│   │       ├── junit.rs
│   │       └── text.rs
│   ├── vallumix-backup/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── lib.rs
│   └── vallumix-cli/
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs
│           ├── commands/
│           │   ├── apply.rs
│           │   ├── audit.rs
│           │   ├── rollback.rs
│           │   └── list.rs
│           └── ui.rs             # Output formateado
├── profiles/
│   ├── web.toml
│   ├── database.toml
│   └── bastion.toml
├── tests/
│   ├── integration/              # Tests de integración con assert_cmd
│   └── snapshots/                # Snapshots de insta
├── docs/
│   ├── book.toml                 # Configuración mdBook
│   ├── es/
│   └── en/
├── docker/
│   ├── Dockerfile.debian12
│   ├── Dockerfile.ubuntu2404
│   └── Dockerfile.rocky9
├── .github/workflows/            # CI/CD
├── CHANGELOG.md
├── CONTRIBUTING.md
├── LICENSE-MIT
├── LICENSE-APACHE
└── README.md
```

### 7.3 Flujo de ejecución

1. **Parsing de argumentos** con `clap`: validación tipada, generación automática de `--help`.
2. **Inicialización de tracing** según nivel de log y formato (texto coloreado o JSON estructurado).
3. **Validación inicial:** privilegios efectivos (`nix::unistd::geteuid`), distribución soportada (`vallumix-core::distro`), espacio en disco para respaldos.
4. **Carga del perfil** desde TOML con `serde` y resolución de la lista de controles a ejecutar.
5. **Construcción del contexto** (`Context`): información del host, paths de trabajo, configuración global.
6. **Creación de directorio de respaldo** versionado en `/var/backups/vallumix/<timestamp>/`.
7. **Iteración por cada control** (paralelizable en modo audit con `rayon`):
   - `pre_check`: ¿el control ya cumple? Si sí, marcar como `Compliant`.
   - `backup`: respaldar archivos antes de modificar.
   - `apply`: aplicar el cambio (en modo `apply`).
   - `post_check`: verificar que el cambio surtió efecto.
   - Registrar resultado tipado en la estructura de reporte.
8. **Generación de reportes** en los formatos solicitados, mediante el trait `Reporter`.
9. **Resumen en consola** con tasa de cumplimiento y enlace al reporte HTML.
10. **Código de salida:** `0` si la tasa de cumplimiento ≥ umbral, `1` si por debajo del umbral, `2` si error de configuración, `3` si error de privilegios.

### 7.4 Manejo de errores

El proyecto adopta el patrón estándar de Rust para CLIs:

- **`thiserror`** en bibliotecas (`vallumix-core`, `vallumix-controls`, etc.) para definir tipos de error específicos del dominio:

```rust
#[derive(thiserror::Error, Debug)]
pub enum ControlError {
    #[error("control {0} not applicable to distribution {1}")]
    NotApplicable(String, Distro),
    #[error("backup failed for {path}: {source}")]
    BackupFailed { path: PathBuf, #[source] source: std::io::Error },
    #[error("post-check failed: expected {expected}, got {actual}")]
    PostCheckFailed { expected: String, actual: String },
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
```

- **`anyhow`** únicamente en `main.rs` para capturar cualquier error y presentarlo al usuario.
- **`miette`** para errores cara al usuario con contexto enriquecido y sugerencias.

---

## 8. Plan de Desarrollo

El proyecto se estructura en seis fases iterativas. La duración estimada total es de 14 a 16 semanas a tiempo parcial; Rust requiere más tiempo inicial por la curva de aprendizaje y la disciplina del sistema de tipos, pero esto se compensa con menor tiempo de debugging y refactor en fases tardías.

| Fase | Duración | Entregables | Hito |
|---|---|---|---|
| **0. Bootstrap** | 1 semana | Workspace inicializado, CI con `cargo build`/`test`/`clippy`/`fmt`, plantillas de issues y PRs, README inicial | v0.0.1 |
| **1. Cimientos** | 3 semanas | Trait `Control` definido, `vallumix-core` completo, CLI con `clap`, 5 controles piloto, perfil web mínimo, formato de reporte JSON | v0.1 alpha |
| **2. Cobertura** | 4 semanas | 60+ controles implementados, los tres perfiles completos, reporters HTML/JSON/JUnit/texto, sistema de backup/rollback completo | v0.5 beta |
| **3. Calidad** | 2 semanas | Tests con cobertura ≥80%, snapshots con `insta`, integración con `assert_cmd`, fixtures Vagrant para test manual, CI multi-arch (x86_64 + aarch64) | v0.8 |
| **4. Pulido** | 2 semanas | Documentación bilingüe con `mdBook`, demos en video, integración Packer ejemplo, página man, autocompletado, paquetes `.deb` y `.rpm` | v0.9 rc |
| **5. Lanzamiento** | 2 semanas | Release firmado con `cosign`, publicación en `crates.io`, post en blog técnico, anuncio en `r/rust`, `r/linux` y Hacker News, attestations SLSA | v1.0 GA |

---

## 9. Riesgos y Mitigaciones

| Riesgo | Probabilidad | Impacto | Mitigación |
|---|---|---|---|
| Curva de aprendizaje de Rust ralentiza el avance inicial | Alta | Medio | Aceptar tempo conservador en fase 1, apoyarse en libros de referencia y patrones idiomáticos del ecosistema |
| Hardening rompe servicios críticos en producción | Media | Alto | Modo dry-run obligatorio en docs, perfiles diferenciados, rollback granular, advertencias en README |
| Divergencias entre versiones del CIS Benchmark | Alta | Medio | Anclar versión específica del benchmark en cada release, registrar en cada control el ID y versión |
| Mantenimiento del proyecto a largo plazo | Media | Alto | Documentación de contribución clara, issues templates, automatización máxima en CI, dual licensing para fomentar contribuciones |
| Incompatibilidades entre distros similares | Alta | Medio | Matriz de testing en CI cubriendo todas las distros soportadas en cada PR, uso de feature detection en lugar de version detection |
| Falsa sensación de seguridad post-hardening | Media | Medio | Disclaimer prominente, recomendar auditorías periódicas y herramientas complementarias |
| Vulnerabilidades en dependencias transitivas | Media | Alto | `cargo-audit` en CI, `cargo-deny` con políticas estrictas, revisiones periódicas de `Cargo.lock` |
| Sobrecarga de complejidad en arquitectura inicial | Media | Medio | YAGNI: arrancar con módulos simples, refactorizar a traits cuando aparezcan duplicaciones reales |

---

## 10. Fuera de Alcance (v1.0)

Las siguientes funcionalidades han sido evaluadas y excluidas deliberadamente de la versión inicial para acotar el esfuerzo y mantener un alcance entregable. Pueden considerarse para versiones posteriores.

- Soporte para distribuciones legacy (CentOS 7, Ubuntu 18.04, Debian 10).
- Soporte para FreeBSD, macOS o Windows (Rust permite la abstracción, pero los controles CIS son específicos de Linux).
- Hardening de contenedores Docker o entornos Kubernetes; estos requieren un enfoque distinto cubierto por CIS Docker Benchmark.
- Interfaz gráfica o web; Vallumix es y se mantendrá como herramienta de línea de comandos. Una TUI con `ratatui` se evalúa para v2.
- Integración nativa con SIEMs comerciales (Splunk, QRadar). El reporte JSON es el contrato de integración.
- Aplicación remota orquestada; Vallumix se ejecuta localmente en cada host. Para flotas se recomienda combinar con Ansible o `pssh`.
- Cumplimiento de estándares regulados específicos como HIPAA o FedRAMP, que requieren controles adicionales fuera del CIS Benchmark base.

---

## 11. Roadmap Futuro

Una vez liberada la v1.0, las siguientes evoluciones se consideran candidatas naturales para versiones futuras:

### v1.x — Refinamiento

- Perfil adicional para servidores de correo (Postfix, Dovecot).
- Perfil para servidores de aplicaciones Java/Tomcat.
- Soporte experimental para Alpine Linux (musl nativo) y openSUSE.
- Integración con HashiCorp Vault para gestión de secretos durante hardening.
- TUI interactiva con `ratatui` para exploración de controles y revisión de reportes.

### v2.0 — Expansión

- Modo cliente-servidor: agente ligero (también en Rust, compartiendo el core) que reporta estado periódicamente a un servidor central.
- Diff temporal: comparación de cumplimiento entre dos puntos en el tiempo.
- Plugin para Ansible que use Vallumix como motor de ejecución mediante FFI.
- Cobertura de CIS Benchmark Level 2 completo con perfiles de alto cumplimiento.
- Bindings a Python con `pyo3` para uso desde scripts existentes.

---

## 12. Anexos

### 12.1 Glosario

- **Hardening:** proceso de reducción de la superficie de ataque de un sistema mediante la desactivación de funcionalidades innecesarias y configuración segura de las restantes.
- **CIS Benchmark:** guías de configuración segura mantenidas por el Center for Internet Security, ampliamente reconocidas como referencia industrial.
- **Idempotencia:** propiedad por la cual ejecutar una operación una o múltiples veces produce el mismo resultado.
- **Bastión (jump host):** servidor expuesto a internet cuyo único propósito es servir como punto de entrada autenticado hacia infraestructura interna.
- **Dry-run:** modo de ejecución que simula los efectos de una operación sin realizarlos efectivamente.
- **MSRV (Minimum Supported Rust Version):** versión mínima del compilador de Rust que el proyecto se compromete a soportar.
- **Crate:** unidad de compilación y distribución en Rust, equivalente conceptual de un paquete.
- **Workspace:** agrupación de múltiples crates relacionados gestionados conjuntamente por Cargo.
- **Trait:** mecanismo de Rust para definir comportamiento compartido, similar a interfaces en otros lenguajes.
- **SLSA (Supply-chain Levels for Software Artifacts):** marco de Google para asegurar la cadena de suministro de software.

### 12.2 Referencias

- CIS Benchmarks — Center for Internet Security (<https://www.cisecurity.org/cis-benchmarks>).
- NIST Special Publication 800-53 Rev. 5 — Security and Privacy Controls.
- ISO/IEC 27001:2022 — Information security management systems.
- The Rust Programming Language — Steve Klabnik, Carol Nichols (<https://doc.rust-lang.org/book/>).
- Rust API Guidelines (<https://rust-lang.github.io/api-guidelines/>).
- Command Line Applications in Rust (<https://rust-cli.github.io/book/>).
- DevSec Hardening Framework (<https://dev-sec.io>).

### 12.3 Criterios de aceptación de la v1.0

La versión 1.0 se considerará lista para release público cuando se cumplan simultáneamente las siguientes condiciones:

1. Todos los KPIs de la sección 3.3 alcanzados o superados.
2. CI verde sobre las cuatro distribuciones soportadas y ambas arquitecturas (x86_64, aarch64) durante 7 días consecutivos.
3. Cero warnings de `clippy --pedantic`, cero items sin documentar (`#[deny(missing_docs)]`).
4. Cero alertas abiertas en `cargo-audit` y `cargo-deny`.
5. Documentación bilingüe completa publicada en GitHub Pages mediante `mdBook`.
6. Al menos tres revisiones externas por pares (issues o PRs aceptadas).
7. Cero issues abiertos con etiqueta `critical` o `security`.
8. Release firmado con `cosign`, publicado en GitHub Releases con notas detalladas, y crates publicados en `crates.io`.
9. Paquetes `.deb` y `.rpm` adjuntos al release y validados mediante instalación en VMs limpias.



---


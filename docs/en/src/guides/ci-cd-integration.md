# CI/CD Integration (UC-05)

This guide describes how to integrate Vallumix into automated provisioning pipelines using Packer, Terraform, and CI platforms such as Jenkins, GitLab CI, or GitHub Actions.

## Scenario

- **Pipeline:** Base image construction with Packer, deployment with Terraform.
- **Requirement:** The base image must comply with at least 95% of CIS controls from the web profile.
- **Objective:** Automatically fail the build if the compliance rate is below the threshold.
- **Report:** JUnit XML consumable by Jenkins to display results in the build dashboard.

## Step 1: Integration with Packer

In your Packer template (for example, `base-image.pkr.hcl`), add a `shell` provisioner that runs Vallumix after the base installation:

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
Use `--threshold 95` so that Vallumix returns exit code `1` if the compliance rate is below 95%. Packer interprets a non-zero exit code as provisioner failure and aborts the image build.
```

## Step 2: Configure Threshold and Exit Codes

Vallumix returns explicit exit codes that CI tools interpret natively:

| Code | Meaning | CI Action |
|---|---|---|
| `0` | Compliance ≥ threshold (or no threshold configured) | Build passes |
| `1` | Compliance < threshold | Build fails — security policy not met |
| `2` | Configuration error (invalid profile, unsupported distro) | Build fails — review configuration |
| `3` | Privilege error (not run as root) | Build fails — review provisioner |

## Step 3: Integration with GitHub Actions

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

The `Publish test results` step displays each CIS control as a test case in the GitHub Actions interface, with failed controls highlighted in red.

## Step 4: Integration with GitLab CI

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

GitLab CI automatically consumes the JUnit XML and displays results in the pipeline's "Tests" tab, with trend charts across executions.

## Step 5: Integration with Jenkins

In your `Jenkinsfile`:

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

Jenkins parses the JUnit XML and displays failed controls as failed tests in the build dashboard. You can configure "unstable build" policies if there are medium or low severity failed controls, and "failed build" if there are critical failed controls.

## Step 6: Terraform Post-Deployment Validation

If you use Terraform to deploy instances, add a `local-exec` provisioner or a `null_resource` that runs Vallumix after deployment:

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
Do not run `vallumix apply` directly from Terraform `remote-exec` unless you have an automated rollback mechanism and a change approval process. Applying hardening in production should be a deliberate operation, not a side effect of infrastructure deployment.
```

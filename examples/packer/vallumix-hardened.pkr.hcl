packer {
  required_plugins {
    qemu = {
      version = ">= 1.0.0"
      source  = "github.com/hashicorp/qemu"
    }
  }
}

variable "vallumix_version" {
  type    = string
  default = "0.0.1"
}

variable "vallumix_deb_url" {
  type    = string
  default = "https://github.com/vallumix/vallumix/releases/download/v0.0.1/vallumix_0.0.1_amd64.deb"
}

source "qemu" "debian-12" {
  iso_url          = "https://cdimage.debian.org/debian-cd/current/amd64/iso-cd/debian-12.9.0-amd64-netinst.iso"
  iso_checksum     = "file:https://cdimage.debian.org/debian-cd/current/amd64/iso-cd/SHA256SUMS"
  output_directory = "output-debian-12"
  shutdown_command = "echo 'packer' | sudo -S shutdown -P now"
  disk_size        = "20000M"
  format           = "qcow2"
  accelerator      = "kvm"
  http_directory   = "http"
  ssh_username     = "packer"
  ssh_password     = "packer"
  ssh_timeout      = "20m"
  vm_name          = "vallumix-hardened"
  net_device       = "virtio-net"
  disk_interface   = "virtio"
  boot_wait        = "5s"
  boot_command = [
    "<esc><wait>",
    "install <wait>",
    " preseed/url=http://{{ .HTTPIP }}:{{ .HTTPPort }}/preseed.cfg <wait>",
    " debian-installer=en_US <wait>",
    " auto <wait>",
    " locale=en_US <wait>",
    " keymap=us <wait>",
    " netcfg/get_hostname=vallumix <wait>",
    " netcfg/get_domain=local <wait>",
    "<enter><wait>"
  ]
  qemuargs = [
    ["-m", "4096"],
    ["-smp", "2"],
  ]
}

build {
  name = "vallumix-hardened"
  sources = [
    "source.qemu.debian-12"
  ]

  provisioner "shell" {
    inline = [
      "echo '--- Installing Vallumix ---'",
      "wget -q ${var.vallumix_deb_url} -O /tmp/vallumix.deb || true",
      "if [ -f /tmp/vallumix.deb ]; then",
      "  sudo dpkg -i /tmp/vallumix.deb",
      "else",
      "  echo 'Vallumix .deb not found, skipping install'",
      "fi",
      "echo '--- Applying Vallumix hardening ---'",
      "sudo vallumix apply --profile web --report html --output /tmp/vallumix-report.html || true",
      "echo '--- Hardening complete ---'",
    ]
  }
}

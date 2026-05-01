# -*- mode: ruby -*-
# vi: set ft=ruby :

Vagrant.configure("2") do |config|
  profile = ENV.fetch("VALLUMIX_PROFILE", "web")
  dry_run = ENV.fetch("VALLUMIX_DRY_RUN", "0")

  config.vm.synced_folder ".", "/vagrant"

  config.vm.define "debian12" do |vm|
    vm.vm.box = "generic/debian12"
    vm.vm.hostname = "debian12"
    vm.vm.network "private_network", ip: "192.168.56.10"
    vm.vm.provider "virtualbox" do |vb|
      vb.memory = "512"
      vb.cpus = 1
    end
    vm.vm.provision "shell", path: "scripts/provision-debian.sh", env: {
      "VALLUMIX_PROFILE" => profile,
      "VALLUMIX_DRY_RUN" => dry_run,
    }
  end

  config.vm.define "ubuntu2404" do |vm|
    vm.vm.box = "generic/ubuntu2404"
    vm.vm.hostname = "ubuntu2404"
    vm.vm.network "private_network", ip: "192.168.56.11"
    vm.vm.provider "virtualbox" do |vb|
      vb.memory = "512"
      vb.cpus = 1
    end
    vm.vm.provision "shell", path: "scripts/provision-ubuntu.sh", env: {
      "VALLUMIX_PROFILE" => profile,
      "VALLUMIX_DRY_RUN" => dry_run,
    }
  end

  config.vm.define "rocky9" do |vm|
    vm.vm.box = "generic/rocky9"
    vm.vm.hostname = "rocky9"
    vm.vm.network "private_network", ip: "192.168.56.12"
    vm.vm.provider "virtualbox" do |vb|
      vb.memory = "512"
      vb.cpus = 1
    end
    vm.vm.provision "shell", path: "scripts/provision-rocky.sh", env: {
      "VALLUMIX_PROFILE" => profile,
      "VALLUMIX_DRY_RUN" => dry_run,
    }
  end
end

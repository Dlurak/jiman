{ pkgs }:
let
  manifest = pkgs.lib.importTOML ../Cargo.toml;
in
pkgs.rustPlatform.buildRustPackage {
  pname = manifest.package.name;
  version = manifest.package.version;

  src = pkgs.lib.cleanSource ./..;
  cargoLock.lockFile = ../Cargo.lock;

  nativeBuildInputs = with pkgs; [
    installShellFiles
  ];

  preFixup = ''
    mkdir completions
    $out/bin/jiman complete bash > completions/jiman.bash
    $out/bin/jiman complete zsh > completions/jiman.zsh
    $out/bin/jiman complete fish > completions/jiman.fish

    installShellCompletion completions/*
  '';

  meta = {
    description = "Pride flags for your terminal";
    homepage = "https://github.com/dlurak/jiman";
    mainProgram = "jiman";
    # license = lib.licenses.eupl12;
    # maintainers = with lib.maintainers; [
    #   dlurak
    # ];
  };
}

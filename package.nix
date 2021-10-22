{ pkgs, rustPlatform, crateName, ... }:
let 
    projectPath = ./.;
in (rustPlatform.buildRustPackage {
  pname = crateName;
  version = "0.1.0";
  src = projectPath;
  cargoLock = { lockFile = projectPath + /Cargo.lock; };
})

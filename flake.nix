{
  description = "Ferris RGB Dev Env";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay, ... }:
    let
      system = "x86_64-linux";
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs { inherit system overlays; };
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          direnv
          rust-bin.stable.latest.default
          rust-analyzer
          pkg-config
          openssl
        ];
        
        # ⚡ Critical: This hook ensures IDEs pick up the environment
        shellHook = ''
          export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath [ pkgs.openssl ]}
        '';
      };
    };
}

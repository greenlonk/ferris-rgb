{
  description = "Ferris RGB Dev Env";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay, ... }:
    let
      # 1. Define supported architectures (Intel + Apple Silicon)
      supportedSystems = [ "x86_64-linux" "aarch64-linux" ];

      # 2. Helper function to generate attributes for each system
      forAllSystems = f: nixpkgs.lib.genAttrs supportedSystems (system: f {
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
      });
    in
    {
      # 3. Use the helper to create devShells for both architectures
      devShells = forAllSystems ({ pkgs }: {
        default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rust-bin.stable.latest.default
            rustup
            rust-analyzer
            pkg-config
            bacon
            cargo-udeps
            cargo-expand
            cargo-chef
            cargo-diet
            openssl
            delta
            glow
            just
            fzf
            ripgrep
            fd
            bat
            tokei
          ];

          shellHook = ''
            export SHELL=$(which zsh)
            export RUST_SRC_PATH=$(rustc --print sysroot)/lib/rustlib/src/rust
            export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath [ pkgs.openssl ]}
          '';
        };
      });
    };
}

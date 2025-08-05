{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    rust-overlay.url = "github:oxalica/rust-overlay";
    nixpkgs.url = "github:nixos/nixpkgs/nixos-25.05";
  };

  outputs = { self, nixpkgs, naersk, rust-overlay }:
    let
      inherit self;
      system = "x86_64-linux";

      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs { inherit system overlays; };

      toolchain = pkgs.rust-bin.fromRustupToolchainFile ./toolchain.toml;

      naersk' = pkgs.callPackage naersk {
        cargo = toolchain;
        rustc = toolchain;
      };

      # make vergen_git2 happy
      VERGEN_IDEMPOTENT = "1";
      VERGEN_GIT_SHA = if (self ? "rev") then (builtins.substring 0 7 self.rev) else "nix-dirty";
    in
    {
      packages.${system}.default = naersk'.buildPackage {
        src = ./.;

        inherit VERGEN_IDEMPOTENT VERGEN_GIT_SHA;
      };

      devShells.${system}.default = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [ git toolchain ];

        shellHook = ''
          echo "Development shell for need"

          alias build='cargo build'
          alias build-release='cargo build --release'
          alias test='cargo test'
        '';

        inherit VERGEN_IDEMPOTENT VERGEN_GIT_SHA;
      };
    };
}

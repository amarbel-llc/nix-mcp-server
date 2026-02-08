{
  description = "MCP server providing nix and FlakeHub operations as tools";

  inputs = {
    nixpkgs-master.url = "github:NixOS/nixpkgs/b28c4999ed71543e71552ccfd0d7e68c581ba7e9";
    nixpkgs.url = "github:NixOS/nixpkgs/23d72dabcb3b12469f57b37170fcbc1789bd7457";
    utils.url = "https://flakehub.com/f/numtide/flake-utils/0.1.102";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane";
    fh.url = "https://flakehub.com/f/DeterminateSystems/fh/*.tar.gz";
    devenv-rust.url = "github:friedenberg/eng?dir=pkgs/alfa/devenv-rust";
  };

  outputs =
    {
      self,
      nixpkgs,
      utils,
      rust-overlay,
      crane,
      fh,
      devenv-rust, nixpkgs-master,
    }:
    utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        rustToolchain = pkgs.rust-bin.stable.latest.default;
        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        src = craneLib.cleanCargoSource ./.;

        commonArgs = {
          inherit src;
          strictDeps = true;
          buildInputs = [ ];
          nativeBuildInputs = [ ];
        };

        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        nix-mcp-server-unwrapped = craneLib.buildPackage (
          commonArgs
          // {
            inherit cargoArtifacts;
          }
        );

        fhPkg = fh.packages.${system}.default;

        # Note: We don't bundle nix itself - we use the system nix.
        # This ensures compatibility with Determinate Nix settings like lazy-trees.
        nix-mcp-server =
          pkgs.runCommand "nix-mcp-server"
            {
              nativeBuildInputs = [ pkgs.makeWrapper ];
            }
            ''
              mkdir -p $out/bin
              makeWrapper ${nix-mcp-server-unwrapped}/bin/nix-mcp-server $out/bin/nix-mcp-server \
                --prefix PATH : ${
                  pkgs.lib.makeBinPath [
                    fhPkg
                    pkgs.cachix
                    pkgs.nil
                  ]
                }
            '';
      in
      {
        packages = {
          default = nix-mcp-server;
          nix-mcp-server = nix-mcp-server;
          unwrapped = nix-mcp-server-unwrapped;
        };

        devShells.default = devenv-rust.devShells.${system}.default.overrideAttrs (oldAttrs: {
          nativeBuildInputs = (oldAttrs.nativeBuildInputs or [ ]) ++ [
            fhPkg
            pkgs.nil
          ];
        });
      }
    );
}

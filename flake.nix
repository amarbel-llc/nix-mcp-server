{
  description = "MCP server providing nix and FlakeHub operations as tools";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
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
      devenv-rust,
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

        apps.install-mcp = {
          type = "app";
          program = toString (
            pkgs.writeShellScript "install-nix-mcp" ''
              set -euo pipefail

              CLAUDE_CONFIG_DIR="''${HOME}/.claude"
              MCP_CONFIG_FILE="''${CLAUDE_CONFIG_DIR}/mcp.json"

              log() {
                ${pkgs.gum}/bin/gum style --foreground 212 "$1"
              }

              log_success() {
                ${pkgs.gum}/bin/gum style --foreground 82 "✓ $1"
              }

              log_error() {
                ${pkgs.gum}/bin/gum style --foreground 196 "✗ $1"
              }

              # Create config directory if needed
              if [[ ! -d "$CLAUDE_CONFIG_DIR" ]]; then
                log "Creating $CLAUDE_CONFIG_DIR..."
                mkdir -p "$CLAUDE_CONFIG_DIR"
              fi

              # Build the flake reference
              FLAKE_REF="${self}"

              # New MCP server entry
              NEW_SERVER=$(${pkgs.jq}/bin/jq -n \
                --arg cmd "nix" \
                --arg flake "$FLAKE_REF" \
                '{command: $cmd, args: ["run", $flake]}')

              if [[ -f "$MCP_CONFIG_FILE" ]]; then
                log "Found existing MCP config at $MCP_CONFIG_FILE"

                # Check if nix server already exists
                if ${pkgs.jq}/bin/jq -e '.mcpServers.nix' "$MCP_CONFIG_FILE" > /dev/null 2>&1; then
                  if ${pkgs.gum}/bin/gum confirm "nix MCP server already configured. Overwrite?"; then
                    UPDATED=$(${pkgs.jq}/bin/jq --argjson server "$NEW_SERVER" '.mcpServers.nix = $server' "$MCP_CONFIG_FILE")
                    echo "$UPDATED" > "$MCP_CONFIG_FILE"
                    log_success "Updated nix MCP server configuration"
                  else
                    log "Skipping installation"
                    exit 0
                  fi
                else
                  UPDATED=$(${pkgs.jq}/bin/jq --argjson server "$NEW_SERVER" '.mcpServers.nix = $server' "$MCP_CONFIG_FILE")
                  echo "$UPDATED" > "$MCP_CONFIG_FILE"
                  log_success "Added nix MCP server to existing configuration"
                fi
              else
                log "Creating new MCP config at $MCP_CONFIG_FILE"
                ${pkgs.jq}/bin/jq -n --argjson server "$NEW_SERVER" '{mcpServers: {nix: $server}}' > "$MCP_CONFIG_FILE"
                log_success "Created MCP configuration"
              fi

              log ""
              log "Installation complete! The nix MCP server will be available in Claude Code."
              log "Configuration written to: $MCP_CONFIG_FILE"
              log ""
              log "To verify, run: cat $MCP_CONFIG_FILE"
            ''
          );
        };
      }
    );
}

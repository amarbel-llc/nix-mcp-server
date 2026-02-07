# nix-mcp-server

An MCP (Model Context Protocol) server that exposes Nix and FlakeHub operations as tools for AI assistants like Claude.

## Installation

### Using Nix Flakes

```bash
nix run github:your-username/nix-mcp-server
```

Or add to your flake inputs:

```nix
{
  inputs.nix-mcp-server.url = "github:your-username/nix-mcp-server";
}
```

## Configuration

Add to your Claude Code MCP configuration (`.mcp.json`):

```json
{
  "mcpServers": {
    "nix": {
      "command": "nix",
      "args": ["run", "github:your-username/nix-mcp-server"]
    }
  }
}
```

Or if installed locally:

```json
{
  "mcpServers": {
    "nix": {
      "command": "/path/to/nix-mcp-server"
    }
  }
}
```

## Available Tools

### Nix Tools

| Tool | Description |
|------|-------------|
| `nix_build` | Build a nix flake package. Returns store paths on success. |
| `nix_flake_show` | List outputs of a nix flake. |
| `nix_flake_check` | Run flake checks and tests. |
| `nix_run` | Run a flake app. |
| `nix_develop_run` | Run a command inside a flake's devShell. |
| `nix_log` | Get build logs for a derivation. |
| `nix_eval` | Evaluate a nix expression. |

### FlakeHub Tools

| Tool | Description |
|------|-------------|
| `fh_search` | Search FlakeHub for flakes matching a query. |
| `fh_add` | Add a flake input to your flake.nix from FlakeHub. |
| `fh_list_flakes` | List public flakes on FlakeHub. |
| `fh_list_releases` | List all releases for a specific flake. |
| `fh_list_versions` | List versions matching a constraint for a flake. |
| `fh_resolve` | Resolve a FlakeHub flake reference to a store path. |

## Tool Details

### nix_build

Build a nix flake package.

**Parameters:**
- `installable` (string, optional): Flake installable (e.g., `.#default`, `nixpkgs#hello`). Defaults to `.#default`.
- `print_build_logs` (boolean, optional): Whether to print build logs. Defaults to true.
- `flake_dir` (string, optional): Directory containing the flake. Defaults to current directory.

### nix_flake_show

List outputs of a nix flake.

**Parameters:**
- `flake_ref` (string, optional): Flake reference (e.g., `.`, `github:NixOS/nixpkgs`). Defaults to `.`.
- `all_systems` (boolean, optional): Show outputs for all systems. Defaults to false.

### nix_flake_check

Run flake checks and tests.

**Parameters:**
- `flake_ref` (string, optional): Flake reference. Defaults to `.`.
- `keep_going` (boolean, optional): Continue on error. Defaults to true.

### nix_run

Run a flake app.

**Parameters:**
- `installable` (string, optional): Flake installable to run. Defaults to `.#default`.
- `args` (array of strings, optional): Arguments to pass to the app.

### nix_develop_run

Run a command inside a flake's devShell.

**Parameters:**
- `flake_ref` (string, optional): Flake reference. Defaults to `.`.
- `command` (string, required): Command to run in the devShell.
- `args` (array of strings, optional): Arguments to pass to the command.

### nix_log

Get build logs for a derivation.

**Parameters:**
- `installable` (string, required): Flake installable or store path.
- `tail` (integer, optional): Only return the last N lines.

### nix_eval

Evaluate a nix expression.

**Parameters:**
- `installable` (string, optional): Flake installable to evaluate (e.g., `.#packages.x86_64-linux`).
- `expr` (string, optional): Nix expression to evaluate (alternative to installable).
- `apply` (string, optional): Function to apply to the result (e.g., `builtins.attrNames`).

### fh_search

Search FlakeHub for flakes matching a query.

**Parameters:**
- `query` (string, required): The search query.
- `max_results` (integer, optional): Maximum number of results to return. Defaults to 10.

### fh_add

Add a flake input to your flake.nix from FlakeHub.

**Parameters:**
- `input_ref` (string, required): The flake reference to add (e.g., `NixOS/nixpkgs` or `NixOS/nixpkgs/0.2411.*`).
- `flake_path` (string, optional): Path to the flake.nix to modify. Defaults to `./flake.nix`.
- `input_name` (string, optional): Name for the flake input. If not provided, inferred from the input URL.

### fh_list_flakes

List public flakes on FlakeHub.

**Parameters:**
- `limit` (integer, optional): Maximum number of flakes to return.

### fh_list_releases

List all releases for a specific flake on FlakeHub.

**Parameters:**
- `flake` (string, required): The flake to list releases for (e.g., `NixOS/nixpkgs`).
- `limit` (integer, optional): Maximum number of releases to return.

### fh_list_versions

List versions matching a constraint for a flake on FlakeHub.

**Parameters:**
- `flake` (string, required): The flake to list versions for (e.g., `NixOS/nixpkgs`).
- `version_constraint` (string, required): Version constraint (e.g., `0.2411.*`, `>=0.2405`).
- `limit` (integer, optional): Maximum number of versions to return.

### fh_resolve

Resolve a FlakeHub flake reference to a store path.

**Parameters:**
- `flake_ref` (string, required): FlakeHub flake reference (e.g., `NixOS/nixpkgs/0.2411.*#hello`).

## Development

```bash
# Enter development shell
nix develop

# Build
cargo build

# Run tests
cargo test

# Format code
cargo fmt

# Run clippy
cargo clippy
```

## License

MIT

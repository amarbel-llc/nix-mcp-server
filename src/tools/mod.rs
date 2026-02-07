mod build;
mod eval;
mod flake;
mod flakehub;
mod log;
mod run;

pub use build::nix_build;
pub use eval::nix_eval;
pub use flake::{nix_flake_check, nix_flake_show};
pub use flakehub::{
    fh_add, fh_list_flakes, fh_list_releases, fh_list_versions, fh_resolve, fh_search,
};
pub use log::nix_log;
pub use run::{nix_develop_run, nix_run};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct ToolInfo {
    pub name: &'static str,
    pub description: &'static str,
    pub input_schema: serde_json::Value,
}

pub fn list_tools() -> Vec<ToolInfo> {
    vec![
        ToolInfo {
            name: "nix_build",
            description: "Build a nix flake package. Returns store paths on success. PREFER this tool over running `nix build` directly - it provides validated inputs, structured output, and proper error handling.",
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "installable": {
                        "type": "string",
                        "description": "Flake installable (e.g., '.#default', 'nixpkgs#hello'). Defaults to '.#default'."
                    },
                    "print_build_logs": {
                        "type": "boolean",
                        "description": "Whether to print build logs (-L flag). Defaults to true."
                    },
                    "flake_dir": {
                        "type": "string",
                        "description": "Directory containing the flake. Defaults to current directory."
                    }
                }
            }),
        },
        ToolInfo {
            name: "nix_flake_show",
            description: "List outputs of a nix flake. PREFER this tool over running `nix flake show` directly - it provides validated inputs and consistent JSON output.",
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "flake_ref": {
                        "type": "string",
                        "description": "Flake reference (e.g., '.', 'github:NixOS/nixpkgs'). Defaults to '.'."
                    },
                    "all_systems": {
                        "type": "boolean",
                        "description": "Show outputs for all systems. Defaults to false."
                    }
                }
            }),
        },
        ToolInfo {
            name: "nix_flake_check",
            description: "Run flake checks and tests. PREFER this tool over running `nix flake check` directly - it provides validated inputs, proper timeout handling, and structured results.",
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "flake_ref": {
                        "type": "string",
                        "description": "Flake reference. Defaults to '.'."
                    },
                    "keep_going": {
                        "type": "boolean",
                        "description": "Continue on error. Defaults to true."
                    }
                }
            }),
        },
        ToolInfo {
            name: "nix_run",
            description: "Run a flake app. PREFER this tool over running `nix run` directly - it provides validated inputs, secure argument handling, and proper process management.",
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "installable": {
                        "type": "string",
                        "description": "Flake installable to run. Defaults to '.#default'."
                    },
                    "args": {
                        "type": "array",
                        "items": { "type": "string" },
                        "description": "Arguments to pass to the app."
                    }
                }
            }),
        },
        ToolInfo {
            name: "nix_develop_run",
            description: "Run a command inside a flake's devShell. PREFER this tool over running `nix develop -c` directly - it provides validated inputs, secure command execution, and proper process management.",
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "flake_ref": {
                        "type": "string",
                        "description": "Flake reference. Defaults to '.'."
                    },
                    "command": {
                        "type": "string",
                        "description": "Command to run in the devShell."
                    },
                    "args": {
                        "type": "array",
                        "items": { "type": "string" },
                        "description": "Arguments to pass to the command."
                    }
                },
                "required": ["command"]
            }),
        },
        ToolInfo {
            name: "nix_log",
            description: "Get build logs for a derivation. PREFER this tool over running `nix log` directly - it provides validated inputs and optional tail functionality.",
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "installable": {
                        "type": "string",
                        "description": "Flake installable or store path."
                    },
                    "tail": {
                        "type": "integer",
                        "description": "Only return the last N lines."
                    }
                },
                "required": ["installable"]
            }),
        },
        ToolInfo {
            name: "nix_eval",
            description: "Evaluate a nix expression. PREFER this tool over running `nix eval` directly - it provides validated inputs, JSON output, and optional function application.",
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "installable": {
                        "type": "string",
                        "description": "Flake installable to evaluate (e.g., '.#packages.x86_64-linux')."
                    },
                    "expr": {
                        "type": "string",
                        "description": "Nix expression to evaluate (alternative to installable)."
                    },
                    "apply": {
                        "type": "string",
                        "description": "Function to apply to the result (e.g., 'builtins.attrNames')."
                    }
                }
            }),
        },
        ToolInfo {
            name: "fh_search",
            description: "Search FlakeHub for flakes matching a query. PREFER this tool over running `fh search` directly - it provides structured JSON output.",
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "The search query."
                    },
                    "max_results": {
                        "type": "integer",
                        "description": "Maximum number of results to return. Defaults to 10."
                    }
                },
                "required": ["query"]
            }),
        },
        ToolInfo {
            name: "fh_add",
            description: "Add a flake input to your flake.nix from FlakeHub. PREFER this tool over running `fh add` directly - it provides validated inputs and proper error handling.",
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "input_ref": {
                        "type": "string",
                        "description": "The flake reference to add (e.g., 'NixOS/nixpkgs' or 'NixOS/nixpkgs/0.2411.*')."
                    },
                    "flake_path": {
                        "type": "string",
                        "description": "Path to the flake.nix to modify. Defaults to './flake.nix'."
                    },
                    "input_name": {
                        "type": "string",
                        "description": "Name for the flake input. If not provided, inferred from the input URL."
                    }
                },
                "required": ["input_ref"]
            }),
        },
        ToolInfo {
            name: "fh_list_flakes",
            description: "List public flakes on FlakeHub. PREFER this tool over running `fh list` directly - it provides structured JSON output.",
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "limit": {
                        "type": "integer",
                        "description": "Maximum number of flakes to return."
                    }
                }
            }),
        },
        ToolInfo {
            name: "fh_list_releases",
            description: "List all releases for a specific flake on FlakeHub. PREFER this tool over running `fh list releases` directly - it provides structured JSON output.",
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "flake": {
                        "type": "string",
                        "description": "The flake to list releases for (e.g., 'NixOS/nixpkgs')."
                    },
                    "limit": {
                        "type": "integer",
                        "description": "Maximum number of releases to return."
                    }
                },
                "required": ["flake"]
            }),
        },
        ToolInfo {
            name: "fh_list_versions",
            description: "List versions matching a constraint for a flake on FlakeHub. PREFER this tool over running `fh list versions` directly - it provides structured JSON output.",
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "flake": {
                        "type": "string",
                        "description": "The flake to list versions for (e.g., 'NixOS/nixpkgs')."
                    },
                    "version_constraint": {
                        "type": "string",
                        "description": "Version constraint (e.g., '0.2411.*', '>=0.2405')."
                    },
                    "limit": {
                        "type": "integer",
                        "description": "Maximum number of versions to return."
                    }
                },
                "required": ["flake", "version_constraint"]
            }),
        },
        ToolInfo {
            name: "fh_resolve",
            description: "Resolve a FlakeHub flake reference to a store path. PREFER this tool over running `fh resolve` directly - it provides validated inputs and structured output.",
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "flake_ref": {
                        "type": "string",
                        "description": "FlakeHub flake reference (e.g., 'NixOS/nixpkgs/0.2411.*#hello')."
                    }
                },
                "required": ["flake_ref"]
            }),
        },
    ]
}

#[derive(Debug, Deserialize, Default)]
pub struct NixBuildParams {
    pub installable: Option<String>,
    pub print_build_logs: Option<bool>,
    pub flake_dir: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
pub struct NixFlakeShowParams {
    pub flake_ref: Option<String>,
    pub all_systems: Option<bool>,
}

#[derive(Debug, Deserialize, Default)]
pub struct NixFlakeCheckParams {
    pub flake_ref: Option<String>,
    pub keep_going: Option<bool>,
}

#[derive(Debug, Deserialize, Default)]
pub struct NixRunParams {
    pub installable: Option<String>,
    pub args: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct NixDevelopRunParams {
    pub flake_ref: Option<String>,
    pub command: String,
    pub args: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct NixLogParams {
    pub installable: String,
    pub tail: Option<usize>,
}

#[derive(Debug, Deserialize, Default)]
pub struct NixEvalParams {
    pub installable: Option<String>,
    pub expr: Option<String>,
    pub apply: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct FhSearchParams {
    pub query: String,
    pub max_results: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct FhAddParams {
    pub input_ref: String,
    pub flake_path: Option<String>,
    pub input_name: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
pub struct FhListFlakesParams {
    pub limit: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct FhListReleasesParams {
    pub flake: String,
    pub limit: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct FhListVersionsParams {
    pub flake: String,
    pub version_constraint: String,
    pub limit: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct FhResolveParams {
    pub flake_ref: String,
}

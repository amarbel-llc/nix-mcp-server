mod build;
mod eval;
mod flake;
mod log;
mod run;

pub use build::nix_build;
pub use eval::nix_eval;
pub use flake::{nix_flake_check, nix_flake_show};
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
            description: "Build a nix flake package. Returns store paths on success.",
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
            description: "List outputs of a nix flake.",
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
            description: "Run flake checks and tests.",
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
            description: "Run a flake app.",
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
            description: "Run a command inside a flake's devShell.",
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
            description: "Get build logs for a derivation.",
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
            description: "Evaluate a nix expression.",
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

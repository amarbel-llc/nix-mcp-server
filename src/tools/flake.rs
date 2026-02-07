use crate::nix_runner::run_nix_command;
use crate::tools::{NixFlakeCheckParams, NixFlakeShowParams};
use crate::validators::validate_flake_ref;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct NixFlakeShowResult {
    pub success: bool,
    pub outputs: serde_json::Value,
    pub stderr: String,
}

pub async fn nix_flake_show(params: NixFlakeShowParams) -> Result<NixFlakeShowResult, String> {
    let flake_ref = params.flake_ref.unwrap_or_else(|| ".".to_string());
    validate_flake_ref(&flake_ref).map_err(|e| e.to_string())?;

    let mut args = vec!["flake", "show", "--json"];

    if params.all_systems.unwrap_or(false) {
        args.push("--all-systems");
    }

    args.push(&flake_ref);

    let result = run_nix_command(&args).await.map_err(|e| e.to_string())?;

    let outputs = if result.success {
        serde_json::from_str(&result.stdout).unwrap_or(serde_json::Value::Null)
    } else {
        serde_json::Value::Null
    };

    Ok(NixFlakeShowResult {
        success: result.success,
        outputs,
        stderr: result.stderr,
    })
}

#[derive(Debug, Serialize)]
pub struct NixFlakeCheckResult {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
}

pub async fn nix_flake_check(params: NixFlakeCheckParams) -> Result<NixFlakeCheckResult, String> {
    let flake_ref = params.flake_ref.unwrap_or_else(|| ".".to_string());
    validate_flake_ref(&flake_ref).map_err(|e| e.to_string())?;

    let mut args = vec!["flake", "check"];

    if params.keep_going.unwrap_or(true) {
        args.push("--keep-going");
    }

    args.push(&flake_ref);

    let result = run_nix_command(&args).await.map_err(|e| e.to_string())?;

    Ok(NixFlakeCheckResult {
        success: result.success,
        stdout: result.stdout,
        stderr: result.stderr,
    })
}

use crate::nix_runner::run_nix_command;
use crate::tools::NixDerivationShowParams;
use crate::validators::{validate_flake_ref, validate_store_path};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct NixDerivationShowResult {
    pub success: bool,
    pub derivation: serde_json::Value,
    pub stderr: String,
}

pub async fn nix_derivation_show(
    params: NixDerivationShowParams,
) -> Result<NixDerivationShowResult, String> {
    let installable = params.installable.unwrap_or_else(|| ".#default".to_string());

    // Validate based on whether it's a store path or installable
    if installable.starts_with("/nix/store/") {
        validate_store_path(&installable).map_err(|e| e.to_string())?;
    } else {
        validate_flake_ref(&installable).map_err(|e| e.to_string())?;
    }

    let mut args = vec!["derivation", "show"];

    if params.recursive.unwrap_or(false) {
        args.push("--recursive");
    }

    args.push(&installable);

    let result = run_nix_command(&args).await.map_err(|e| e.to_string())?;

    let derivation = if result.success {
        serde_json::from_str(&result.stdout).unwrap_or(serde_json::Value::Null)
    } else {
        serde_json::Value::Null
    };

    Ok(NixDerivationShowResult {
        success: result.success,
        derivation,
        stderr: result.stderr,
    })
}

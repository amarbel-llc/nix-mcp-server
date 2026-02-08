use crate::nix_runner::run_nix_command;
use crate::tools::{NixCopyParams, NixStoreGcParams, NixStorePathInfoParams};
use crate::validators::{validate_flake_ref, validate_no_shell_metacharacters, validate_store_path};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct NixStorePathInfoResult {
    pub success: bool,
    pub path_info: serde_json::Value,
    pub stderr: String,
}

pub async fn nix_store_path_info(
    params: NixStorePathInfoParams,
) -> Result<NixStorePathInfoResult, String> {
    let mut args = vec!["path-info", "--json"];

    // Validate based on whether it's a store path or installable
    let path = &params.path;
    if path.starts_with("/nix/store/") {
        validate_store_path(path).map_err(|e| e.to_string())?;
    } else {
        validate_flake_ref(path).map_err(|e| e.to_string())?;
    }

    if params.closure.unwrap_or(false) {
        args.push("--closure");
    }

    if params.derivation.unwrap_or(false) {
        args.push("--derivation");
    }

    args.push(path);

    let result = run_nix_command(&args).await.map_err(|e| e.to_string())?;

    let path_info = if result.success {
        serde_json::from_str(&result.stdout).unwrap_or(serde_json::Value::Null)
    } else {
        serde_json::Value::Null
    };

    Ok(NixStorePathInfoResult {
        success: result.success,
        path_info,
        stderr: result.stderr,
    })
}

#[derive(Debug, Serialize)]
pub struct NixStoreGcResult {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
}

pub async fn nix_store_gc(params: NixStoreGcParams) -> Result<NixStoreGcResult, String> {
    let mut args = vec!["store", "gc"];

    if params.dry_run.unwrap_or(false) {
        args.push("--dry-run");
    }

    let max_freed_str;
    if let Some(ref max) = params.max_freed {
        validate_no_shell_metacharacters(max).map_err(|e| e.to_string())?;
        max_freed_str = max.clone();
        args.push("--max");
        args.push(&max_freed_str);
    }

    let result = run_nix_command(&args).await.map_err(|e| e.to_string())?;

    Ok(NixStoreGcResult {
        success: result.success,
        stdout: result.stdout,
        stderr: result.stderr,
    })
}

#[derive(Debug, Serialize)]
pub struct NixCopyResult {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
}

pub async fn nix_copy(params: NixCopyParams) -> Result<NixCopyResult, String> {
    let mut args = vec!["copy"];

    let to_store;
    if let Some(ref to) = params.to {
        validate_no_shell_metacharacters(to).map_err(|e| e.to_string())?;
        to_store = to.clone();
        args.push("--to");
        args.push(&to_store);
    }

    let from_store;
    if let Some(ref from) = params.from {
        validate_no_shell_metacharacters(from).map_err(|e| e.to_string())?;
        from_store = from.clone();
        args.push("--from");
        args.push(&from_store);
    }

    // Validate the installable/path
    let path = &params.installable;
    if path.starts_with("/nix/store/") {
        validate_store_path(path).map_err(|e| e.to_string())?;
    } else {
        validate_flake_ref(path).map_err(|e| e.to_string())?;
    }

    args.push(path);

    let result = run_nix_command(&args).await.map_err(|e| e.to_string())?;

    Ok(NixCopyResult {
        success: result.success,
        stdout: result.stdout,
        stderr: result.stderr,
    })
}

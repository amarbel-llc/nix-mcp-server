use crate::nix_runner::run_nix_command;
use crate::tools::NixSearchParams;
use crate::validators::{validate_flake_ref, validate_no_shell_metacharacters};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct NixSearchResult {
    pub success: bool,
    pub packages: serde_json::Value,
    pub stderr: String,
}

pub async fn nix_search(params: NixSearchParams) -> Result<NixSearchResult, String> {
    let flake_ref = params.flake_ref.unwrap_or_else(|| "nixpkgs".to_string());
    validate_flake_ref(&flake_ref).map_err(|e| e.to_string())?;

    validate_no_shell_metacharacters(&params.query).map_err(|e| e.to_string())?;

    let mut args = vec!["search", "--json", &flake_ref, &params.query];

    // Add --exclude patterns if provided
    let excludes = params.exclude.unwrap_or_default();
    let exclude_args: Vec<String> = excludes
        .iter()
        .flat_map(|e| vec!["--exclude".to_string(), e.clone()])
        .collect();
    for arg in &exclude_args {
        args.push(arg);
    }

    let result = run_nix_command(&args).await.map_err(|e| e.to_string())?;

    let packages = if result.success {
        serde_json::from_str(&result.stdout).unwrap_or(serde_json::Value::Null)
    } else {
        serde_json::Value::Null
    };

    Ok(NixSearchResult {
        success: result.success,
        packages,
        stderr: result.stderr,
    })
}

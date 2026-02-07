use crate::nix_runner::{parse_json_store_paths, parse_store_paths, run_nix_command};
use crate::tools::NixBuildParams;
use crate::validators::validate_installable;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct NixBuildResult {
    pub success: bool,
    pub store_paths: Vec<String>,
    pub stderr: String,
}

pub async fn nix_build(params: NixBuildParams) -> Result<NixBuildResult, String> {
    let installable = params.installable.unwrap_or_else(|| ".#default".to_string());
    validate_installable(&installable).map_err(|e| e.to_string())?;

    let mut args = vec!["build", "--json", "--print-out-paths"];

    if params.print_build_logs.unwrap_or(true) {
        args.push("-L");
    }

    args.push(&installable);

    let result = run_nix_command(&args).await.map_err(|e| e.to_string())?;

    let mut store_paths = parse_json_store_paths(&result.stdout);
    if store_paths.is_empty() {
        store_paths = parse_store_paths(&result.stdout);
    }

    Ok(NixBuildResult {
        success: result.success,
        store_paths,
        stderr: result.stderr,
    })
}

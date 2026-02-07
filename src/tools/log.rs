use crate::nix_runner::run_nix_command;
use crate::tools::NixLogParams;
use crate::validators::validate_installable;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct NixLogResult {
    pub success: bool,
    pub log: String,
    pub stderr: String,
}

pub async fn nix_log(params: NixLogParams) -> Result<NixLogResult, String> {
    validate_installable(&params.installable).map_err(|e| e.to_string())?;

    let args = vec!["log", &params.installable];

    let result = run_nix_command(&args).await.map_err(|e| e.to_string())?;

    let log = if let Some(tail) = params.tail {
        result
            .stdout
            .lines()
            .rev()
            .take(tail)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect::<Vec<_>>()
            .join("\n")
    } else {
        result.stdout
    };

    Ok(NixLogResult {
        success: result.success,
        log,
        stderr: result.stderr,
    })
}

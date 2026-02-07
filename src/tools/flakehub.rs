use crate::nix_runner::run_fh_command;
use crate::tools::{
    FhAddParams, FhListFlakesParams, FhListReleasesParams, FhListVersionsParams, FhResolveParams,
    FhSearchParams,
};
use crate::validators::validate_no_shell_metacharacters;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct FhSearchResult {
    pub success: bool,
    pub results: serde_json::Value,
    pub stderr: String,
}

#[derive(Debug, Serialize)]
pub struct FhAddResult {
    pub success: bool,
    pub output: String,
    pub stderr: String,
}

#[derive(Debug, Serialize)]
pub struct FhListResult {
    pub success: bool,
    pub results: serde_json::Value,
    pub stderr: String,
}

#[derive(Debug, Serialize)]
pub struct FhResolveResult {
    pub success: bool,
    pub result: serde_json::Value,
    pub stderr: String,
}

pub async fn fh_search(params: FhSearchParams) -> Result<FhSearchResult, String> {
    validate_no_shell_metacharacters(&params.query).map_err(|e| e.to_string())?;

    let mut args = vec!["search", "--json"];
    args.push(&params.query);

    let max_results_str;
    if let Some(max_results) = params.max_results {
        max_results_str = max_results.to_string();
        args.push("--max-results");
        args.push(&max_results_str);
    }

    let result = run_fh_command(&args).await.map_err(|e| e.to_string())?;

    let results = if result.success {
        serde_json::from_str(&result.stdout).unwrap_or(serde_json::Value::Null)
    } else {
        serde_json::Value::Null
    };

    Ok(FhSearchResult {
        success: result.success,
        results,
        stderr: result.stderr,
    })
}

pub async fn fh_add(params: FhAddParams) -> Result<FhAddResult, String> {
    validate_no_shell_metacharacters(&params.input_ref).map_err(|e| e.to_string())?;

    let mut args = vec!["add"];

    let flake_path;
    if let Some(ref path) = params.flake_path {
        validate_no_shell_metacharacters(path).map_err(|e| e.to_string())?;
        flake_path = path.clone();
        args.push("--flake-path");
        args.push(&flake_path);
    }

    let input_name;
    if let Some(ref name) = params.input_name {
        validate_no_shell_metacharacters(name).map_err(|e| e.to_string())?;
        input_name = name.clone();
        args.push("--input-name");
        args.push(&input_name);
    }

    args.push(&params.input_ref);

    let result = run_fh_command(&args).await.map_err(|e| e.to_string())?;

    Ok(FhAddResult {
        success: result.success,
        output: result.stdout,
        stderr: result.stderr,
    })
}

pub async fn fh_list_flakes(params: FhListFlakesParams) -> Result<FhListResult, String> {
    let mut args = vec!["list", "flakes", "--json"];

    let limit_str;
    if let Some(limit) = params.limit {
        limit_str = limit.to_string();
        args.push("--limit");
        args.push(&limit_str);
    }

    let result = run_fh_command(&args).await.map_err(|e| e.to_string())?;

    let results = if result.success {
        serde_json::from_str(&result.stdout).unwrap_or(serde_json::Value::Null)
    } else {
        serde_json::Value::Null
    };

    Ok(FhListResult {
        success: result.success,
        results,
        stderr: result.stderr,
    })
}

pub async fn fh_list_releases(params: FhListReleasesParams) -> Result<FhListResult, String> {
    validate_no_shell_metacharacters(&params.flake).map_err(|e| e.to_string())?;

    let mut args = vec!["list", "releases", "--json"];
    args.push(&params.flake);

    let limit_str;
    if let Some(limit) = params.limit {
        limit_str = limit.to_string();
        args.push("--limit");
        args.push(&limit_str);
    }

    let result = run_fh_command(&args).await.map_err(|e| e.to_string())?;

    let results = if result.success {
        serde_json::from_str(&result.stdout).unwrap_or(serde_json::Value::Null)
    } else {
        serde_json::Value::Null
    };

    Ok(FhListResult {
        success: result.success,
        results,
        stderr: result.stderr,
    })
}

pub async fn fh_list_versions(params: FhListVersionsParams) -> Result<FhListResult, String> {
    validate_no_shell_metacharacters(&params.flake).map_err(|e| e.to_string())?;
    validate_no_shell_metacharacters(&params.version_constraint).map_err(|e| e.to_string())?;

    let mut args = vec!["list", "versions", "--json"];
    args.push(&params.flake);
    args.push(&params.version_constraint);

    let limit_str;
    if let Some(limit) = params.limit {
        limit_str = limit.to_string();
        args.push("--limit");
        args.push(&limit_str);
    }

    let result = run_fh_command(&args).await.map_err(|e| e.to_string())?;

    let results = if result.success {
        serde_json::from_str(&result.stdout).unwrap_or(serde_json::Value::Null)
    } else {
        serde_json::Value::Null
    };

    Ok(FhListResult {
        success: result.success,
        results,
        stderr: result.stderr,
    })
}

pub async fn fh_resolve(params: FhResolveParams) -> Result<FhResolveResult, String> {
    validate_no_shell_metacharacters(&params.flake_ref).map_err(|e| e.to_string())?;

    let args = vec!["resolve", "--json", &params.flake_ref];

    let result = run_fh_command(&args).await.map_err(|e| e.to_string())?;

    let resolve_result = if result.success {
        serde_json::from_str(&result.stdout).unwrap_or(serde_json::Value::Null)
    } else {
        serde_json::Value::Null
    };

    Ok(FhResolveResult {
        success: result.success,
        result: resolve_result,
        stderr: result.stderr,
    })
}

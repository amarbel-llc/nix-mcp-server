use regex::Regex;
use std::sync::LazyLock;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("invalid flake reference: {0}")]
    InvalidFlakeRef(String),

    #[error("shell metacharacters not allowed: {0}")]
    ShellMetacharacters(String),

    #[error("invalid attribute path: {0}")]
    InvalidAttrPath(String),
}

static FLAKE_REF_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9._\-/:#+]+$").unwrap());

static ATTR_PATH_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9._\-]+$").unwrap());

static SHELL_METACHARACTERS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[;&|`$(){}\\<>!]").unwrap());

pub fn validate_installable(installable: &str) -> Result<&str, ValidationError> {
    if !FLAKE_REF_PATTERN.is_match(installable) {
        return Err(ValidationError::InvalidFlakeRef(installable.to_string()));
    }
    Ok(installable)
}

pub fn validate_flake_ref(flake_ref: &str) -> Result<&str, ValidationError> {
    if !FLAKE_REF_PATTERN.is_match(flake_ref) {
        return Err(ValidationError::InvalidFlakeRef(flake_ref.to_string()));
    }
    Ok(flake_ref)
}

pub fn validate_attr_path(attr_path: &str) -> Result<&str, ValidationError> {
    if !ATTR_PATH_PATTERN.is_match(attr_path) {
        return Err(ValidationError::InvalidAttrPath(attr_path.to_string()));
    }
    Ok(attr_path)
}

pub fn validate_no_shell_metacharacters(input: &str) -> Result<&str, ValidationError> {
    if SHELL_METACHARACTERS.is_match(input) {
        return Err(ValidationError::ShellMetacharacters(input.to_string()));
    }
    Ok(input)
}

pub fn validate_args(args: &[String]) -> Result<(), ValidationError> {
    for arg in args {
        validate_no_shell_metacharacters(arg)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_installables() {
        assert!(validate_installable(".#default").is_ok());
        assert!(validate_installable("nixpkgs#hello").is_ok());
        assert!(validate_installable("github:NixOS/nixpkgs#hello").is_ok());
        assert!(validate_installable(".#packages.x86_64-linux.default").is_ok());
    }

    #[test]
    fn test_invalid_installables() {
        assert!(validate_installable("$(malicious)").is_err());
        assert!(validate_installable("; rm -rf /").is_err());
        assert!(validate_installable("hello`whoami`").is_err());
    }

    #[test]
    fn test_shell_metacharacters() {
        assert!(validate_no_shell_metacharacters("hello").is_ok());
        assert!(validate_no_shell_metacharacters("hello; rm -rf").is_err());
        assert!(validate_no_shell_metacharacters("$(cmd)").is_err());
        assert!(validate_no_shell_metacharacters("foo | bar").is_err());
    }
}

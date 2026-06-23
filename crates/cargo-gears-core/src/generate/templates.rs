use anyhow::{Context, bail};
use std::path::Path;
use std::process::Command;

use super::{DEFAULT_BRANCH, DEFAULT_GIT_URL};

/// Discover template directories from a remote git repository.
///
/// Performs a shallow, blobless clone into a temp directory and runs
/// `git ls-tree` to list immediate subdirectories under `subfolder`.
pub fn list_remote_templates(
    git_url: Option<&str>,
    branch: Option<&str>,
    subfolder: &str,
) -> anyhow::Result<Vec<String>> {
    let url = git_url.unwrap_or(DEFAULT_GIT_URL);
    let branch = branch.unwrap_or(DEFAULT_BRANCH);

    let tmp = tempfile::TempDir::new().context("failed to create temp directory")?;
    let tmp_path = tmp.path();

    let clone_status = Command::new("git")
        .args([
            "clone",
            "--depth",
            "1",
            "--no-checkout",
            "--filter=blob:none",
            "--branch",
            branch,
            url,
        ])
        .arg(tmp_path)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::piped())
        .status()
        .context("failed to run git clone — is git installed?")?;

    if !clone_status.success() {
        bail!("failed to clone template repository '{url}' (branch '{branch}')");
    }

    let ls_output = Command::new("git")
        .arg("-C")
        .arg(tmp_path)
        .args(["ls-tree", "--name-only", "-d", "HEAD"])
        .arg(format!("{subfolder}/"))
        .output()
        .context("failed to run git ls-tree")?;

    if !ls_output.status.success() {
        bail!(
            "failed to list templates under '{subfolder}/' in repository '{url}' (branch '{branch}')"
        );
    }

    let stdout = String::from_utf8_lossy(&ls_output.stdout);
    let prefix = format!("{subfolder}/");
    let templates: Vec<String> = stdout
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.strip_prefix(&prefix).unwrap_or(line).to_owned())
        .collect();

    Ok(templates)
}

/// Discover template directories from a local path.
pub fn list_local_templates(
    local_path: &str,
    subfolder: Option<&str>,
) -> anyhow::Result<Vec<String>> {
    let base = Path::new(local_path);
    let dir = subfolder.map_or_else(|| base.to_path_buf(), |sub| base.join(sub));

    if !dir.is_dir() {
        bail!("template directory '{}' does not exist", dir.display());
    }

    let mut templates = Vec::new();
    for entry in
        std::fs::read_dir(&dir).with_context(|| format!("can't read '{}'", dir.display()))?
    {
        let entry = entry?;
        if entry.file_type()?.is_dir()
            && let Some(name) = entry.file_name().to_str()
        {
            templates.push(name.to_owned());
        }
    }
    templates.sort();
    Ok(templates)
}

/// Print a formatted template list to stdout.
pub fn print_template_list(kind: &str, templates: &[String]) {
    if templates.is_empty() {
        println!("No {kind} templates found.");
        return;
    }
    println!("Available {kind} templates:");
    for name in templates {
        println!("  - {name}");
    }
}

/// Print an "unknown template" error to stderr, then list available templates.
pub fn print_unknown_template_error(kind: &str, name: &str, templates: &[String]) {
    eprintln!("error: unknown {kind} template '{name}'");
    eprintln!();
    print_template_list(kind, templates);
}

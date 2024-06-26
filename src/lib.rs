use std::io::Result;
use std::process::Command;

use colored::Colorize;
use spinoff::{spinners, Color, Spinner};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Package {
    name: String,
    version: String,
    latest_version: String,
}

impl Package {
    fn to_formatted(&self) -> Self {
        Self {
            name: self.name.clone(),
            version: format!("v{}", self.version).bright_red().to_string(),
            latest_version: format!("v{}", self.latest_version)
                .bright_green()
                .to_string(),
        }
    }
}

// run in shell `pip list --outdated --format=json` and parse the json output
fn get_outdated() -> Result<Vec<Package>> {
    let mut spinner = Spinner::new(
        spinners::Dots,
        "Scanning for outdated packages...",
        Color::Cyan,
    );
    // get the json output
    let output = Command::new("pip")
        .args(["list", "--outdated", "--not-required", "--format=json"])
        .output()?;

    // parse the json output
    let packages: Vec<Package> = match serde_json::from_slice(&output.stdout) {
        Ok(packages) => packages,
        Err(error) => {
            spinner.fail(&format!("Failed to parse json:\n  {error}"));
            return Ok(vec![]);
        }
    };

    spinner.clear();

    Ok(packages)
}

/// # Errors
/// Will return `Err` if the command fails to execute
///
/// # Panics
/// Will panic if the command fails to execute
pub fn show_outdated() -> Result<()> {
    let packages = get_outdated()?;
    if packages.is_empty() {
        return Ok(());
    }
    for package in packages {
        let formatted = package.to_formatted();
        println!(
            "🐍 {}: {} -> {}",
            formatted.name, formatted.version, formatted.latest_version
        );
    }

    Ok(())
}

/// # Errors
/// Will return `Err` if the command fails to execute
///
/// # Panics
/// Will panic if the command fails to execute
pub fn update_all() -> Result<()> {
    // for each package, run `pip install --upgrade <package>`
    for package in get_outdated()? {
        let formatted = package.to_formatted();

        let message = format!(
            "Upgrading {} from {} to {}",
            formatted.name, formatted.version, formatted.latest_version
        );

        let mut spinner = Spinner::new(spinners::Dots, message, Color::Cyan);

        let output = std::process::Command::new("pip")
            .args(["install", "--upgrade", &formatted.name])
            .output()?;

        let stdout = String::from_utf8(output.stdout).unwrap_or_default();
        let stdout_output = stdout.lines().last().unwrap_or_default().trim();

        match output.status.code().unwrap_or(1) {
            0 => spinner.success(stdout_output),
            1 => spinner.fail(stdout_output),
            _ => spinner.warn(stdout_output),
        }
    }

    Ok(())
}

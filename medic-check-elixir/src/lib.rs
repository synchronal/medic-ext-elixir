#![cfg_attr(feature = "strict", deny(warnings))]

pub mod cli;

use medic_lib::CheckResult::{self, CheckError, CheckOk};
use medic_lib::std_to_string;

use std::fs;
use std::path::Path;
use std::process::Command;

pub fn mix_installed() -> CheckResult {
    match Command::new("which").args(["mix"]).output() {
        Ok(which) => {
            if which.status.success() {
                CheckOk
            } else {
                let stdout = std_to_string(which.stdout);
                let stderr = std_to_string(which.stderr);
                CheckError(
                    "Unable to find mix.".into(),
                    Some(stdout),
                    Some(stderr),
                    Some("asdf install elixir".into()),
                )
            }
        }
        Err(_err) => CheckError(
            "Unable to search for mix. Is `which` in your PATH?".into(),
            None,
            None,
            None,
        ),
    }
}

pub fn mix_project_exists(path: &Path) -> CheckResult {
    let mix_exs = path.join("mix.exs");
    if mix_exs.exists() {
        CheckOk
    } else {
        CheckError(
            "Could not find mix project. Please run from a directory with a mix.exs file".into(),
            Some(format!("Expected file: {mix_exs:?}")),
            None,
            None,
        )
    }
}

pub fn archive_installed(archive_name: String) -> CheckResult {
    mix_installed()?;
    match Command::new("mix").args(["archive"]).output() {
        Ok(output) => {
            let stdout = std_to_string(output.stdout);
            let stderr = std_to_string(output.stderr);
            if output.status.success() {
                let archive_substr = format!("* {archive_name}-");
                if stdout.contains(&archive_substr) {
                    CheckOk
                } else {
                    CheckError(
                        "Mix archive is not installed.".into(),
                        Some(stdout),
                        Some(stderr),
                        Some(format!("mix archive.install hex {archive_name} --force")),
                    )
                }
            } else {
                CheckError(
                    "Unable to determine which mix packages are installed.".into(),
                    Some(stdout),
                    Some(stderr),
                    None,
                )
            }
        }
        Err(_err) => CheckError(
            "Unable to determine which mix archives are installed.".into(),
            None,
            None,
            None,
        ),
    }
}

pub fn check_unused_deps(cd: Option<String>) -> CheckResult {
    mix_installed()?;
    let directory = cd.clone().unwrap_or(".".to_string());
    if let Ok(path) = fs::canonicalize(directory) {
        mix_project_exists(&path)?;
        match Command::new("mix")
            .args(["deps.unlock", "--check-unused"])
            .current_dir(&path)
            .output()
        {
            Ok(output) => {
                let stdout = std_to_string(output.stdout);
                let stderr = std_to_string(output.stderr);
                if output.status.success() {
                    CheckOk
                } else {
                    let remedy = match cd {
                        Some(dir) => format!("(cd {dir} && mix deps.unlock --unused)"),
                        None => "mix deps.unlock --unused".to_string(),
                    };
                    CheckError(
                        "Unused dependencies detected.".into(),
                        Some(stdout),
                        Some(stderr),
                        Some(remedy),
                    )
                }
            }
            Err(_err) => CheckError("Unable to check for unused deps.".into(), None, None, None),
        }
    } else {
        CheckError(
            "Given a `cd` param to a directory that does not exist.".into(),
            None,
            None,
            None,
        )
    }
}

pub fn local_mix_installed() -> CheckResult {
    mix_installed()?;
    match Command::new("mix").args(["archive"]).output() {
        Ok(output) => {
            let stdout = std_to_string(output.stdout);
            let stderr = std_to_string(output.stderr);
            if output.status.success() {
                if stdout.contains("* hex-") {
                    CheckOk
                } else {
                    CheckError(
                        "Mix hex archive is not installed.".into(),
                        Some(stdout),
                        Some(stderr),
                        Some("mix local.hex --force".into()),
                    )
                }
            } else {
                CheckError(
                    "Unable to determine which mix packages are installed.".into(),
                    Some(stdout),
                    Some(stderr),
                    Some("asdf install elixir".into()),
                )
            }
        }
        Err(_err) => CheckError(
            "Unable to determine which mix archives are installed.".into(),
            None,
            None,
            None,
        ),
    }
}

pub fn local_rebar_installed() -> CheckResult {
    mix_installed()?;
    match Command::new("mix")
        .args(["local.rebar", "--if-missing"])
        .output()
    {
        Ok(_) => CheckOk,
        Err(_) => CheckError(
            "Unable to install local rebar.".into(),
            None,
            None,
            Some("mix local.rebar".into()),
        ),
    }
}

pub fn packages_compiled(cd: Option<String>) -> CheckResult {
    mix_installed()?;
    let directory = cd.clone().unwrap_or(".".to_string());
    if let Ok(path) = fs::canonicalize(directory) {
        mix_project_exists(&path)?;
        match Command::new("mix")
            .args(["deps"])
            .current_dir(path)
            .output()
        {
            Ok(output) => {
                let stdout = std_to_string(output.stdout);
                let stderr = std_to_string(output.stderr);
                if output.status.success() {
                    if stdout.contains("the dependency build is outdated") {
                        let remedy = match cd {
                            Some(dir) => format!("(cd {dir} && mix deps.compile)"),
                            None => "mix deps.compile".to_string(),
                        };
                        CheckError(
                            "Mix deps are not compiled.".into(),
                            Some(stdout),
                            Some(stderr),
                            Some(remedy),
                        )
                    } else {
                        CheckOk
                    }
                } else {
                    CheckError(
                        "Unable to determine which mix packages are installed.".into(),
                        Some(stdout),
                        Some(stderr),
                        None,
                    )
                }
            }
            Err(_err) => CheckError(
                "Unable to determine which mix packages are installed.".into(),
                None,
                None,
                Some("asdf install elixir".into()),
            ),
        }
    } else {
        CheckError(
            "Given a `cd` param to a directory that does not exist.".into(),
            None,
            None,
            None,
        )
    }
}

pub fn packages_installed(cd: Option<String>) -> CheckResult {
    mix_installed()?;
    let directory = cd.clone().unwrap_or(".".to_string());
    if let Ok(path) = fs::canonicalize(directory) {
        mix_project_exists(&path)?;
        match Command::new("mix")
            .args(["deps"])
            .current_dir(path)
            .output()
        {
            Ok(output) => {
                let stdout = std_to_string(output.stdout);
                let stderr = std_to_string(output.stderr);
                if output.status.success() {
                    if stdout.contains("dependency is not available")
                        || stdout.contains("is out of date")
                    {
                        let remedy = match cd {
                            Some(dir) => format!("(cd {dir} && mix deps.get)"),
                            None => "mix deps.get".to_string(),
                        };
                        CheckError(
                            "Mix deps are out of date.".into(),
                            Some(stdout),
                            Some(stderr),
                            Some(remedy),
                        )
                    } else {
                        CheckOk
                    }
                } else {
                    CheckError(
                        "Unable to determine which mix packages are installed.".into(),
                        Some(stdout),
                        Some(stderr),
                        None,
                    )
                }
            }
            Err(_err) => CheckError(
                "Unable to determine which mix packages are installed.".into(),
                None,
                None,
                None,
            ),
        }
    } else {
        CheckError(
            "Given a `cd` param to a directory that does not exist.".into(),
            None,
            None,
            None,
        )
    }
}

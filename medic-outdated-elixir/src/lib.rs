#![cfg_attr(feature = "strict", deny(warnings))]

pub mod cli;
mod outdated;

use cli::CliArgs;
use medic_lib::std_to_string;
pub use outdated::{Dependency, OutdatedInfo};
use regex::Regex;
use std::error::Error;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::thread;

pub fn check_outdated(_args: CliArgs) -> Result<(), Box<dyn Error>> {
    let mut command = Command::new("mix");
    command.args(["hex.outdated", "--all"]);

    match command.output() {
        Ok(output) => {
            let stdout = std_to_string(output.stdout);
            let outdated: OutdatedInfo = OutdatedInfo::from_hex_outdated(stdout)?;
            for d in &outdated.dependencies {
                println!(
                    "::outdated::name={}::version={}::latest={}",
                    d.name, d.current, d.latest,
                );
            }
            if !outdated.dependencies.is_empty() {
                println!("::remedy::mix deps.update --all")
            }
        }
        Err(_) => return Err("::failure::Unable to get outdated".into()),
    }

    Ok(())
}

pub fn install_local_hex() -> Result<(), Box<dyn Error>> {
    let installed = local_hex_installed()?;
    if installed {
        return Ok(());
    }

    let mut command = Command::new("mix");
    command.args(["local.hex", "--force"]);
    command.stderr(Stdio::piped()).stdout(Stdio::piped());

    eprintln!("::action::mix-local-hex-install::Installing local.hex");

    let mut child = command.spawn().unwrap();
    let stderr = child.stderr.take().unwrap();
    let stdout = child.stdout.take().unwrap();

    let out_thr = thread::spawn(move || {
        let reader = BufReader::new(stdout);
        reader
            .lines()
            .map_while(Result::ok)
            .for_each(|line| eprintln!("::info::mix-local-hex-install::{}", &line));
    });
    let err_thr = thread::spawn(move || {
        let reader = BufReader::new(stderr);
        reader
            .lines()
            .map_while(Result::ok)
            .for_each(|line| eprintln!("::info::mix-local-hex-install::{}", &line));
    });

    let output = child.wait_with_output();
    out_thr.join().unwrap();
    err_thr.join().unwrap();

    match output {
        Ok(_) => eprintln!("::success::mix-local-hex-install::"),
        Err(_) => {
            eprintln!("::failure::mix-local-hex-install::");
            return Err("Unable to install mix local.hex".into());
        }
    }

    Ok(())
}

pub fn local_hex_installed() -> Result<bool, Box<dyn Error>> {
    let mut command = Command::new("mix");
    command.args(["archive"]);

    match command.output() {
        Ok(output) => {
            if output.status.success() {
                let hex_re = Regex::new(r"\A\* hex\-").unwrap();
                let stdout = std_to_string(output.stdout);
                if hex_re.is_match(&stdout) {
                    Ok(true)
                } else {
                    Ok(false)
                }
            } else {
                let stderr = std_to_string(output.stderr);
                Err(format!("Unable to check for mix local.hex: {stderr}").into())
            }
        }
        Err(err) => Err(format!("Unable to check for mix local.hex: {err}").into()),
    }
}

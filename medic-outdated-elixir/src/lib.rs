#![cfg_attr(feature = "strict", deny(warnings))]

pub mod cli;
mod outdated;

use cli::CliArgs;
use medic_lib::std_to_string;
use outdated::OutdatedInfo;
use std::error::Error;
use std::process::Command;

pub fn check_outdated(_args: CliArgs) -> Result<(), Box<dyn Error>> {
    let mut command = Command::new("mix");
    command.args(["hex.outdated", "--all"]);

    match command.output() {
        Ok(output) => {
            let stdout = std_to_string(output.stdout);
            let outdated: OutdatedInfo = OutdatedInfo::from_str(stdout)?;
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

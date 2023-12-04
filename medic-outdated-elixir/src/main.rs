use medic_outdated_elixir::check_outdated;
use medic_outdated_elixir::cli::CliArgs;
use medic_outdated_elixir::install_local_hex;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = CliArgs::new();
    install_local_hex()?;
    check_outdated(cli)
}

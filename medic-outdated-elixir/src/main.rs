use medic_outdated_elixir::check_outdated;
use medic_outdated_elixir::cli::CliArgs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = CliArgs::new();
    check_outdated(cli)
}

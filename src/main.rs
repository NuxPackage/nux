mod commands;
mod handler;
mod help;
use handler::cli_handler;
mod cli;
mod nixpkgs;

fn main() {
    cli_handler::handle_cli(cli::app::make_clap_app().get_matches());
    // println!("Hello, world!");
}

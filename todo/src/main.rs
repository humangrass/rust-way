mod models;
mod cli;
mod storage;
mod service;

fn main() {
    let cli = cli::setup_cli();
    let cli_matches = cli.get_matches();
    println!("{:?}", cli_matches)
}

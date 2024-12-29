use crate::models::task::Task;

mod cli;
mod models;
mod storage;

fn main() {
    let cli = cli::setup_cli();
    let matches = cli.get_matches();

    let subcommand = matches.subcommand();
    match matches.subcommand() {
        Some((cli::COMMAND_ADD, sub_matches)) => {
            Task::new(sub_matches);
        }
        Some((cli::COMMAND_START, sub_matches)) => {
            println!("Start task");
        }
        Some((cli::COMMAND_DELETE, sub_matches)) => {
            println!("Delete task");
        }
        Some((cli::COMMAND_LIST, sub_matches)) => {
            Task::show_list();
        }
        Some((cli::COMMAND_EDIT, sub_matches)) => {
            println!("Edite task");
        }
        Some((cli::COMMAND_COMPLETE, sub_matches)) => {
            println!("Complete task");
        }
        _ => {
            println!(
                "Other commands are not implemented yet. Command: {:?}",
                subcommand
            );
        }
    }
}

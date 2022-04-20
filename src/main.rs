use clap::{arg, command, Command};
use taskmaster::cli::{clear_done, list_add, list_list, mark_done};
use taskmaster::task::TaskList;

fn main() {
    let matches = command!()
        .arg_required_else_help(true)
        .subcommand(
            Command::new("add")
                .about("Add a task to the list")
                .arg(arg!([DESCRIPTION]).required(true)),
        )
        .subcommand(
            Command::new("list")
                .about("List tasks")
                .arg(arg!(-d --description <DESCRIPTION>).required(false))
                .arg(arg!(-a --all).required(false)),
        )
        .subcommand(
            Command::new("do")
                .about("Mark a task as done")
                .arg(arg!([INDEX]).required(true)),
        )
        .subcommand(
            Command::new("clear")
                .about("Remove done tasks from the list")
        )
        .get_matches();

    let mut list = TaskList::import();

    if let Some(cmd) = matches.subcommand() {
        match cmd {
            ("add", sub_matches) => list_add(&mut list, sub_matches.clone()),
            ("list", sub_matches) => list_list(&mut list, sub_matches.clone()),
            ("do", sub_matches) => mark_done(&mut list, sub_matches.clone()),
            ("clear", _) => clear_done(&mut list),
            _ => unreachable!(),
        }
    }

    list.export();
}

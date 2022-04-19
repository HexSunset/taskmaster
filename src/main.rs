use clap::{arg, command};
use taskmaster::cli::{list_add, list_list};
use taskmaster::task::TaskList;

fn main() {
    let matches = command!()
        .arg(
            arg!([SUBCOMMAND])
                .required(true)
                .possible_values(["add", "list", "remove"]),
        )
        .get_matches();

    let mut list = TaskList::import();

    if let Some(cmd) = matches.value_of("SUBCOMMAND") {
        match cmd {
            "add" => list_add(&mut list),
            "list" => list_list(&mut list),
            _ => unreachable!(),
        }
    }

    list.export();
}

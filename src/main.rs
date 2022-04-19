use clap::{arg, command};
use taskmaster::cli::{list_add, list_list, mark_done};
use taskmaster::task::TaskList;

fn main() {
    let matches = command!()
        .arg(
            arg!([SUBCOMMAND])
                .possible_values(["add", "list", "remove", "done"])
                .default_value("list"),
        )
        .get_matches();

    let mut list = TaskList::import();

    if let Some(cmd) = matches.value_of("SUBCOMMAND") {
        match cmd {
            "add" => list_add(&mut list),
            "list" => list_list(&mut list),
            "done" => mark_done(&mut list),
            _ => unreachable!(),
        }
    }

    list.export();
}

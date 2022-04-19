use crate::task::{new_task, Task, TaskList};
use std::io::{self, Write};

pub fn print_task(t: (usize, Task)) {
    print!("{}. ", t.0);

    if t.1.get_done() {
        print!("[x] ");
    } else {
        print!("[ ] ");
    }

    println!("{}", t.1.get_desc());
}

pub fn list_add(list: &mut TaskList) {
    print!("description: ");
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    let mut t = new_task();
    t.set_desc(buffer);

    list.add(t);
}

pub fn list_list(list: &mut TaskList) {
    let mut description = String::new();
    print!("filter by description: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut description).unwrap();
    description = description.trim().to_string();

    let mut filter_done = String::new();
    print!("filter out done ('n'): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut filter_done).unwrap();
    filter_done = filter_done.trim().to_string();
    let filter_done = match filter_done.as_str() {
        "y" => true,
        _ => false,
    };

    let mut filter_undone = String::new();
    print!("filter out undone ('n'): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut filter_undone).unwrap();
    filter_undone = filter_undone.trim().to_string();
    let filter_undone = match filter_undone.as_str() {
        "y" => true,
        _ => false,
    };

    println!("---");

    let tasks = list.get(description, filter_done, filter_undone);
    if let Some(tasks) = tasks {
        for t in tasks {
            print_task(t);
        }
    } else {
        println!("No tasks to print");
    }
}

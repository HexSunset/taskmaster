use crate::task::{new_task, Task, TaskList};
use clap::ArgMatches;

pub fn print_task(t: (usize, Task)) {
    print!("{}. ", t.0);

    if t.1.is_done() {
        print!("[X] ");
    } else {
        print!("[ ] ");
    }

    println!("{}", t.1.get_desc());
}

pub fn list_add(list: &mut TaskList, sub: ArgMatches) {
    let desc = sub.value_of("DESCRIPTION").unwrap();
    let mut t = new_task();
    t.set_desc(desc.to_string());

    list.add(t);

    println!("Task added!");
}

pub fn list_list(list: &mut TaskList, sub: ArgMatches) {
    let description: String = match sub.value_of("description") {
        Some(d) => d.to_string(),
        None => String::new(),
    };

    let filter_done = !sub.is_present("all");

    let tasks = list.get(description, filter_done);
    if let Some(tasks) = tasks {
        for t in tasks {
            print_task(t);
        }
    } else {
        println!("No tasks to print");
    }
}

pub fn mark_done(list: &mut TaskList, sub: ArgMatches) {
    let index: usize = sub.value_of("INDEX").unwrap().parse().unwrap();

    let task = list.get_index(index);
    if let Some(task) = task {
        task.set_done(true);
        println!("Task {} marked as done", index);
    } else {
        println!("No task with index {}", index);
    }
}

pub fn clear_done(list: &mut TaskList) {
    println!("Removing done tasks...");
    list.rm_done();
}

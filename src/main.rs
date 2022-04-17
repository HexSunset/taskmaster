use taskmaster::cli::print_task;
use taskmaster::task::Task;

fn main() {
    let mut t = Task::new();
    t.set_desc("Task in progress".to_string());
    print_task(t);

    let mut t2 = Task::new();
    t2.set_desc("Task you've completed".to_string());
    t2.set_done(true);
    print_task(t2);
}

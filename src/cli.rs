use crate::task::Task;

pub fn print_task(t: Task) {
    if t.get_done() {
        print!("(x) ");
    } else {
        print!("( ) ");
    }

    println!("{}", t.get_desc());
}

use ron;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskList {
    tasks: Vec<Task>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    desc: String,
    done: bool,
}

impl Task {
    pub fn get_desc(&self) -> String {
        self.desc.clone()
    }

    pub fn get_done(&self) -> bool {
        self.done
    }

    pub fn set_desc(&mut self, desc: String) {
        self.desc = desc;
    }

    pub fn set_done(&mut self, state: bool) {
        self.done = state;
    }
}

impl TaskList {
    pub fn new() -> TaskList {
        TaskList { tasks: vec![] }
    }

    // TODO: Add print info
    pub fn import() -> TaskList {
        let list_str = std::fs::read_to_string("/home/aurora/taskmaster.ron");
        if let Err(_) = list_str {
            TaskList::new()
        } else {
            ron::de::from_str(&list_str.unwrap()).unwrap()
        }
    }

    // TODO: Add print info
    pub fn export(&self) {
        let file = std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open("/home/aurora/taskmaster.ron");
        if file.is_err() {
            let file = std::fs::File::create("/home/aurora/taskmaster.ron").unwrap();
            ron::ser::to_writer(file, &self).unwrap();
        } else {
            let file = file.unwrap();
            ron::ser::to_writer(file, &self).unwrap();
        }
    }

    pub fn add(&mut self, t: Task) {
        self.tasks.push(t);
    }

    pub fn remove(&mut self, i: usize) {
        if i < self.tasks.len() {
            self.tasks.remove(i);
        }
    }

    pub fn get(
        &self,
        name: String,
        filter_done: bool,
        filter_undone: bool,
    ) -> Option<Vec<(usize, Task)>> {
        let tasks = self.tasks.clone().into_iter();
        let tasks = tasks.enumerate();
        let mut tasks: Vec<(usize, Task)> = tasks.collect();
        tasks.retain(|t| t.1.get_desc().contains(&name));
        if filter_done {
            tasks.retain(|t| t.1.get_done())
        }
        if filter_undone {
            tasks.retain(|t| !t.1.get_done())
        }
        if tasks.len() > 0 {
            Some(tasks)
        } else {
            None
        }
    }
}

pub fn new_task() -> Task {
    Task {
        desc: String::new(),
        done: false,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn task_creation() {
        let mut t = new_task();
        assert_eq!(t.desc, "".to_string());
        t.set_desc("Test".to_string());
        assert!(!t.desc.is_empty());
    }

    #[test]
    fn get_by_desc() {
        let mut l = TaskList::new();

        let mut t = new_task();
        t.set_desc("one test".to_string());
        l.add(t);

        let mut t2 = new_task();
        t2.set_desc("another test".to_string());
        l.add(t2);

        assert!(l.get("one".to_string(), false, false).unwrap().len() == 1);
        assert!(l.get("another".to_string(), false, false).unwrap().len() == 1);

        assert!(l.get("test".to_string(), false, false).unwrap().len() == 2);

        assert!(l.get("third".to_string(), false, false).is_none());
    }

    #[test]
    fn get_done() {
        let mut l = TaskList::new();

        let mut t = new_task();
        t.set_desc("done task".to_string());
        t.set_done(true);
        l.add(t);

        let mut t2 = new_task();
        t2.set_desc("in progress task".to_string());
        l.add(t2);

        assert!(l.get("".to_string(), true, false).unwrap().len() == 1);
        assert!(l.get("".to_string(), false, true).unwrap().len() == 1);
    }
}

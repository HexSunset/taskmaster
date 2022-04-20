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

    pub fn is_done(&self) -> bool {
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
        let home = std::env::var("HOME").unwrap();
        let list_str = std::fs::read_to_string(format!("{}/taskmaster.ron", home));
        if let Err(_) = list_str {
            TaskList::new()
        } else {
            ron::de::from_str(&list_str.unwrap()).unwrap()
        }
    }

    // TODO: Add print info
    pub fn export(&self) {
        let home = std::env::var("HOME").unwrap();
        let conf = ron::ser::PrettyConfig::new();
        let file = std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(format!("{}/taskmaster.ron", home));
        if file.is_err() {
            let file = std::fs::File::create(format!("{}/taskmaster.ron", home)).unwrap();
            ron::ser::to_writer_pretty(file, &self, conf.clone()).unwrap();
        } else {
            let file = file.unwrap();
            ron::ser::to_writer_pretty(file, &self, conf).unwrap();
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

    pub fn get_index(&mut self, i: usize) -> Option<&mut Task> {
        if i < self.tasks.len() {
            return Some(&mut self.tasks[i]);
        } else {
            return None;
        }
    }

    pub fn get(&self, name: String, filter_done: bool) -> Option<Vec<(usize, Task)>> {
        let tasks = self.tasks.clone().into_iter();
        let tasks = tasks.enumerate();
        let mut tasks: Vec<(usize, Task)> = tasks.collect();
        tasks.retain(|t| t.1.get_desc().contains(&name));
        if filter_done {
            tasks.retain(|t| !t.1.is_done())
        }

        if tasks.len() > 0 {
            Some(tasks)
        } else {
            None
        }
    }

    pub fn rm_done(&mut self) {
        self.tasks.retain(|t| !t.is_done())
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

        assert!(l.get("one".to_string(), false).unwrap().len() == 1);
        assert!(l.get("another".to_string(), false).unwrap().len() == 1);

        assert!(l.get("test".to_string(), false).unwrap().len() == 2);

        assert!(l.get("third".to_string(), false).is_none());
    }

    #[test]
    fn is_done() {
        let mut l = TaskList::new();

        let mut t = new_task();
        t.set_desc("done task".to_string());
        t.set_done(true);
        l.add(t);

        let mut t2 = new_task();
        t2.set_desc("in progress task".to_string());
        l.add(t2);

        assert!(l.get("".to_string(), true).unwrap().len() == 1);
        assert!(l.get("".to_string(), false).unwrap().len() == 2);
    }

    #[test]
    fn remove_done() {
        let mut l = TaskList::new();

        let mut t = new_task();
        t.set_done(true);
        l.add(t);

        let t2 = new_task();
        l.add(t2);

        let t3 = new_task();
        l.add(t3);

        assert_eq!(l.get("".to_string(), false).unwrap().len(), 3);

        l.rm_done();

        assert_eq!(l.get("".to_string(), false).unwrap().len(), 2);
    }
}

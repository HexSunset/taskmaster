pub struct TaskList {
    tasks: Vec<Task>,
}

#[derive(Clone)]
pub struct Task {
    desc: String,
    done: bool,
}

impl Task {
    pub fn new() -> Task {
        Task {
            desc: String::new(),
            done: false,
        }
    }

    pub fn get_desc(&self) -> String {
        self.desc.clone()
    }

    pub fn get_done(&self) -> bool {
        self.done
    }

    pub fn set_desc(&mut self, desc: String) {
        self.desc= desc;
    }

    pub fn set_done(&mut self, state: bool) {
        self.done = state;
    }
}

impl TaskList {
    pub fn new() -> TaskList {
        TaskList { tasks: vec![] }
    }

    pub fn add(&mut self, t: Task) {
        self.tasks.push(t);
    }

    pub fn get_by_desc(&self, name: String) -> Option<Vec<Task>> {
        let mut tasks = self.tasks.clone();
        tasks.retain(|t| t.get_desc().contains(&name) );
        if tasks.len() > 0 {
            Some(tasks)
        } else {
            None
        }
    }

    pub fn get_done(&self) -> Option<Vec<Task>> {
        let mut tasks = self.tasks.clone();
        tasks.retain(|t| t.get_done());

        if tasks.len() > 0 {
            Some(tasks)
        } else {
            None
        }
    }

    pub fn get_not_done(&self) -> Option<Vec<Task>> {
        let mut tasks = self.tasks.clone();
        tasks.retain(|t| !t.get_done());

        if tasks.len() > 0 {
            Some(tasks)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn task_creation() {
        let mut t = Task::new();
        assert_eq!(t.desc, "".to_string());
        t.set_desc("Test".to_string());
        assert!(!t.desc.is_empty());
    }
    
    #[test]
    fn get_by_desc() {
        let mut l = TaskList::new();

        let mut t = Task::new();
        t.set_desc("one test".to_string());
        l.add(t);

        let mut t2 = Task::new();
        t2.set_desc("another test".to_string());
        l.add(t2);

        assert!(l.get_by_desc("one".to_string()).unwrap().len() == 1);
        assert!(l.get_by_desc("another".to_string()).unwrap().len() == 1);

        assert!(l.get_by_desc("test".to_string()).unwrap().len() == 2);

        assert!(l.get_by_desc("third".to_string()).is_none());
    }
    
    #[test]
    fn get_done() {
        let mut l = TaskList::new();

        let mut t = Task::new();
        t.set_desc("done task".to_string());
        t.set_done(true);
        l.add(t);

        let mut t2 = Task::new();
        t2.set_desc("in progress task".to_string());
        l.add(t2);

        assert!(l.get_done().unwrap().len() == 1);
        assert!(l.get_not_done().unwrap().len() == 1);
    }
}

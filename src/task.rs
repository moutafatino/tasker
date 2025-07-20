use std::collections::HashMap;

pub struct Task {
    id: u32,
    title: String,
    status: Status,
}

pub enum Status {
    Todo,
    InProgress,
    Done,
}

pub struct TaskManager {
    tasks: HashMap<u32, Task>,
    next_id: u32,
}

impl TaskManager {
    pub fn init() -> Self {
        Self {
            tasks: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn add_task(&mut self, title: &str) {
        self.tasks.insert(
            self.next_id,
            Task {
                id: self.next_id,
                title: title.into(),
                status: Status::Todo,
            },
        );
    }
}

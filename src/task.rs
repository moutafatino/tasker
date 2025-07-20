use core::fmt;
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

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Status::Todo => write!(f, "Todo"),
            Status::InProgress => write!(f, "In Progress"),
            Status::Done => write!(f, "Done"),
        }
    }
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

    pub fn list_tasks(&self) {
        for task in self.tasks.values() {
            println!("[{}] - {} - {}", task.id, task.title, task.status);
        }
    }

    pub fn cmd_help(&self) {
        let commands = vec![
            ("add <title>", "- Add a new task"),
            ("list", "- List tasks"),
        ];

        println!("Available Commands:");
        for (cmd, desc) in commands {
            println!("   {:<30} {}", cmd, desc);
        }
    }
}

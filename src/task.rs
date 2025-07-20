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

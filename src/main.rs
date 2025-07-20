use task::{Task, TaskManager};

mod task;
fn main() {
    let mut app = TaskManager::init();

    app.add_task("Test task");

    app.list_tasks();
}

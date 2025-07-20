use task::{Task, TaskManager};

mod task;
fn main() {
    let mut app = TaskManager::init();

    app.cmd_help();

    app.add_task("Test task");

    app.list_tasks();
}

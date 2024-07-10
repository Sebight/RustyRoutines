use std::fmt;

pub struct TaskContext {
    pub task_id: u32,
    pub task_name: String,
    pub task_description: String,
    pub task_status: String,
}

pub fn create_task(ctx: TaskContext) {
    println!("Task created!");
    println!("Task ctx: {}", ctx);
}

impl fmt::Display for TaskContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Task ID: {}, Task Name: {}, Task Description: {}, Task Status: {}",
            self.task_id, self.task_name, self.task_description, self.task_status
        )
    }
}
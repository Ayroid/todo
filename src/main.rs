mod task;
use task::Task;

fn main() {
    let task = Task::new(1, String::from("Learn Rust"));
    println!("Task Created: {:?}", task);
}

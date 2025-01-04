mod storage;
mod task;

use storage::TaskStorage;

fn main() {
    let mut storage = TaskStorage::new();

    storage.add_task(String::from("Learn Rust"));
    storage.add_task(String::from("CLI Application"));

    for task in storage.list_tasks() {
        println!("#{}: {}", task.id, task.title);
    }
}

#![no_std]

extern crate alloc;
extern crate task_async;
#[macro_use] extern crate terminal_print;

use alloc::string::String;
use task_async::AsyncTask;
use task_async::async_executor::AsyncExecutor;

// A future representing the real computation
async fn test() -> String {
    String::from("Hello")
}

// Return a future that will be wrapped in an async task
async fn print() {
    let msg = test().await;
    println!("message = {}", msg);
}

// Test code for async executor
pub fn main() -> isize {
    println!("test_async.");

    // Create an async executor
    let mut executor = AsyncExecutor::new();
    let task = AsyncTask::new(print());
    // Spawn a new task
    executor.spawn(task);

    // Start to poll the async tasks with current thread
    println!("Ready to run executor.");
    executor.run();

    0
}

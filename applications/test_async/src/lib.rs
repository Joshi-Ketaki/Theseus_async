#![no_std]

extern crate alloc;
extern crate task_async;
#[macro_use] extern crate terminal_print;

use alloc::string::String;
use task_async::AsyncTask;
use task_async::async_executor::AsyncExecutor;

async fn test() -> String {
    String::from("Hello")
}

async fn print() {
    let msg = test().await;
    println!("message = {}", msg);
}

pub fn main() -> isize {
    println!("test_async.");

    let mut executor = AsyncExecutor::new();
    let task = AsyncTask::new(print());
    executor.spawn(task);

    println!("Ready to run executor.");
    executor.run();

    0
}

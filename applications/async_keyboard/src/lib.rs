#![no_std]

extern crate alloc;
extern crate task;
extern crate keycodes_ascii;
extern crate app_io;
extern crate stdio;
extern crate bare_io;
extern crate futures_util;
#[macro_use] extern crate log;
#[macro_use] extern crate terminal_print;

use spin::Mutex;
use alloc::{sync::Arc, string::{String, ToString}};
use core::{str, pin::Pin, task::{Poll, Context}};
use futures_util::stream::{Stream, StreamExt};
use task_async::AsyncTask;
use task_async::async_executor::AsyncExecutor;
use keycodes_ascii::KeyAction;
use stdio::KeyEventReadGuard;
use libterm::Terminal;

pub struct KeyEventStream {
    num_of_char: u64,
    read_guard: KeyEventReadGuard,
    terminal: Arc<Mutex<Terminal>>,
}

impl KeyEventStream {
    pub fn new(num: u64, read_guard: KeyEventReadGuard, terminal: Arc<Mutex<Terminal>>) -> Self {
        KeyEventStream {
            num_of_char: num,
            read_guard: read_guard,
            terminal: terminal,
        }
    }
}

impl Stream for KeyEventStream {
    type Item = u8;
    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context) -> Poll<Option<u8>> {
        if self.num_of_char == 0 { return Poll::Ready(None); }      // completed
        if let Some(ref key_event_queue) = *self.read_guard {
            loop {
                match key_event_queue.read_one() {
                    Some(keyevent) => {
                        if keyevent.action == KeyAction::Pressed { 
                            match keyevent.keycode.to_ascii(keyevent.modifiers) {
                                Some(c) => {
                                    self.num_of_char -= 1;
                                    let mut locked_terminal = self.terminal.lock();
                                    locked_terminal.print_to_terminal(c.to_string());
                                    if let Err(e) = locked_terminal.refresh_display() {
                                        error!("{}", e);
                                    }
                                    return Poll::Ready(Some(c as u8));
                                },
                                _ => {      // early exit
                                    error!("Couldn't get key event");
                                    return Poll::Ready(None);
                                },
                            }
                        }
                        else {
                            continue;           // other than pressing a key
                        }
                    },
                    _ => return Poll::Pending,  // no key event
                }
            }
        }
        else {      // early exit
            error!("failed to take key event reader");
            return Poll::Ready(None);
        }
    }
}

async fn async_read(mut key_event_stream: KeyEventStream) {
    let mut message = String::new();
    while let Some(c) = key_event_stream.next().await {
        message.push(c as char);
    }
    println!("message = {}", message);
}

fn run() -> Result<(), &'static str> {
    {
        // Acquire key event queue read guard.
        let key_event_queue = app_io::take_key_event_queue()?;

        // Get a reference to this task's terminal. The terminal is *not* locked here.
        if let Some(terminal) = app_io::get_my_terminal() {
            let key_event_stream = KeyEventStream::new(10, key_event_queue, terminal);  // key event stream for async read
            println!("Ready to call async_read.");
            let mut executor = AsyncExecutor::new();
            let task = AsyncTask::new(async_read(key_event_stream));
            executor.spawn(task);

            println!("Ready to run executor.");
            executor.run();     // current thread is used for executor, so blocking the following code
        }
        else {
            return Err("couldn't get terminal for `keyboard_async` app");
        }
    }

    println!("key_event_queue is returned and try to read from stdio.");
    let mut stdin = app_io::stdin()?;
    let mut message = String::new();
    let _cnt = stdin.read_line(&mut message);
    println!("{}", message);
    Ok(())
}

pub fn main() -> isize {
    if let Err(e) = run() {
        error!("{}", e);
        return 1;
    }
    return 0;
}
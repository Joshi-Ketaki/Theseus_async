

#![no_std]

extern crate alloc;
extern crate task;
extern crate keycodes_ascii;
extern crate app_io;
extern crate stdio;
extern crate bare_io;
#[macro_use] extern crate log;

use keycodes_ascii::{Keycode, KeyAction};
use core::str;
use alloc::{
    string::{String, ToString},
};

use stdio::{KeyEventQueueReader};

/// TODO : ** Make the read here async. **
/// So this will be async key_event_handler
fn key_event_handler_loop(key_event_queue: &KeyEventQueueReader)
                      -> Result<(), &'static str> {

    // Get a reference to this task's terminal. The terminal is *not* locked here.
    let terminal = app_io::get_my_terminal().ok_or("couldn't get terminal for `keyboard_async` app")?;
    
    // Handle user keyboard strikes.
    loop {
        // TODO: maybe the way to make this async is to have a read here which basically polls the queue
        // So instead of read_one , something like read_any ? And whichever is ready, passes through the
        // rest of the code.
        match key_event_queue.read_one() {
            Some(keyevent) => {
                if keyevent.action != KeyAction::Pressed { continue; }
                match keyevent.keycode {
                    // Quit the program on "Q".
                    Keycode::Q => {
                        let mut locked_terminal = terminal.lock();
                        locked_terminal.clear();
                        return locked_terminal.refresh_display();
                    },
                    //Print the typed charachter on screen
                    // FIXME: The Num1 keycode has just been used at the moment to get print to scrren working
                    // FIX : Fix treating of ascii charachters as done in handle_key_event.
                    Keycode::Num1 => {
                        let mut locked_terminal = terminal.lock();
                        locked_terminal.clear();
                        locked_terminal.print_to_terminal(
                           "hi I am in async_keyboard".to_string()
                        );
                        return locked_terminal.refresh_display();
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    }
}


pub fn main() -> isize {

    if let Err(e) = run() {
        error!("{}", e);
        return 1;
    }
    return 0;
}

fn run() -> Result<(), String> {

    // Acquire key event queue.
    let key_event_queue = app_io::take_key_event_queue()?;
    let key_event_queue = (*key_event_queue).as_ref()
                          .ok_or("failed to take key event reader")?;

    // Process the key which has been hit asynchronously here 
    Ok(key_event_handler_loop(key_event_queue)?)
}




extern crate alloc;

use super::AsyncTask;
use alloc::collections::VecDeque;
use core::task::{Waker, RawWaker};
use core::task::RawWakerVTable;
use core::task::{Context, Poll};

pub struct AsyncExecutor {
    async_tasklist: VecDeque<AsyncTask>,
}

// init a new tasklist.
//TODO : think of restricting queue size later, if do not want infinite async requests
impl AsyncExecutor {
    pub fn new() -> AsyncExecutor {
        AsyncExecutor {
            async_tasklist: VecDeque::new(),
        }
    }

    // push the async task in the queue when spawned
    pub fn spawn(&mut self, task: AsyncTask) {
        self.async_tasklist.push_back(task)
    }
}

// Implementing the waker here:

fn async_waker() -> Waker {
    unsafe { Waker::from_raw(async_rawwaker()) }
}
fn async_rawwaker() -> RawWaker {
    //TODO : We need to figure out what functions should be called for the keyboard when 
    // Waker is invoked. Right now adding this dummy functions as suggested in the blog 
    // to chekc our basic async support inside of theseus
    fn no_op(_: *const ()) {}
    fn clone(_: *const ()) -> RawWaker {
        async_rawwaker()
    }

    let vtable = &RawWakerVTable::new(clone, no_op, no_op, no_op);
    RawWaker::new(0 as *const (), vtable)
}

impl AsyncExecutor {
    pub fn run(&mut self) {
        while let Some(mut task) = self.async_tasklist.pop_front() {
            let waker = async_waker();
            let mut context = Context::from_waker(&waker);
            match task.poll(&mut context) {
                Poll::Ready(()) => {}   // task is done and has been popped already. 
                Poll::Pending => {      // explicit push again
                    self.async_tasklist.push_back(task);
                },
            }
        }
    }
}
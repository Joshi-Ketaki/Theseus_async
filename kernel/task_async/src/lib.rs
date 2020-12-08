#![no_std]

extern crate alloc;
extern crate memory;
extern crate x86_64;

use alloc::boxed::Box;
use core::{future::Future, pin::Pin};
use core::task::{Context, Poll};

pub struct AsyncTask {
    // skipping name and id fields. not needed.
    future: Pin<Box<dyn Future<Output = ()>>>,
}

// pin future
impl AsyncTask {
    pub fn new(future: impl Future<Output = ()> + 'static) -> AsyncTask {
        AsyncTask {
            future: Box::pin(future),
        }
    }
}

// poll function 
impl AsyncTask {
    fn poll(&mut self, context: &mut Context) -> Poll<()> {
        self.future.as_mut().poll(context)
    }
}

pub mod async_executor;
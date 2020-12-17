# Asynchronous IO in Theseus : 

Theseus : https://github.com/theseus-os/Theseus is an experimental OS written in Rust to leverage the intralinguality of Rust and designed to shift the OS responsibilities onto the compiler.
We exploit this intralinguality power of Rust to reason about a non-intuitive paradigm - Asynchronous IO. We believe that shifting most of the responsibilities to the compiler can help 
This is an experimental branch of Theseus where we introduce first-of-its-kind asynchronous IO framework in Theseus.
This is a first step in providing asynchronous support in Theseus and is in its nascent stage. It now has a grounding inside Theseus based on which newer asynchronous IO could be added.

# Design details :

# Setting up Theseus :

Information about building, setting up and debugging Theseus and its dependencies still remains the same as mentioned here : https://github.com/theseus-os/Theseus.

# File structure (Files added/ Modeified): 
1. Asynchronous Keyboard crate [applications/async_keyboard] : Asynchronous Keyboard IO test application
1. Asynchronous test crate [applications/test_async] : Asynchrnous framework test
1. Asynchronous Task crate [kernel/task_async] : Add framework for an asynchrnous task in Theseus
1. Stdio Crate (Modified) : Add waker support in the stdio crate for asynchronous IO.

# Running the code :
After you do 'make run' to run Theseus (steps here : https://github.com/theseus-os/Theseus. ), you can run the following applications to learn about Theseus's asynchronous 
IO support.

Applications:
async_keyboard : Run 'async_keyboard' on the QEMU shell
It will expect you to key-input certain keys which will be displayed after a delay (asynchronous processing) highlighting that it is being processed in the background and 
could be overlapped with other work.

test_async : This is the 'hello world' test fopr async framework in Theseus. 

# Limitations and future improvements :
The current executor keeps on polling and hence takes some time to print out the asynchronous operation result. Due to lack of time, this version does not have that capability. This is the next logical step to the current implementation and the solution is to make the executor an asynchronous executor so that it f=does not do busy polling.

## Acknowledgements
We would like acknowledge Philipp Oppermann's [blog_os](https://os.phil-opp.com/). It is a great starting point in understanding building of an OS in Rust and asynchronous support in Theseus.



# License
Theseus's source code is licensed under the MIT License. See the LICENSE-MIT file for more. 


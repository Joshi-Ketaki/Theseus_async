# Asynchronous IO in Theseus : 

Theseus : https://github.com/theseus-os/Theseus is an experimental OS written in Rust to leverage the intralinguality of Rust and designed to shift the OS responsibilities onto the compiler.
We exploit this intralinguality power of Rust to reason about a non-intuitive paradigm - Asynchronous IO. 
This is an experimental branch of Theseus where we introduce first-of-its-kind asynchronous IO framework in Theseus.
This is a first step in providing asynchronous support in Theseus and is in its nascent stage. It now has a grounding inside Theseus based on which newer asynchronous IO could be added.

# Benefits of Asynchronous IO:
IO operations are usually blocking. If overlapped with other work, it is beneficial to increase system utilization. Also multiple IO operations can proceed in the background with other operations happening in the foreground and hence increasing the system's throughput. Hence asynchronous IO not only improves CPU utilization but also improves system throughput. However, asynchrony is non-intutive interms of programmability, maintainence and debugging.
## Challenges in Asynchrony:
Asynchrony is a non-intuitive programming model to reason about. It is difficult to predict control akin to an event state machine a programmer builds. This is because there is no local control as to when an asynchronous operation will return. This brings with itself many scheduling and task priority ordering challenges.

## Why Rust ?
We leverage Rust's intralingual power to shift the hard-to-reason about parts of asynchrony such as task ordering, priorities, signalling task completion, maintaining local ordering etc to improve programmability, maintainence and debugging while providing the power of asynchronous IO.

# Design details :
We choose a clean approach which does not disturb existing robust and tried synchronous paths. The benefit is that existing scheduler and task structures and their dependencies remain untouched, leaving the OS in an operable robust fallback state always. Also, the existing scheduler can be leveraged as is for the new asynchronous task structure and framework. 

# Benefits of current support :
Foundation for Async IO in Theseus : This is afirst of its kind support and is far from being perfect. This version mainly sets up the asynchronous framework in Theseus and enables one of the IO devices (keyboard) in leveraging this support.

Ease of programmability - Providing asynchrony with Rust primitives enabled us to leverage async IO without worrying about changing the scheduler or managing returns from an async call for managing data consistency across tasks, managing task priorities , notifiying task completions in lieu of concurrency etc. Rust async primitives such as waker's register and atomic waker act as synchronization primitives and push these responsibilities to the compiler. This made async support addition much better in terms of programmability as opposed to doing this in a C based OS where onus of data consistency, priority management etc falls on the programmer.


# Limitations and Future Improvements :

Performance : The current executor keeps on polling and hence takes some time to print out the asynchronous operation result. Due to lack of time, this version does not have that capability. This is the next logical step to the current implementation and the solution is to make the executor an asynchronous executor so that it does not do busy polling.
# Setting up Theseus :

Information about building, setting up and debugging Theseus and its dependencies still remains the same as mentioned here : https://github.com/theseus-os/Theseus.

# File structure (Files added/ Modified): 
1. Asynchronous Keyboard crate [applications/async_keyboard] : Asynchronous Keyboard IO test application
1. Asynchronous Test crate [applications/test_async] : Asynchrnous framework test
1. Asynchronous Task crate [kernel/task_async] : Add framework for an asynchrnous task in Theseus
1. Stdio Crate (Modified) : Add waker support in the stdio crate for asynchronous IO.

# Running the code :
After you do 'make run' to run Theseus (steps here : https://github.com/theseus-os/Theseus. ), you can run the following applications to learn about Theseus's asynchronous 
IO support.

## Applications:
async_keyboard : Run 'async_keyboard' on the QEMU shell
It will expect you to key-input certain keys which will be displayed after a delay (asynchronous processing) highlighting that it is being processed in the background and 
could be overlapped with other work.

test_async : This is the 'hello world' test for async framework in Theseus. 



## Acknowledgements
We would like acknowledge existing Theseus framework and Kevin Boos for his help in bouncing off ideas and discussions. Also, Philipp Oppermann's [blog_os](https://os.phil-opp.com/). It is a great starting point in understanding building of an OS in Rust and asynchronous support in Theseus.

# References:
[1] Writing an OS in Rust [Writing an OS in Rust (phil-opp.com)]
[2] Getting Started - Asynchronous Programming in Rust (rust-lang.github.io)



# License
Theseus's source code is licensed under the MIT License. See the LICENSE-MIT file for more. 


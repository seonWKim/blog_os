- To write an OS, we need code that doesn't depend on any OS features
    - e.g. threads, files, heap memory, network, random numbers, standard output that requires any other
      features requiring OS abstractions or specific hardware
- What can we benefit from Rust when writing OS?
    - Surely, not the OS related features but, we still have
        - iterators
        - closures
        - pattern matching
        - option, result
        - string formatting
        - ownership
    - We need "freestanding" or "bare-metal" executable which doesn't depend on the underlying OS
- By default, Rust crates link the standard lib, which depends on the OS for features such as
    - threads, files, or networking
    - `libc`, which is the C standard library, which closely interacts with OS services

- Language item in Rust
    - A special function, type, or trait that the compiler needs to know about in order to generate code
      correctly.

- Stack unwinding
    - a process of cleaning up the stack when a program encounters an error or exception
    - How it works
        1. Traverses the call stack from the point of the error back to the entry point of the program
        2. Executes any necessary cleanup code, such as destructors for objects that go out of scope
        3. Releases resources that were allocated during the execution of the program
    - In Rust, stack unwinding is closely related to `panic!` macro, which triggers a panic and starts the
      unwinding process.
    - This ensures that all used memory is freed and allows the parent thread to catch the panic and continue
      execution.
        - But it requires some OS specific libraries e.g. `libunwind`

- Start attribute
    - Actually, `main` is not the actual starting point of programs, `runtime system` must be executed before
      the main
        - e.g. `Java` need to execute it's runtime before `main` for it's GC and etc
    - Typical rust binary that links the std lib, execution starts in a C runtime library called `crt0`(C
      runtime zero), which sets up the environment for a C application. The C runtime then invokes the entry
      point of the Rust runtime, which is marked by the `start` language item

- Linker 
  - a program that combines the generated code into an executable 

- Rust target triple 
  - A string that specifies the target architecture, vendor, OS and environment for which the code is being compiled 
  - Used by the rust compiler to generate code that is compatible with the specified target 
  - e.g. `x86_64-unknown-linux-gnu`

- Rust's cross compile 
  - The process of compiling code on one platform to run on another platform 
    - useful when the target platform is different from the host platform 
      - e.g. compiling code on the MacOS for Linux or Windows 
    - 

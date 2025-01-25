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
    - A string that specifies the target architecture, vendor, OS and environment for which the code is being
      compiled
    - Used by the rust compiler to generate code that is compatible with the specified target
    - e.g. `x86_64-unknown-linux-gnu`

- Rust's cross compile
    - The process of compiling code on one platform to run on another platform
        - useful when the target platform is different from the host platform
            - e.g. compiling code on the MacOS for Linux or Windows
        -

- The Boot process
    - When you turn on your computer
        1. Executes firmware code that is stored in motherboard ROM
            - Performs a power-on self-test, and detect available RAM, and pre-initializes the CPU and hardware
        2. Looks for a bootable disk and starts booting the OS
    - Firmware standards
        1. BIOS(Basic Input/Output System)
            - old, outdated but it's simple and well supported on any x86 machine
        2. UEFI(Unified Extensible Firmware Interface)
            - modern and has much more features, but is more complex to set up
    - How BIOS Boot works
        1. Turn on your computer
        2. Loads the BIOS from some special flash memory located on the motherboard
            - BIOS runs self-test and initialization routines of the hardware
            - Looks for bootable disks -> Pass control to the bootloader if found
        3. Bootloader
            - Bootloader is a 512-byte portion of executable code stored at the disk's beginning
            - Most bootloaders are larger than 512 bytes, so they are commonly split into a small first page,
              which fits into 512 bytes, and a second stage, which is subsequently loaded by the first page
            - Determines the location of the kernel image on the disk and load it into memory
            - Switches the CPU from the 16-bit real mode to the 32-bit protected mode, and then to the 64-bit
              long mode(64-bit registers and the complete main memory are available)
    - Real mode
        - operating on a 16-bit environment, allowing access to only 1MB of memory
        - no memory protection or multitasking; all programs can access any part of the memory
    - Protected mode
        - allows access to more than 1MB of memory
        - supports memory protection, paging and hardware-level multitasking
        - each program runs in its own protected memory space, preventing them from interfering with each other
    - Long mode
        - allows access to a 64-bit address space, enabling the use of more than 4GB of RAM
        - includes all the features of protected mode but extends them to a 64 bit
    - Multiboot standard
        - No more custom bootloaders -> open standard
        - defines an interface between the bootloader and the OS
        - Any MULTIBOOT-COMPLIANT bootloaders can load MULTIBOOT-COMPATIBLE OS

- Rust release channels
    - Nightly: experimental new features
        - each night, a new nightly version of Rust is produced
    - Beta: experimental new features
        - every six weeks, `beta` branch of the Rust repository branches off from the `master` branch used by
          nightly
        - `beta` branch branches off from the nightly branch
    - Stable: mostly used
        - six weeks after the first `beta` was created, it's time for a stable release. `stable` branch is
          produced from the `beta` branch
    - Unstable features
        - rust uses a technique called `feature flags` to determine what features are enabled in a given release
        - if you wish to use work-in-progress feature, you can, but you have to use a nightly release of Rust
          and annotate your source code with the appropriate flag to opt in
    - By default, you will have stable Rust installed
        - you can override the rust settings by using `rustup override set nightly` ->
        - check which version of rust is being used by `rustup -V`
    - By the way, what is `rustup`?
        - toolchain installer for the Rust language
        - features
            - installing rust: install the latest stable, beta or nightly versions of rust
            - toolchain management: switch between different versions of rust toolchains
            - component management: install additional components like `rustfmt`, `clippy` and more
            - cross-compilation: add and manage target platforms for cross-compiling

- Target specification
    - target triple: `CPU architecture` + `vendor` + `ABI`
        - e.g. `arm-linux-androidabi`, `wasm32-unknown-unknown`
    - Rust allows us to define our own target through a JSON file
        - Information that are required by the LLVM to generate code for that platform

    - `mmx`, `sse` features support SIMD(Single Instruction Multiple Data) instructions
        - using large SIMD registers in OS kernels leads to performance problems
        - the reason is that the kernel needs to restore all registers to their original state before continuing
          an interrupted program
            - kernel has to save the complete SIMD state in main memory on each system call or hardware
              interrupt
            - SIMD state is commonly very large (512 ~ 1600 bytes)
        - `x86_64`require SIMD registers by default, to solve this problem, we add `soft-float` feature, which
          emulates all floating point operations through software functions based on normal integers 
        - 

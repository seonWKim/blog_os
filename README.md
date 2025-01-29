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

- Printing to Screen
    - the easiest way to print text to the screen is the VGA text buffer
    - it's a special memory area mapped to the VGA hardware that contains the contents displayed on the screen

- Rust `unsafe` allows
    1. Dereferencing raw pointers
    2. Calling unsafe functions or methods
    3. Accessing or modifying mutable static variables
    4. Implement unsafe traits
    5. Accessing union fields
- Rust's `Union`
    - a data structure that can store different types in the same memory location
    - allows you to define multiple fields but only one of them can be used at a time
    - accessing the fields of a union requires an `unsafe` block because it's up to the programmer to ensure
      that the correct field is accessed, as the rust compiler can't guarantee the safety of such operation

- VGA
    - VGA text buffer is a two-dimensional array with typically 25 rows and 80 columns, which is directly
      rendered to the screen
    - VGA text buffer is accessible via memory-mapped I/O to the address `0xb8000`
        - this means that reads and writes to that address don't access the RAM but directly access the text
          buffer on the VGA hardware

- Memory Mapped I/O
    - A method used to perform input/output operations between the CPU and peripheral devices by mapping device
      registers into the same address space as the program memory
        - Device registers
            - Special memory locations used by the CPU to control and communicate with hardware devices
            - Registers are typically mapped to the system's address space, allowing the CPU to read from and
              write to them using standard memory instructions
            - Each register usually has a specific function eg. controlling the device, reporting its status, or
              transferring data
                - Control: used to send commands to the device
                - Status: used to read the current status of the device
                - Data: used to transfer data to and from the device
    - Allows the CPU to interact with hardware devices using the standard memory instructions(e.g. `load`,
      `store`), treating device drivers as if they were like regular memory locations
- Port Mapped I/O
    - A method used to perform input/output operations between the CPU and peripheral devices by using a
      separate address space for I/O operations

- `repr(u8)`
    - specifies that the Rust enum should be represented as an 8-bit unsigned integer
    - by default, Rust enums are represented using the smallest integer type that can fit all of their variants.
      This means that the size of the enum depends on the number of variants it has
        - e.g. 1 ~ 256 variants -> u8, 257 ~ 65,536 variants -> u16 ...
- `repr(transparent)`
    - an attribute in rust used to specify that a struct should have the same memory layout as its single
      non-zero-sized field
    - useful for creating newtypes that are guaranteed to have the same representation as the underlying type(
      which can be important for FFI and other low-level programming tasks)
    - ```rust
      #[repr(transparent)]

        struct Wrapper(u32);

        fn main() {
        let x = Wrapper(42);
        let y: u32 = unsafe { std::mem::transmute(x) };
        println!("{}", y); // Outputs: 42
        }
      ```
        - the `Wrapper` has the same memory layout as the u32, allowing safe transmutation between the two types
        - `transmute` allows you to reinterpret the bits of a value as another type
- `repr(C)`
    - specifies that a struct or enum should use the C programming language's memory layout
    - important when interoperability with C code and for ensuring a predictable memory layout, which can be
      crucial for low-level programming tasks
        - by default, Rust's struct's field ordering is undefined

- Derive macros
    - `#[derive(Debug, Clone, Copy, PartialEq, Eq)]`
    - Called derive macros because it automatically generates implementations of certain traits for the
      annotated type
    - Examples
        - Debug
            - allows for formatting a value using the `{:?` formatter
        - Clone
            - deep copy
        - Copy
            - shallow copy(copying bits)
        - PartialEq
            - allows for comparison of values using `==` and `!=`
            - can only be derived if all fields of the type implements `PartialEq`
            - does not require the full properties of a `total equivalence` relation
                - e.g. floating point numbers implement `PartialEq` but not `Eq` because they can represent
                  `NaN` values, which are not equal to themselves, violating the reflexive property
        - Eq
            - indicates a type has a total equivalence relation
                - total equivalence should satisfy below conditions
                    - reflexive: x == x
                    - symmetric: if (x == y) then y == x
                    - transitive: if (x == y && y == z) then x == z
            - can only be derived if all fields of the type implements `Eq`
            - requires `PartialEq` to be implemented

- ASCII, Utf-8, Utf-16
    - ASCII
        - 7-bit character encoding standard(128)
        - each character is represented by a single byte, with the most significant bit always set to 0
    - UTF-8(Unicode Transformation Format - 8 bit)
        - A variable length character encoding for unicode
        - Able to represent every character in the Unicode character set, which includes characters from almost
          all written languages
        - UTF-8 is backward compatible with ASCII, meaning any valid ASCII text is also valid UTF-8 text
        - How encoding works
            - 1 byte for ASCII characters
            - 2 ~ 4 bytes for other characters
    - UTF-16(Unicode Transformation Format - 16 bit)
        - How encoding works
            - 1 code unit(2 bytes) for characters in the BMP(Basic Multilingual Plane), which includes most
              common characters
            - 2 code unit(4 bytes) for characters outside the BMP, known as supplementary characters
        - Not backward compatible with ASCII

- `volatile` crate
    - Used to ensure that reads and writes to a memory location are not optimized away by the compiler
    - Reasons for using it
        - Prevent compiler optimization
            - compiler might optimize away reads and writes to memory locations that it deems unnecessary
        - Memory Mapped I/O
            - ensuring that every read and writes are performed
        - Concurrency
            - ensuring that changes are visible to different threads or interrupt handlers

- `static`s in rust
    - Are initialized at compile time, in contrast to normal variables that are initialized at runtime
        - Rust's `const evaluator` evaluates such initialization expression
    - `lazy_static` to the rescue
        - instead of computing its value at compiler time, the `static` lazily initializes itself when accessed
          for the first time, so that the initialization happens at runtime

- `write!` macro
    - used for formatted output, but it writes to a specified writer instead of standard output

- conditional compilation
    - `#[cfg]`
    - allows you to compile code based on a flag passed to the compiler

- I/O ports
    - How does CPU communicate with peripherals?
        - Memory Mapped I/O
            - e.g. VGA Text Buffer
        - Port Mapped I/O
            - Uses a separate I/O bus for communication
            - Each connected peripheral has one or more port numbers
            - There are special CPU instructions called `in` and `out`, which take a port number and a data byte

- Serial ports
    - A communication interface through which data is transferred one bit at a time over a single communication
      line
    - Commonly used for communication between computers and peripheral devices

- Integration test in Rust
    - putting integration test into tests directory is convention in rust
    - all integration tests are their own executables and completely separate from `main.rs`

- `main.rs` and `lib.rs` in rust
    - `main.rs`
        - defines the entry point for a binary crate
        - used to create executable programs
        - contains `main` function
    - `lib.rs`
        - defines the entry point for a library crate
        - used to create reusable libraries that can be included in other projects

- test `harness` option
    - definition: a set of straps and fittings by which a horse or other draft animal is fastened to a cart,
      plow, etc
    - specifies whether to use the default test harness for a test
        - when set to `false`, the test will not use the default test harness provided by rust

    - CPU Exceptions(by CPU interrupts)
        - Classification
            - Faults: can be corrected and the program may continue as if nothing happened
                - Division error
                - Bound range exceeded
                - Invalid Opcode
                - ...
            - Traps: reported immediately after the execution of the trapping instruction
                - Debug
                - Breakpoint
                - Overflow
            - Aborts: severe unrecoverable error
                - Double fault
        - When an error occurs:
            - CPU interrupts its current work and immediately calls a specific exception handler function
        - Most important exceptions
            - page fault
                - occurs on illegal memory access
                - e.g. if the current instruction tries to read from an unmapped page or tries to write to a
                  read-only age
            - invalid opcode
                - occurs when the current instruction is invalid
            - general protection fault
                - occurs on various kinds of access violations
                - e.g. trying to execute privileged instruction in user-level code or writing reserved fields in
                  configuration registers
            - double fault
                - if another exception occurs while calling the exception handler, or when there is no handler
                  function registered for an exception
            - triple fault
                - if an exception occurs while the CPU tries to call the double fault handler function
                - if we can't catch or handle a triple fault, most processors react by resetting themselves and
                  rebooting the OS
        - IDT(Interrupt Descriptor Table)
            - specifies handler functions for each exception
        - How exceptions are handled
            1. Push some registers on the stack, including the instruction pointer and the RFLAGS register
            2. Read the corresponding entry from the IDT(Interrupt Descriptor Table)
            3. Check if the entry is present and, if not, raise a double fault
            4. Disable hardware interrupts if the entry is an interrupt gate
            5. Load the specified GDT selector in the CS(code segment)
            6. Jump to the specified handler function
        - IDT Type
            - ```rust
              #[repr(C)]
              pub struct InterruptDescriptorTable {
              pub divide_by_zero: Entry<HandlerFunc>,
              pub debug: Entry<HandlerFunc>,
              pub non_maskable_interrupt: Entry<HandlerFunc>,
              pub breakpoint: Entry<HandlerFunc>,
              pub overflow: Entry<HandlerFunc>,
              pub bound_range_exceeded: Entry<HandlerFunc>,
              pub invalid_opcode: Entry<HandlerFunc>,
              pub device_not_available: Entry<HandlerFunc>,
              pub double_fault: Entry<HandlerFuncWithErrCode>,
              pub invalid_tss: Entry<HandlerFuncWithErrCode>,
              pub segment_not_present: Entry<HandlerFuncWithErrCode>,
              pub stack_segment_fault: Entry<HandlerFuncWithErrCode>,
              pub general_protection_fault: Entry<HandlerFuncWithErrCode>,
              pub page_fault: Entry<PageFaultHandlerFunc>,
              pub x87_floating_point: Entry<HandlerFunc>,
              pub alignment_check: Entry<HandlerFuncWithErrCode>,
              pub machine_check: Entry<HandlerFunc>,
              pub simd_floating_point: Entry<HandlerFunc>,
              pub virtualization: Entry<HandlerFunc>,
              pub security_exception: Entry<HandlerFuncWithErrCode>,
              // some fields omitted
          }        
          // ...

               type HandlerFunc = extern "x86-interrupt" fn(_: InterruptStackFrame);```
                ```

            - `extern`
                - defines a function with a foreign calling convention which is often used to communicate with C
                  code
            - foreign calling convention
                - Most foreign code exposes a C ABI, and Rust uses the platform's C calling convention by
                  default
                - calling conventions specify the details of a function call
                    - e.g. specifies where function parameters are placed(e.g. in registers or on the stack) and
                      how
                      results are returned
                    - e.g. x864_64_Linux, the following rules apply for C functions(specified in the System V
                      ABI)
                        - the first 6 integer arguments are passed in registers: `rdi`, `rsi`, `rdx`, `rcx`,
                          `r8`, `r9`
                        - additional arguments are passed on the stack
                        - results are returned in `rax` and `rdx`
        - Preserved and Scratch registers
            - calling convention divides the registers into 2 parts: preserved / scratch registers
            - Preserved registers(callee-saved)
                - Preserved registers must remain unchanged across function calls
                - A called function(callee) is ONLY ALLOWED to overwrite these registers if it restores their
                  original value before returning
                - a common pattern is to save these registers to the stack at the function's beginning and
                  restore them just before returning
            - Scratch registers(caller-saved)
                - called function is allowed to overwrite scratch registers without restrictions
                - if the caller wants to preserve the value of a scratch register across a function call, it
                  needs to backup and restore it before the function call(e.g. by pushing it to the stack)
            - x86_64 example
                - preserved registers: `rbp`, `rbx`, `rsp`, `r12`, `r13`, `r14`, `r15`
                - scratch registers: `rax`, `rcx`, `rdx`, `rsi`, `rdi`, `r8`, `r9`, `r10`, `r11`
        - Exception and registers
            - Since we don't know when an exception occurs, we can't backup any registers before
            - We need a calling convention that preserves all registers => `x86-interrupt`
                - note that this doesn't mean all registers are saved to the stack at function entry. Instead,
                  the compiler only backs up the registers that are overwritten by the function
        - Interrupt Stack Frame
            - How normal function call works
                1. Caller function calls(`call` instruction) the callee function
                2. Push return address to the stack
                3. Callee function get executed
                4. On return(`ret` instruction), CPU pops the return address and jumps to it
            - Exceptions and interrupt handlers are different
                1. Save the old stack pointer: CPU reads the stack pointer(`rsp`) and stack segment(`ss`)
                   register values and remembers them in an internal buffer
                2. Aligning the stack pointer: An interrupt can occur at any instruction, so the stack pointer
                   can have any value. However, some CPU instructions require that the stack pointer be aligned
                   on a 16-byte boundary, so the CPU performs such an alignment right after the interrupt
                3. Switching stacks: occurs when the CPU privilege level changes(e.g. when a CPU occurs in user
                   mode)
                4. Push the old stack pointer: pushes the `rsp` and `ss` values from step 1 to the stack. This
                   makes it possible to restore the original stack pointer when returning from an interrupt
                   handler
                5. Pushing and updating the `RFLAGS` register: contains various control and status bits. On
                   interrupt entry, the CPU changes some bits and pushes the old value
                6. Pushing the instruction pointer: before jumping to the interrupt handler, the CPU pushes the
                   instruction pointer(`rip`) and the code segment(`cs`). This is comparable to the return
                   address push of a normal function call
                7. Pushing an error code: for some specific exceptions, such as page faults, the CPU pushes an
                   error code
                8. Invoking the interrupt handler: reads the address and the segment descriptor of the interrupt
                   handler function from the corresponding field in the IDT. It then invokes this handler by
                   loading the values into the `rip` and `cs` registers

- Implementing CPU exceptions(breakpoint exception)
    - Breakpoint exception is commonly used in debuggers
    - When the user sets a breakpoint, the debugger overwrites the corresponding instruction with the `int3`
      instruction so that the CPU throws the breakpoint exception when it reaches that line
    - When the user continues the program, the debugger replaces the `int3` instruction with the original
      instruction again and continues the program

- Double fault
    - similar to programming language's `try...catch(Exception e)`
    - If double fault is unhandled, a fatal triple fault occurs which can't be caught and most hardware reacts
      with a system reset
    - Double fault exception CAN occur when a second exception occurs during the handling of a prior exception
      handler
        - only very specific combinations of exceptions can lead to a double fault
- Kernel stack overflow
    - when kernel overflows its stack, it hits the guard page
    - guard page
        - a special memory page at the bottom of a stack that makes it possible to detect stack overflows
    - bootloader sets up a guard page for the kernel stack, so a stack overflow causes a page fault

- Diverging functions
    - Functions that never return and marked with `-> !`
        - Different from the `()` type, which has exactly one possible value

- Switching Stacks
    - `x86_64` arch is able to switch to a predefined, known-good stack when an exception occurs
        - this switch happens at the hardware level, so it can perform before the CPU pushes the exception stack
          frame
    - IST(Interrupt Stack Table)
        - table of 7 pointers to known-good stacks
        - for each exception handler, we can choose a stack from the IST

- TSS(Task State Segment)
    - A special data structure used by the x86 architecture to store information about task
        - e.g. state of the CPU registers, stack pointers and segment selectors
    - Primarily used for hardware task switching and handling interrupts that require switching to a different
      task
- GDT(Global Descriptor Table)
    - A data structure used by the x86 architecture to define the characteristics of the various memory segments
      used in the system
    - Each entry(segment descriptor) specifies the base address, size and access privileges of a segment

- (Memory) Segmentation system
    - A memory management schema that divides the memory into different segments
    - Concepts
        1. Segment: a contiguous block of memory with a specific starting address and length
        2. Segment Descriptor: contains information about a segment, such as its base address, limit(size) and
           access rights
        3. Segment Selector: a value that identifies a segment descriptor in the GDT(Global Descriptor Table) or
           LDT(Local Descriptor Table)
    - Not used these days, instead modern systems use a flat memory model with paging for memory management,
      which simplifies memory access and provides better support for features like virtual memory and memory
      protection

- Interrupts
    - Provides a way to notify the CPU from attached hardware devices(kernel doesn't have to poll)
    - Connecting all hardware devices directly to CPU isn't possible
        - Instead, a separate interrupt controller aggregates the interrupts from all devices and then notifies
          the CPU
    - Interrupt controllers are programmable
        - Priority and etc
    - Hardware interrupts occur ASYNCHRONOUSLY
        - completely independent of the executed code and can occur at any time
    - Each controller can be configured through 2 I/O ports
        - Command port: used to send commands to PIC 
        - Data port: used to send or receive data from PIC 
- EOI(End of Interrupt) signal
    - This signal tells the controller that the interrupt was processed and that the system is ready to receive
      the next interrupt
    - With no EOI, PIC thinks we're still busy processing the first timer interrupt and waits patiently for the
      EOI signal before sending the next one 
- PIT(Programmable Interval Timer)
  - The hardware timer we are using 
  - It's possible to configure the interval between 2 interrupts 
- 

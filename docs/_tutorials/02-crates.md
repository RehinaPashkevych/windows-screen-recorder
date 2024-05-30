---
layout: tutorial
title: "02: Crates"
---

In this chapter, I will discuss the reasons behind my choice of these particular crates.

> A crate is synonymous with a ‘library’ or ‘package’ in other languages. Hence “Cargo” as the name of Rust’s package management tool: you ship your crates to others with Cargo. Crates can produce an executable or a library, depending on the project.
>
> Source: [Rust Documentation](https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/crates-and-modules.html)

## Architecture of the app

I had the idea to develop an app that can record my screen. Why? Windows doesn't automatically include this feature, and I'm tired of looking for a good free one. That's why I decided to make my own. What do we need for this tool?

 * GUI
 * the code which will do the main part of the task - ✨Recording✨

### GUI

Rust offers several graphical crates that are useful for creating interfaces. This app is one of my initial projects in Rust, so I haven't explored others yet. However, I have discovered some crates that might be useful for you or for my future projects:

 - [egui](https://github.com/emilk/egui)
 - [iced](https://iced.rs)
 - [tauri](https://tauri.app)
 - [slint](https://slint.dev)
 - [rinf](https://github.com/cunarist/rinf)
 - [gtk-rs](https://gtk-rs.org) 
 - [relm](https://relm4.org)
 - [dioxus](https://dioxuslabs.com)
 - [fltk](https://docs.rs/fltk/latest/fltk/)
 - [azul](https://azul.rs)

Considering the variety of options available, you can see how active and promising the Rust community is. For my project, I chose the first crate because I found the tutorials on their GitHub page very helpful.

### Recording

I am drawn to challenging tasks because they truly help shape and educate you. Indeed, it might have been easier to develop this app using .NET and C#, which offer straightforward access to Windows APIs and a rich framework. However, I faced two objectives: to use a functional language and to learn something new. While there is a Rust crate for Windows integration - [windows](https://crates.io/crates/windows), building the entire app from scratch seemed daunting, especially given the limited time I have for a routine school project. I considered experimenting with streaming, encoding, and decoding images using [FFmpeg](https://ffmpeg.org/about.html), but ultimately decided to explore existing resources. I discovered the [windows_capture](https://github.com/NiiightmareXD/windows-capture) crate, which suits my needs perfectly. For more options, you can explore other crates under [#screen-capture](https://lib.rs/keywords/screen-capture). The `windows_capture` crate builds upon the `windows` crate, offering robust error handling. From what I can see, the author is actively improving the project and addressing issues.

## Other important crates and definitions

Rust offer synchronous programming be default.

>Synchronous or sequential programming is when tasks occur separately or one after another. The program pauses while the system performs the action and only responds once it receives the result. It features a single-thread model, meaning that it only performs a single action at a time.
Asynchronous programming differs in that it allows multiple tasks to run at the same time, and the programmer can often manage these tasks directly. It allows programs to continue to run even after you start a specific action.
>
> Source: [What Is Asynchronous Programming? (And When To Use It)](https://www.indeed.com/career-advice/career-development/asynchronous-programming)

If you're a beginner, you might initially think that synchronous programming is suitable for this code, given that it involves sessions of recording the screen. However, that's not the case here. When we run the app, it includes a GUI that remains responsive. For example, when you click the `Start` button, the interface continues to wait for you to click `Stop`. If we were using a timer that blocked the program until it finished, that would be synchronous programming. But that's not our situation.

Therefore, we need to implement asynchronous programming to handle these tasks without blocking the user interface. The crate that helps us tackle this challenge is [`tokio`](https://docs.rs/tokio/latest/tokio/), which allows for efficient asynchronous operations in Rust. This means that while the recording is ongoing, the GUI can still receive and process user interactions like stopping the recording.


### Tokio

Setting up the Tokio runtime looks something like this:

`` let rt = tokio::runtime::Runtime::new().unwrap(); ``

Now we start to dig into the code.

*   [**`tokio::runtime::Runtime`**](https://docs.rs/tokio/latest/tokio/runtime/index.html):

    * We declare that we want to use Tokio Runtime component. We could do it using `use` statement too, like in 1-8 lines in the `main.rs`.
    *   **Tokio Runtime**: This is the heart of Tokio's asynchronous model. The `Runtime` contains all the infrastructure for executing asynchronous tasks, managing their lifecycles, and handling scheduling.
    *   **Namespace and Module**: `tokio::runtime` indicates that the [`Runtime`](https://docs.rs/tokio/latest/tokio/runtime/struct.Runtime.html) struct is being accessed from the `runtime` module of the Tokio crate.

*   **`new()`**:
    
    *   This is a method called on the `Runtime` struct that constructs a new instance of `Runtime`. It sets up everything needed for the runtime to execute asynchronous tasks, such as an executor and resources for handling I/O events.

*   **`unwrap()`**:
    
    *   Since `Runtime::new()` can potentially fail (the operation might return a `Result` type, depending on the Tokio version), `unwrap()` is used here to handle the `Result`. Using `unwrap()` will either return the `Runtime` object if the creation was successful or panic (terminate the application) if an error occurred during the initialization of the runtime.
    *   The use of `unwrap()` is common in examples or smaller programs but is generally discouraged in production code. In a real-world application, you should handle the error appropriately (for example, by using pattern matching or error propagation) to avoid panics.
    * The detailed explanation is on [Stackoverflow](https://stackoverflow.com/questions/36362020/what-is-unwrap-in-rust-and-what-is-it-used-for) or on [Youtube](https://www.google.com/search?client=opera&hs=81j&sca_esv=7978873cf7ff4aec&sca_upv=1&sxsrf=ADLYWILRXoQNpdI95EmHjncsS3GkKWbs-Q:1715088604992&q=unwrap+rust&tbm=vid&source=lnms&prmd=visnbz&sa=X&ved=2ahUKEwiL9rv40vuFAxXOPxAIHaRQDucQ0pQJegQIDhAB&biw=1482&bih=706&dpr=1.25#fpstate=ive&vld=cid:420c0b95,vid:-tv9wNrwmsk,st:0)

Once we have a Runtime, we can use it to execute async blocks of code. Here’s how we would use the runtime:

```rust
rt.block_on(async {
    // The code of our GUI, which will be part of the asynchronous programming
});
```
It will be used to run the main event loop of the GUI asynchronously, ensuring that the GUI remains responsive while handling other tasks in the background. Our application uses asynchronous operations to manage video encoding and other I/O without blocking the main thread, which is crucial for keeping the GUI responsive. block_on allows these operations to be integrated seamlessly by running an asynchronous block until it completes, while still pumping the GUI events and updates.


### once_cell

Other crate which was suggested by Rust compiler was `once_cell`. In Rust, you can't directly initialize a static variable with non-constant function calls due to the constraints of the Rust compile-time evaluation. The initializer for a static variable must be a constant expression or involve the direct use of a const function, which are currently limited in what they can do (as of my last training data). Rust's static variables require values that are known at compile time, unless wrapped in a synchronization primitive like Mutex<T> or using lazy initialization patterns like once_cell::Lazy.

I wanted to create a STATE variable for saving the setting of our app. I did it in this way:

```rust
static STATE: Arc<Mutex<GuiState>> = Arc::new(Mutex::new(GuiState::default()));
```

This line of code won't compile because Arc::new() and Mutex::new() are not const functions, and thus cannot be evaluated at compile time. The use of once_cell::Lazy in this code is a workaround for this limitation, as Lazy provides a way to delay initialization until runtime.

```rust
static STATE: Lazy<Arc<Mutex<GuiState>>> = Lazy::new(|| {
    Arc::new(Mutex::new(GuiState::default()))
});
```

Let`s clarify all things in this snippet. 

* `static`: This keyword in Rust is used to define a variable that is globally accessible and lives for the entire duration of the program. It's like a global constant in other languages but can be mutable.

* The `GuiState` is a struct designed to store necessary fields for recording the screen. It includes the following fields:

    *   `selected_encoder`: This field holds the identifier for the encoder chosen by the user to encode the screen recording.
    *   `selected_quality`: This field specifies the quality level of the recording, as selected by the user.
    *   `selected_path`: This field stores the file path where the recording will be saved.
    *   `recording_active`: A boolean field that indicates whether recording is currently active.
    *   `recording_start`: This field stores the timestamp or other relevant information marking the start of the recording.

* `Lazy<Arc<Mutex<GuiState>>> `

    * `Lazy`: This is a utility that ensures the initialization of the variable it wraps only happens once and only when it's actually needed (not when the program starts). It's useful for improving performance and avoiding unnecessary computations.
    * `Arc`: This stands for "Atomic Reference Counting". `Arc` is used to enable multiple parts of your program to own a piece of data and to access it concurrently, in a thread-safe way. This means multiple parts of the program can hold a reference to the GuiState and request access to it (by trying to lock the Mutex).
    *  `Mutex`: Mutex stands for "mutual exclusion". A `Mutex` allows only one thread to access some data at any one time, which helps prevent bugs that occur when multiple threads try to change the same data simultaneously. 

* `Lazy::new()`: This function creates a new Lazy instance. The new part is a common method in Rust used for creating new objects.

* `|| { ... }`: This is a Rust [closure](https://doc.rust-lang.org/book/ch13-01-closures.html) (similar to an anonymous function in JavaScript or a lambda in Python). It contains the code that initializes the STATE variable. The closure is only called the first time STATE is accessed, thanks to Lazy.

The continuation of the implementation for the "recording" code will be covered in the next chapter.

---
layout: tutorial
title: "03: Recording"
---

In the previous chapter I mentioned about GuiState structure, STATE variable. `Capture` struct and an implementation GraphicsCaptureApiHandleris given from an author [usage](https://github.com/NiiightmareXD/windows-capture/tree/main). A struct in Rust is similar to classes in other languages but primarily used for data storage. GraphicsCaptureApiHandler is an implementation of the trait for the Capture struct. 

> In Rust, traits and impl are fundamental concepts used to define and implement shared behavior across different data types.

> A **trait** in Rust is akin to an interface in other programming languages. It's a collection of methods that define behavior. However, unlike interfaces in some languages, traits can provide default method implementations. Traits allow you to specify that a certain type has particular behavior, or can do specific things. For example, the `std::fmt::Display` trait specifies that a type can be displayed (converted to a string in a human-readable format).

> The **`impl`** keyword is used to implement blocks in Rust, which can be either for implementing a trait for a specific type or for adding methods and associated functions directly to the type itself. When used with a trait, `impl` defines how the methods of that trait are implemented for a specific type. When used without a trait, it allows defining methods that are directly associated with a specific type.

I define the implementation `Default` for `Guistate` struct  to determine default variables if an user does not provide it.  The `Default` trait is a standard Rust trait used to create an instance of a type with default values. Implementing the Default trait for a type allows us to use the Default::default() function to get a new instance of that type, populated with default values. Here is short tutorial how implementations are created: 

```rust 
impl Default for GuiState {
    fn default() -> Self {
        Self {
            selected_encoder: VideoEncoderType::Mp4,
                //...
        }
    }
}
``` 

* **fn default() -> Self**: This is the signature of the default() function. Self refers to the type GuiState itself. `This` function must return an instance of GuiState.
 
* **Self { ... }**: Inside the function, you create a new instance of GuiState using the struct initializer syntax.

So when we created STATE we wrote 

```rust
Arc::new(Mutex::new(GuiState::default()))
```

This call generates a new GuiState using the default values you specified in the impl Default for GuiState. This demonstrates how the Default trait can simplify initialization patterns, especially when combined with other layers of functionality like Arc, Mutex, and Lazy.

## `GraphicsCaptureApiHandlers` Trait Implementation

### Error Handling

You can see the line 54:

```rust
type Error = Box<dyn std::error::Error + Send + Sync>;
```

It is the statement, which involves several key concepts from the language, specifically around error handling, trait objects, and thread safety. 

* `Box<dyn std::error::Error>`

    *   **`Box`**: This is a smart pointer in Rust. `Box` allocates data on the heap, which means the data it points to is not stored on the stack (where local variables typically live). This is useful for when you don't know the size of the data at compile time, or when you want to transfer ownership of some data without copying it, or when you deal with large data structures or recursive types.
        
    *   **`dyn`**: This keyword is used to define a "dynamic dispatch" trait object. It indicates that the specific type implementing the given trait will be determined at runtime. This is in contrast to "static dispatch," where the compiler knows the exact type at compile time.
        
    *   **`std::error::Error`**: This is a trait provided by Rust's standard library that types can implement to be used as "errors." It has methods that allow you to describe the error and trace its cause, among other things. Any type that implements this trait can be used as an error.

    * Combining these, Box<dyn std::error::Error> is a trait object that allows for any type that implements the Error trait to be boxed (heap allocated) and handled polymorphically. This means you can handle various kinds of errors through a unified interface.

*   **`Send`**: A marker trait that indicates that ownership of values of the type implementing this trait can be transferred between threads. This means you can safely move the boxed error across threads.
    
*   **`Sync`**: A marker trait that indicates that it is safe to reference values of this type from multiple threads concurrently. This means you can safely share a reference to the boxed error across multiple threads.

This type of error definition is commonly seen in Rust applications that require robust error handling across various modules and threads. It allows developers to write functions that can return virtually any error, provided it implements the std::error::Error trait, while still being thread-safe for applications that need such functionality.


### Methods


TODO
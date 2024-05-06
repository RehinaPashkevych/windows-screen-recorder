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

## Other important crates 

TODO:
write about tokio and  asynchronous programming, maybe add some RUST perspective

            once_cell: lazines at runtime because it is RUST

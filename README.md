#### Introduction

Screen Recorder is a simple tool for recording screen activities on Windows. Built with Rust, it supports various video formats and includes a customizable GUI for ease of use. Reach the tutorial for this app: https://rehinapashkevych.github.io/windows-screen-recorder/

#### Features

*   Record screen activities in multiple video formats (MP4, AVI, WMV, HEVC).
*   Simple GUI for starting and stopping recordings.
*   Custom filename formats including date and time.
*   Uses asynchronous Rust for efficient performance.

#### Prerequisites

To run Screen Recorder, ensure you have the following installed:

*   Rust and Cargo (latest stable version recommended)

#### Installation

1.  **Clone the repository:**
    
    `git clone https://github.com/yourgithubusername/screen-recorder.git cd screen-recorder`
    
2.  **Build the project:**
    
    `cargo build`
    

#### Usage

To start the Screen Recorder, navigate to the project directory and run:

`cargo run`

Use the GUI to start and stop recordings. Recordings will be saved in the specified directory with timestamps. If you do not specify the directory, the video will be saved in the working directory.

#### Code Analysis

*   **main.rs**: Contains the core application logic, handling the GUI state and user interactions.
*   **Capture Handling**: Utilizes `windows-capture` for screen capture and `tokio` for managing asynchronous tasks efficiently.
*   **GUI**: Built with `eframe` and `egui`, providing a responsive user interface.
*   **Video Encoding**: Supports multiple encoding types and qualities, configurable through the GUI.
*   **docs directory**: Jekyll tutorial.

#### Dependencies
- [`windows-capture`](https://github.com/NiiightmareXD/windows-capture/tree/main) for screen capture functionality. This crate allows the application to capture screen data efficiently.
- [`eframe`](https://docs.rs/eframe/latest/eframe/) and `egui` for GUI components.
- [`tokio`](https://docs.rs/tokio/latest/tokio/) for asynchronous operations.
- `chrono` for time-related functions.
- `nonce_cell` for unique ID generation.

<div align="center">
<!-- make the headline bold and font-size biger -->
    <h1 style="font-size: 50px; font-weight: bold;">📚 Diwan</h1>

[![GitHub Release](https://img.shields.io/github/v/release/Abdogouhmad/Diwan)](https://github.com/Abdogouhmad/Diwan/releases/latest)
[![GitHub contributors](https://img.shields.io/github/contributors/Abdogouhmad/Diwan)](https://github.com/Abdogouhmad/Diwan/graphs/contributors)
![GitHub top language](https://img.shields.io/github/languages/top/Abdogouhmad/Diwan?style=flat&logo=rust&logoSize=auto&color=%23b7410e)
![GitHub last commit (branch)](https://img.shields.io/github/last-commit/Abdogouhmad/Diwan/main?style=flat&logo=github)

</div>

# What is Diwan?

The name <strong>"Diwan"</strong> comes from the Arabic language and has deep-rooted
cultural significance in the Arab and Islamic world. In its literary context, a "diwan"
(dee-waan) refers to an anthology or collection of writings, poems, and literary works by a single author.

# Tasks

- [x] Step the folder structure
  - [x] Create a `diwan_cli` folder
  - [x] Create a `diwan_core` folder
  - [x] Create a `diwan_lib` folder
  - [x] Create an `img` folder
  - [x] Create necessary files:
    - [x] `diwan_cli/src/main.rs`
    - [x] `diwan_core/src/lib.rs`
    - [x] `diwan_core/src/buffer.rs`
    - [x] `diwan_core/src/cursor.rs`
    - [x] `diwan_core/src/editor.rs`
    - [x] `diwan_core/src/file_io.rs`
    - [x] `diwan_core/src/input.rs`
    - [x] `diwan_core/src/renderer.rs`
    - [x] `diwan_lib/src/lib.rs`
    - [x] `img/logo.svg`
- [x] Create a `README.md` file
- [x] Create a `.gitignore` file
- [x] Create a `Cargo.toml` file at root
- [ ] Crates
  - [ ] Filter the crates available on the internet
  - [ ] Add them to the `diwan_core/Cargo.toml`
    - [x] `crossterm`
    - [x] `anyhow`
- [x] CLI has two functions to open the file and to run the editor if the file is not provided
- [ ] Create the base of the project
  - [ ] Implement `Buffer` struct in `buffer.rs`
    - [ ] Methods for inserting and deleting characters
  - [ ] Implement `Cursor` struct in `cursor.rs`
    - [] Methods for moving the cursor
  - [ ] Implement main logic in `editor.rs`
    - [ ] Integrate buffer and cursor
    - [ ] Define modes and handle input
  - [ ] Implement file I/O in `file_io.rs`
    - [ ] Functions for loading and saving text files
  - [ ] Implement input handling in `input.rs`
    - [ ] Function to handle input
  - [ ] Implement screen rendering in `renderer.rs`
    - [ ] Function to render the screen efficiently
- [ ] Efficiency improvements
  - [ ] Avoid redundant flushing
  - [ ] Batch updates to minimize cursor movements
  - [ ] Efficient rendering to update only changed parts of the screen
- [ ] Testing
  - [ ] Write unit tests for buffer operations
  - [ ] Write unit tests for cursor movements
  - [ ] Write integration tests for editor commands

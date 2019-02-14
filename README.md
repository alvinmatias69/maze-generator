# Maze Generator
A simple maze generator algorithm using backtrack, written in Rust. Generated mazes will be guaranted to have a path from the start (top left) to the exit (bottom right).

## Getting Started
> This algorithm developed and tested on rust 1.31.1
1. Install [rust](https://www.rust-lang.org/tools/install) on your system.
1. Clone the repository.
    ```shell
    $ git clone git@github.com:alvinmatias69/maze-generator.git
    ```
1. Build and run the software
    ```shell
    $ cargo build
    $ ./target/debug/maze_generator
    ```
    or
    ```shell
    $ cargo run
    ```
    or in production mode
    ```shell
    $ cargo build --release
    $ ./target/release/maze_generator
    ```
1. Change maze dimension (Optional)

    Change the maze dimension from `main.rs` file. Change this line:
    ```rust
    let mut maze: Maze = Maze::init(10);
    ```
    to your preferred dimension.

## TODO
- Add file documentation
- Add WASM support

---

**Matias 2019**
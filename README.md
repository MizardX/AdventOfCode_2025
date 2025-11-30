# Advent of Code 2025 in rust

This library uses [`cargo-aoc`](https://github.com/gobanos/cargo-aoc) as runtime.

1. Install `cargo-aoc` with
    ```sh
    cargo install cargo-aoc
    ```
2. Log in to the [Advent of Code](https://adventofcode.com/) site.
3. Open developer tools, and navigate to Application -> Storage -> Cookies -> https://adventofcode.com
4. Copy the value of the "session" cookie.
5. Save the cookie to cargo-aoc with
    ```sh
    cargo aoc credentials {token}
    ```
6. Run with
    ```sh
    cargo aoc run
    ```

# Rulox Interpreter
<p align="center"><img src="./crafting_interpreters.png" alt="Crafting Interpreters Logo with Ferris the Crab"></p>
<p align="center">(Image credits: main image taken from <a href="https://craftinginterpreters.com/">Crafting Interpreters</a>, Ferris the Crab from <a href="https://www.rustacean.net/">Rustacean.net</a>)</p>

Rulox is an imperative, dynamically typed scripting language with C-like syntax implemented in Rust and based on the Lox programming language found in the [Crafting Interpreters](https://craftinginterpreters.com/) book by Bob Nystrom. The Rulox interpreter can either be run from the command line as a REPL or compiled to WebAssembly and run in the browser.

Currently this project is divided into three workspaces: `rulox-core` (the core interpreter), `rulox-cli` (a command line REPL), and `rulox-web` (a WebAssembly compilation for a simple web interface).

## Contents 
* [Installation](#installation)
* Usage
  *  CLI
  *  WebAssembly
* The Rulox Language
  *  Data types
  *  Variables
  *  Control flow
  *  Functions
  *  Classes
  *  Standard Library
* Bugs / TODO

### Installation
* Install Rust using the [official installation guide](https://www.rust-lang.org/learn/get-started), which will also install the `cargo` build system.

<!-- 
* Clone the GitHub repository and build the emulator:

```
$ git clone https://github.com/ChrisRG/rulox
$ cd rulox
$ cargo build --release
```
The binary can then be found in `./target/release`.

## Usage

#### CLI

#### WebAssembly
-->


# Rulox Interpreter
<p align="center"><img src="./crafting_interpreters.png" alt="Crafting Interpreters Logo with Ferris the Crab"></p>
<p align="center">(Image credits: main image taken from <a href="https://craftinginterpreters.com/">Crafting Interpreters</a>, Ferris the Crab from <a href="https://www.rustacean.net/">Rustacean.net</a>)</p>

Rulox is an imperative, dynamically typed scripting language with C-like syntax implemented in Rust and based on the Lox programming language found in the [Crafting Interpreters](https://craftinginterpreters.com/) book by Bob Nystrom. 

```
// Here's an example of recursive function to print the first ten Fibonacci numbers
fun fib(n) {
  if (n <= 1) return n; 
  return fib(n - 2) + fib(n - 1); 
} 

for (var i = 0; i <= 10; i = i + 1) { 
  print fib(i); 
}
```

The Rulox interpreter can either be run from the command line as a REPL or compiled to WebAssembly and run in the browser.

Currently this project is divided into three workspaces: `rulox-core` (the core interpreter), `rulox-cli` (a command line REPL), and `rulox-web` (a WebAssembly compilation for a simple web interface).

## Contents 
* [Installation](#installation)
* [Usage](#usage)
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

## Installation
#### Core / Command Line Interface
- Install Rust using the [official installation guide](https://www.rust-lang.org/learn/get-started), which will also install the `cargo` build system.
- Clone the GitHub repository and build the binary for the interpreter:

```
$ git clone https://github.com/ChrisRG/rulox
$ cd rulox
$ cargo build --release
```

The executable binary can then be found in `rulox/target/release`.

#### Rulox Web

To run `rulox-web` locally you'll need to download and install the JavaScript package manager [npm](https://www.npmjs.com/get-npm) in order to set up the necessary package dependencies and run a local server. 

If you already have `npm` installed, make sure that it's up to date:

```
$ npm install npm@latest -g
```

After setting up `npm`, make sure that the server dependencies are installed by running in the `rulox-web/www` directory:
```
$ npm install
```


## Usage
#### CLI
To access the Rulox interactive console or to execute files as scripts, you can either use the executable binary or `cargo`.

#### With the binary
To run the interactive REPL from the main directory:
```
$ ./target/release/rulox-cli
```

To read and exexcute a file:
```
$ ./target/release/rulox-cli examples/fibonacci.lox
```

#### With `cargo`
To run the interactive REPL from the main directory:
```
$ cargo run
```

To read and execute a file:
```
$ cargo run examples/fibonacci.lox
```

#### Rulox Web
To start the Rulox web server in the background, in the `rulox-web/www` directory run:
```
npm run start
```

Then open a Web browser and navigate to [http://localhost:8080/](http://localhost:8080/), where you will see a small code editor, a compile button, an output window, as well as several windows containing information about compiling the current code (token stream, Abstract Syntax Tree, and levels of variable scope).


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
* [The Rulox Language](#the-rulox-language)
  *  [Data types](#data-types)
  *  [Variables](#variables)
  *  [Control flow](#control-flow)
  *  [Functions](#functions)
  *  [Classes](#classes)
  *  [Standard Library](#standard-library)
* [To do](#to-do)

<br>

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

<br>

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


<br>

## The Rulox Language
Rulox is an [imperative](https://en.wikipedia.org/wiki/Imperative_programming) language: it evaluates both expressions (for determining the value of data) and statements (for changing the program's state). It is also [dynamically typed](https://en.wikipedia.org/wiki/Dynamic_programming_language), meaning that the interpreter makes very few guarantees about the types assigned to different variables and the operations
performed on those types.

### Data types
Currently there are only a handful of basic data types: 

| type | examples |
| ------- | ------- |
| boolean | `true`, `false` |
| integer | `1`, `5000` | 
| float | `1.0`, `5000.5` | 
| string | `"Hello world"` |

### Variables
Variables can be declared and assigned using the `var` keyword, note that the semicolon `;` is mandatory:

```
var x;
x = 10;
var y = 20;
```

Variables are lexically scoped. They cannot be referenced from outside of their current block. However variable names can be shadowed within an inner scope without losing the original binding:

```
var w = 10;
{
    var w = 30;
    print(w); 	// Prints 30
}
print(w);		    // Prints 10
```

### Control flow
Rulox supports two conventional C-style loop statements: `while` and `for`.

```
var x = 0;

while (x < 5) {
	print(x);
	x = x + 1;
}
```

```
for (var i = 0; i <= 10; i = i + 1) { 
	print fib(i); 
}
```

There is also a rudimentary `if-else` statement. Note that there `else if` has not yet been implemented, although there is a rather ugly workaround.

```
var x = 7;
if (x < 5) {
    print("Less than 5.");
} else {
    if (x > 10) {
        print("More than 10.");
    } else {
        print("Between 5 and 10.");
    }
}
```


### Functions
Functions are declared with the `fun` keyword followed by a comma-separated list of optional parameters. Parentheses are required for both declaring and calling functions.

```
fun add(x, y) {
	print(x + y);
}
add(5, 10);                 // Prints 15
add("Hello ", "world!");    // Prints "Hello world!"
```

A value can also be returned from the function.

```
fun multiply(a, b) {
	return(a * b);
}
print(multiply(3,3)); 	    // Prints 9

```

Functions in Rulox are both first class, meaning that they can be treated as variables...
```
fun add(x, y) {
	return(x + y);
}
var add_nums = add;
print(add_nums(1,2) + add_nums(3,4)); 	// Prints 10
```

... and higher order, meaning that they can be used as both the input and output for other functions.

```
fun increment(x) {
  return x + 1;
}

fun two_times(func) {
  fun inner_func(x){
    return func (func (x));
  }
  return inner_func;
}

var two = two_times(increment)(0);
print(two);           // Prints 2
```


### Classes
While the original Lox language in _Crafting Interpreters_ has classes for basic object-oriented programming (including inheritance and polymorphism), Rulox does not currently support them.

### Standard Library
At the moment, the only built-in function in Rulox is `print`.

<br>

## To do
Rulox is a work in progress! Here are some of the current goals:

#### Short term
- [ ] User input
- [ ] Improved error handling and messages
- [ ] Basic data structures

#### Long term
- [ ] Implement the bytecode compiler from _Crafting Interpreters_

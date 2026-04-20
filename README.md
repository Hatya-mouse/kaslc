# kaslc

kaslc is a CLI tool to compile & run KASL programs.

# Install

## Install via cargo

If you have cargo installed, just run:
```
cargo install kaslc
```

## Install without cargo

### macOS & Linux

Open your terminal and run:
```
curl -fsSL https://raw.githubusercontent.com/Hatya-mouse/kaslc/main/install.sh | sh
```

### Windows

Windows installer script is not yet available. If you don't have cargo installed, please download the binary directly from Release.

# Introduction to KASL

KASL is a programming language designed for audio processing.
The KASL language itself is implemented as a simple numerical calculation language with unique `input`, `output` and `state` variables, and can be used for audio processing with my [Knodiq DAW](https://github.com/hatya-mouse/knodiq), which is under development.

## How to Run the KASL Programs

Currently, there are no hosts that can run KASL program for audio processing.
However, you can run KASL code for genuine numerical calculation and there two ways to run it:

- [`kaslc`](https://github.com/hatya-mouse/kaslc) — A command-line tool to run KASL programs locally.
- [`kasl.hatya.dev`](https://kasl.hatya.dev) — Features **online KASL playground** where you can run KASL programs without installation.

## Example — FizzBuzz

This example is a FizzBuzz implementation on KASL.
The result is an array of integer number with 100 items, where "Fizz" is represented as -1, "Buzz" is represented as -2, and "FizzBuzz" is represented as -3.

```kasl
import std

output result = [0; 100]

func main() {
    var i = 1
    loop 100 {
        if i % 15 == 0 {
            result[i - 1] = -3
        } else if i % 3 == 0 {
            result[i - 1] = -1
        } else if i % 5 == 0 {
            result[i - 1] = -2
        } else {
            result[i - 1] = i
        }
        i = i + 1
    }
}
```

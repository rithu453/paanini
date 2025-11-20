# 🕉️ Paanini - Sanskrit Programming Language

[![Crates.io](https://img.shields.io/crates/v/paanini-lang)](https://crates.io/crates/paanini-lang)
[![Documentation](https://docs.rs/paanini-lang/badge.svg)](https://docs.rs/paanini-lang)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**Paanini** is a Sanskrit programming language that combines the beauty of Devanagari script with Python-like syntax. Write code using Sanskrit keywords and execute it seamlessly!

## ✨ Features

- 🎯 **Sanskrit Keywords**: Use देवनागरी script for programming constructs
- 🔄 **Python-like Syntax**: Familiar indentation-based structure
- 🚀 **Multiple Interfaces**: CLI, REPL, Web IDE, and file execution
- 🔧 **Transpilation**: Convert Sanskrit code to Rust binaries
- 🌐 **Web IDE**: Browser-based development environment with virtual keyboard
- 📝 **Real-time Transliteration**: Type English, get Sanskrit automatically

## 🚀 Quick Start

### Installation

> **Note**: Install with `cargo install paanini-lang`, then use the `paanini` command.

#### From Crates.io (Recommended)
```bash
# Install the package
cargo install paanini-lang
# Then use the 'paanini' command
paanini --version
```

#### From Source
```bash
git clone https://github.com/YOUR_USERNAME/paanini-lang.git
cd paanini-lang
cargo install --path .
```

### Your First Paanini Program

Create `hello.paanini`:
```sanskrit
!! नमस्ते विश्व - Hello World
दर्श("नमस्ते विश्व")

!! चर और गणना - Variables and Math
x = 5
y = 10
योग = x + y
दर्श("योग:", योग)

!! शर्त - Conditionals
यदि x < y:
    दर्श("x छोटा है")
अन्यथा:
    दर्श("x बड़ा है")
```

Run it:
```bash
paanini run hello.paanini
```

## 🎛️ CLI Commands

### Interactive REPL
```bash
paanini              # Start REPL (default)
paanini repl         # Explicit REPL command
```

### File Execution
```bash
paanini run file.paanini           # Execute Sanskrit source file
paanini run file.paanini --verbose # Show execution details
```

### Build to Binary
```bash
paanini build file.paanini                    # Transpile and build
paanini build file.paanini -o myapp          # Custom output name
paanini build file.paanini --release         # Optimized build
```

### Web IDE Server
```bash
paanini serve                    # Start on port 8080
paanini serve --port 3000       # Custom port
```

### Help & Examples
```bash
paanini --help                  # Show all commands
paanini example                 # Display example code
```

## 📚 Language Reference

### Sanskrit Keywords

| Sanskrit | English | Description |
|----------|---------|-------------|
| `दर्श()` | `darsh()` | Print/Display |
| `यदि` | `yadi` | If condition |
| `अन्यथा` | `anyatha` | Else |
| `यावत्` | `yavat` | While loop |
| `परिभ्रमण` | `paribhraman` | For loop |
| `परिधि()` | `paridhi()` | Range function |
| `कार्य` | `karya` | Function definition |
| `!!` | `!!` | Comments |

### Basic Syntax

#### Variables
```sanskrit
नाम = "भारत"
संख्या = 42
सत्य = true
```

#### Functions
```sanskrit
कार्य greet(नाम):
    दर्श("नमस्ते", नाम)

greet("विश्व")
```

#### Conditionals
```sanskrit
यदि संख्या > 0:
    दर्श("धनात्मक")
अन्यथा:
    दर्श("ऋणात्मक")
```

#### Loops
```sanskrit
!! While Loop
count = 0
यावत् count < 5:
    दर्श(count)
    count = count + 1

!! For Loop  
परिभ्रमण i in परिधि(5):
    दर्श("Iteration:", i)
```

## 🌐 Web IDE

The web IDE provides a complete development environment:

### Features
- 🎹 **Virtual Keyboard**: English QWERTY + Sanskrit Devanagari layouts
- 🔄 **Real-time Transliteration**: Type English → Get Sanskrit
- 🎯 **Key Highlighting**: Visual feedback while typing
- 📖 **Tutor Mode**: Live EN→SA conversion log
- 🪟 **Floating Interface**: Draggable and resizable keyboard

### Access
```bash
paanini serve
# Open http://localhost:8080
```

## 📖 Examples

### Calculator
```sanskrit
!! गणक - Calculator
कार्य add(a, b):
    return a + b

x = 15
y = 25
दर्श("योग:", add(x, y))
```

### Fibonacci Sequence
```sanskrit
!! फिबोनाची श्रृंखला
कार्य fibonacci(n):
    यदि n <= 1:
        return n
    अन्यथा:
        return fibonacci(n-1) + fibonacci(n-2)

परिभ्रमण i in परिधि(10):
    दर्श(fibonacci(i))
```

## 📄 License

This project is licensed under the [MIT License](LICENSE).

---

**नमस्ते! Start coding in Sanskrit today! 🕉️**

## Run (Windows PowerShell)
```powershell
# Build and run the web IDE
cargo run
# Expected output:
# Paanini IDE running at http://localhost:8080
# Open http://localhost:8080 in your browser
```

## CLI File Execution
```powershell
# Run a file and print results to stdout
cargo run -- path\to\program.paanini
```

## Language Basics
```text
!! टिप्पणी: मूल उदाहरणम् (Python-रूपेण)
x = 5
दर्श(x)
# -> 5

नाम = "विश्व"
दर्श("नमस्ते " + नाम)

यदि x == 5:
	दर्श("सत्यं")
अन्यथा:
	दर्श("असत्यं")

यावत् x < 8:
	दर्श(x)
	x = x + 1

परिभ्रमण i in परिधि(3):
	दर्श(i)

कार्य greet(नाम):
	दर्श("नमस्ते " + नाम)
greet("भारत")
```

## Extending Paanini
- Add Sanskrit keywords (`यदि` for `if`, `अन्यथा` for `else`) to the interpreter.
- Expand the glossary for richer translations.
- Implement arithmetic with numbers.
- Improve parsing (currently minimal quoting rules).

## Build Release
```powershell
cargo build --release
# Binary: target\release\paanini(.exe)
```

## Notes
- This is a teaching/demo project. Not a full language.
- The glossary is small and case-insensitive on English inputs.
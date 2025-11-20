# Paanini Language Guide

This document explains the syntax, keywords, and tooling for the **Paanini** Sanskrit-inspired programming language. It is intended as a companion to the README so that new users can discover the language constructs and workflow in one place.

---

## 1. Getting Started

### 1.1 Installation

Install the CLI from either distribution channel:

- **Rust**: `cargo install paanini-lang`
- **Node.js** (wraps the same binary): `npm install -g paanini-lang`

Both installers expose a `paanini` executable. The binary bundles the web IDE assets, so no additional static files are required.

### 1.2 Running Code

| Command | Description |
|---------|-------------|
| `paanini` | Start the interactive REPL. Type `help` inside the REPL for a quick syntax reminder. |
| `paanini run file.paanini` | Execute the specified source file. |
| `paanini build file.paanini` | Transpile to Rust and build an executable (see README for release flags). |
| `paanini serve [--port 8080]` | Launch the Web IDE; open the printed `http://localhost:<port>` URL. |

All source files use the `.paanini` extension and must be UTF-8 encoded to preserve Devanagari glyphs.

### 1.3 Project Layout

A minimal project often contains:

```
hello.paanini  # entry point
README.md      # optional instructions
```

The language does not currently provide a module system, so each run processes a single file.

---

## 2. Syntax Basics

### 2.1 Comments

Two comment styles are supported:

```sanskrit
!! यह टिप्पणी है  (Python-style!)
# This is also a comment (for convenience)
```

### 2.2 Whitespace & Blocks

Blocks are indentation-sensitive, similar to Python. Start a block with a trailing colon and indent the statements within it.

```sanskrit
यदि (x < 10):
    दर्श("x छोटा है")
अन्यथा:
    दर्श("x बड़ा है")
```

Internally the interpreter converts indentation to braces, so consistent spaces are recommended (tabs are normalized to spaces automatically).

### 2.3 Identifiers

Identifiers may contain Devanagari or Latin letters, digits, and underscores. Examples: `गणना`, `data_value`, `संख्या2`.

### 2.4 Literals

| Literal | Example | Notes |
|---------|---------|-------|
| Number | `42`, `3.14` | Stored as 64-bit floating point. |
| String | `"नमस्ते"` | Must use double quotes. |
| Boolean | `सत्य`, `असत्य` | Sanskrit words for `true` and `false`. |
| Null | implicitly `null` result | Returned when an expression fails or a function has no explicit result. |

### 2.5 Variables & Assignment

Use `=` to assign expression results to identifiers (no declaration keyword is needed).

```sanskrit
नाम = "भारत"
संख्या = 108
```

Assignment is separate from comparison (`==`). Other comparison operators include `!=`, `>`, `<`, `>=`, and `<=`.

### 2.6 Expressions

- Arithmetic: Only addition (`+`) is implemented.
- String concatenation: `+` works with strings and numbers; non-string values are coerced to their textual form.
- Parentheses group subexpressions: `(x + 5)`.

### 2.7 Printing

Use the Sanskrit verb **दर्श** ("show"):

```sanskrit
दर्श("नमस्ते विश्व")
```

`दर्श(expr)` evaluates the expression and writes its textual representation to standard output.

---

## 3. Control Flow

### 3.1 Conditional (`यदि` / `अन्यथा`)

```sanskrit
यदि (संख्या > 0):
    दर्श("धनात्मक")
अन्यथा:
    दर्श("ऋणात्मक")
```

Conditions must compare numeric expressions. Unsupported comparisons emit runtime errors.

### 3.2 While Loop (`यावत्`)

```sanskrit
गणक = 0
यावत् (गणक < 5):
    दर्श(गणक)
    गणक = गणक + 1
```

The interpreter includes a safety guard (10,000 iterations) to prevent accidental infinite loops.

### 3.3 For Loop (`परिभ्रमण`)

```sanskrit
परिभ्रमण i in परिधि(5):
    दर्श("Iteration: " + i)
```

The loop header must follow `परिभ्रमण <variable> in परिधि(<limit>)`. The helper **परिधि** returns a list or range from `0` up to (but excluding) the provided upper bound.

---

## 4. Functions

### 4.1 Defining Functions (`कार्य`)

```sanskrit
कार्य greet(नाम):
    दर्श("नमस्ते " + नाम)
```

- Parameters are comma-separated, and their names must be valid identifiers.
- A function currently returns `null` implicitly; use `दर्श` for observable output.

### 4.2 Calling Functions

Call functions with Sanskrit identifiers just like Python:

```sanskrit
greet("विश्व")
```

### 4.3 Built-in Functions

| Built-in | Description |
|----------|-------------|
| `दर्श(expr)` | Print the value of `expr`. |
| `परिधि(n)` | Return a list-like object containing integers `0..n-1`. Used internally by `परिभ्रमण`. |
| `help` | When entered in the REPL, prints a quick language summary. |

---

## 5. REPL Convenience Commands

Inside the REPL:

- `help` – display a concise syntax cheat sheet.
- Arrow keys / prompt editing – provided by the host terminal.
- Multi-line input – paste or type blocks; the REPL executes once a full construct is entered.

---

## 6. Example Program

```sanskrit
!! गणक उदाहरण
कार्य square(n):
    परिणाम = n * n  !! लक्षात्: currently only `+` is supported; emulate multiplication via addition.

संख्या = 5
यदि (संख्या < 10):
    दर्श("संख्या: " + संख्या)
अन्यथा:
    दर्श("बड़ी संख्या")

परिणाम = संख्या + 5
दर्श("योग: " + परिणाम)

परिभ्रमण i in परिधि(3):
    दर्श("सत्तम् " + i)
```

> **Note:** Multiplication and subtraction are not implemented yet; use repeated addition or extend the interpreter to add more operators.

---

## 7. Error Messages

The interpreter emits diagnostics in Sanskrit (prefixed with `त्रुटिः`). Common messages include:

- `त्रुटिः: असाइनस्य नाम अवैधम्` – invalid identifier on the left side of `=`.
- `त्रुटिः: दर्श प्रयोगः केवलं दर्श(expr) स्वरूपेण भवेत्` – malformed print call.
- `त्रुटिः: परिभ्रमण केवलं परिधि(n) सह समर्थितम्` – for loops must use the `परिधि` helper.
- `त्रुटिः: यदि शर्ता अवैध` – conditional expression could not be evaluated.

Understanding these terms helps debug programs without switching languages mid-stream.

---

## 8. Extending the Language

The Rust interpreter (`src/interpreter.rs`) is intentionally small and approachable. To experiment:

1. Clone the repository and open `src/interpreter.rs`.
2. Add new token handlers or extend `eval_expr` for additional operators.
3. Rebuild with `cargo build --release` and replace the binary inside `npm/bin/` if you ship a fresh npm package.

Contributions are welcome to enrich the glossary, add arithmetic primitives, or implement return values for user-defined functions.

---

Happy coding in Sanskrit! 🙏

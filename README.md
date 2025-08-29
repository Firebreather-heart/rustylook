# RustyLook 🎨

[![Crates.io](https://img.shields.io/crates/v/rustylook.svg)](https://crates.io/crates/rustylook)
[![Documentation](https://docs.rs/rustylook/badge.svg)](https://docs.rs/rustylook)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)

A simple and lightweight Rust library for adding colors to your terminal output using ANSI escape codes. Perfect for creating colorful CLI applications and improving terminal user experience.

## Features

- 🌈 Support for 8 standard terminal colors
- 🎯 Simple and intuitive API
- 🚀 Zero dependencies
- 📦 Lightweight and fast
- 🔧 Works on all platforms that support ANSI escape codes

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rustylook = "0.1.0"
```

## Quick Start

```rust
use rustylook::Brush;

fn main() {
    // Create a brush and paint text
    let red_brush = Brush::new("red").unwrap();
    println!("{}", red_brush.paint("Hello, World!"));
    
    // Or use the quick paint method
    println!("{}", Brush::q_paint("Quick and easy!", "blue"));
}
```

## Available Colors

| Color  | ANSI Code | Example |
|--------|-----------|---------|
| red    | 31        | 🔴 Error messages |
| green  | 32        | ✅ Success messages |
| yellow | 33        | ⚠️ Warning messages |
| blue   | 34        | ℹ️ Info messages |
| purple | 35        | 💜 Highlights |
| cyan   | 36        | 🔵 Special text |
| gray   | 30        | 📝 Comments |
| white  | 37        | ⚪ Default text |

Note: `gray` and `grey` are both accepted spellings.

## Usage Examples

### Basic Usage

```rust
use rustylook::Brush;

fn main() {
    // Create a brush instance
    if let Some(brush) = Brush::new("green") {
        let message = brush.paint("Operation successful!");
        println!("{}", message);
    }
    
    // Handle invalid colors
    match Brush::new("invalid_color") {
        Some(brush) => println!("{}", brush.paint("This won't print")),
        None => println!("Invalid color specified"),
    }
}
```

### Quick Painting

```rust
use rustylook::Brush;

fn main() {
    println!("{}", Brush::q_paint("Error: File not found", "red"));
    println!("{}", Brush::q_paint("Warning: Low disk space", "yellow"));
    println!("{}", Brush::q_paint("Success: File saved", "green"));
    println!("{}", Brush::q_paint("Info: Processing...", "blue"));
}
```

### Building a Colorful CLI

```rust
use rustylook::Brush;

fn print_status(status: &str, message: &str) {
    let (color, prefix) = match status {
        "error" => ("red", "❌ ERROR"),
        "warning" => ("yellow", "⚠️  WARN"),
        "success" => ("green", "✅ SUCCESS"),
        "info" => ("blue", "ℹ️  INFO"),
        _ => ("white", "📝 LOG"),
    };
    
    println!("{}: {}", 
        Brush::q_paint(prefix, color), 
        message
    );
}

fn main() {
    print_status("info", "Starting application...");
    print_status("warning", "Configuration file not found, using defaults");
    print_status("success", "Connected to database");
    print_status("error", "Failed to load user data");
}
```

### Advanced Usage with Error Handling

```rust
use rustylook::Brush;

fn colorize_log_level(level: &str, message: &str) -> String {
    let color = match level.to_lowercase().as_str() {
        "debug" => "gray",
        "info" => "blue", 
        "warn" => "yellow",
        "error" => "red",
        _ => "white",
    };
    
    format!("[{}] {}", 
        Brush::q_paint(&level.to_uppercase(), color),
        message
    )
}

fn main() {
    println!("{}", colorize_log_level("debug", "Debug information"));
    println!("{}", colorize_log_level("info", "Application started"));
    println!("{}", colorize_log_level("warn", "Memory usage high"));
    println!("{}", colorize_log_level("error", "Critical failure"));
}
```

## API Reference

### `Brush::new(color: &str) -> Option<Self>`

Creates a new brush with the specified color.

**Parameters:**
- `color`: A string slice representing the color name

**Returns:**
- `Some(Brush)` if the color is valid
- `None` if the color is not recognized

### `brush.paint(text: &str) -> String`

Applies the brush's color to the given text.

**Parameters:**
- `text`: The string slice to be colored

**Returns:**
- A `String` with ANSI color codes applied

### `Brush::q_paint(text: &str, color: &str) -> String`

Quickly colors text with the specified color (static method).

**Parameters:**
- `text`: The string slice to be colored
- `color`: The color name as a string slice

**Returns:**
- A `String` with ANSI color codes applied (defaults to white for invalid colors)

## Platform Compatibility

This library works on:
- ✅ Linux
- ✅ macOS  
- ✅ Windows (Windows 10 version 1511 and later with ANSI support enabled)
- ✅ Most terminal emulators

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

### Development

```bash
# Clone the repository
git clone https://github.com/firebreather-heart/rustylook.git
cd rustylook

# Run tests
cargo test

# Build documentation
cargo doc --open

# Check formatting
cargo fmt --check
```

## License

This project is licensed under 
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

## Changelog

### 0.1.0
- Initial release
- Basic color support for 8 standard ANSI colors
- `Brush::new()` and `Brush::paint()` methods
- `Brush::q_paint()` convenience method

---

Made with ❤️ by [Firebreather](https://github.com/firebreather-heart)
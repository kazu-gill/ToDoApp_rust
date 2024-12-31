# Rust Todo App

A simple desktop Todo application built with Rust

## Features

- Add, delete, and manage task completion
- Due date setting with calendar picker
- Color-coded due dates (Red: Overdue, Yellow: Within 24 hours)
- Automatic task sorting (by completion status and due date)
- Batch operations (check all, uncheck all, remove completed)
- Pomodoro timer (15min, 30min, 1hour)
- Data persistence using JSON
- Japanese UI support

## Requirements

- Rust 1.70 or higher
- macOS (for Japanese font support)

## Installation

```bash
# Clone the repository
git clone [your-repository-url]
cd todo-app

# Build and run
cargo build
cargo run
```

## Development Setup

1. Install Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

2. Install dependencies
```bash
cargo build
```

3. Reference to System Font (macOS)
```bash
# The application uses the macOS system font (ヒラギノ角ゴシック W3.ttc)
# No additional setup is required as it references the system font directly:
# /System/Library/Fonts/ヒラギノ角ゴシック W3.ttc
```

Note: The application is designed to use the macOS system font. This is intentional to avoid font license issues and ensure consistent rendering on macOS systems.

## Main Dependencies

- eframe: GUI framework
- serde: JSON serialization
- chrono: Date and time handling

## Maintenance

### Version Updates

Update dependencies:
```bash
cargo update
```

### Building

Development build:
```bash
cargo build
```

Release build:
```bash
cargo build --release
```

### Testing

```bash
cargo test
```

## License

MIT License

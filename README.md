# rCandle

**A Rust GRBL Controller Application with G-Code Visualizer**

rCandle is a modern reimplementation of the [Candle](https://github.com/Denvi/Candle) CNC controller application, written in Rust for improved performance, safety, and maintainability.

![Status: In Development](https://img.shields.io/badge/status-in%20development-yellow)
![License: GPL-3.0](https://img.shields.io/badge/license-GPL--3.0-blue)
![Rust Version](https://img.shields.io/badge/rust-1.75%2B-orange)

## Overview

rCandle is designed for controlling CNC machines equipped with GRBL firmware using a PC. It supports 3-axis milling machines and laser plotters with comprehensive G-Code manipulation and visualization capabilities.

### Key Features

- **GRBL Communication**: Serial port, Telnet, and WebSocket connections to GRBL controllers
- **G-Code Management**: Load, edit, save, and send G-Code files to CNC machines
- **3D Visualization**: Real-time 3D rendering of toolpaths using modern graphics APIs (WGPU)
- **Machine Control**: Manual jogging, coordinate system management, spindle control
- **Height Mapping**: Surface scanning and automatic height compensation
- **Scripting**: Custom automation with embedded scripting engine
- **Cross-Platform**: Windows, Linux, and macOS support

## Status

This project is currently in **active development**. See the [Roadmap](.specify/ROADMAP.md) for development progress and timeline.

### Completed

- ✅ Project specification and architecture
- ✅ Development roadmap
- ✅ Initial project scaffolding

### In Progress

- 🚧 Foundation infrastructure (logging, configuration, error handling)

### Planned

- [ ] G-Code parser
- [ ] Serial communication
- [ ] 3D visualization
- [ ] User interface
- [ ] Height mapping
- [ ] Scripting engine

## Building from Source

### Prerequisites

- Rust 1.75 or later
- Git
- CMake (for some dependencies)

**Platform-specific requirements:**

**Linux:**
```bash
sudo apt update
sudo apt install build-essential pkg-config libudev-dev
```

**Windows:**
- Visual Studio 2019 or later with C++ build tools

**macOS:**
```bash
xcode-select --install
```

### Build

```bash
git clone https://github.com/yourusername/rCandle.git
cd rCandle
cargo build --release
```

The executable will be in `target/release/rcandle` (or `rcandle.exe` on Windows).

### Run

```bash
cargo run --release
```

## Documentation

- **[Specification](.specify/SPECIFICATION.md)**: Complete project specification
- **[Architecture](.specify/ARCHITECTURE.md)**: Technical architecture documentation
- **[Roadmap](.specify/ROADMAP.md)**: Development timeline and milestones
- **[User Manual](docs/user_manual.md)**: User guide (coming soon)
- **[Developer Guide](docs/development.md)**: Contributing and development guide (coming soon)

## Quick Start

*Coming soon - once the application is functional*

## Architecture

rCandle is organized into several key modules:

- **Connection Module**: Abstract interface for GRBL communication (serial, telnet, websocket)
- **Parser Module**: G-Code parsing and preprocessing
- **Renderer Module**: 3D visualization using WGPU
- **State Module**: Application and machine state management
- **UI Module**: User interface (Iced/egui framework)
- **HeightMap Module**: Surface scanning and height compensation
- **Script Module**: Scripting engine for automation

See the [Architecture Document](.specify/ARCHITECTURE.md) for detailed information.

## Comparison with Original Candle

| Feature | Candle (C++/Qt) | rCandle (Rust) |
|---------|----------------|----------------|
| Language | C++ | Rust |
| UI Framework | Qt5 | egui + eframe |
| Graphics | OpenGL 2.0 | WGPU (Vulkan/Metal/DX12) |
| Memory Safety | Manual | Guaranteed by compiler |
| Async I/O | Qt event loop | Tokio async runtime |
| Build System | CMake | Cargo |
| Package Manager | vcpkg | Cargo |
| Platforms | Windows, Linux, macOS | Windows, Linux, macOS |

### Technical Stack
- **Language**: Rust 2021 edition
- **UI Framework**: egui + eframe (immediate mode)
- **Graphics**: WGPU (Vulkan/Metal/DX12)
- **Async Runtime**: Tokio
- **Parser**: nom combinators
- **Scripting**: Rhai

### Goals

- **Feature Parity**: Implement all features from the original Candle
- **Performance**: Match or exceed C++ version performance
- **Safety**: Leverage Rust's memory safety guarantees
- **Maintainability**: Cleaner, more maintainable codebase
- **Modern**: Use modern graphics APIs and UI frameworks

## Contributing

Contributions are welcome! This project is in early development, so there's plenty to do.

### How to Contribute

1. Check the [Roadmap](.specify/ROADMAP.md) for current priorities
2. Look at open issues or create a new one
3. Fork the repository
4. Create a feature branch
5. Make your changes
6. Submit a pull request

### Development Setup

```bash
# Clone the repository
git clone https://github.com/yourusername/rCandle.git
cd rCandle

# Install development dependencies
cargo install cargo-watch cargo-audit

# Run tests
cargo test

# Run with auto-reload during development
cargo watch -x run

# Format code
cargo fmt

# Lint code
cargo clippy -- -D warnings
```

## Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test parser_tests

# Run with output
cargo test -- --nocapture

# Run benchmarks
cargo bench
```

## License

This project is licensed under the GNU General Public License v3.0 - see the [LICENSE](LICENSE) file for details.

This is the same license as the original Candle application, ensuring compatibility and continued open-source development.

## Credits

- **Original Candle**: [Denvi/Candle](https://github.com/Denvi/Candle) by Denis Ravilevich Hayrullin
- **GRBL**: [gnea/grbl](https://github.com/gnea/grbl)
- **GRBL Documentation**: [GRBL 1.1f Customized](https://github.com/craftweeks/grbl-1.1f.customized-for-laser/tree/master/doc/markdown)
- **Rust Community**: For excellent crates and documentation

## Support

- **Issues**: [GitHub Issues](https://github.com/yourusername/rCandle/issues)
- **Discussions**: [GitHub Discussions](https://github.com/yourusername/rCandle/discussions)
- **Documentation**: See `docs/` directory

## Key Resources

### GRBL Firmware Documentation
- **Primary Documentation**: [GRBL 1.1f Markdown Docs](https://github.com/craftweeks/grbl-1.1f.customized-for-laser/tree/master/doc/markdown)
  - Commands reference (G-codes, M-codes, $ commands)
  - Serial interface protocol
  - Configuration settings
  - Real-time jogging
  - Laser mode features
- **GRBL v1.1 Wiki**: https://github.com/gnea/grbl/wiki
- **GRBL v0.9 Wiki**: https://github.com/grbl/grbl/wiki

### Project Documentation
- [Specification](.specify/SPECIFICATION.md) - Complete requirements and design
- [Architecture](.specify/ARCHITECTURE.md) - Technical architecture details
- [Roadmap](.specify/ROADMAP.md) - 20-week development plan
- [Dependencies](.specify/DEPENDENCIES.md) - Crate selection rationale
- [Migration Guide](.specify/MIGRATION_GUIDE.md) - C++ to Rust patterns

## Acknowledgments

This project is a Rust migration of Candle, originally created by Denis Ravilevich Hayrullin. We are grateful for the original work and aim to continue its legacy with a modern, safe implementation.

## Related Projects

- [Candle](https://github.com/Denvi/Candle) - Original C++/Qt application
- [GRBL](https://github.com/gnea/grbl) - CNC controller firmware
- [bCNC](https://github.com/vlachoudis/bCNC) - Python-based GRBL interface
- [CNCjs](https://github.com/cncjs/cncjs) - Web-based CNC interface

## Roadmap Highlights

- **Phase 1 (Weeks 1-2)**: Foundation and project setup ✅
- **Phase 2 (Weeks 3-4)**: G-Code parser
- **Phase 3 (Weeks 5-6)**: Serial communication
- **Phase 4 (Week 7)**: State management
- **Phase 5 (Weeks 8-10)**: 3D visualization
- **Phase 6 (Weeks 11-13)**: User interface
- **Phase 7 (Weeks 14-15)**: Height mapping
- **Phase 8 (Weeks 16-17)**: Advanced features
- **Phase 9 (Weeks 18-20)**: Polish and release

See the full [Roadmap](.specify/ROADMAP.md) for details.

---

**Note**: This project is in active development. Features and documentation will be added progressively according to the roadmap.

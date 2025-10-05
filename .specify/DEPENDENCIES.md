# rCandle Dependencies Analysis

## Overview

This document analyzes the key Rust crates (dependencies) used in rCandle, their purpose, alternatives, and rationale for selection.

## Core Dependencies

### 1. Async Runtime

#### Selected: `tokio` (v1.35+)
- **Purpose**: Async runtime for I/O operations (serial, network)
- **Features Used**: `full` (includes all features for ease of use)
- **Why**: Industry standard, excellent ecosystem, well-maintained
- **Alternatives**:
  - `async-std`: Simpler API, but smaller ecosystem
  - `smol`: Lightweight, but less mature ecosystem
- **License**: MIT

### 2. Serial Communication

#### Selected: `serialport` (v4.3+)
- **Purpose**: Cross-platform serial port access for GRBL communication
- **Why**: Most mature Rust serial port library, tokio compatible
- **GRBL Protocol Support**: Implements serial communication following GRBL interface specifications
  - Documented at: https://github.com/craftweeks/grbl-1.1f.customized-for-laser/blob/master/doc/markdown/interface.md
  - Supports real-time commands, status queries, and command streaming
- **Alternatives**:
  - `tokio-serial`: Wrapper around serialport, could be used in addition
  - Direct OS APIs: Too much platform-specific code
- **License**: MPL-2.0
- **Platform Support**: Windows, Linux, macOS, BSD

### 3. UI Framework

#### Selected: `egui` + `eframe` (v0.27+) **[PRIMARY CHOICE]**
- **Purpose**: Immediate mode GUI framework
- **Why**: 
  - Simple to use and highly flexible
  - Excellent for tools and technical applications
  - Mature and stable with proven track record
  - Very flexible for custom widgets
  - Great performance
- **Pros**:
  - Rapid development
  - Easy to integrate with custom rendering (wgpu)
  - Portable (can run in browser with wasm)
  - Great for prototyping and iteration
  - Excellent documentation and examples
  - Works great with async Rust
- **Cons**:
  - Immediate mode may be less familiar
  - More manual state management required
  - Some styling limitations compared to retained mode
- **License**: MIT/Apache-2.0

**Decision Rationale**: egui was chosen as the primary UI framework for rCandle due to its maturity, flexibility, and proven success in similar technical applications. The immediate mode paradigm, while different from Qt, offers simplicity and direct control that's ideal for a tool like rCandle. Its excellent integration with wgpu for 3D rendering and ability to create custom widgets makes it perfect for our needs.

### 4. Graphics API

#### Selected: `wgpu` (v0.19+)
- **Purpose**: Modern, safe graphics API
- **Why**: 
  - Cross-platform (Vulkan, Metal, DX12, OpenGL/ES fallback)
  - Safe Rust API
  - Future-proof (based on WebGPU standard)
  - Good ecosystem
- **Alternatives**:
  - `glium`: OpenGL wrapper, simpler but less modern
  - `glow`: Lower-level OpenGL, more control but more unsafe
  - `ash`: Vulkan bindings, too low-level for this project
- **License**: MIT/Apache-2.0
- **Shader Language**: WGSL (WebGPU Shading Language)

### 5. Math Library

#### Selected: `glam` (v0.27+)
- **Purpose**: Fast vector/matrix math for graphics
- **Why**: 
  - Designed for game/graphics programming
  - SIMD optimized
  - Small binary size
  - Works well with wgpu
- **Features**: `serde` for serialization
- **Alternatives**:
  - `nalgebra`: More comprehensive, but heavier
  - `cgmath`: Good but less active development
- **License**: MIT/Apache-2.0

#### Additional: `nalgebra` (v0.32+)
- **Purpose**: Linear algebra for height map interpolation
- **Why**: More complete for numerical computations
- **Use Case**: Height map interpolation, scientific calculations
- **License**: Apache-2.0

### 6. Parsing

#### Selected: `nom` (v7.1+)
- **Purpose**: Parser combinator library for G-Code parsing
- **Why**: 
  - Zero-copy parsing
  - Composable parsers
  - Excellent error messages
  - Battle-tested
- **Alternatives**:
  - `pest`: PEG parser, more declarative but slower
  - `combine`: Similar to nom, slightly different API
  - Manual parsing: Error-prone, not recommended
- **License**: MIT

#### Supporting: `regex` (v1.10+)
- **Purpose**: Regular expressions for pattern matching
- **Why**: Sometimes simpler than parser combinators for simple patterns
- **License**: MIT/Apache-2.0

### 7. Serialization

#### Selected: `serde` (v1.0+)
- **Purpose**: Serialization/deserialization framework
- **Why**: De facto standard, huge ecosystem
- **License**: MIT/Apache-2.0

#### Selected: `serde_json` (v1.0+)
- **Purpose**: JSON format support
- **Use Case**: Configuration files, data interchange
- **License**: MIT/Apache-2.0

#### Selected: `toml` (v0.8+)
- **Purpose**: TOML format support
- **Use Case**: Primary configuration file format
- **Why**: More human-friendly than JSON for config
- **License**: MIT/Apache-2.0

### 8. Error Handling

#### Selected: `thiserror` (v1.0+)
- **Purpose**: Derive macro for error types
- **Use Case**: Library code (internal crates)
- **Why**: Clean error definitions, good error messages
- **License**: MIT/Apache-2.0

#### Selected: `anyhow` (v1.0+)
- **Purpose**: Flexible error handling
- **Use Case**: Application code
- **Why**: Easy error propagation with context
- **License**: MIT/Apache-2.0

### 9. Logging

#### Selected: `tracing` (v0.1+)
- **Purpose**: Structured logging and instrumentation
- **Why**: 
  - More powerful than `log` crate
  - Async-aware
  - Structured logging
  - Excellent for debugging
- **License**: MIT

#### Selected: `tracing-subscriber` (v0.3+)
- **Purpose**: Log formatting and output
- **Features**: `env-filter`, `json`
- **Why**: Flexible log configuration
- **License**: MIT

### 10. Scripting Engine

#### Selected: `rhai` (v1.17+)
- **Purpose**: Embedded scripting language
- **Why**: 
  - Designed for Rust integration
  - Simple syntax (similar to JavaScript)
  - Safe by default
  - Good performance
  - Easy to sandbox
- **Features**: `sync` for thread safety
- **Alternatives**:
  - `mlua`: Lua bindings, more mature language but less Rust-native
  - `deno_core`: JavaScript engine, heavy dependency
  - `rlua`: Older Lua bindings, less maintained
- **License**: MIT/Apache-2.0

**Example Rhai Script**:
```javascript
// Simple and familiar syntax
let x = 10;
send_command("G0 X" + x);
wait_idle();
print("Done!");
```

### 11. Networking

#### Selected: `tokio-tungstenite` (v0.21+)
- **Purpose**: WebSocket client/server
- **Use Case**: WebSocket connection to GRBL
- **Why**: Async, integrates with tokio
- **License**: MIT

## Utility Dependencies

### File Dialogs

#### Selected: `rfd` (v0.14+)
- **Purpose**: Native file dialogs
- **Why**: Cross-platform, native look-and-feel
- **License**: MIT

### CLI Parsing

#### Selected: `clap` (v4.5+)
- **Purpose**: Command-line argument parsing
- **Features**: `derive` for clean API
- **Use Case**: CLI tools for testing
- **License**: MIT/Apache-2.0

### Configuration Management

#### Selected: `config` (v0.14+)
- **Purpose**: Layered configuration system
- **Why**: Supports multiple sources (files, env vars, defaults)
- **License**: MIT/Apache-2.0

### Date/Time

#### Selected: `chrono` (v0.4+)
- **Purpose**: Date and time handling
- **Use Case**: Timestamps, time estimates
- **License**: MIT/Apache-2.0

### Path Management

#### Selected: `directories` (v5.0+)
- **Purpose**: Cross-platform directory paths
- **Use Case**: Config, cache, data directories
- **Why**: Follows platform conventions
- **License**: MPL-2.0

### Binary Serialization

#### Selected: `bytemuck` (v1.14+)
- **Purpose**: Safe casting for vertex data
- **Features**: `derive`
- **Use Case**: GPU vertex buffers
- **Why**: Safe, zero-cost abstraction
- **License**: Zlib/MIT/Apache-2.0

## Development Dependencies

### Testing

#### Selected: `tokio-test` (v0.4+)
- **Purpose**: Testing utilities for async code
- **License**: MIT

#### Selected: `mockall` (v0.12+)
- **Purpose**: Mock object generation
- **Use Case**: Testing with mock connections
- **License**: MIT/Apache-2.0

#### Selected: `proptest` (v1.4+)
- **Purpose**: Property-based testing
- **Use Case**: Parser fuzzing
- **License**: MIT/Apache-2.0

### Benchmarking

#### Selected: `criterion` (v0.5+)
- **Purpose**: Benchmarking framework
- **Use Case**: Performance regression detection
- **License**: MIT/Apache-2.0

## Dependency Security

### Audit Process

```bash
# Install cargo-audit
cargo install cargo-audit

# Run security audit
cargo audit

# Check for advisories
cargo audit --deny warnings
```

### License Compliance

All dependencies must be compatible with GPL-3.0. Current selections are:
- MIT: Compatible ✅
- Apache-2.0: Compatible ✅
- MPL-2.0: Compatible ✅
- Zlib: Compatible ✅

### Supply Chain Security

```bash
# Install cargo-deny
cargo install cargo-deny

# Check licenses and security
cargo deny check
```

## Build Dependencies

Minimal build dependencies to reduce build complexity:
- No C++ dependencies
- No Qt dependencies
- No external build tools beyond Cargo

## Platform-Specific Dependencies

### Linux
- `libudev-dev`: Required by serialport
- No additional runtime dependencies

### Windows
- No additional dependencies (uses Windows API)

### macOS
- No additional dependencies (uses IOKit)

## Dependency Updates

### Update Strategy
- Review updates monthly
- Test thoroughly before updating major versions
- Pin versions in `Cargo.lock` for reproducible builds
- Use `cargo-outdated` to check for updates

```bash
cargo install cargo-outdated
cargo outdated
```

### Minimal Version Policy
- Use `^` (caret) for version requirements (default)
- Allows patch and minor updates
- Example: `tokio = "1.35"` allows 1.35.x and 1.x.x (x < 2.0)

## Bundle Size Considerations

### Release Binary Size
- Target: < 50 MB
- Achieved through:
  - LTO (Link Time Optimization)
  - Strip symbols
  - Optimize dependencies
  - Avoid unnecessary features

### Compilation Time
- Development build: < 1 minute
- Release build: < 2 minutes
- Incremental builds: < 10 seconds

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
| Target Platforms | Windows, Linux, macOS | Windows, Linux, macOS |

### Minimal Dependencies (CLI-only version)
If creating a headless CLI version:
- Remove: iced/egui, wgpu
- Add: `termion` or `crossterm` for terminal UI
- Benefit: Much smaller binary, faster compile

### Web Version (Future)
If targeting WebAssembly:
- Keep: Most core logic
- Replace: serialport (use Web Serial API via wasm-bindgen)
- Replace: Native file dialogs (use browser APIs)
- Benefit: Runs in browser

### Embedded Version (Future)
If targeting embedded systems:
- Remove: Full UI, scripting
- Use: `embedded-hal` for hardware access
- Target: No-std or embedded-friendly crates

## Conclusion

The selected dependencies provide a solid foundation for rCandle, balancing:
- **Maturity**: Proven crates with active maintenance
- **Performance**: Efficient implementations
- **Safety**: Rust-native, memory-safe APIs
- **Compatibility**: GPL-3.0 compatible licenses
- **Maintainability**: Good documentation and community support

The modular architecture allows for dependency substitution if needed without major refactoring.

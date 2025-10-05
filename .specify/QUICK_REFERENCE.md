# rCandle Quick Reference Guide

A one-page overview of the rCandle project.

## What is rCandle?

A **Rust reimplementation** of Candle, a GRBL controller application with G-Code visualizer for CNC machines.

## Tech Stack

```
┌─────────────────────────────────────┐
│   UI: egui + eframe                 │
├─────────────────────────────────────┤
│   Graphics: WGPU                    │
├─────────────────────────────────────┤
│   Async: Tokio                      │
├─────────────────────────────────────┤
│   Serial: serialport                │
├─────────────────────────────────────┤
│   Parsing: nom                      │
├─────────────────────────────────────┤
│   Math: glam + nalgebra             │
└─────────────────────────────────────┘
```

## Target Platforms

- ✅ **Windows** 10/11 (x64)
- ✅ **Linux** Ubuntu 20.04+, Arch, Fedora (x64)
- ✅ **macOS** 12+ Monterey and later (x64, Apple Silicon)

## Core Modules

| Module | Purpose | Key Crates |
|--------|---------|------------|
| **Connection** | Serial/network comm | `serialport`, `tokio-tungstenite` |
| **Parser** | G-Code parsing | `nom`, `regex` |
| **Renderer** | 3D visualization | `wgpu`, `glam` |
| **State** | App/machine state | `tokio::sync` |
| **UI** | User interface | `egui`, `eframe` |
| **HeightMap** | Surface mapping | `nalgebra` |
| **Script** | Automation | `rhai` |

## 20-Week Timeline

```
Week 1-2    ███ Foundation (config, logging, errors)
Week 3-4    ███ G-Code Parser
Week 5-6    ███ Serial Connection
Week 7      ██  State Management
Week 8-10   █████ 3D Visualization
Week 11-13  ██████ User Interface
Week 14-15  ████ Height Mapping
Week 16-17  ████ Advanced Features
Week 18-20  ██████ Polish & Testing
```

## Key Features

### Core Functionality
✅ GRBL communication (serial/telnet/websocket)  
✅ G-Code parsing and editing  
✅ 3D toolpath visualization  
✅ Manual machine control (jog)  
✅ Coordinate system management  
✅ Real-time status monitoring  

### Advanced Features
✅ Height map creation & compensation  
✅ Custom scripting (Rhai)  
✅ User-defined commands  
✅ Override controls  
✅ Settings profiles  

## Architecture Pattern

```
User Input → UI → State Manager
                      ↓
                  Connection ← GRBL
                      ↓
                  Parser
                      ↓
                  Renderer → Display
```

## Communication Pattern

```rust
// Command flow
UI --[Command]-> Channel -> Connection -> Hardware

// Status flow  
Hardware -> Connection -> Channel -> UI + Logger + Renderer
```

## File Structure

```
rCandle/
├── src/
│   ├── main.rs              # Entry point
│   ├── connection/          # Serial/network
│   ├── parser/              # G-Code parsing
│   ├── renderer/            # 3D graphics
│   ├── state/               # State management
│   ├── ui/                  # User interface
│   ├── heightmap/           # Height mapping
│   ├── script/              # Scripting engine
│   └── grbl/                # GRBL protocol
├── tests/                   # Integration tests
├── examples/                # Example programs
└── assets/                  # Shaders, icons
```

## Common Tasks

### Build & Run
```bash
cargo build --release        # Build
cargo run                    # Run
cargo test                   # Test
cargo clippy -- -D warnings  # Lint
```

### Development
```bash
cargo watch -x run           # Auto-reload
cargo doc --open             # Generate docs
cargo bench                  # Benchmarks
```

## Key Design Decisions

| Decision | Choice | Why |
|----------|--------|-----|
| UI Framework | **egui + eframe** | Mature, flexible, immediate mode, great for tools |
| Graphics | **WGPU** | Safe, cross-platform, future-proof |
| Async Runtime | **Tokio** | Industry standard, mature |
| Parser | **nom** | Zero-copy, composable |
| Scripting | **Rhai** | Rust-native, safe |

## Performance Targets

- 📊 **Rendering**: 60 FPS with 1M line segments
- ⚡ **Parsing**: <2 seconds for 100k lines
- 🔌 **Serial**: <10ms command latency
- 💾 **Memory**: <500 MB typical usage
- 📦 **Binary**: <50 MB

## Success Criteria

### Must Have (v1.0)
- [ ] Connect to GRBL via serial
- [ ] Load and visualize G-Code
- [ ] Send commands to machine
- [ ] Manual jog controls
- [ ] Height map compensation
- [ ] Cross-platform (Windows, Linux, macOS)

### Quality Metrics
- [ ] Test coverage >70%
- [ ] No crashes during normal use
- [ ] Responsive UI (<16ms frame time)
- [ ] Complete documentation

## Qt to Rust Quick Reference

| Qt | Rust |
|----|------|
| `QString` | `String` or `&str` |
| `QVector<T>` | `Vec<T>` |
| `QMap<K,V>` | `HashMap<K,V>` or `BTreeMap<K,V>` |
| `QSharedPointer<T>` | `Arc<T>` |
| `QMutex` | `Mutex<T>` |
| `QThread` | `tokio::task` |
| Signal/Slot | `mpsc::channel` |
| `QTimer` | `tokio::time::interval` |

## Error Handling

```rust
// Library code: use thiserror
#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Invalid command: {0}")]
    InvalidCommand(String),
}

// Application code: use anyhow
use anyhow::{Context, Result};

fn load_file(path: &Path) -> Result<String> {
    std::fs::read_to_string(path)
        .context("Failed to load G-Code file")
}
```

## Concurrency Pattern

```rust
// Shared state with interior mutability
use std::sync::{Arc, Mutex};

let state = Arc::new(Mutex::new(State::new()));

// Clone for each thread
let state_clone = Arc::clone(&state);
tokio::spawn(async move {
    let mut s = state_clone.lock().unwrap();
    s.update();
});
```

## Testing Strategy

```rust
// Unit tests
#[cfg(test)]
mod tests {
    #[test]
    fn test_parser() { /* ... */ }
}

// Integration tests (in tests/)
#[tokio::test]
async fn test_connection() { /* ... */ }

// Benchmarks (in benches/)
fn bench_parser(c: &mut Criterion) { /* ... */ }
```

## Resources

- 📖 Specification: `.specify/SPECIFICATION.md`
- 🏗️ Architecture: `.specify/ARCHITECTURE.md`
- 🗺️ Roadmap: `.specify/ROADMAP.md`
- 📦 Dependencies: `.specify/DEPENDENCIES.md`
- 🔄 Migration: `.specify/MIGRATION_GUIDE.md`

### GRBL Documentation
- 📘 GRBL 1.1f Documentation: https://github.com/craftweeks/grbl-1.1f.customized-for-laser/tree/master/doc/markdown
  - Commands reference
  - Interface protocol
  - Settings guide
  - Jogging protocol
  - Laser mode
- 📗 GRBL v1.1 Wiki: https://github.com/gnea/grbl/wiki
- 📕 GRBL v0.9 Wiki: https://github.com/grbl/grbl/wiki

## Getting Help

1. Check specification documents
2. Review architecture diagrams
3. Consult migration guide for patterns
4. Search Rust documentation
5. Ask in project discussions

---

**Remember**: Focus on small, incremental changes. Test frequently. Document as you go.

🦀 Happy Rust development!

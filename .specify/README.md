# rCandle Specification Package

This directory contains the complete specification and planning documents for the rCandle project - a Rust migration of the Candle GRBL controller application.

## Document Overview

### 📋 [SPECIFICATION.md](SPECIFICATION.md)
**Complete project specification** - 1,000+ lines

The main specification document covering:
- Project overview and goals
- Functional requirements (GRBL communication, G-Code management, visualization, etc.)
- Architecture components and modules
- Technology stack and dependencies
- Project structure
- Implementation phases (20-week timeline)
- Key design decisions
- Testing strategy
- Performance considerations
- Cross-platform support
- Security and licensing
- Success criteria

**Start here** if you want to understand what rCandle will do and how it will work.

### 🏗️ [ARCHITECTURE.md](ARCHITECTURE.md)
**Technical architecture documentation** - 900+ lines

Detailed technical architecture including:
- System architecture overview with diagrams
- Module breakdown (Connection, Parser, Renderer, State, HeightMap, UI, Script)
- Communication patterns (message passing, async tasks)
- Data flow examples
- Error handling strategy
- Performance optimization strategies
- Testing architecture
- Build and deployment
- Monitoring and logging

**Read this** for in-depth technical design and implementation patterns.

### 🗺️ [ROADMAP.md](ROADMAP.md)
**20-week development roadmap** - 1,000+ lines

Phase-by-phase development plan:
- **Phase 1 (Weeks 1-2)**: Foundation - project setup, logging, configuration
- **Phase 2 (Weeks 3-4)**: G-Code Parser - parsing and preprocessing
- **Phase 3 (Weeks 5-6)**: Connection Module - serial communication
- **Phase 4 (Week 7)**: State Management - application state tracking
- **Phase 5 (Weeks 8-10)**: Visualization Core - 3D rendering with WGPU
- **Phase 6 (Weeks 11-13)**: UI Framework - complete GUI with Iced
- **Phase 7 (Weeks 14-15)**: Height Map - surface scanning and compensation
- **Phase 8 (Weeks 16-17)**: Advanced Features - scripting, user commands
- **Phase 9 (Weeks 18-20)**: Polish & Testing - comprehensive testing and release

Each phase includes:
- Detailed day-by-day tasks
- Deliverables and success criteria
- Testing requirements
- Risk mitigation strategies

**Use this** to track development progress and plan work.

### 📦 [DEPENDENCIES.md](DEPENDENCIES.md)
**Comprehensive dependency analysis** - 500+ lines

Analysis of all Rust crates used:
- Core dependencies (tokio, serialport, wgpu, iced, etc.)
- Rationale for each selection
- Alternative options considered
- License compliance (GPL-3.0 compatible)
- Security considerations
- Platform-specific requirements
- Update strategy

**Consult this** when adding or evaluating dependencies.

### 🔄 [MIGRATION_GUIDE.md](MIGRATION_GUIDE.md)
**C++ to Rust migration patterns** - 800+ lines

Practical guide for translating C++/Qt code to Rust:
- General translation patterns (classes → structs, signals/slots → channels)
- Qt to Rust type mappings (QString → String, QVector → Vec, etc.)
- Memory management (ownership, borrowing, reference counting)
- Concurrency patterns (QThread → tokio::task)
- Error handling (exceptions → Result)
- Common pitfalls and solutions
- Complete class migration example

**Reference this** when migrating specific code from Candle.

## Quick Start Guide

### For Project Managers
1. Read [SPECIFICATION.md](SPECIFICATION.md) sections 1-6 for project overview
2. Review [ROADMAP.md](ROADMAP.md) for timeline and milestones
3. Check success criteria in [SPECIFICATION.md](SPECIFICATION.md) section 14

### For Developers
1. Read [ARCHITECTURE.md](ARCHITECTURE.md) for technical design
2. Review [MIGRATION_GUIDE.md](MIGRATION_GUIDE.md) for coding patterns
3. Check [DEPENDENCIES.md](DEPENDENCIES.md) for available crates
4. Follow [ROADMAP.md](ROADMAP.md) for implementation order

### For Contributors
1. Understand the project via [SPECIFICATION.md](SPECIFICATION.md)
2. Check current phase in [ROADMAP.md](ROADMAP.md)
3. Review coding patterns in [MIGRATION_GUIDE.md](MIGRATION_GUIDE.md)
4. Follow architectural guidelines in [ARCHITECTURE.md](ARCHITECTURE.md)

## Key Numbers

### Project Scope
- **Duration**: 20 weeks (5 months)
- **Phases**: 9 major phases
- **Modules**: 7 core modules
- **Target Platforms**: Windows, Linux, macOS
- **Source Application**: Candle v10.10.2 (1,319 C++ files)

### Technical Stack
- **Language**: Rust 2021 edition
- **UI Framework**: Iced (recommended) or egui
- **Graphics**: WGPU (Vulkan/Metal/DX12)
- **Async Runtime**: Tokio
- **Parser**: nom combinators
- **Scripting**: Rhai

### Goals
- **Performance**: ≥60 FPS rendering, <2s file loading (100k lines)
- **Binary Size**: <50 MB
- **Memory**: <500 MB typical usage
- **Test Coverage**: >70% for core modules
- **Latency**: <10ms serial communication

## Document Status

| Document | Status | Last Updated | Completeness |
|----------|--------|--------------|--------------|
| SPECIFICATION.md | ✅ Complete | 2024 | 100% |
| ARCHITECTURE.md | ✅ Complete | 2024 | 100% |
| ROADMAP.md | ✅ Complete | 2024 | 100% |
| DEPENDENCIES.md | ✅ Complete | 2024 | 100% |
| MIGRATION_GUIDE.md | ✅ Complete | 2024 | 100% |

## Related Files

- **[../Cargo.toml](../Cargo.toml)**: Project dependencies configuration
- **[../README.md](../README.md)**: Main project README
- **[../LICENSE](../LICENSE)**: GPL-3.0 license (when added)

## References

### Original Candle
- Repository: https://github.com/Denvi/Candle
- User Manual: https://github.com/Denvi/Candle/blob/master/doc/help_en.html
- Version: 10.10.2

### GRBL Firmware Documentation
- **Primary Documentation**: https://github.com/craftweeks/grbl-1.1f.customized-for-laser/tree/master/doc/markdown
  - `commands.md` - Complete G-code and $ command reference
  - `interface.md` - Serial protocol and communication specs
  - `settings.md` - GRBL configuration parameters
  - `jogging.md` - Real-time jogging protocol
  - `laser_mode.md` - Laser-specific features
  - `change_summary.md` - Version change history
- GRBL v1.1 Wiki: https://github.com/gnea/grbl/wiki
- GRBL v0.9 Wiki: https://github.com/grbl/grbl/wiki

### Rust Resources
- Rust Book: https://doc.rust-lang.org/book/
- Tokio Tutorial: https://tokio.rs/tokio/tutorial
- WGPU Guide: https://wgpu.rs/
- egui Documentation: https://docs.rs/egui/
- eframe Documentation: https://docs.rs/eframe/

## Version History

- **v1.0** (2024): Initial complete specification package

## How to Use This Specification

### Phase 1: Planning
1. Review all documents to understand scope
2. Validate requirements with stakeholders
3. Set up development environment
4. Initialize repository structure

### Phase 2-9: Implementation
1. Follow ROADMAP.md phase-by-phase
2. Reference ARCHITECTURE.md for design decisions
3. Use MIGRATION_GUIDE.md for code translation
4. Check DEPENDENCIES.md for crate selection
5. Update ROADMAP.md with progress

### Maintenance
- Update ROADMAP.md weekly with progress
- Note deviations from ARCHITECTURE.md
- Document new dependencies in DEPENDENCIES.md
- Keep SPECIFICATION.md as source of truth

## Contributing to Specification

If you need to update these documents:

1. **SPECIFICATION.md**: Changes to requirements, features, or project goals
2. **ARCHITECTURE.md**: Changes to technical design or module structure
3. **ROADMAP.md**: Updates to timeline, task completion, or phase adjustments
4. **DEPENDENCIES.md**: New dependencies or major version updates
5. **MIGRATION_GUIDE.md**: New patterns or improved translation examples

All changes should maintain consistency across documents.

## Questions?

For questions about:
- **Requirements**: See SPECIFICATION.md or open an issue
- **Implementation**: See ARCHITECTURE.md or MIGRATION_GUIDE.md
- **Timeline**: See ROADMAP.md
- **Dependencies**: See DEPENDENCIES.md

---

**This specification package provides everything needed to build rCandle from scratch.**

Happy coding! 🦀

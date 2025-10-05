#!/bin/bash
# rCandle Phase 1 Day 1 Setup Script
# This script automates the initial project structure creation

set -e  # Exit on error

echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║             rCandle Phase 1 - Day 1 Setup                     ║"
echo "╚═══════════════════════════════════════════════════════════════╝"
echo ""

# Check we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Error: Cargo.toml not found. Please run from project root."
    exit 1
fi

echo "✓ Found Cargo.toml"
echo ""

# Create directory structure
echo "📁 Creating directory structure..."
mkdir -p src/{connection,parser,renderer,state,heightmap,script,ui,grbl,utils}
mkdir -p src/bin
mkdir -p tests/{integration,common}
mkdir -p examples
mkdir -p benches
mkdir -p assets/{shaders,icons,fonts}
mkdir -p resources/sample_gcode
mkdir -p docs

echo "✓ Directories created"
echo ""

# Create main.rs
echo "📄 Creating src/main.rs..."
cat > src/main.rs << 'EOF'
//! rCandle - GRBL Controller Application
//! 
//! A Rust-based GRBL controller with G-Code visualization.

fn main() {
    println!("rCandle v{}", env!("CARGO_PKG_VERSION"));
    println!("Initializing...");
    
    // TODO: Initialize application
}
EOF

echo "✓ Created src/main.rs"

# Create lib.rs
echo "📄 Creating src/lib.rs..."
cat > src/lib.rs << 'EOF'
//! rCandle core library
//! 
//! This library provides the core functionality for the rCandle GRBL controller.

#![warn(missing_docs)]
#![warn(clippy::all)]

pub mod connection;
pub mod parser;
pub mod renderer;
pub mod state;
pub mod heightmap;
pub mod script;
pub mod ui;
pub mod grbl;
pub mod utils;

/// Application version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Application name
pub const APP_NAME: &str = env!("CARGO_PKG_NAME");
EOF

echo "✓ Created src/lib.rs"

# Create module files
echo "📄 Creating module files..."
for module in connection parser renderer state heightmap script ui grbl utils; do
    cat > "src/$module/mod.rs" << EOF
//! ${module^} module
//!
//! TODO: Add module documentation

#![allow(dead_code)] // Remove after implementation
EOF
    echo "  ✓ Created src/$module/mod.rs"
done

# Create .gitignore if it doesn't exist
if [ ! -f ".gitignore" ]; then
    echo "📄 Creating .gitignore..."
    cat > .gitignore << 'EOF'
# Rust
/target/
Cargo.lock
**/*.rs.bk
*.pdb

# IDE
.vscode/
.idea/
*.swp
*.swo
*~

# OS
.DS_Store
Thumbs.db

# Build artifacts
*.exe
*.dll
*.so
*.dylib

# Configuration
config.toml
*.local.toml

# Logs
*.log
logs/

# Test outputs
test_results/
coverage/
EOF
    echo "✓ Created .gitignore"
else
    echo "✓ .gitignore already exists"
fi

echo ""
echo "🔨 Building project..."
cargo build

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ Build successful!"
else
    echo ""
    echo "❌ Build failed. Please check errors above."
    exit 1
fi

echo ""
echo "📝 Creating git commit..."
git add .
git commit -m "Initialize Phase 1: Project structure and skeleton

- Created source directory structure
- Added main.rs and lib.rs  
- Created all module stubs
- Added .gitignore (if needed)
- Verified project builds successfully

Phase 1 Day 1 complete."

echo "✓ Changes committed"
echo ""

echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║                    ✨ Day 1 Complete! ✨                       ║"
echo "╚═══════════════════════════════════════════════════════════════╝"
echo ""
echo "Next Steps:"
echo "1. Push changes: git push origin master"
echo "2. Continue with Day 2: See .specify/IMPLEMENTATION_PLAN.md"
echo "3. Track progress: Check off tasks in .specify/PHASE1_CHECKLIST.md"
echo ""
echo "Quick checks:"
echo "  cargo run      # Run the application"
echo "  cargo test     # Run tests (none yet)"
echo "  cargo doc      # Generate documentation"
echo ""

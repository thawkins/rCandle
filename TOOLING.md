# Development Tooling Guide

This document provides instructions for setting up the development tools used in the rCandle project, specifically the GitHub Copilot CLI and GitHub Spec-Kit.

## GitHub Copilot CLI

The GitHub Copilot CLI is an AI-powered command-line tool that helps with various development tasks including code generation, refactoring, debugging, and task automation.

### Prerequisites

- Active GitHub Copilot subscription
- Git installed and configured
- GitHub CLI (`gh`) installed

### Installation

#### 1. Install GitHub CLI (if not already installed)

**Linux/WSL:**
```bash
# Debian/Ubuntu
type -p curl >/dev/null || sudo apt install curl -y
curl -fsSL https://cli.github.com/packages/githubcli-archive-keyring.gpg | sudo dd of=/usr/share/keyrings/githubcli-archive-keyring.gpg \
&& sudo chmod go+r /usr/share/keyrings/githubcli-archive-keyring.gpg \
&& echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main" | sudo tee /etc/apt/sources.list.d/github-cli.list > /dev/null \
&& sudo apt update \
&& sudo apt install gh -y
```

**macOS:**
```bash
brew install gh
```

**Windows:**
```powershell
winget install --id GitHub.cli
# or
choco install gh
```

#### 2. Authenticate GitHub CLI

```bash
gh auth login
```

Follow the prompts to authenticate with your GitHub account.

#### 3. Install GitHub Copilot CLI Extension

```bash
gh extension install github/gh-copilot
```

#### 4. Verify Installation

```bash
gh copilot --version
```

### Usage

The GitHub Copilot CLI provides several commands:

#### General Command Suggestions

```bash
gh copilot suggest "how do I build a rust project"
```

#### Git-Specific Suggestions

```bash
gh copilot suggest -t git "undo last commit"
```

#### Shell Command Suggestions

```bash
gh copilot suggest -t shell "find all rust files modified in last week"
```

#### Explain Commands

```bash
gh copilot explain "cargo build --release"
```

### Aliases (Optional but Recommended)

Add these to your shell configuration file (`~/.bashrc`, `~/.zshrc`, etc.):

```bash
# GitHub Copilot CLI aliases
alias ghcs='gh copilot suggest'
alias ghce='gh copilot explain'
```

## GitHub Spec-Kit

GitHub Spec-Kit is a tool for managing project specifications and breaking down complex development tasks into manageable pieces.

### Prerequisites

- Node.js (v18 or higher)
- npm or yarn
- Active GitHub account

### Installation

#### Using npm (Global Installation)

```bash
npm install -g @github/spec-kit
```

#### Using yarn (Global Installation)

```bash
yarn global add @github/spec-kit
```

#### Verify Installation

```bash
spec-kit --version
```

### Usage

#### Initialize Specification in a Project

```bash
cd /path/to/project
spec-kit init
```

This creates a `.spec` directory with your project specifications.

#### Create a New Specification

```bash
spec-kit create "Feature Name"
```

#### View Specifications

```bash
spec-kit list
```

#### Break Down a Specification into Tasks

```bash
spec-kit plan <spec-id>
```

#### Generate Implementation Plan

```bash
spec-kit implement <spec-id>
```

#### Track Progress

```bash
spec-kit status
```

### Integration with GitHub Copilot CLI

The Spec-Kit and Copilot CLI work together seamlessly:

1. **Create specification** with Spec-Kit
2. **Generate implementation plan** with Spec-Kit
3. **Use Copilot CLI** to implement individual tasks
4. **Track progress** with Spec-Kit

Example workflow:
```bash
# Create specification
spec-kit create "Add GRBL connection feature"

# Generate plan
spec-kit plan spec-001

# Use Copilot to implement
gh copilot suggest "implement serial port connection in Rust using tokio-serial"

# Track progress
spec-kit status
```

## rCandle-Specific Workflow

For the rCandle project, here's the recommended workflow:

### 1. Specification Phase

```bash
# Create specification for new feature
spec-kit create "Your Feature Name"

# Add details to the specification in .spec/ directory
# Edit the generated YAML/Markdown file

# Generate implementation plan
spec-kit plan <spec-id>
```

### 2. Implementation Phase

```bash
# For each task in the plan
gh copilot suggest "how to implement <task description> in Rust"

# Get explanations for unfamiliar code
gh copilot explain "<code snippet>"

# Git operations
gh copilot suggest -t git "create feature branch and commit changes"
```

### 3. Documentation Phase

```bash
# Generate documentation suggestions
gh copilot suggest "how to document this Rust module"

# Update specs
spec-kit update <spec-id>
```

## Tips and Best Practices

### GitHub Copilot CLI

1. **Be specific in prompts**: Include context about the project (e.g., "in Rust using egui")
2. **Use type flags**: `-t shell`, `-t git`, `-t gh` for targeted suggestions
3. **Iterate on suggestions**: If first suggestion isn't perfect, refine your prompt
4. **Explain before modifying**: Use `explain` to understand code before changing it

### GitHub Spec-Kit

1. **Start with high-level specs**: Break down into smaller tasks later
2. **Keep specs up-to-date**: Update as implementation progresses
3. **Link specs to issues**: Reference GitHub issues in specifications
4. **Version control specs**: Commit `.spec` directory changes

## Troubleshooting

### Copilot CLI Issues

**"Not authenticated"**
```bash
gh auth refresh
gh auth status
```

**Extension not found**
```bash
gh extension list
gh extension install github/gh-copilot
```

**Rate limiting**
- Wait a few minutes between requests
- Check your Copilot subscription status

### Spec-Kit Issues

**Command not found**
```bash
# Check installation
npm list -g @github/spec-kit

# Reinstall if needed
npm install -g @github/spec-kit
```

**Permission errors**
```bash
# Use sudo on Linux/macOS if needed
sudo npm install -g @github/spec-kit
```

## Additional Resources

### GitHub Copilot CLI
- [Official Documentation](https://docs.github.com/en/copilot/github-copilot-in-the-cli)
- [GitHub CLI Documentation](https://cli.github.com/manual/)

### GitHub Spec-Kit
- [Spec-Kit Repository](https://github.com/github/spec-kit)
- [Specification Format Guide](https://github.com/github/spec-kit/blob/main/docs/spec-format.md)

### Rust Development
- [Rust Book](https://doc.rust-lang.org/book/)
- [egui Documentation](https://docs.rs/egui/)
- [GRBL Documentation](https://github.com/craftweeks/grbl-1.1f.customized-for-laser/tree/master/doc/markdown)

## Project-Specific Configuration

### Environment Setup for rCandle

```bash
# Clone the repository
git clone https://github.com/yourusername/rCandle.git
cd rCandle

# Install Rust toolchain (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build the project
cargo build

# Run tests
cargo test

# Run the application
cargo run
```

### Using Copilot CLI with rCandle

```bash
# Get help with Rust-specific tasks
gh copilot suggest "how to add a new module to a Rust project"

# GRBL-related queries
gh copilot suggest "how to parse GRBL responses in Rust"

# egui UI questions
gh copilot suggest "how to create a responsive layout in egui"

# Serial communication
gh copilot suggest "how to implement serial port communication in Rust"
```

## Contributing

When contributing to rCandle:

1. Create a specification for your feature using Spec-Kit
2. Use Copilot CLI to assist with implementation
3. Update documentation as you go
4. Commit both code and specification changes

## Version History

- **v1.0** (2024) - Initial tooling documentation
  - GitHub Copilot CLI setup
  - GitHub Spec-Kit integration
  - rCandle-specific workflows

---

For questions or issues with these tools, please:
- Check the official documentation links above
- Open an issue in the rCandle repository
- Consult the GitHub Community Forum

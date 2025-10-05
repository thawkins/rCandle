# GitHub Repository Setup Guide

## Quick Setup Instructions

### Option 1: Using GitHub CLI (Recommended)

If you have GitHub CLI installed:

```bash
cd ~/projects/rCandle

# Create repository on GitHub
gh repo create rCandle --public --source=. --remote=origin \
  --description="A Rust GRBL controller application with G-Code visualizer"

# Push all commits
git push -u origin master
```

### Option 2: Using GitHub Web Interface

1. **Create Repository on GitHub**
   - Go to https://github.com/new
   - Repository name: `rCandle`
   - Description: `A Rust GRBL controller application with G-Code visualizer`
   - Visibility: Public (or Private)
   - **DO NOT** initialize with README, .gitignore, or license (we already have these)
   - Click "Create repository"

2. **Push Local Repository**

After creating the repository, GitHub will show you commands. Use these:

```bash
cd ~/projects/rCandle

# Add GitHub as remote
git remote add origin https://github.com/YOUR_USERNAME/rCandle.git

# Or use SSH (if you have SSH keys set up)
git remote add origin git@github.com:YOUR_USERNAME/rCandle.git

# Push all commits
git push -u origin master

# Push all tags (if any)
git push --tags
```

### Option 3: Using Personal Access Token

If you prefer HTTPS and want to avoid password prompts:

1. Create a Personal Access Token:
   - Go to https://github.com/settings/tokens
   - Click "Generate new token" → "Generate new token (classic)"
   - Select scopes: `repo` (full control of private repositories)
   - Generate and copy the token

2. Push with token:

```bash
cd ~/projects/rCandle

# Add remote with token embedded (less secure, but convenient)
git remote add origin https://YOUR_TOKEN@github.com/YOUR_USERNAME/rCandle.git

# Or add remote normally and use token when prompted
git remote add origin https://github.com/YOUR_USERNAME/rCandle.git
git push -u origin master
# When prompted for password, enter your Personal Access Token
```

## What Will Be Pushed

### Repository Contents

```
rCandle/
├── .github/
│   └── prompts/          # AI assistant prompts (7 files)
├── .specify/
│   ├── memory/           # Project memory
│   ├── scripts/          # Utility scripts
│   ├── templates/        # Document templates
│   ├── SPECIFICATION.md          (25KB) - Complete requirements
│   ├── ARCHITECTURE.md           (24KB) - Technical architecture
│   ├── ROADMAP.md                (24KB) - 20-week timeline
│   ├── DEPENDENCIES.md           (12KB) - Crate analysis
│   ├── MIGRATION_GUIDE.md        (18KB) - C++ to Rust patterns
│   ├── GRBL_RESOURCES.md         (8.2KB) - GRBL documentation
│   ├── IMPLEMENTATION_PLAN.md    (28KB) - Phase 1 details
│   ├── PHASE1_CHECKLIST.md       (5KB) - Task checklist
│   ├── QUICK_REFERENCE.md        (7.4KB) - Quick lookup
│   ├── FILE_STRUCTURE.md         (8.3KB) - Project structure
│   ├── UPDATE_LOG.md             (4KB) - Change history
│   └── README.md                 (7.9KB) - Spec navigation
├── Cargo.toml                    (2.5KB) - Rust project config
├── README.md                     (6.8KB) - Project overview
└── GITHUB_SETUP.md               (THIS FILE)

Total: ~270KB of specification and planning documents
Commits: 7 commits with complete project specification
```

### Statistics

- **Specification Documents**: 12 markdown files
- **Total Size**: ~270KB
- **Total Lines**: ~6,500 lines of documentation
- **Git Commits**: 7 commits
- **Branches**: master (main branch)

## After Pushing

### Recommended GitHub Settings

1. **Branch Protection** (if public repository):
   - Go to Settings → Branches
   - Add rule for `master` branch
   - Require pull request reviews
   - Require status checks to pass

2. **Enable GitHub Actions**:
   - CI/CD will run automatically once you implement Phase 1
   - Workflow is ready in `.github/workflows/ci.yml` (will be created in Phase 1)

3. **Add Topics/Tags**:
   - Go to repository main page
   - Click gear icon next to "About"
   - Add topics: `rust`, `grbl`, `cnc`, `gcode`, `controller`, `visualization`

4. **Add Description**:
   - "A Rust GRBL controller application with G-Code visualizer for CNC machines"

5. **Enable Discussions** (optional):
   - Settings → Features → Discussions

### Repository Links to Share

After pushing, your repository will be available at:
- **HTTPS**: `https://github.com/YOUR_USERNAME/rCandle`
- **SSH**: `git@github.com:YOUR_USERNAME/rCandle.git`

### Clone Instructions for Others

```bash
# HTTPS
git clone https://github.com/YOUR_USERNAME/rCandle.git

# SSH
git clone git@github.com:YOUR_USERNAME/rCandle.git
```

## Verification

After pushing, verify everything uploaded correctly:

```bash
# Check remote is set
git remote -v

# Verify all commits were pushed
git log --oneline origin/master

# Check repository on GitHub
# Visit: https://github.com/YOUR_USERNAME/rCandle
```

You should see:
- ✅ All 7 commits
- ✅ All specification documents in `.specify/`
- ✅ README.md displayed on repository home page
- ✅ Cargo.toml visible

## Troubleshooting

### Issue: "remote origin already exists"

```bash
git remote remove origin
git remote add origin https://github.com/YOUR_USERNAME/rCandle.git
```

### Issue: Authentication failed

```bash
# Use Personal Access Token instead of password
# Or set up SSH keys: https://docs.github.com/en/authentication/connecting-to-github-with-ssh
```

### Issue: "Updates were rejected"

```bash
# Force push (only if you're sure)
git push -u origin master --force
```

### Issue: Large files warning

All files in this repository are documentation (text), so there shouldn't be any large file issues.

## Next Steps After GitHub Setup

1. ✅ Repository is live on GitHub
2. Review the specification at `.specify/SPECIFICATION.md`
3. Start Phase 1 implementation following `.specify/IMPLEMENTATION_PLAN.md`
4. Use `.specify/PHASE1_CHECKLIST.md` to track progress
5. Create feature branches for development:
   ```bash
   git checkout -b phase1-foundation
   # ... make changes ...
   git push -u origin phase1-foundation
   ```

## GitHub Repository Best Practices

### Branching Strategy

```
master (main)          # Stable releases
  └─ develop           # Integration branch
       ├─ phase1-foundation
       ├─ phase2-parser
       └─ feature-xyz
```

### Commit Message Format

```
type(scope): subject

body (optional)

footer (optional)
```

Example:
```
feat(parser): implement G-code tokenizer

- Add nom-based tokenizer
- Handle comments and line numbers
- Add comprehensive tests

Closes #123
```

### Issues and Project Management

Create issues for:
- Each phase implementation
- Bug reports
- Feature requests
- Documentation improvements

Use labels:
- `phase-1`, `phase-2`, etc.
- `bug`, `enhancement`, `documentation`
- `good-first-issue` for newcomers

---

**Ready to push?** Follow the instructions above based on your preferred method!

Last updated: 2024

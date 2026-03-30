# Git Commit Message Generator 📝

An intelligent git commit message generator that analyzes your changes and suggests conventional commit messages. Save time on tedious commit messages and maintain consistent commit history.

## Features

- 🔍 **Automatic Analysis**: Analyzes git diffs to understand what changed
- 💡 **Smart Suggestions**: Generates multiple conventional commit message options
- 📏 **Conventional Commits**: Follows [conventional commits](https://www.conventionalcommits.org/) specification
- 🎯 **Type Detection**: Automatically suggests appropriate commit types (feat, fix, refactor, etc.)
- ⚡ **CLI Tool**: Fast Rust implementation with beautiful terminal output
- 🔧 **Configurable**: Team conventions and custom rules support
- ✅ **Validation**: Validates commit messages against conventional commits spec

## Installation

### From Source

```bash
git clone https://github.com/EonHermes/git-commit-generator.git
cd git-commit-generator
cargo install --path .
```

### Direct Cargo Install

```bash
cargo install git-commit-generator
```

## Usage

### Basic Usage

Navigate to your git repository and run:

```bash
# Analyze unstaged changes and show suggestions
git-commit-gen

# Analyze staged changes
git-commit-gen --stage

# Show suggestions without creating commit
git-commit-gen --suggest

# Verbose output with analysis details
git-commit-gen --verbose
```

### Examples

```bash
# Generate commit for staged changes
$ git add src/main.rs
$ git-commit-gen --stage

🔍 Analyzing changes...

📊 Analysis Summary:
  Files changed: 1
  Lines added: 45
  Lines removed: 12
  Primary type: feat

💡 Suggested Commit Messages:

  1. feat(core): add new feature (57 lines changed)
  2. fix(core): fix bug in core module
  3. refactor(core): refactor single component

# Create commit with first suggestion
$ git-commit-gen --stage
✅ Commit created successfully!
   feat(core): add new feature (57 lines changed)
```

### Command Line Options

| Option | Short | Description |
|--------|-------|-------------|
| `--repo` | `-r` | Path to the git repository (default: current directory) |
| `--stage` | `-s` | Generate commit message for staged changes |
| `--suggest` | `-S` | Show suggestions without creating commit |
| `--type` | `-t` | Filter by conventional commit type |
| `--breaking` | `-b` | Include breaking change indicator |
| `--verbose` | `-v` | Show detailed analysis output |

### Filtering by Type

```bash
# Force feature type regardless of analysis
git-commit-gen --stage --type feat

# Generate fix-type commit
git-commit-gen --stage --type fix
```

### Breaking Changes

```bash
# Mark as breaking change
git-commit-gen --stage --breaking

# Output:
feat(core)!

BREAKING CHANGE: Significant API or behavior change

Changes:
- 3 files modified
- 150 lines added, 45 lines removed
- Affected types: rs, toml
```

## Configuration

Create `.git-commit-gen.json` in your repository root:

```json
{
  "default_type": "feat",
  "max_subject_length": 72,
  "include_scope": true,
  "auto_detect_breaking": true,
  "team_conventions": {
    "allowed_types": ["feat", "fix", "docs", "refactor"],
    "required_scopes": ["core", "api", "ui"],
    "custom_rules": []
  }
}
```

## Conventional Commit Types

The generator supports all conventional commit types:

- `feat`: A new feature
- `fix`: A bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `test`: Test additions or corrections
- `build`: Build system changes
- `ci`: CI configuration changes
- `chore`: Other maintenance tasks
- `revert`: Reverts a previous commit

## How It Works

1. **Diff Analysis**: The tool analyzes your git diff to determine:
   - Files changed and their types
   - Lines added/removed
   - Primary change type based on file patterns
   
2. **Type Detection**: Uses heuristics to suggest the most appropriate commit type:
   - `.rs` files → `feat`, `fix`, or `refactor`
   - Test files → `test`
   - Documentation → `docs`
   - Config files → `chore`

3. **Message Generation**: Creates conventional commit messages with:
   - Appropriate type and scope
   - Concise subject line (max 72 chars)
   - Optional breaking change indicator
   - Detailed body for significant changes

## Integration

### Git Alias

Add to your `~/.gitconfig`:

```ini
[alias]
    commit-gen = "!git-commit-gen --stage"
```

Now you can use:
```bash
git commit-gen
```

### Pre-commit Hook

Create `.git/hooks/pre-commit`:

```bash
#!/bin/bash
# Show suggested commit message before committing
git-commit-gen --stage --suggest || true
```

## Testing

Run the test suite:

```bash
cargo test
```

Expected output:
```
running 10 tests
test analyzer::tests::test_not_a_repo ... ok
test analyzer::tests::test_valid_repo ... ok
test generator::tests::test_generate_subject_length ... ok
test generator::tests::test_validate_conventional_invalid ... ok
test generator::tests::test_validate_conventional_valid ... ok
test config::tests::test_default_config ... ok
test config::tests::test_load_nonexistent ... ok
test config::tests::test_save_and_load ... ok
...

test result: ok. 10 passed
```

## License

MIT - See [LICENSE](LICENSE) for details.

## Contributing

Contributions are welcome! Please read our contributing guidelines before submitting PRs.

## Related Projects

- [Conventional Commits](https://www.conventionalcommits.org/)
- [Commitizen](https://commitizen-tools.github.io/commitizen/)
- [cz-git](https://cz-git.qbb.sh/)

---

Built with ❤️ using Rust

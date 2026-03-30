# Contributing to Git Commit Message Generator

Thank you for your interest in contributing! This document provides guidelines and instructions.

## Development Setup

1. Clone the repository:
```bash
git clone https://github.com/EonHermes/git-commit-generator.git
cd git-commit-generator
```

2. Build the project:
```bash
cargo build
```

3. Run tests:
```bash
cargo test
```

## Code Style

- Follow Rust idioms and best practices
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- All code must pass CI checks

## Pull Request Process

1. Fork the repository
2. Create a feature branch: `git checkout -b feat/amazing-feature`
3. Make your changes
4. Write/update tests as needed
5. Ensure all tests pass: `cargo test`
6. Run clippy: `cargo clippy -- -D warnings`
7. Commit with conventional commits: `feat: add amazing feature`
8. Push to your fork and submit a PR

## Code Review

- Be respectful and constructive
- Explain the reasoning behind changes
- Address all review comments
- Keep PRs focused on single concerns

## Adding Features

Before adding major features:
1. Open an issue to discuss the idea
2. Get maintainer approval
3. Implement with tests

## Bug Reports

When filing a bug report, include:
- Description of the issue
- Steps to reproduce
- Expected vs actual behavior
- Environment details (OS, Rust version)
- Error messages if applicable

## Questions?

Open an issue for questions or join the discussion in existing issues.

---

Thanks for contributing! 🚀

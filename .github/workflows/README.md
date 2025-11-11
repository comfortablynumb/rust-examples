# GitHub Actions Workflows

This directory contains CI/CD workflows for the rust-examples repository.

## CI Workflow

The `ci.yml` workflow runs automatically on:
- Pull requests to `main`
- Pushes to `main`

### Jobs

#### 1. Lint
Ensures code quality and consistency:
- **Formatting check**: Runs `cargo fmt --check` on all projects
- **Clippy**: Runs `cargo clippy` with warnings treated as errors
- Fails the build if any formatting issues or clippy warnings are found

#### 2. Test
Runs all tests across the repository:
- **Unit tests**: Runs `cargo test` on all projects
- Ensures all examples compile and pass their tests
- Verbose output for debugging

#### 3. Build
Builds all projects with multiple Rust versions:
- **Stable Rust**: Production-ready version
- **Beta Rust**: Upcoming release testing
- **Release mode**: Builds with optimizations
- **Run examples**: Executes each example to ensure they work

#### 4. Check Matrix
Cross-platform compatibility testing:
- **Platforms**: Ubuntu, Windows, macOS
- **Quick check**: Runs `cargo check` on all projects
- Ensures examples work on all major operating systems

### Caching Strategy

The workflow uses GitHub Actions caching to speed up builds:
- Cargo registry cache
- Cargo git index cache
- Build artifacts cache

This significantly reduces build times on subsequent runs.

### How It Works

The workflow discovers all Cargo projects by finding `Cargo.toml` files:

```bash
find . -name "Cargo.toml" -not -path "*/target/*"
```

Then runs the appropriate cargo command in each project directory.

### Adding New Examples

When you add a new example:
1. Create a new directory with a `Cargo.toml`
2. The CI will automatically discover and test it
3. No workflow changes needed!

### Local Testing

Before pushing, you can run the same checks locally:

```bash
# Format check
cargo fmt -- --check

# Clippy
cargo clippy -- -D warnings

# Tests
cargo test

# Build
cargo build --release
```

Or test all examples at once:

```bash
# From the repository root
find . -name "Cargo.toml" -not -path "*/target/*" | while read manifest; do
  dir=$(dirname "$manifest")
  echo "Testing $dir"
  (cd "$dir" && cargo fmt -- --check && cargo clippy -- -D warnings && cargo test && cargo build)
done
```

## Troubleshooting

### Formatting Failures
Run `cargo fmt` in the affected project to auto-fix formatting issues.

### Clippy Warnings
Address the specific warnings. Clippy suggestions usually improve code quality.

### Test Failures
Check the test output in the workflow logs for details on which test failed and why.

### Build Failures
Review compiler errors in the workflow logs. The verbose flag provides detailed error messages.

## Badge

The CI status badge in the main README shows the current build status:

```markdown
[![CI](https://github.com/comfortablynumb/rust-examples/workflows/CI/badge.svg)](https://github.com/comfortablynumb/rust-examples/actions)
```

- ✅ Green: All checks passing
- ❌ Red: One or more checks failing
- 🟡 Yellow: Workflow running

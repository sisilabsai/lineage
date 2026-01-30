# Crates.io Publication Guide

This document provides instructions for publishing Lineage to crates.io.

## Pre-Publication Checklist

- [ ] Version bumped in `Cargo.toml` (following semver)
- [ ] `CHANGELOG.md` updated with release notes
- [ ] `CRATES_IO_README.md` reviewed and finalized
- [ ] All tests passing: `cargo test`
- [ ] Documentation builds: `cargo doc --no-deps`
- [ ] Examples run successfully
- [ ] Code formatted: `cargo fmt`
- [ ] No warnings: `cargo clippy`
- [ ] Git repository clean and committed

## Publishing Steps

### 1. Create an Account

If you don't have a crates.io account:

```bash
# Visit https://crates.io and create an account
# Generate API token at https://crates.io/me
```

### 2. Authenticate Locally

```bash
cargo login
# Paste your API token when prompted
```

### 3. Update Version

In `Cargo.toml`:

```toml
[package]
name = "lineage"
version = "0.1.0"  # Update this following semver
```

### 4. Verify Package Contents

```bash
# See what will be published
cargo package --allow-dirty

# List files to be included
cargo package --list
```

### 5. Publish

```bash
# Dry run (recommended first)
cargo publish --dry-run

# Actual publish
cargo publish
```

### 6. Verify on Crates.io

1. Visit https://crates.io/crates/lineage
2. Check documentation: https://docs.rs/lineage
3. Verify examples and badges render correctly

## Version Numbers

Follow semantic versioning:

- **0.1.0** - First stable release (current)
- **0.1.1** - Bug fixes, no API changes
- **0.2.0** - New features, backward compatible
- **1.0.0** - Major API stabilization

## Documentation

The README that appears on crates.io comes from `Cargo.toml`:

```toml
[package]
readme = "CRATES_IO_README.md"
```

This file is optimized for crates.io and differs from the main `README.md`.

## Keywords and Categories

In `Cargo.toml`:

```toml
keywords = ["identity", "consequence", "immutable", "permanent", "trustworthy"]
categories = ["development-tools", "simulation"]
```

## Links in Cargo.toml

```toml
[package]
repository = "https://github.com/sisilabsai/lineage"
documentation = "https://docs.rs/lineage"
homepage = "https://github.com/sisilabsai/lineage"
```

## Important Notes

1. **Once published, versions cannot be deleted** - Only yank (mark as unavailable)
2. **crates.io index is immutable** - All versions remain accessible
3. **API stability** - Breaking changes require new major version
4. **Documentation** - Automatically built and hosted on docs.rs

## Yanking (If Needed)

If a version has a critical issue:

```bash
cargo yank --vers 0.1.0
```

Users with exact version locks will break, but semver-flexible ones will move to next version.

## Post-Publication

### Announce Release

- Tweet/post about release
- Update GitHub release notes
- Announce in Rust forums/Reddit if appropriate
- Share in relevant communities

### Monitor

- Watch for issues/questions on crates.io
- Monitor docs.rs build status
- Track download metrics

## Troubleshooting

### Build Failures

Check with:

```bash
cargo build --target wasm32-unknown-unknown  # Web support
cargo build --target x86_64-pc-windows-gnu   # Windows
cargo build --target x86_64-apple-darwin     # macOS
```

### Documentation Issues

```bash
cargo doc --no-deps --open
cargo doc --no-deps --document-private-items
```

### MSRV Validation

Test with minimum supported Rust version:

```bash
rustup install 1.70
cargo +1.70 build
```

## Release Checklist Template

```markdown
## Version X.Y.Z Release

- [ ] Reviewed all changes since last version
- [ ] Updated version number in Cargo.toml
- [ ] Updated CHANGELOG.md
- [ ] All tests passing: `cargo test`
- [ ] No clippy warnings: `cargo clippy`
- [ ] Code formatted: `cargo fmt`
- [ ] Documentation builds: `cargo doc --no-deps`
- [ ] Examples tested: `cargo run --example <name>`
- [ ] CRATES_IO_README.md updated
- [ ] Git tag created: `git tag vX.Y.Z`
- [ ] Published: `cargo publish`
- [ ] Verified on crates.io
- [ ] Released on GitHub
```

## Questions?

Consult the official guide:  
https://doc.rust-lang.org/cargo/publishing/

---

**Ready to publish?**

1. Ensure checklist is complete
2. Run `cargo publish --dry-run`
3. Run `cargo publish`
4. Verify on crates.io and docs.rs
5. Announce to community

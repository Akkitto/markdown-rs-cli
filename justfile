# Default local sanity gate.
default: sanity

# Local full check, including dirty publish dry-run.
sanity: check publish-dry-run-dirty

# Release-grade full check; requires clean committed state.
release-sanity: check publish-dry-run

# Shared quality gate.
check: fmt-check clippy test build-release

# Apply rustfmt formatting.
fmt:
  @cargo fmt --all

# Verify rustfmt formatting.
fmt-check:
  @cargo fmt --all -- --check

# Run Clippy strictly across all targets/features.
clippy:
  @cargo clippy --all-targets --all-features --locked -- -D warnings

# Run all tests with locked dependencies.
test:
  @cargo test --all-targets --all-features --locked

# Verify optimized release build.
build-release:
  @cargo build --release --locked

# Validate crate packaging from current worktree.
publish-dry-run-dirty:
  @cargo publish --dry-run --locked --allow-dirty

# Validate crate packaging for release.
publish-dry-run:
  @cargo publish --dry-run --locked
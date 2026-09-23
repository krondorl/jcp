# Rust Binary Optimization Case Study

I made an experiment optimizing the release binary size on Windows 11.

## Using cargo.toml settings

```toml
[profile.release]
strip = true  # Strip symbols from the binary
opt-level = "z"  # Optimize for size
lto = true # Enable Link Time Optimization
codegen-units = 1 # Reduce Parallel Code Generation Units to Increase Optimization
panic = "abort" # Remove unwinding machinery
debug = false # Remove debug info completely
```

## With rand crate

- debug size 396k
- release size 137k

## With crates tinyrand and tinyrand-std

- debug size 162k
- release size 122k

## Summary

I could reach a total of 69.2% decrease in binary size with these methods.

Date: 2025-10-11
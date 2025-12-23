# Change: 实现跨平台支持 / Implement Cross-platform Support

## Why

跨平台支持确保 Kode 可以在不同操作系统上运行，扩大用户群。

Cross-platform support ensures Kode can run on different operating systems, expanding user base.

## What Changes

- Linux 测试和优化
- macOS 测试和优化
- Windows 测试和优化
- 跨平台构建
  - x86_64-unknown-linux-gnu
  - x86_64-apple-darwin
  - aarch64-apple-darwin
  - x86_64-pc-windows-msvc

- Linux testing and optimization
- macOS testing and optimization
- Windows testing and optimization
- Cross-platform builds
  - x86_64-unknown-linux-gnu
  - x86_64-apple-darwin
  - aarch64-apple-darwin
  - x86_64-pc-windows-msvc

## Impact

**Affected specs:**
- (all specs - platform compatibility)

**Affected code:**
- `crates/*/src/**/*.rs` (平台特定代码)
- `.github/workflows/build.yml` (跨平台构建)

# Delta Spec: Cross-platform Support

## MODIFIED Requirements

### Requirement: 平台兼容性 / Platform Compatibility
The system SHALL work on multiple operating systems.

系统应在多个操作系统上工作。

#### Scenario: Linux 支持 / Linux Support
- **WHEN** 在 Linux 上运行时
- **THEN** 所有功能正常工作
- **AND** 通过所有测试

- **WHEN** running on Linux
- **THEN** all features work correctly
- **AND** passes all tests

#### Scenario: macOS 支持 / macOS Support
- **WHEN** 在 macOS 上运行时
- **THEN** 所有功能正常工作
- **AND** 支持 Apple Silicon (ARM64)

- **WHEN** running on macOS
- **THEN** all features work correctly
- **AND** supports Apple Silicon (ARM64)

#### Scenario: Windows 支持 / Windows Support
- **WHEN** 在 Windows 上运行时
- **THEN** 所有功能正常工作
- **AND** 路径处理正确

- **WHEN** running on Windows
- **THEN** all features work correctly
- **AND** path handling is correct

### Requirement: 跨平台构建 / Cross-platform Builds
The system SHALL provide binaries for multiple platforms.

系统应提供多平台的 binaries。

#### Scenario: 构建目标 / Build Targets
- **WHEN** 构建发布版本时
- **THEN** 支持 x86_64-unknown-linux-gnu
- **AND** 支持 x86_64-apple-darwin
- **AND** 支持 aarch64-apple-darwin
- **AND** 支持 x86_64-pc-windows-msvc

- **WHEN** building release version
- **THEN** supports x86_64-unknown-linux-gnu
- **AND** supports x86_64-apple-darwin
- **AND** supports aarch64-apple-darwin
- **AND** supports x86_64-pc-windows-msvc

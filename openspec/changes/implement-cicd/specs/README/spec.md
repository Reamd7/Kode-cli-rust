# Delta Spec: CI/CD

## MODIFIED Requirements

### Requirement: GitHub Actions / GitHub Actions
The system SHALL include automated CI/CD workflows.

系统应包含自动化 CI/CD 工作流。

#### Scenario: 自动测试 / Automated Testing
- **WHEN** 提交 PR 时
- **THEN** 自动运行所有测试
- **AND** 运行 clippy 检查
- **AND** 运行格式检查
- **AND** 生成代码覆盖率报告

- **WHEN** submitting PR
- **THEN** automatically runs all tests
- **AND** runs clippy checks
- **AND** runs format checks
- **AND** generates code coverage report

#### Scenario: 自动构建 / Automated Building
- **WHEN** 推送到 main 分支时
- **THEN** 自动构建所有平台
- **AND** 生成 release binaries

- **WHEN** pushing to main branch
- **THEN** automatically builds for all platforms
- **AND** generates release binaries

### Requirement: Release 流程 / Release Process
The system SHALL automate release process.

系统应自动化发布流程。

#### Scenario: 自动版本号 / Automatic Versioning
- **WHEN** 发布新版本时
- **THEN** 使用 release-plz 自动版本号
- **AND** 自动生成 CHANGELOG
- **AND** 自动创建 Git tag

- **WHEN** releasing new version
- **THEN** uses release-plz for automatic versioning
- **AND** automatically generates CHANGELOG
- **AND** automatically creates Git tag

#### Scenario: 自动发布 / Automatic Publishing
- **WHEN** 创建 GitHub Release 时
- **THEN** 自动发布到 crates.io
- **AND** 上传 binaries 到 GitHub Release

- **WHEN** creating GitHub Release
- **THEN** automatically publishes to crates.io
- **AND** uploads binaries to GitHub Release

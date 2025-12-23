# Change: 实现 CI/CD 完善 / Implement CI/CD

## Why

完善的 CI/CD 确保代码质量，自动化发布流程，提高开发效率。

Complete CI/CD ensures code quality, automates release process, and improves development efficiency.

## What Changes

- GitHub Actions 配置
  - 自动测试
  - 自动构建
  - 代码覆盖率
- Release 流程
  - 自动版本号
  - 自动生成 CHANGELOG
  - 自动发布 binaries

- GitHub Actions configuration
  - Automated testing
  - Automated building
  - Code coverage
- Release process
  - Automatic versioning
  - Automatic CHANGELOG generation
  - Automatic binary publishing

## Impact

**Affected specs:**
- (infrastructure - no spec changes)

**Affected code:**
- `.github/workflows/` (新建)
- `.github/release.yaml` (新建)
- `release-plz.toml` (新建)

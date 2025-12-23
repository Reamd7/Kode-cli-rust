# Change: 实现综合测试 / Implement Comprehensive Tests

## Why

全面的测试覆盖确保代码质量，减少 bug，提高可维护性。

Comprehensive test coverage ensures code quality, reduces bugs, and improves maintainability.

## What Changes

- 单元测试覆盖检查
  - 目标：核心逻辑 > 80%
  - 补充缺失测试
- 集成测试
  - 完整工作流测试
  - 多场景测试
- 兼容性测试
  - 加载 TS 版本配置
  - 加载 TS 版本 Agent
  - MCP 互操作测试
- 性能基准测试
  - 启动时间
  - 内存占用
  - UI 渲染性能

- Unit test coverage check
  - Target: core logic > 80%
  - Fill missing tests
- Integration tests
  - Complete workflow tests
  - Multi-scenario tests
- Compatibility tests
  - Load TS version config
  - Load TS version agents
  - MCP interoperability tests
- Performance benchmarks
  - Startup time
  - Memory usage
  - UI rendering performance

## Impact

**Affected specs:**
- (all specs - testing)

**Affected code:**
- `tests/integration/` (新建)
- `tests/compatibility/` (新建)
- `benches/` (新建)
- `crates/*/tests/` (补充)

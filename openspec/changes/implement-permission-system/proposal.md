# Change: 实现权限系统 / Implement Permission System

## Why

权限系统保护用户安全，在执行危险操作前请求用户确认。

Permission system protects user safety by requesting confirmation before dangerous operations.

## What Changes

- 实现 `PermissionManager`
- 实现权限检查逻辑
- 实现权限请求 UI 对话框
- 集成到工具执行流程
- 实现权限缓存（记住选择）

- Implement `PermissionManager`
- Implement permission check logic
- Implement permission request UI dialog
- Integrate into tool execution flow
- Implement permission cache (remember choice)

## Impact

**Affected specs:**
- tui-interface (MODIFIED)
- tool-system (MODIFIED)

**Affected code:**
- `crates/kode-core/src/permission.rs` (新建)
- `crates/kode-ui/src/widgets/permission.rs` (新建)

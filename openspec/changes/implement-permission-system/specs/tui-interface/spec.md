# Delta Spec: Permission System

## ADDED Requirements

### Requirement: PermissionManager / PermissionManager
The system SHALL implement permission manager for dangerous operations.

系统应实现权限管理器用于危险操作。

#### Scenario: 权限检查 / Permission Check
- **WHEN** 工具需要权限时
- **THEN** PermissionManager 检查是否已授权
- **AND** 如果未授权，显示权限请求对话框
- **AND** 等待用户确认
- **AND** 缓存用户选择

- **WHEN** tool requires permission
- **THEN** PermissionManager checks if already authorized
- **AND** if not authorized, shows permission request dialog
- **AND** waits for user confirmation
- **AND** caches user choice

### Requirement: 权限请求 UI / Permission Request UI
The system SHALL implement permission request dialog.

系统应实现权限请求对话框。

#### Scenario: 对话框显示 / Dialog Display
- **WHEN** 显示权限请求时
- **THEN** 显示工具名称和描述
- **AND** 显示参数详情
- **AND** 提供允许/拒绝选项
- **AND** 提供"记住选择"选项

- **WHEN** showing permission request
- **THEN** shows tool name and description
- **AND** shows parameter details
- **AND** provides allow/deny options
- **AND** provides "remember choice" option

# Config Loading Specification

配置加载系统规范 - 管理全局配置和项目配置的加载、合并和持久化。

## Purpose

配置系统负责从多个来源加载和管理 Kode 应用的配置。支持全局配置（`~/.kode.json`）和项目配置（`./.kode.json`）的分层管理，以及项目配置对全局配置的覆盖合并。配置格式与 TypeScript 版本完全兼容。

## ADDED Requirements

### Requirement: 配置文件加载
The system SHALL load configuration from both global (~/.kode.json) and project (./.kode.json) locations.

系统必须能够从全局配置文件（~/.kode.json）和项目配置文件（./.kode.json）加载配置。

#### Scenario: 加载全局配置
- **WHEN** 应用启动时
- **THEN** 系统自动从用户主目录加载 ~/.kode.json
- **AND** 如果文件不存在，使用默认配置

#### Scenario: 加载项目配置
- **WHEN** 用户在项目目录中运行应用
- **THEN** 系统从当前目录加载 ./.kode.json
- **AND** 如果文件不存在，使用空项目配置

#### Scenario: 处理无效 JSON
- **WHEN** 配置文件包含无效的 JSON
- **THEN** 系统返回 ConfigParseError
- **AND** 错误信息包含文件路径和解析错误详情

### Requirement: 配置合并
The system SHALL merge project configuration with global configuration, with project values taking precedence.

系统必须将项目配置与全局配置合并，项目配置的值具有优先权。

#### Scenario: 合并 MCP 服务器配置
- **WHEN** 全局配置和项目配置都定义了 MCP 服务器
- **THEN** 合并后的配置包含所有服务器
- **AND** 项目配置中的服务器可以覆盖全局同名服务器

#### Scenario: 保留全局独立配置
- **WHEN** 项目配置只包含部分字段
- **THEN** 全局配置的其他字段保持不变
- **AND** 合并后的配置是完整有效的

### Requirement: 配置持久化
The system SHALL support saving configuration to file paths.

系统必须支持将配置保存到指定文件路径。

#### Scenario: 保存配置到文件
- **WHEN** 调用保存配置方法
- **THEN** 系统创建必要的父目录
- **AND** 配置以格式化的 JSON 写入文件
- **AND** 使用 camelCase 字段命名

#### Scenario: 处理保存失败
- **WHEN** 目标路径不可写
- **THEN** 系统返回 ConfigSaveError
- **AND** 错误信息包含路径和失败原因

### Requirement: 默认配置
The system SHALL provide sensible default values for all configuration fields.

系统必须为所有配置字段提供合理的默认值。

#### Scenario: 使用默认主题
- **WHEN** 配置文件未指定主题
- **THEN** 默认使用 "dark" 主题

#### Scenario: 使用默认模型指针
- **WHEN** 配置文件未定义模型指针
- **THEN** 所有模型指针（main, task, reasoning, quick）均为 None

#### Scenario: 使用默认 MCP 服务器列表
- **WHEN** 配置文件未定义 MCP 服务器
- **THEN** MCP 服务器列表为空 HashMap

### Requirement: 配置类型定义
The system SHALL support configuration for models, MCP servers, and project settings.

系统必须支持模型配置、MCP 服务器配置和项目设置的类型定义。

#### Scenario: Anthropic 模型配置
- **WHEN** 定义 Anthropic 模型配置
- **THEN** 必须包含 provider 为 "anthropic"
- **AND** 必须包含 model_name 和 api_key
- **AND** 可选包含 base_url、max_tokens、context_length、temperature

#### Scenario: MCP STDIO 服务器配置
- **WHEN** 定义 STDIO 类型的 MCP 服务器
- **THEN** 必须包含 command 和 args 字段
- **AND** 可选包含 env 环境变量映射

#### Scenario: MCP SSE 服务器配置
- **WHEN** 定义 SSE 类型的 MCP 服务器
- **THEN** 必须包含 url 字段

#### Scenario: 项目权限配置
- **WHEN** 定义项目配置
- **THEN** 可选包含 allowed_tools、allowed_commands、allowed_paths
- **AND** 可选包含 enable_architect_tool 标志
- **AND** 可选包含 context 上下文字典

### Requirement: 配置格式兼容性
The system SHALL maintain 100% compatibility with TypeScript version configuration format.

系统必须与 TypeScript 版本配置格式保持 100% 兼容。

#### Scenario: camelCase 字段命名
- **WHEN** 序列化配置到 JSON
- **THEN** 所有字段使用 camelCase 命名
- **AND** 如 modelProfiles、defaultModel、mcpServers

#### Scenario: TS 版本配置可加载
- **WHEN** 加载 TS 版本生成的 .kode.json
- **THEN** 配置成功解析
- **AND** 所有字段正确映射

#### Scenario: Rust 版本配置可被 TS 读取
- **WHEN** Rust 版本写入配置文件
- **THEN** TS 版本可以读取该配置
- **AND** 格式完全兼容

#### Scenario: 配置字段完整性
- **WHEN** 对比配置定义
- **THEN** 包含所有 TS 版本支持的字段
- **AND** 不缺失任何配置选项

### Requirement: 配置文件查找
The system SHALL discover configuration files using a hierarchical search strategy.

系统必须使用分层搜索策略发现配置文件。

#### Scenario: 查找顺序
- **WHEN** 搜索配置文件
- **THEN** 按顺序查找项目配置、全局配置
- **AND** 返回找到的所有有效路径

#### Scenario: 当前目录优先
- **WHEN** 在项目目录运行
- **THEN** ./.kode.json 优先级高于 ~/.kode.json
- **AND** 项目配置覆盖全局配置

### Requirement: 配置验证
The system SHALL validate configuration values.

系统必须验证配置值。

#### Scenario: 模型配置验证
- **WHEN** 定义模型配置
- **THEN** 验证必需字段存在
- **AND** 验证 provider 值有效
- **AND** 验证 API 密钥非空

#### Scenario: MCP 服务器验证
- **WHEN** 定义 MCP 服务器
- **THEN** STDIO 类型必须有 command
- **AND** SSE 类型必须有有效 URL
- **AND** 验证命令可执行

## Configuration Format Specification

### Requirement: .kode.json 格式
The system SHALL support the exact .json format used by TypeScript version.

系统必须支持 TypeScript 版本使用的精确 JSON 格式。

#### Scenario: 完整配置示例
- **WHEN** 定义完整配置
- **THEN** 配置应符合以下格式：
```json
{
  "modelProfiles": {
    "claude-sonnet": {
      "provider": "anthropic",
      "apiKey": "sk-ant-...",
      "modelName": "claude-sonnet-4-5-20250929"
    },
    "deepseek": {
      "provider": "openai-compatible",
      "apiKey": "sk-...",
      "baseURL": "https://api.deepseek.com",
      "modelName": "deepseek-chat"
    }
  },
  "defaultModel": "claude-sonnet",
  "modelPointers": {
    "main": "claude-sonnet",
    "task": "deepseek",
    "reasoning": null,
    "quick": null
  },
  "mcpServers": {
    "filesystem": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-filesystem", "/path"]
    }
  }
}
```

#### Scenario: 项目配置示例
- **WHEN** 定义项目配置
- **THEN** 配置应符合以下格式：
```json
{
  "allowedTools": ["FileRead", "Grep", "Bash"],
  "allowedCommands": ["git", "npm", "cargo"],
  "allowedPaths": ["/Users/.../project"],
  "enableArchitectTool": true,
  "mcpServers": {},
  "context": {
    "project_type": "rust",
    "framework": "tokio"
  }
}
```

## Performance Requirements

### Requirement: 加载性能
The system SHALL load configurations quickly.

系统必须快速加载配置。

#### Scenario: 冷启动加载
- **WHEN** 应用启动时加载配置
- **THEN** 配置加载时间 < 50ms
- **AND** 不阻塞应用启动

#### Scenario: 缓存配置
- **WHEN** 配置已加载
- **THEN** 后续访问从内存读取
- **AND** 无需重新解析文件

## Error Handling Requirements

### Requirement: 错误类型
The system SHALL provide specific error types for configuration failures.

系统必须为配置失败提供特定的错误类型。

#### Scenario: ConfigLoadError
- **WHEN** 文件读取失败
- **THEN** 返回 ConfigLoadError
- **AND** 包含文件路径和 IO 错误原因

#### Scenario: ConfigParseError
- **WHEN** JSON 解析失败
- **THEN** 返回 ConfigParseError
- **AND** 包含文件路径和解析错误详情

#### Scenario: ConfigSaveError
- **WHEN** 文件写入失败
- **THEN** 返回 ConfigSaveError
- **AND** 包含文件路径和写入错误原因

## Non-Goals

本规范不包含：
- 环境变量覆盖配置（未来扩展）
- 配置文件热重载
- 配置验证和约束检查（高级）
- 配置迁移和版本管理
- 远程配置下载

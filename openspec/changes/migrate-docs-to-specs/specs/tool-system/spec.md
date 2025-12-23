# Tool System Specification

工具系统规范 - 定义可执行工具的抽象和管理机制。

## Purpose

工具系统定义了 AI 可以调用的能力抽象。Tool trait 描述了工具的接口，ToolRegistry 管理工具的注册和查找，工具可以过滤可见性，支持权限控制。

## ADDED Requirements

### Requirement: 工具接口
The system SHALL define a Tool trait for executable capabilities.

系统必须定义 Tool trait 用于可执行能力。

#### Scenario: 工具元数据
- **WHEN** 查询工具信息
- **THEN** 工具提供唯一名称
- **AND** 工具提供描述信息
- **AND** 工具提供 JSON Schema 格式的参数定义

#### Scenario: 工具执行
- **WHEN** 调用工具 execute 方法
- **THEN** 接收参数值和执行上下文
- **AND** 返回工具执行结果
- **AND** 支持异步执行

#### Scenario: 权限要求
- **WHEN** 工具需要用户权限
- **THEN** requires_permission 返回 true
- **AND** 系统在执行前请求用户批准

#### Scenario: 危险操作标记
- **WHEN** 工具执行可能危险的操作
- **THEN** 工具标记为危险
- **AND** 系统显示额外的警告

### Requirement: 工具注册表
The system SHALL provide a ToolRegistry for managing available tools.

系统必须提供 ToolRegistry 管理可用工具。

#### Scenario: 注册工具
- **WHEN** 向注册表添加工具
- **THEN** 使用工具名称作为键
- **AND** 同名工具替换已注册的工具
- **AND** 注册表支持并发访问

#### Scenario: 查找工具
- **WHEN** 按名称查找工具
- **THEN** 返回匹配的工具实例
- **AND** 未找到时返回 None

#### Scenario: 列出工具
- **WHEN** 获取所有工具列表
- **THEN** 返回已注册工具的元数据
- **AND** 支持按名称过滤

#### Scenario: 工具过滤
- **WHEN** Agent 有工具过滤列表
- **THEN** 只返回允许的工具
- **AND** "all" 表示所有工具可用

### Requirement: 工具执行上下文
The system SHALL provide ToolContext for tool execution.

系统必须提供 ToolContext 用于工具执行。

#### Scenario: 传递配置信息
- **WHEN** 执行工具
- **THEN** 上下文包含当前配置
- **AND** 上下文包含工作目录信息

#### Scenario: 传递模型信息
- **WHEN** 执行工具
- **THEN** 上下文包含当前使用的模型
- **AND** 上下文支持访问 ModelAdapter

#### Scenario: 传递状态信息
- **WHEN** 执行工具
- **THEN** 上下文包含会话状态
- **AND** 工具可以读取和更新状态

### Requirement: 文件操作工具
The system SHALL provide file operation tools (read, write, edit).

系统必须提供文件操作工具（读取、写入、编辑）。

#### Scenario: 读取文件
- **WHEN** 调用 FileReadTool
- **THEN** 读取指定路径的文件内容
- **AND** 支持按行号范围读取
- **AND** 处理不存在的文件错误

#### Scenario: 写入文件
- **WHEN** 调用 FileWriteTool
- **THEN** 创建或覆盖文件
- **AND** 自动创建必要的父目录
- **AND** 写入前请求权限确认

#### Scenario: 编辑文件
- **WHEN** 调用 FileEditTool
- **THEN** 执行精确的字符串替换
- **AND** 替换失败时返回错误
- **AND** 支持多处替换

#### Scenario: 路径验证
- **WHEN** 文件工具接收路径参数
- **THEN** 验证路径不包含遍历攻击
- **AND** 限制访问范围到允许的路径

### Requirement: 命令执行工具
The system SHALL provide BashTool for executing shell commands.

系统必须提供 BashTool 执行 shell 命令。

#### Scenario: 执行命令
- **WHEN** 调用 BashTool
- **THEN** 在 shell 中执行指定命令
- **AND** 捕获标准输出和标准错误
- **AND** 返回命令退出码

#### Scenario: 超时控制
- **WHEN** 命令执行时间过长
- **THEN** 在超时后终止命令
- **AND** 返回超时错误

#### Scenario: 权限检查
- **WHEN** 执行危险命令
- **THEN** 请求用户明确批准
- **AND** 支持允许/禁止的命令列表

#### Scenario: 防止命令注入
- **WHEN** 构建命令字符串
- **THEN** 正确转义用户输入
- **AND** 不直接拼接未验证的输入

### Requirement: 搜索工具
The system SHALL provide search tools (Grep, Glob).

系统必须提供搜索工具（Grep, Glob）。

#### Scenario: Grep 正则搜索
- **WHEN** 调用 GrepTool
- **THEN** 使用正则表达式搜索文件内容
- **AND** 支持文件类型过滤
- **AND** 支持上下文行 (-A, -B, -C)

#### Scenario: Glob 模式匹配
- **WHEN** 调用 GlobTool
- **THEN** 使用 glob 模式查找文件
- **AND** 支持递归目录搜索
- **AND** 返回匹配的文件路径列表

#### Scenario: 搜索性能
- **WHEN** 搜索大型代码库
- **THEN** 在合理时间内完成
- **AND** 支持增量搜索结果

### Requirement: 参数验证
The system SHALL validate tool parameters against JSON Schema.

系统必须根据 JSON Schema 验证工具参数。

#### Scenario: Schema 验证
- **WHEN** 工具接收参数
- **THEN** 根据 tool_schema 验证参数
- **AND** 类型不匹配时返回错误
- **AND** 必填字段缺失时返回错误

#### Scenario: 默认值处理
- **WHEN** 参数有默认值
- **THEN** 未提供时使用默认值
- **AND** 默认值在 schema 中定义

#### Scenario: 自定义验证
- **WHEN** 工具需要额外验证
- **THEN** 可以实现自定义验证逻辑
- **AND** 验证失败返回详细错误信息

## Non-Goals

本规范不包含：
- MCP 工具的动态加载（见 mcp-integration）
- 工具结果缓存（未来扩展）
- 并发工具执行（未来扩展）
- 工具链和复合工具（未来扩展）

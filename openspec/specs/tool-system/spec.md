# 工具系统规范 / Tool System Specification

## Purpose

工具系统提供可扩展的工具框架，允许 AI Agent 调用各种功能（文件操作、命令执行、搜索等）来完成任务。

The tool system provides an extensible tool framework, allowing AI agents to call various functions (file operations, command execution, search, etc.) to accomplish tasks.

## Requirements

### Requirement: Tool trait 定义 / Tool Trait Definition
The system SHALL define a Tool trait that all tools must implement.

系统应定义所有工具必须实现的 Tool trait。

#### Scenario: 工具基本信息 / Tool Basic Information
- **WHEN** 查询工具信息时
- **THEN** 工具提供名称、描述和参数 schema
- **AND** schema 遵循 JSON Schema 格式

- **WHEN** querying tool information
- **THEN** the tool provides name, description, and parameter schema
- **AND** the schema follows JSON Schema format

#### Scenario: 工具执行 / Tool Execution
- **WHEN** 调用工具的 execute 方法时
- **THEN** 工具执行其功能
- **AND** 返回执行结果

- **WHEN** the tool's execute method is called
- **THEN** the tool performs its function
- **AND** returns the execution result

#### Scenario: 权限要求 / Permission Requirements
- **WHEN** 工具执行需要用户确认时
- **THEN** 工具标记 requires_permission 为 true
- **AND** 系统在执行前请求用户批准

- **WHEN** a tool execution requires user confirmation
- **THEN** the tool marks requires_permission as true
- **AND** the system requests user approval before execution

### Requirement: 工具注册表 / Tool Registry
The system SHALL provide a registry for managing available tools.

系统应提供注册表来管理可用工具。

#### Scenario: 注册工具 / Register Tool
- **WHEN** 添加新工具到注册表时
- **THEN** 工具被存储并可通过名称访问
- **AND** 不允许重复名称的工具

- **WHEN** adding a new tool to the registry
- **THEN** the tool is stored and accessible by name
- **AND** tools with duplicate names are not allowed

#### Scenario: 获取工具 / Get Tool
- **WHEN** 通过名称请求工具时
- **THEN** 返回对应的工具实例
- **AND** 如果工具不存在则返回 None

- **WHEN** requesting a tool by name
- **THEN** return the corresponding tool instance
- **AND** return None if the tool does not exist

#### Scenario: 列出工具 / List Tools
- **WHEN** 请求所有工具列表时
- **THEN** 返回所有已注册工具的名称和描述

- **WHEN** requesting a list of all tools
- **THEN** return the names and descriptions of all registered tools

#### Scenario: 过滤工具 / Filter Tools
- **WHEN** 请求特定工具子集时
- **THEN** 仅返回指定名称的工具
- **AND** 忽略不存在的工具名称

- **WHEN** requesting a specific subset of tools
- **THEN** return only the tools with specified names
- **AND** ignore non-existent tool names

### Requirement: 文件操作工具 / File Operation Tools
The system SHALL provide tools for reading, writing, and editing files.

系统应提供读取、写入和编辑文件的工具。

#### Scenario: 读取文件 / Read File
- **WHEN** 使用 FileReadTool 读取文件时
- **THEN** 返回文件内容
- **AND** 支持相对路径和绝对路径
- **AND** 如果文件不存在则返回错误

- **WHEN** using FileReadTool to read a file
- **THEN** return the file content
- **AND** support both relative and absolute paths
- **AND** return an error if the file does not exist

#### Scenario: 写入文件 / Write File
- **WHEN** 使用 FileWriteTool 写入文件时
- **THEN** 创建或覆盖文件
- **AND** 自动创建必要的目录
- **AND** 请求用户确认

- **WHEN** using FileWriteTool to write a file
- **THEN** create or overwrite the file
- **AND** automatically create necessary directories
- **AND** request user confirmation

#### Scenario: 编辑文件 / Edit File
- **WHEN** 使用 FileEditTool 编辑文件时
- **THEN** 执行精确的字符串替换
- **AND** 如果未找到匹配字符串则返回错误

- **WHEN** using FileEditTool to edit a file
- **THEN** perform exact string replacement
- **AND** return an error if the matching string is not found

### Requirement: 搜索工具 / Search Tools
The system SHALL provide tools for searching file content and matching file patterns.

系统应提供搜索文件内容和匹配文件模式的工具。

#### Scenario: 内容搜索 / Content Search
- **WHEN** 使用 GrepTool 搜索内容时
- **THEN** 返回匹配的行和上下文
- **AND** 支持正则表达式
- **AND** 支持文件类型过滤

- **WHEN** using GrepTool to search content
- **THEN** return matching lines and context
- **AND** support regular expressions
- **AND** support file type filtering

#### Scenario: 文件匹配 / File Pattern Matching
- **WHEN** 使用 GlobTool 匹配文件时
- **THEN** 返回匹配模式的文件路径列表
- **AND** 支持通配符模式

- **WHEN** using GlobTool to match files
- **THEN** return a list of file paths matching the pattern
- **AND** support wildcard patterns

### Requirement: 命令执行工具 / Command Execution Tool
The system SHALL provide a tool for executing shell commands.

系统应提供执行 shell 命令的工具。

#### Scenario: 执行命令 / Execute Command
- **WHEN** 使用 BashTool 执行命令时
- **THEN** 在子进程中运行命令
- **AND** 捕获标准输出和标准错误
- **AND** 返回退出码和输出

- **WHEN** using BashTool to execute a command
- **THEN** run the command in a subprocess
- **AND** capture stdout and stderr
- **AND** return exit code and output

#### Scenario: 超时控制 / Timeout Control
- **WHEN** 命令执行超过超时时间时
- **THEN** 终止进程
- **AND** 返回超时错误

- **WHEN** command execution exceeds timeout
- **THEN** terminate the process
- **AND** return a timeout error

#### Scenario: 权限检查 / Permission Check
- **WHEN** 执行潜在危险的命令时
- **THEN** 请求用户确认
- **AND** 显示将要执行的命令

- **WHEN** executing potentially dangerous commands
- **THEN** request user confirmation
- **AND** display the command to be executed

### Requirement: Task 工具 / Task Tool
The system SHALL provide a tool for delegating tasks to sub-agents.

系统应提供将任务委托给子 Agent 的工具。

#### Scenario: 委托任务 / Delegate Task
- **WHEN** 使用 TaskTool 委托任务时
- **THEN** 加载指定的子 Agent
- **AND** 使用过滤后的工具集创建子上下文
- **AND** 返回子 Agent 的响应

- **WHEN** using TaskTool to delegate a task
- **THEN** load the specified sub-agent
- **AND** create a sub-context with filtered tool set
- **AND** return the sub-agent's response

#### Scenario: 工具过滤 / Tool Filtering
- **WHEN** 子 Agent 定义了工具过滤器时
- **THEN** 仅向子 Agent 暴露允许的工具
- **AND** 隐藏其他工具

- **WHEN** a sub-agent defines a tool filter
- **THEN** only expose allowed tools to the sub-agent
- **AND** hide other tools

## Non-Goals

- 本规范不包含工具的权限管理详细实现
- 不包含工具执行的并发控制
- 不包含 MCP 工具的动态加载（在 mcp-integration 规范中）

- This specification does not include detailed implementation of tool permission management
- Does not include concurrency control for tool execution
- Does not include dynamic loading of MCP tools (in mcp-integration specification)

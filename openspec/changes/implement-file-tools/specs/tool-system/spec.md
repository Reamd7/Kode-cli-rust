# Delta Spec: Basic File Tools

## ADDED Requirements

### Requirement: Tool trait / Tool Trait
The system SHALL define a trait for tools.

系统应定义工具的 trait。

#### Scenario: Tool trait 定义 / Tool Trait Definition
- **WHEN** 定义 Tool trait 时
- **THEN** 包含 name() 方法 -> &str
- **AND** 包含 description() 方法 -> &str
- **AND** 包含 schema() 方法 -> ToolSchema
- **AND** 包含 execute() 方法 -> async fn
- **AND** 包含 requires_permission() 方法 -> bool (默认 false)

- **WHEN** defining Tool trait
- **THEN** includes name() method -> &str
- **AND** includes description() method -> &str
- **AND** includes schema() method -> ToolSchema
- **AND** includes execute() method -> async fn
- **AND** includes requires_permission() method -> bool (default false)

### Requirement: ToolRegistry / ToolRegistry
The system SHALL implement a tool registry.

系统应实现工具注册表。

#### Scenario: 工具注册 / Tool Registration
- **WHEN** 调用 register() 时
- **THEN** 将工具添加到 HashMap
- **AND** 使用工具名称作为 key

- **WHEN** calling register()
- **THEN** adds tool to HashMap
- **AND** uses tool name as key

#### Scenario: 工具查找 / Tool Lookup
- **WHEN** 调用 get(name) 时
- **THEN** 返回对应的工具引用
- **AND** 如果不存在返回 None

- **WHEN** calling get(name)
- **THEN** returns reference to corresponding tool
- **AND** returns None if not found

### Requirement: FileReadTool / FileReadTool
The system SHALL implement file reading tool.

系统应实现文件读取工具。

#### Scenario: 读取文件 / Read File
- **WHEN** 执行 FileReadTool 时
- **THEN** 接受 file_path 参数
- **AND** 支持可选的 offset 和 limit 参数
- **AND** 返回文件内容

- **WHEN** executing FileReadTool
- **THEN** accepts file_path parameter
- **AND** supports optional offset and limit parameters
- **AND** returns file content

### Requirement: FileWriteTool / FileWriteTool
The system SHALL implement file writing tool.

系统应实现文件写入工具。

#### Scenario: 写入文件 / Write File
- **WHEN** 执行 FileWriteTool 时
- **THEN** 接受 file_path 和 content 参数
- **AND** 创建父目录（如不存在）
- **AND** 写入文件内容

- **WHEN** executing FileWriteTool
- **THEN** accepts file_path and content parameters
- **AND** creates parent directories (if not exist)
- **AND** writes file content

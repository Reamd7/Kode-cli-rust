# Delta Spec: Basic File Tools

> **Delta 说明 / Delta Note**
>
> 本 Delta Spec 扩展了 `tool-system` 规范，添加基础文件工具的具体实现需求。
>
> This Delta Spec extends the `tool-system` specification, adding specific implementation requirements for basic file tools.

---

## ADDED Requirements

### Requirement: Extended Tool trait / 扩展的 Tool trait

The system SHALL extend the Tool trait with additional metadata and permission methods.

系统应扩展 Tool trait，添加额外的元数据和权限方法。

#### Scenario: 工具元数据查询 / Tool Metadata Query
- **WHEN** 查询工具信息时
- **THEN** 工具提供 name() -> &str
- **AND** 工具提供 description() -> &str
- **AND** 工具提供 schema() -> ToolSchema (JSON Schema 格式)

- **WHEN** querying tool information
- **THEN** tool provides name() -> &str
- **AND** tool provides description() -> &str
- **AND** tool provides schema() -> ToolSchema (JSON Schema format)

#### Scenario: 权限和并发标志 / Permission and Concurrency Flags
- **WHEN** 检查工具属性时
- **THEN** is_read_only() 返回是否为只读工具
- **AND** is_concurrency_safe() 返回是否可并发执行
- **AND** needs_permission(params) 返回是否需要用户确认

- **WHEN** checking tool attributes
- **THEN** is_read_only() returns whether tool is read-only
- **AND** is_concurrency_safe() returns whether tool can run concurrently
- **AND** needs_permission(params) returns whether user confirmation is required

#### Scenario: 输入验证 / Input Validation
- **WHEN** 执行工具前验证参数时
- **THEN** validate_input(params, context) 返回 ValidationResult
- **AND** ValidationResult 包含 result: bool
- **AND** ValidationResult 包含可选的 message: String
- **AND** ValidationResult 包含可选的 meta: Value

- **WHEN** validating parameters before tool execution
- **THEN** validate_input(params, context) returns ValidationResult
- **AND** ValidationResult includes result: bool
- **AND** ValidationResult includes optional message: String
- **AND** ValidationResult includes optional meta: Value

---

### Requirement: ToolContext / ToolContext

The system SHALL provide execution context for tools.

系统应为工具提供执行上下文。

#### Scenario: 上下文初始化 / Context Initialization
- **WHEN** 创建 ToolContext 时
- **THEN** 包含 cwd: PathBuf (当前工作目录)
- **AND** 包含 read_timestamps: HashMap<PathBuf, u64> (文件读取时间戳)
- **AND** 包含 safe_mode: bool (安全模式标志)
- **AND** 包含 abort: AbortHandle (取消支持)

- **WHEN** creating ToolContext
- **THEN** includes cwd: PathBuf (current working directory)
- **AND** includes read_timestamps: HashMap<PathBuf, u64> (file read timestamps)
- **AND** includes safe_mode: bool (safe mode flag)
- **AND** includes abort: AbortHandle (cancellation support)

#### Scenario: 文件时间戳追踪 / File Timestamp Tracking
- **WHEN** 读取文件时
- **THEN** 调用 context.track_read(path) 记录读取时间
- **AND** 存储文件的 mtime 到 read_timestamps

- **WHEN** reading a file
- **THEN** call context.track_read(path) to record read time
- **AND** store file's mtime in read_timestamps

---

### Requirement: ToolResult / ToolResult

The system SHALL provide structured tool execution results.

系统应提供结构化的工具执行结果。

#### Scenario: 结果构造 / Result Construction
- **WHEN** 构造 ToolResult 时
- **THEN** 包含 output: String (输出内容)
- **AND** 包含可选的 metadata: Value (元数据)
- **AND** 包含 is_error: bool (错误标志)

- **WHEN** constructing ToolResult
- **THEN** includes output: String (output content)
- **AND** includes optional metadata: Value (metadata)
- **AND** includes is_error: bool (error flag)

---

### Requirement: ToolRegistry Extensions / ToolRegistry 扩展

The system SHALL extend ToolRegistry with filtering and querying capabilities.

系统应扩展 ToolRegistry，添加过滤和查询功能。

#### Scenario: 工具过滤 / Tool Filtering
- **WHEN** 调用 filter(predicate) 时
- **THEN** 返回满足条件的工具列表
- **AND** predicate 是闭包 Fn(&Arc<dyn Tool>) -> bool

- **WHEN** calling filter(predicate)
- **THEN** returns list of tools satisfying condition
- **AND** predicate is closure Fn(&Arc<dyn Tool>) -> bool

#### Scenario: 列出只读工具 / List Read-Only Tools
- **WHEN** 调用 list_read_only() 时
- **THEN** 返回所有只读工具的名称
- **AND** 用于数据浏览场景

- **WHEN** calling list_read_only()
- **THEN** returns names of all read-only tools
- **AND** used for data browsing scenarios

#### Scenario: 批量获取工具 / Batch Tool Retrieval
- **WHEN** 调用 get_by_names(names) 时
- **THEN** 返回指定名称的工具列表
- **AND** 忽略不存在的工具

- **WHEN** calling get_by_names(names)
- **THEN** returns list of tools with specified names
- **AND** ignores non-existent tools

---

### Requirement: SecureFileService / 安全文件服务

The system SHALL provide secure file operations service.

系统应提供安全文件操作服务。

#### Scenario: 路径验证 / Path Validation
- **WHEN** 调用 validate_path(path) 时
- **THEN** 规范化路径（解析 . 和 ..）
- **AND** 检测路径遍历攻击（.., ~）
- **AND** 检测可疑模式（正则表达式）
- **AND** 检查路径长度限制（4096 字符）
- **AND** 验证路径在白名单中
- **AND** 返回验证后的绝对路径或错误

- **WHEN** calling validate_path(path)
- **THEN** normalizes path (resolves . and ..)
- **AND** detects path traversal attacks (.., ~)
- **AND** detects suspicious patterns (regex)
- **AND** checks path length limit (4096 chars)
- **AND** validates path is in whitelist
- **AND** returns validated absolute path or error

#### Scenario: 安全文件信息获取 / Safe File Info Retrieval
- **WHEN** 调用 safe_get_file_info(path) 时
- **THEN** 返回 FileInfo { size, modified, is_file, is_dir }
- **AND** 如果路径不安全则返回错误

- **WHEN** calling safe_get_file_info(path)
- **THEN** returns FileInfo { size, modified, is_file, is_dir }
- **AND** returns error if path is unsafe

#### Scenario: 安全文件读取 / Safe File Reading
- **WHEN** 调用 safe_read_file(path, options) 时
- **THEN** 支持 encoding 选项（UTF-8, UTF-16 等）
- **AND** 支持 max_size 限制
- **AND** 返回文件内容或错误

- **WHEN** calling safe_read_file(path, options)
- **THEN** supports encoding option (UTF-8, UTF-16, etc.)
- **AND** supports max_size limit
- **AND** returns file content or error

---

### Requirement: File Utilities / 文件工具函数

The system SHALL provide file utility functions.

系统应提供文件工具函数。

#### Scenario: 编码检测 / Encoding Detection
- **WHEN** 调用 detect_encoding(path) 时
- **THEN** 检测文件编码（UTF-8, UTF-16, ASCII 等）
- **AND** 返回编码名称

- **WHEN** calling detect_encoding(path)
- **THEN** detects file encoding (UTF-8, UTF-16, ASCII, etc.)
- **AND** returns encoding name

#### Scenario: 行结束符检测 / Line Ending Detection
- **WHEN** 调用 detect_line_endings(content) 时
- **THEN** 返回 CRLF 或 LF
- **AND** 检测混合模式并返回 Mixed

- **WHEN** calling detect_line_endings(content)
- **THEN** returns CRLF or LF
- **AND** detects mixed patterns and returns Mixed

#### Scenario: 行号添加 / Line Numbering
- **WHEN** 调用 add_line_numbers(content, start_line) 时
- **THEN** 格式化为 `  1 | content`
- **AND** 行号右对齐
- **AND** 从 start_line 开始编号

- **WHEN** calling add_line_numbers(content, start_line)
- **THEN** formats as `  1 | content`
- **AND** line numbers are right-aligned
- **AND** numbering starts from start_line

#### Scenario: 分块文本读取 / Chunked Text Reading
- **WHEN** 调用 read_text_content(path, offset, limit) 时
- **THEN** 返回 ReadResult { content, line_count, total_lines, start_line }
- **AND** offset 是 0-based 起始行号
- **AND** limit 是可选的行数限制

- **WHEN** calling read_text_content(path, offset, limit)
- **THEN** returns ReadResult { content, line_count, total_lines, start_line }
- **AND** offset is 0-based starting line number
- **AND** limit is optional line count limit

---

### Requirement: FileReadTool / FileReadTool

The system SHALL implement file reading tool.

系统应实现文件读取工具。

#### Scenario: 读取文本文件 / Read Text File
- **WHEN** 执行 FileReadTool 时
- **THEN** 接受 file_path, offset, limit 参数
- **AND** 验证文件存在
- **AND** 验证文件大小（text: 250KB, image: 3.75MB）
- **AND** 大文件强制提供 offset 或 limit
- **AND** 返回带行号的内容
- **AND** 记录文件读取时间戳

- **WHEN** executing FileReadTool
- **THEN** accepts file_path, offset, limit parameters
- **AND** validates file exists
- **AND** validates file size (text: 250KB, image: 3.75MB)
- **AND** requires offset or limit for large files
- **AND** returns content with line numbers
- **AND** records file read timestamp

#### Scenario: 读取图片文件 / Read Image File
- **WHEN** 读取图片文件（PNG, JPG, GIF, BMP, WebP）时
- **THEN** 返回 Base64 编码的内容
- **AND** 验证图片尺寸（MAX: 2000x2000）
- **AND** 如果尺寸过大，提示压缩

- **WHEN** reading image file (PNG, JPG, GIF, BMP, WebP)
- **THEN** returns Base64 encoded content
- **AND** validates image dimensions (MAX: 2000x2000)
- **AND** prompts compression if dimensions too large

#### Scenario: 文件不存在 / File Not Found
- **WHEN** 文件不存在时
- **THEN** 返回错误消息
- **AND** 尝试查找相似文件名
- **AND** 在错误消息中建议相似文件

- **WHEN** file does not exist
- **THEN** returns error message
- **AND** attempts to find similar filename
- **AND** suggests similar file in error message

---

### Requirement: FileWriteTool / FileWriteTool

The system SHALL implement file writing tool.

系统应实现文件写入工具。

#### Scenario: 创建新文件 / Create New File
- **WHEN** 写入不存在的文件时
- **THEN** 自动创建父目录
- **AND** 使用 UTF-8 编码
- **AND** 检测仓库行结束符（CRLF/LF）
- **AND** 写入文件内容
- **AND** 记录文件时间戳

- **WHEN** writing to non-existent file
- **THEN** automatically creates parent directories
- **AND** uses UTF-8 encoding
- **AND** detects repository line endings (CRLF/LF)
- **AND** writes file content
- **AND** records file timestamp

#### Scenario: 更新现有文件 / Update Existing File
- **WHEN** 写入已存在的文件时
- **THEN** 验证文件已被读取（检查时间戳）
- **AND** 验证文件未被修改（mtime <= read_timestamp）
- **AND** 检测文件编码
- **AND** 检测行结束符
- **AND** 保持原有编码和行结束符
- **AND** 写入文件内容
- **AND** 更新文件时间戳

- **WHEN** writing to existing file
- **THEN** validates file has been read (check timestamp)
- **AND** validates file not modified (mtime <= read_timestamp)
- **AND** detects file encoding
- **AND** detects line endings
- **AND** preserves original encoding and line endings
- **AND** writes file content
- **AND** updates file timestamp

#### Scenario: 文件被修改 / File Modified
- **WHEN** 文件在读取后被修改时
- **THEN** 拒绝写入操作
- **AND** 返回错误消息
- **AND** 提示用户重新读取文件

- **WHEN** file was modified after read
- **THEN** rejects write operation
- **AND** returns error message
- **AND** prompts user to re-read file

---

### Requirement: FileEditTool / FileEditTool

The system SHALL implement file editing tool.

系统应实现文件编辑工具。

#### Scenario: 字符串替换 / String Replacement
- **WHEN** 执行 FileEditTool 时
- **THEN** 接受 file_path, old_string, new_string 参数
- **AND** 验证文件已被读取
- **AND** 验证文件未被修改
- **AND** 验证 old_string 存在
- **AND** 验证 old_string 只出现一次
- **AND** 执行精确字符串替换
- **AND** 计算差异（diff）
- **AND** 返回编辑片段（前后 N 行上下文）

- **WHEN** executing FileEditTool
- **THEN** accepts file_path, old_string, new_string parameters
- **AND** validates file has been read
- **AND** validates file not modified
- **AND** validates old_string exists
- **AND** validates old_string appears only once
- **AND** performs exact string replacement
- **AND** computes diff
- **AND** returns edit snippet (N lines before/after context)

#### Scenario: 多匹配检测 / Multiple Matches Detection
- **WHEN** old_string 在文件中出现多次时
- **THEN** 拒绝编辑操作
- **AND** 返回错误消息
- **AND** 提示添加更多上下文

- **WHEN** old_string appears multiple times in file
- **THEN** rejects edit operation
- **AND** returns error message
- **AND** prompts to add more context

#### Scenario: 创建新文件 / Create New File via Edit
- **WHEN** old_string 为空字符串时
- **THEN** 允许创建新文件
- **AND** 使用 new_string 作为文件内容
- **AND** 文件不存在时创建
- **AND** 文件存在时返回错误

- **WHEN** old_string is empty string
- **THEN** allows creating new file
- **AND** uses new_string as file content
- **AND** creates file if not exists
- **AND** returns error if file exists

---

### Requirement: Parameter Validation / 参数验证

The system SHALL validate tool parameters.

系统应验证工具参数。

#### Scenario: JSON Schema 验证 / JSON Schema Validation
- **WHEN** 验证参数时
- **THEN** 使用 JSON Schema 验证参数类型
- **AND** 验证必需字段存在
- **AND** 验证字段类型正确
- **AND** 验证字段格式正确

- **WHEN** validating parameters
- **THEN** uses JSON Schema to validate parameter types
- **AND** validates required fields exist
- **AND** validates field types are correct
- **AND** validates field formats are correct

#### Scenario: 自定义验证 / Custom Validation
- **WHEN** JSON Schema 验证通过后
- **THEN** 执行工具特定的验证逻辑
- **AND** 验证文件存在性
- **AND** 验证文件大小
- **AND** 验证文件新鲜度
- **AND** 验证业务逻辑

- **WHEN** JSON Schema validation passes
- **THEN** executes tool-specific validation logic
- **AND** validates file existence
- **AND** validates file size
- **AND** validates file freshness
- **AND** validates business logic

---

## MODIFIED Requirements

本次变更不修改任何现有的 Requirement。

This change does not modify any existing Requirements.

---

## REMOVED Requirements

本次变更不删除任何现有的 Requirement。

This change does not remove any existing Requirements.

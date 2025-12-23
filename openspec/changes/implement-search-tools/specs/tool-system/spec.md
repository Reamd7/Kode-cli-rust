# Delta Spec: Search Tools

## ADDED Requirements

### Requirement: GrepTool / GrepTool
The system SHALL implement content search tool using regex.

系统应实现基于正则表达式的内容搜索工具。

#### Scenario: 正则搜索 / Regex Search
- **WHEN** 执行 GrepTool 时
- **THEN** 接受 pattern 参数（正则表达式）
- **AND** 接受可选的 file_type 参数
- **AND** 支持上下文行参数 (-A, -B, -C)
- **AND** 返回匹配结果（包含行号）

- **WHEN** executing GrepTool
- **THEN** accepts pattern parameter (regex)
- **AND** accepts optional file_type parameter
- **AND** supports context line parameters (-A, -B, -C)
- **AND** returns matches with line numbers

### Requirement: GlobTool / GlobTool
The system SHALL implement file pattern matching tool.

系统应实现文件模式匹配工具。

#### Scenario: 文件匹配 / File Matching
- **WHEN** 执行 GlobTool 时
- **THEN** 接受 pattern 参数（支持通配符）
- **AND** 使用 glob crate 进行匹配
- **AND** 返回匹配的文件路径列表

- **WHEN** executing GlobTool
- **THEN** accepts pattern parameter (supports wildcards)
- **AND** uses glob crate for matching
- **AND** returns list of matched file paths

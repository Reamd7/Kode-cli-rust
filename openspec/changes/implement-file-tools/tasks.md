# 实现任务 - 基础文件工具 / Implementation Tasks - Basic File Tools

## Phase 1: 核心 Tool Trait 和 Registry 扩展

### 1.1 扩展 Tool trait
- [ ] 1.1.1 添加 `description()` 方法 -> &str
- [ ] 1.1.2 添加 `is_read_only()` 方法 -> bool (默认 false)
- [ ] 1.1.3 添加 `is_concurrency_safe()` 方法 -> bool (默认 false)
- [ ] 1.1.4 添加 `needs_permission(&self, params: &Value) -> bool` 方法
- [ ] 1.1.5 添加 `validate_input(&self, params: &Value, context: &ToolContext) -> Result<ValidationResult>` 方法
- [ ] 1.1.6 更新 `ToolSchema` 结构支持 JSON Schema 格式
- [ ] 1.1.7 添加完整的 rustdoc 文档和示例

**参考**: `/Users/gemini/Documents/backup/Kode-cli/src/Tool.ts` (lines 1-80)

### 1.2 扩展 ToolContext
- [ ] 1.2.1 添加 `cwd: PathBuf` - 当前工作目录
- [ ] 1.2.2 添加 `read_timestamps: HashMap<PathBuf, u64>` - 文件读取时间戳
- [ ] 1.2.3 添加 `safe_mode: bool` - 安全模式标志
- [ ] 1.2.4 添加 `abort: AbortHandle` - 取消支持
- [ ] 1.2.5 实现 `new()` 构造函数
- [ ] 1.2.6 实现 `track_read()` 方法记录文件读取

**参考**: `/Users/gemini/Documents/backup/Kode-cli/src/Tool.ts` (ToolUseContext interface)

### 1.3 扩展 ToolResult
- [ ] 1.3.1 添加 `metadata: Option<Value>` - 元数据支持
- [ ] 1.3.2 添加 `is_error: bool` - 错误标志
- [ ] 1.3.3 实现 `with_metadata()` 辅助方法
- [ ] 1.3.4 实现 `error()` 构造函数

### 1.4 扩展 ToolRegistry
- [ ] 1.4.1 实现 `filter<F>(predicate: F) -> Vec<Arc<dyn Tool>>` where F: Fn(&Arc<dyn Tool>) -> bool
- [ ] 1.4.2 实现 `list_enabled() -> Vec<String>` - 返回启用的工具
- [ ] 1.4.3 实现 `list_read_only() -> Vec<String>` - 返回只读工具
- [ ] 1.4.4 实现 `get_by_names(names: &[String]) -> Vec<Arc<dyn Tool>>` - 批量获取
- [ ] 1.4.5 添加 `Default` 实现

**参考**: `/Users/gemini/Documents/backup/Kode-cli/src/tools.ts` (getAllTools, getTools, getReadOnlyTools)

### 1.5 参数验证类型
- [ ] 1.5.1 定义 `ValidationResult` 结构体
  - `result: bool`
  - `message: Option<String>`
  - `error_code: Option<u32>`
  - `meta: Option<Value>`
- [ ] 1.5.2 实现 `ValidationResult::success()` 构造函数
- [ ] 1.5.3 实现 `ValidationResult::error()` 构造函数
- [ ] 1.5.4 添加单元测试

**参考**: `/Users/gemini/Documents/backup/Kode-cli/src/Tool.ts` (ValidationResult interface)

---

## Phase 2: 安全文件操作层

### 2.1 实现 secure_file.rs
- [ ] 2.1.1 定义 `SecureFileService` 结构体
  - `allowed_base_paths: Vec<PathBuf>`
  - `max_file_size: usize`
  - `allowed_extensions: HashSet<String>`
- [ ] 2.1.2 实现 `validate_path(&self, path: &Path) -> Result<PathBuf>`
  - 路径规范化
  - 路径遍历检测 (`..`, `~`)
  - 可疑模式检测（正则表达式）
  - 路径长度限制（4096 字符）
  - 白名单检查
- [ ] 2.1.3 实现 `safe_exists(&self, path: &Path) -> bool`
- [ ] 2.1.4 实现 `safe_get_file_info(&self, path: &Path) -> Result<FileInfo>`
  - 返回 `FileInfo { size, modified, is_file, is_dir }`
- [ ] 2.1.5 实现 `safe_read_file(&self, path: &Path, options: ReadOptions) -> Result<Buffer>`
  - 支持 `encoding` 选项
  - 支持 `max_size` 限制
- [ ] 2.1.6 添加完整的单元测试
  - 测试路径遍历攻击
  - 测试可疑模式
  - 测试白名单

**参考**: `/Users/gemini/Documents/backup/Kode-cli/src/utils/secureFile.ts` (lines 1-300)

### 2.2 实现 file_utils.rs
- [ ] 2.2.1 实现 `normalize_path(path: &Path, cwd: &Path) -> Result<PathBuf>`
  - 处理相对路径
  - 处理 `.` 和 `..`
  - 路径清理
- [ ] 2.2.2 实现 `detect_encoding(path: &Path) -> Result<String>`
  - 检测 UTF-8, UTF-16, ASCII 等
  - 使用 `encoding_rs` 或 `chardetng`
- [ ] 2.2.3 实现 `detect_line_endings(content: &str) -> LineEnding`
  - 返回 `CRLF` 或 `LF`
  - 检测混合模式
- [ ] 2.2.4 实现 `detect_repo_line_endings(repo_path: &Path) -> Result<LineEnding>`
  - 扫描 `.gitattributes` (可选)
  - 检测主要行结束符
- [ ] 2.2.5 实现 `add_line_numbers(content: &str, start_line: usize) -> String`
  - 格式: `  1 | content`
  - 右对齐行号
- [ ] 2.2.6 实现 `read_text_content(path: &Path, offset: usize, limit: Option<usize>) -> Result<ReadResult>`
  - 返回 `ReadResult { content, line_count, total_lines, start_line }`
- [ ] 2.2.7 实现 `write_text_content(path: &Path, content: &str, encoding: &str, line_ending: LineEnding) -> Result<()>`
  - 自动创建父目录
  - 编码转换
  - 行结束符转换
- [ ] 2.2.8 添加单元测试

**参考**: `/Users/gemini/Documents/backup/Kode-cli/src/utils/file.ts` (lines 1-406)

---

## Phase 3: 文件工具实现

### 3.1 实现 FileReadTool
- [ ] 3.1.1 定义 `FileReadTool` 结构体
  - `name: &'static str` = "Read"
  - `secure_service: Arc<SecureFileService>`
- [ ] 3.1.2 实现 `Tool trait`
  - `name()` -> "Read"
  - `description()` -> "读取文件内容，支持行号范围和图片文件"
  - `schema()` -> JSON Schema
  - `is_read_only()` -> true
  - `is_concurrency_safe()` -> true
  - `needs_permission()` -> 根据路径判断
- [ ] 3.1.3 实现 `validate_input()`
  - 检查 `file_path` 参数
  - 验证文件存在
  - 文件大小检查（text: 250KB, image: 3.75MB）
  - 大文件必须提供 `offset` 或 `limit`
  - 建议相似文件名（如果文件不存在）
- [ ] 3.1.4 实现 `execute()`
  - 支持参数: `file_path`, `offset`, `limit`
  - 文本文件: 返回带行号的内容
  - 图片文件: 返回 Base64 编码
  - 图片尺寸验证 (MAX_WIDTH: 2000, MAX_HEIGHT: 2000)
  - 记录文件读取时间戳
- [ ] 3.1.5 添加单元测试
  - 测试文本文件读取
  - 测试图片文件读取
  - 测试 offset/limit
  - 测试大文件拒绝
  - 测试不存在的文件

**参考**: `/Users/gemini/Documents/backup/Kode-cli/src/tools/FileReadTool/FileReadTool.tsx`

### 3.2 实现 FileWriteTool
- [ ] 3.2.1 定义 `FileWriteTool` 结构体
  - `name: &'static str` = "Write"
  - `secure_service: Arc<SecureFileService>`
- [ ] 3.2.2 实现 `Tool trait`
  - `name()` -> "Write"
  - `description()` -> "写入文件内容，创建目录并保持编码和行结束符"
  - `schema()` -> JSON Schema
  - `is_read_only()` -> false
  - `is_concurrency_safe()` -> false
  - `needs_permission()` -> true
- [ ] 3.2.3 实现 `validate_input()`
  - 检查 `file_path` 和 `content` 参数
  - 验证文件已读取（检查时间戳）
  - 验证文件未被修改（mtime <= read_timestamp）
  - 新文件允许写入
- [ ] 3.2.4 实现 `execute()`
  - 支持参数: `file_path`, `content`
  - 自动创建父目录
  - 检测文件编码（旧文件）或使用 UTF-8（新文件）
  - 检测行结束符（旧文件）或检测仓库（新文件）
  - 写入文件
  - 更新读取时间戳
  - 返回操作类型 (create/update) 和行数
- [ ] 3.2.5 添加单元测试
  - 测试创建新文件
  - 测试更新现有文件
  - 测试目录自动创建
  - 测试编码保持
  - 测试行结束符保持
  - 测试时间戳验证

**参考**: `/Users/gemini/Documents/backup/Kode-cli/src/tools/FileWriteTool/FileWriteTool.tsx`

### 3.3 实现 FileEditTool
- [ ] 3.3.1 定义 `FileEditTool` 结构体
  - `name: &'static str` = "Edit"
  - `secure_service: Arc<SecureFileService>`
- [ ] 3.3.2 实现 `Tool trait`
  - `name()` -> "Edit"
  - `description()` -> "编辑文件，精确字符串替换"
  - `schema()` -> JSON Schema
  - `is_read_only()` -> false
  - `is_concurrency_safe()` -> false
  - `needs_permission()` -> true
- [ ] 3.3.3 实现 `validate_input()`
  - 检查 `file_path`, `old_string`, `new_string` 参数
  - 验证文件已读取（检查时间戳）
  - 验证文件未被修改
  - 检查 `old_string` 不等于 `new_string`
  - 检查 `old_string` 在文件中存在
  - 检查 `old_string` 只出现一次（多匹配拒绝）
  - 新文件允许创建 (old_string == "")
  - 拒绝编辑 Notebook 文件
- [ ] 3.3.4 实现 `execute()`
  - 支持参数: `file_path`, `old_string`, `new_string`
  - 执行字符串替换
  - 检测编码和行结束符
  - 写入文件
  - 更新读取时间戳
  - 计算差异（diff）
  - 返回编辑片段（前后 N 行上下文）
- [ ] 3.3.5 实现 `apply_edit()` 辅助函数
  - 执行替换
  - 生成结构化 diff
  - 处理创建/更新/删除
- [ ] 3.3.6 实现 `get_snippet()` 辅助函数
  - 提取编辑前后上下文（N 行）
- [ ] 3.3.7 添加单元测试
  - 测试简单替换
  - 测试多行替换
  - 测试创建新文件
  - 测试删除内容
  - 测试多匹配拒绝
  - 测试时间戳验证
  - 测试字符串不存在

**参考**: `/Users/gemini/Documents/backup/Kode-cli/src/tools/FileEditTool/FileEditTool.tsx`
**参考**: `/Users/gemini/Documents/backup/Kode-cli/src/tools/FileEditTool/utils.ts`

---

## Phase 4: 工具参数验证框架

### 4.1 实现 validation.rs
- [ ] 4.1.1 定义 `ValidationError` 枚举
  - `FileNotFound`
  - `FileTooLarge`
  - `InvalidPath`
  - `MissingParameter`
  - `InvalidType`
  - `StringNotFound`
  - `MultipleMatches`
  - `FileModified`
- [ ] 4.1.2 实现 `json_schema_validator` 模块
  - 使用 `jsonschema` crate
  - 编译和验证 JSON Schema
- [ ] 4.1.3 实现 `path_validator` 模块
  - 路径存在性检查
  - 路径安全性检查
  - 相对路径转绝对路径
- [ ] 4.1.4 实现 `file_validator` 模块
  - 文件大小检查
  - 文件类型检查
  - 文件修改时间检查
- [ ] 4.1.5 添加单元测试

---

## Phase 5: 集成和导出

### 5.1 更新 lib.rs
- [ ] 5.1.1 导出 `secure_file` 模块
- [ ] 5.1.2 导出 `file_utils` 模块
- [ ] 5.1.3 导出 `validation` 模块
- [ ] 5.1.4 导出 `file_read`, `file_write`, `file_edit` 模块
- [ ] 5.1.5 添加模块文档

### 5.2 更新 Cargo.toml
- [ ] 5.2.1 添加 `encoding_rs = "0.8"` 依赖
- [ ] 5.2.2 添加 `chardetng = "0.1"` 依赖（备选）
- [ ] 5.2.3 添加 `jsonschema = "0.18"` 依赖
- [ ] 5.2.4 添加 `image = { version = "0.25", optional = true }` 依赖
- [ ] 5.2.5 添加 `similar = "2.5"` 依赖（用于 diff）
- [ ] 5.2.6 添加 `ignore = "0.4"` 依赖（已在 workspace）

### 5.3 创建工具工厂函数
- [ ] 5.3.1 实现 `create_file_tools(secure_service: Arc<SecureFileService>) -> Vec<Arc<dyn Tool>>`
  - 返回 FileReadTool
  - 返回 FileWriteTool
  - 返回 FileEditTool
- [ ] 5.3.2 实现 `register_all_tools(registry: &mut ToolRegistry, secure_service: Arc<SecureFileService>)`
  - 注册所有文件工具到注册表

---

## Phase 6: 测试和文档

### 6.1 单元测试
- [ ] 6.1.1 Tool trait 扩展测试
- [ ] 6.1.2 ToolRegistry 扩展测试
- [ ] 6.1.3 SecureFileService 测试（包括安全测试）
- [ ] 6.1.4 FileReadTool 测试
- [ ] 6.1.5 FileWriteTool 测试
- [ ] 6.1.6 FileEditTool 测试
- [ ] 6.1.7 参数验证测试
- [ ] 6.1.8 集成测试（工具注册和调用）

### 6.2 文档
- [ ] 6.2.1 为所有公共类型添加 rustdoc
- [ ] 6.2.2 为所有 trait 方法添加示例
- [ ] 6.2.3 添加 README.md 到 `crates/kode-tools/`
- [ ] 6.2.4 添加工具使用指南

---

## Phase 7: 质量检查

### 7.1 代码质量
- [ ] 7.1.1 运行 `cargo fmt --check`（格式化）
- [ ] 7.1.2 运行 `cargo clippy -- -D warnings`（无警告）
- [ ] 7.1.3 运行 `cargo test`（所有测试通过）
- [ ] 7.1.4 运行 `cargo doc --no-deps`（文档生成）

### 7.2 兼容性验证
- [ ] 7.2.1 对比原版 TypeScript 工具行为
- [ ] 7.2.2 验证配置格式兼容性
- [ ] 7.2.3 测试边界情况处理

---

## 依赖关系

任务执行顺序建议：
1. Phase 1 → Phase 2 → Phase 3 → Phase 4 → Phase 5 → Phase 6 → Phase 7
2. Phase 3 中的 3.1, 3.2, 3.3 可以并行开发
3. Phase 6 可以在 Phase 3 完成后开始编写

**关键路径**: 1.1 → 1.2 → 1.3 → 1.4 → 2.1 → 2.2 → 3.1 → 3.2 → 3.3 → 5.1 → 6.1 → 7.1

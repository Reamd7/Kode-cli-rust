# Change: 实现基础文件工具 / Implement Basic File Tools

## Why

文件操作工具是 AI Agent 的基础能力，需要实现读取、写入和编辑文件的功能。通过分析原版 TypeScript 实现，我们发现需要实现以下关键功能：

### 原版实现分析

通过对 `/Users/gemini/Documents/backup/Kode-cli` 的分析，原版实现包含：

1. **Tool 接口** (`src/Tool.ts` - 200+ 行)
   - 完整的 TypeScript 泛型接口定义
   - 工具元数据（name, description, inputSchema）
   - 权限管理（needsPermissions, isReadOnly, isConcurrencySafe）
   - 渲染方法（renderToolUseMessage, renderToolResultMessage）
   - 异步执行器（AsyncGenerator 支持流式输出）
   - 输入验证（validateInput）

2. **FileReadTool** (`src/tools/FileReadTool/FileReadTool.tsx` - 400+ 行)
   - 支持文本和图片文件读取
   - 行号范围（offset/limit）
   - 文件大小限制（0.25MB for text, 3.75MB for images）
   - 图片压缩和尺寸调整
   - Base64 编码输出
   - 安全文件路径验证
   - 文件新鲜度追踪

3. **FileWriteTool** (`src/tools/FileWriteTool/FileWriteTool.tsx` - 350+ 行)
   - 创建或覆盖文件
   - 自动创建父目录
   - 编码检测（UTF-8, UTF-16 等）
   - 行结束符检测和保持（CRLF/LF）
   - 文件修改时间戳验证
   - 差异生成（structured patch）

4. **FileEditTool** (`src/tools/FileEditTool/FileEditTool.tsx` - 350+ 行)
   - 精确字符串替换
   - 多匹配检测（安全性）
   - 文件新鲜度验证
   - 差异高亮显示
   - Notebook 文件特殊处理

5. **安全文件服务** (`src/utils/secureFile.ts` - 500+ 行)
   - 路径遍历保护
   - 路径长度限制
   - 可疑模式检测
   - 允许路径白名单
   - 文件大小限制
   - 扩展名验证

### Rust 实现需求

原版使用 TypeScript + React（TUI），Rust 版本需要：
- 使用 `async-trait` 定义异步 trait
- 使用 `serde_json` 处理参数 schema
- 使用 `tokio::fs` 进行异步文件操作
- 参考 `secureFile` 实现安全文件操作
- 保持与原版配置格式 100% 兼容

## What Changes

### Phase 1: 核心 Tool Trait 和 Registry
- ✅ 已有基础 `Tool` trait 定义（需要扩展）
- ✅ 已有 `ToolRegistry` 实现（需要扩展）
- **新增**: 工具元数据（description, schema）
- **新增**: 权限管理方法（is_read_only, is_concurrency_safe, needs_permission）
- **新增**: 工具过滤和列表功能
- **新增**: JSON Schema 生成支持

### Phase 2: 安全文件操作层
- **新建**: `secure_file.rs` - 安全文件服务
  - 路径验证（防止路径遍历攻击）
  - 文件大小限制
  - 允许路径白名单
  - 编码检测（UTF-8, UTF-16, ASCII 等）
  - 行结束符检测和转换（CRLF/LF）
- **新建**: `file_utils.rs` - 文件工具函数
  - 路径规范化
  - 编码检测
  - 行号添加
  - 文件差异计算

### Phase 3: 文件工具实现
- **新建**: `file_read.rs` - FileReadTool
  - 文本文件读取（支持 offset/limit）
  - 图片文件读取（Base64 编码）
  - 文件大小限制（text: 250KB, image: 3.75MB）
  - 大文件分块读取
  - 图片尺寸验证和压缩提示
- **新建**: `file_write.rs` - FileWriteTool
  - 创建或覆盖文件
  - 自动创建父目录
  - 编码保持
  - 行结束符检测和保持
  - 文件修改时间戳验证
- **新建**: `file_edit.rs` - FileEditTool
  - 精确字符串替换
  - 多匹配检测（拒绝不安全操作）
  - 文件新鲜度验证
  - 差异生成

### Phase 4: 工具参数验证
- **新建**: `validation.rs` - 参数验证框架
  - JSON Schema 验证
  - 文件路径验证
  - 文件存在性检查
  - 文件大小检查
  - 自定义验证规则

### Phase 5: 上下文和结果管理
- **扩展**: `tool.rs` - ToolContext
  - 当前工作目录
  - 文件读取时间戳追踪
  - AbortSignal 支持
  - 安全模式标志
- **扩展**: `tool.rs` - ToolResult
  - 结构化输出
  - 元数据支持
  - 错误上下文

## Dependencies

### 依赖的 Spec
- **config-loading** ✅ (已完成) - 需要读取全局配置
- **message-model** ✅ (已完成) - 工具调用使用消息格式

### 解锁的功能
完成后将解锁以下变更：
- `implement-bash-tool` (P1) - 依赖 Tool trait
- `implement-search-tools` (P1) - 依赖 Tool trait 和 Registry
- `implement-task-tool` (P1) - 依赖工具系统
- `implement-permission-system` (P2) - 依赖 needs_permission

## Impact

**Affected specs:**
- tool-system (新增基础工具实现)

**Affected code:**
- `crates/kode-tools/src/tool.rs` (扩展 - 添加权限和元数据方法)
- `crates/kode-tools/src/registry.rs` (扩展 - 添加过滤和列表功能)
- `crates/kode-tools/src/secure_file.rs` (新建 - 安全文件服务)
- `crates/kode-tools/src/file_utils.rs` (新建 - 文件工具函数)
- `crates/kode-tools/src/file_read.rs` (新建 - FileReadTool)
- `crates/kode-tools/src/file_write.rs` (新建 - FileWriteTool)
- `crates/kode-tools/src/file_edit.rs` (新建 - FileEditTool)
- `crates/kode-tools/src/validation.rs` (新建 - 参数验证)
- `crates/kode-tools/Cargo.toml` (添加依赖)
  - encoding_rs (编码检测)
  - image (图片处理，可选)
  - jsonschema (JSON Schema 验证)

## Non-Goals

本次变更不包含：
- ❌ TUI 渲染组件（在 tui-interface spec 中）
- ❌ 流式输出支持（在 streaming-response spec 中）
- ❌ Notebook 文件支持（后续变更）
- ❌ MCP 工具集成（在 mcp-integration spec 中）
- ❌ 文件权限系统详细实现（在 permission-system spec 中）

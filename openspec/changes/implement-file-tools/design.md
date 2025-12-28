# 设计文档：基础文件工具 / Design Document: Basic File Tools

## 概述 / Overview

本文档描述实现基础文件工具系统的技术设计决策，包括架构设计、关键组件和实现策略。

This document describes the technical design decisions for implementing the basic file tools system, including architecture design, key components, and implementation strategies.

---

## 1. 架构设计 / Architecture Design

### 1.1 分层架构 / Layered Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    Tools Layer                          │
│  ┌──────────────┬──────────────┬──────────────┐        │
│  │ FileReadTool │ FileWriteTool│ FileEditTool │        │
│  └──────────────┴──────────────┴──────────────┘        │
└─────────────────────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────┐
│                    Core Layer                           │
│  ┌──────────────┬──────────────┬──────────────────────┐ │
│  │ Tool Trait   │ToolRegistry  │ ToolContext/Result   │ │
│  └──────────────┴──────────────┴──────────────────────┘ │
└─────────────────────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────┐
│                  Infrastructure Layer                    │
│  ┌──────────────┬──────────────┬──────────────────────┐ │
│  │SecureFileSvc │ FileUtils    │ Validation           │ │
│  └──────────────┴──────────────┴──────────────────────┘ │
└─────────────────────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────┐
│                    System Layer                         │
│           tokio::fs  |  std::path  |  std::fs          │
└─────────────────────────────────────────────────────────┘
```

### 1.2 模块组织 / Module Organization

```
crates/kode-tools/src/
├── lib.rs              # 库入口，导出公共 API
├── tool.rs             # Tool trait 定义
├── registry.rs         # ToolRegistry 实现
├── secure_file.rs      # 安全文件服务
├── file_utils.rs       # 文件工具函数
├── validation.rs       # 参数验证框架
├── file_read.rs        # FileReadTool
├── file_write.rs       # FileWriteTool
└── file_edit.rs        # FileEditTool
```

---

## 2. 核心设计决策 / Core Design Decisions

### 2.1 Tool Trait 设计 / Tool Trait Design

**决策**: 使用 `async-trait` 定义异步 trait

**原因**:
- Rust 原生不支持异步 trait 方法
- `async-trait` 是社区标准解决方案
- 提供良好的性能和可读性

**权衡**:
- ✅ 类型安全的异步接口
- ✅ 良好的错误处理
- ❌ 额外的 Box 分配开销（可接受）

**代码示例**:
```rust
#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn schema(&self) -> ToolSchema;
    async fn execute(&self, params: Value, context: &ToolContext) -> Result<ToolResult>;

    // 默认实现
    fn is_read_only(&self) -> bool { false }
    fn is_concurrency_safe(&self) -> bool { false }
    fn needs_permission(&self, params: &Value) -> bool { false }
}
```

### 2.2 安全文件操作 / Secure File Operations

**决策**: 实现独立的安全文件服务层

**原因**:
- 集中管理安全策略
- 防止路径遍历攻击
- 支持白名单和黑名单
- 便于测试和维护

**安全措施**:
1. **路径验证**:
   - 检测 `..` 和 `~` 遍历
   - 检测可疑模式（正则表达式）
   - 路径长度限制（4096 字符）
   - 白名单目录检查

2. **文件大小限制**:
   - 文本文件: 250KB
   - 图片文件: 3.75MB
   - 大文件强制分块读取

3. **文件类型检查**:
   - 扩展名白名单
   - MIME 类型验证

**代码示例**:
```rust
pub struct SecureFileService {
    allowed_base_paths: Vec<PathBuf>,
    max_file_size: usize,
    allowed_extensions: HashSet<String>,
}

impl SecureFileService {
    pub fn validate_path(&self, path: &Path) -> Result<PathBuf> {
        // 1. 规范化路径
        let normalized = path.normalize()?;
        
        // 2. 检测遍历攻击
        if normalized.contains("..") || normalized.contains("~") {
            return Err(SecurityError::PathTraversal);
        }
        
        // 3. 检测可疑模式
        for pattern in &SUSPICIOUS_PATTERNS {
            if pattern.is_match(&normalized.to_string_lossy()) {
                return Err(SecurityError::SuspiciousPattern);
            }
        }
        
        // 4. 白名单检查
        if !self.is_in_allowed_paths(&normalized) {
            return Err(SecurityError::PathNotAllowed);
        }
        
        Ok(normalized)
    }
}
```

### 2.3 文件编码检测 / File Encoding Detection

**决策**: 使用 `encoding_rs` crate

**原因**:
- Mozilla 维护，可靠性高
- 性能优秀
- 支持常见编码（UTF-8, UTF-16, ASCII 等）
- 无需外部 C 库

**备选方案**: `chardetng`
- 更准确的检测
- 但性能稍低
- 作为备选保留

### 2.4 行结束符处理 / Line Ending Handling

**决策**: 自动检测和保持

**策略**:
1. 读取文件时检测当前行结束符
2. 写入时保持原有行结束符
3. 新文件检测仓库主要行结束符
4. 支持 `.gitattributes` (可选)

**代码示例**:
```rust
pub enum LineEnding {
    CRLF,  // Windows
    LF,    // Unix/Mac
    Mixed, // 混合（警告）
}

pub fn detect_line_endings(content: &str) -> LineEnding {
    let crlf_count = content.matches("\r\n").count();
    let lf_count = content.matches('\n').count() - crlf_count;
    
    match (crlf_count, lf_count) {
        (0, 0) => LineEnding::LF,  // 默认
        (_, 0) => LineEnding::CRLF,
        (0, _) => LineEnding::LF,
        _ => {
            if crlf_count > lf_count { LineEnding::CRLF }
            else if lf_count > crlf_count { LineEnding::LF }
            else { LineEnding::Mixed }
        }
    }
}
```

### 2.5 文件新鲜度追踪 / File Freshness Tracking

**决策**: 使用时间戳验证

**机制**:
1. 文件读取时记录 `mtime`
2. 写入前验证 `mtime <= read_timestamp`
3. 如果文件被修改，拒绝写入
4. 提示用户重新读取文件

**优势**:
- 防止覆盖用户修改
- 防止覆盖 linter 修改
- 简单高效

**代码示例**:
```rust
pub struct ToolContext {
    read_timestamps: HashMap<PathBuf, u64>,
    // ...
}

impl FileWriteTool {
    async fn validate_input(&self, params: &Value, ctx: &ToolContext) -> Result<ValidationResult> {
        let path = params["file_path"].as_str().ok_or(..)?;
        
        // 检查时间戳
        if let Some(&read_ts) = ctx.read_timestamps.get(path) {
            let metadata = tokio::fs::metadata(path).await?;
            let current_mtime = metadata.modified()?.duration_since(SystemTime::UNIX_EPOCH)?.as_millis() as u64;
            
            if current_mtime > read_ts {
                return Ok(ValidationResult::error(
                    "File has been modified since read. Read it again before writing."
                ));
            }
        }
        
        Ok(ValidationResult::success())
    }
}
```

### 2.6 图片处理 / Image Processing

**决策**: 使用 `image` crate（可选）

**功能**:
- 读取常见图片格式（PNG, JPG, GIF, BMP, WebP）
- 尺寸验证（MAX: 2000x2000）
- 大小限制（3.75MB）
- Base64 编码输出

**权衡**:
- ✅ 支持常见格式
- ✅ 尺寸验证
- ❌ 增加依赖大小
- ❌ 压缩功能复杂

**简化方案**:
- 仅支持基本读取
- 不做压缩（提示用户使用工具）
- 使用 feature flag 可选

### 2.7 参数验证 / Parameter Validation

**决策**: 使用 JSON Schema + 自定义验证

**方案**:
1. **JSON Schema 验证**:
   - 使用 `jsonschema` crate
   - 验证参数类型和格式
   - 自动生成文档

2. **自定义验证**:
   - 文件存在性检查
   - 文件大小检查
   - 文件新鲜度检查
   - 业务逻辑验证

**代码示例**:
```rust
impl FileReadTool {
    async fn validate_input(&self, params: &Value, ctx: &ToolContext) -> Result<ValidationResult> {
        // 1. JSON Schema 验证
        self.schema().validate(params)?;
        
        // 2. 自定义验证
        let path = params["file_path"].as_str().ok_or(..)?;
        let full_path = ctx.secure_service.validate_path(path)?;
        
        if !ctx.secure_service.safe_exists(&full_path) {
            return Ok(ValidationResult::error(
                "File does not exist.",
                Some(find_similar_file(&full_path))
            ));
        }
        
        let file_info = ctx.secure_service.safe_get_file_info(&full_path)?;
        if file_info.size > MAX_FILE_SIZE && !params.get("offset").is_some() {
            return Ok(ValidationResult::error(
                &format!("File too large ({} bytes). Use offset/limit.", file_info.size)
            ));
        }
        
        Ok(ValidationResult::success())
    }
}
```

---

## 3. 并发和性能 / Concurrency and Performance

### 3.1 并发安全 / Concurrency Safety

**工具分类**:
- **只读工具** (Read): `is_concurrency_safe() = true`
- **写入工具** (Write, Edit): `is_concurrency_safe() = false`

**策略**:
- 只读工具可并行执行
- 写入工具需要序列化
- 使用 `tokio::sync::Mutex` 保护共享状态

### 3.2 性能优化 / Performance Optimization

**策略**:
1. **异步 I/O**: 使用 `tokio::fs`
2. **路径缓存**: 缓存规范化路径
3. **编码检测缓存**: 缓存文件编码
4. **流式读取**: 大文件分块读取
5. **并发工具**: 并行执行只读工具

---

## 4. 错误处理 / Error Handling

### 4.1 错误类型 / Error Types

```rust
pub enum ToolError {
    // 验证错误
    Validation(ValidationError),
    
    // 安全错误
    Security(SecurityError),
    
    // I/O 错误
    Io(io::Error),
    
    // 编码错误
    Encoding(String),
    
    // 其他错误
    Other(anyhow::Error),
}
```

### 4.2 错误传播 / Error Propagation

**策略**:
- 使用 `anyhow` 提供上下文
- 使用 `thiserror` 定义错误类型
- 使用 `?` 传播错误
- 提供用户友好的错误消息

---

## 5. 测试策略 / Testing Strategy

### 5.1 单元测试 / Unit Tests

- **Tool trait**: 测试所有方法
- **ToolRegistry**: 测试注册、获取、过滤
- **SecureFileService**: 测试安全验证
- **文件工具**: 测试各种场景

### 5.2 集成测试 / Integration Tests

- 工具注册和调用
- 多工具协作
- 错误场景
- 边界情况

### 5.3 安全测试 / Security Tests

- 路径遍历攻击
- 大文件拒绝
- 可疑模式检测
- 时间戳验证

---

## 6. 依赖管理 / Dependency Management

### 6.1 必需依赖 / Required Dependencies

```toml
[dependencies]
# 异步运行时
tokio = { workspace = true }
async-trait = { workspace = true }

# 序列化
serde = { workspace = true }
serde_json = { workspace = true }

# 文件操作
glob = { workspace = true }
ignore = { workspace = true }

# 编码
encoding_rs = "0.8"

# 验证
jsonschema = "0.18"

# 差异计算
similar = "2.5"

# 错误处理
anyhow = { workspace = true }
thiserror = { workspace = true }
```

### 6.2 可选依赖 / Optional Dependencies

```toml
[features]
default = []
image = ["dep:image", "dep:image/base64"]

[dependencies]
image = { version = "0.25", optional = true }
```

---

## 7. 兼容性 / Compatibility

### 7.1 配置兼容性 / Configuration Compatibility

- 保持与原版 TypeScript 配置格式 100% 兼容
- 工具名称和参数一致
- 错误消息格式一致

### 7.2 行为兼容性 / Behavior Compatibility

- 文件编码检测行为一致
- 行结束符处理一致
- 时间戳验证逻辑一致
- 安全验证规则一致

---

## 8. 未来扩展 / Future Extensions

### 8.1 计划功能 / Planned Features

1. **Notebook 工具**: 支持 `.ipynb` 文件
2. **多文件编辑**: 批量编辑多个文件
3. **文件监听**: 监听文件变化
4. **文件历史**: 维护文件版本历史
5. **高级 diff**: 支持更多 diff 格式

### 8.2 扩展点 / Extension Points

1. **自定义验证器**: 插件式验证系统
2. **自定义编码器**: 支持更多编码
3. **文件过滤器**: 更强大的文件过滤
4. **钩子系统**: 前置/后置钩子

---

## 9. 风险和缓解 / Risks and Mitigations

### 9.1 已识别风险 / Identified Risks

| 风险 | 影响 | 缓解措施 |
|------|------|----------|
| 编码检测不准确 | 文件乱码 | 使用多个检测库，提供手动指定 |
| 性能问题 | 响应慢 | 使用缓存，异步 I/O |
| 安全漏洞 | 路径遍历 | 严格验证，安全测试 |
| 兼容性问题 | 配置不一致 | 对比测试，验证脚本 |

### 9.2 缓解策略 / Mitigation Strategies

1. **编码检测**: 结合 `encoding_rs` 和 `chardetng`
2. **性能**: 基准测试，热点优化
3. **安全**: 模糊测试，安全审计
4. **兼容性**: 自动化测试套件

---

## 10. 参考资料 / References

1. **原版 TypeScript 实现**:
   - `/Users/gemini/Documents/backup/Kode-cli/src/Tool.ts`
   - `/Users/gemini/Documents/backup/Kode-cli/src/tools/FileReadTool/`
   - `/Users/gemini/Documents/backup/Kode-cli/src/tools/FileWriteTool/`
   - `/Users/gemini/Documents/backup/Kode-cli/src/tools/FileEditTool/`
   - `/Users/gemini/Documents/backup/Kode-cli/src/utils/secureFile.ts`
   - `/Users/gemini/Documents/backup/Kode-cli/src/utils/file.ts`

2. **Rust Crates 文档**:
   - [async-trait](https://docs.rs/async-trait/)
   - [encoding_rs](https://docs.rs/encoding_rs/)
   - [jsonschema](https://docs.rs/jsonschema/)
   - [image](https://docs.rs/image/)
   - [similar](https://docs.rs/similar/)

3. **安全最佳实践**:
   - [Rust Security Guidelines](https://doc.rust-lang.org/nomicon/security.html)
   - [OWASP Path Traversal](https://owasp.org/www-community/attacks/Path_Traversal)

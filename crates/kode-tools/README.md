# Kode Tools

用于 Kode CLI 的工具系统实现。

## 概述

这个 crate 实现了 AI Agent CLI 工具的核心工具系统，包括：

- **Tool trait**: 定义工具接口和元数据
- **ToolRegistry**: 工具注册和管理
- **文件工具**: 文件读取、写入、编辑工具
- **安全框架**: 路径验证和文件安全操作

## 核心功能

### 1. Tool Trait 系统

```rust
use kode_tools::Tool;
use async_trait::async_trait;

#[async_trait]
impl Tool for MyTool {
    fn name(&self) -> &str {
        "MyTool"
    }

    fn description(&self) -> &str {
        "我的工具描述"
    }

    fn schema(&self) -> ToolSchema {
        // JSON Schema 定义参数
    }

    async fn execute(&self, params: Value, context: &ToolContext) -> Result<ToolResult> {
        // 工具执行逻辑
    }
}
```

### 2. 文件工具

#### FileReadTool
读取文件内容，支持：
- 文本文件（带行号）
- 图片文件（Base64 编码）
- 分块读取（offset/limit）
- 相似文件名建议

```rust
use kode_tools::{FileReadTool, SecureFileService};

let service = SecureFileService::new(vec![PathBuf::from(".")]);
let tool = FileReadTool::new(Arc::new(service));

let params = json!({
    "file_path": "./src/main.rs",
    "offset": 0,
    "limit": 100
});

let result = tool.execute(params, &context).await?;
```

#### FileWriteTool
写入文件内容，支持：
- 创建新文件
- 覆盖现有文件
- 自动创建目录
- 编码和行结束符检测

```rust
let tool = FileWriteTool::new(service);

let params = json!({
    "file_path": "./output.txt",
    "content": "Hello, World!"
});

let result = tool.execute(params, &context).await?;
```

#### FileEditTool
编辑文件内容，支持：
- 精确字符串替换
- 多重匹配检测
- 文件新鲜度验证
- 差异计算

```rust
let tool = FileEditTool::new(service);

let params = json!({
    "file_path": "./config.txt",
    "old_string": "old value",
    "new_string": "new value"
});

let result = tool.execute(params, &context).await?;
```

### 3. 安全框架

```rust
use kode_tools::SecureFileService;

// 创建安全文件服务
let service = SecureFileService::new(vec![
    PathBuf::from("/home/user/projects"),
    PathBuf::from("/tmp")
]);

// 验证路径
let safe_path = service.validate_path(&user_path).await?;

// 获取文件信息
let info = service.safe_get_file_info(&safe_path).await?;
```

## 工具注册

```rust
use kode_tools::{ToolRegistry, create_file_tools, SecureFileService};

// 创建注册表
let mut registry = ToolRegistry::new();

// 创建安全服务
let service = SecureFileService::new(vec![PathBuf::from(".")]);

// 注册所有文件工具
for tool in create_file_tools(Arc::new(service)) {
    registry.register(tool);
}

// 查询工具
let tool = registry.get("Read").unwrap();
let read_only_tools = registry.list_read_only();
```

## 安全特性

### 路径验证
- ✅ 路径遍历攻击检测 (`..`, `~`)
- ✅ 可疑模式正则检测
- ✅ 路径白名单机制
- ✅ 路径长度限制（4096 字符）

### 文件保护
- ✅ 文件大小限制
- ✅ 文件新鲜度追踪（修改时间验证）
- ✅ Notebook 文件保护（拒绝编辑 .ipynb）
- ✅ 多重匹配检测（防止意外替换）

## 测试

运行测试：

```bash
# 单元测试
cargo test --package kode-tools

# 带输出的测试
cargo test --package kode-tools -- --nocapture

# 特定测试
cargo test --package kode-tools test_file_read
```

## 代码质量

```bash
# 格式化检查
cargo fmt --check --package kode-tools

# Clippy 检查
cargo clippy --package kode-tools -- -D warnings

# 文档生成
cargo doc --package kode-tools --no-deps
```

## 项目结构

```
src/
├── lib.rs              # 库入口和导出
├── tool.rs             # Tool trait 定义
├── registry.rs         # 工具注册表
├── secure_file.rs      # 安全文件服务
├── file_utils.rs       # 文件工具函数
├── file_read.rs        # 文件读取工具
├── file_write.rs       # 文件写入工具
├── file_edit.rs        # 文件编辑工具
└── validation.rs       # 参数验证框架
```

## 依赖

- `async-trait`: 异步 trait 支持
- `serde_json`: JSON 序列化
- `tokio`: 异步运行时
- `regex`: 正则表达式
- `encoding_rs`: 编码检测
- `similar`: 差异计算
- `thiserror`: 错误处理
- `anyhow`: 错误上下文
- `tempfile`: 测试用临时文件

## 许可证

MIT

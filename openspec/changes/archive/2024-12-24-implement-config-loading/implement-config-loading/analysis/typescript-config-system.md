# Kode 配置系统完整分析（基于 TypeScript 实现）

> **文档说明 / Document Notice**
>
> 本文档是 [implement-config-loading](../) 变更的知识库文档，为 Rust 版本实施提供详细的技术分析和参考。
>
> This document is a knowledge base for the [implement-config-loading](../) change, providing detailed technical analysis and reference for the Rust implementation.

---

## 一、配置文件路径

### 1.1 全局配置
- **文件路径**: `~/.kode.json`
- **常量定义**: `GLOBAL_CLAUDE_FILE`
- **环境变量覆盖**: 
  - `KODE_CONFIG_DIR` → `{$KODE_CONFIG_DIR}/config.json`
  - `CLAUDE_CONFIG_DIR` → `{$CLAUDE_CONFIG_DIR}/config.json`

### 1.2 项目配置
- **存储位置**: 存储在全局配置的 `projects` 字段中
- **Key 格式**: 项目路径的绝对路径（如 `/Users/user/project`）
- **不是独立文件**: 项目配置不是单独的 ./.kode.json 文件

## 二、配置优先级（从高到低）

```
1. 环境变量 (最高优先级)
   ├─ OPENAI_API_KEY
   ├─ ANTHROPIC_API_KEY
   ├─ KODE_CONFIG_DIR
   └─ CLAUDE_CONFIG_DIR

2. 运行时标志 (CLI 参数)
   ├─ --verbose
   ├─ --model
   └─ 其他 CLI 参数

3. 项目配置
   └─ GlobalConfig.projects[{current_dir}]

4. 全局配置
   └─ ~/.kode.json

5. 默认值 (最低优先级)
   └─ DEFAULT_GLOBAL_CONFIG, DEFAULT_PROJECT_CONFIG
```

## 三、配置结构

### 3.1 GlobalConfig（~/.kode.json）

包含 24 个字段，详见源码：`/Users/gemini/Documents/backup/Kode-cli/src/utils/config.ts`

### 3.2 ProjectConfig

包含 19 个字段，详见源码。

## 四、关键 API 函数（共 30+ 个）

详见源码分析。

## 五、配置文件操作特性

- 保存时过滤默认值
- 错误容忍（文件不存在、解析失败都返回默认值）
- 项目配置覆盖全局配置

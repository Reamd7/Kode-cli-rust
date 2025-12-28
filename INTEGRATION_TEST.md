# 集成测试使用说明 / Integration Test Usage Guide

## 📋 概述 / Overview

`integration_test.rs` 是一个独立的 Rust 脚本，用于测试 Anthropic API 服务的基本功能。

## 🔑 配置 / Configuration

### 1. 创建配置文件

```bash
cp .env.example .env
```

### 2. 编辑 `.env` 文件

```bash
# Anthropic API Key (必需)
ANTHROPIC_API_KEY=your_actual_api_key_here

# 可选：自定义 Base URL
ANTHROPIC_BASE_URL=https://api.anthropic.com

# 可选：模型名称
ANTHROPIC_MODEL=claude-3-5-sonnet-20241022
```

### 3. 获取 API Key

访问 [Anthropic Console](https://console.anthropic.com/) 获取你的 API Key。

## 🚀 运行测试

```bash
chmod +x integration_test.rs
./integration_test.rs
```

或者使用 cargo：

```bash
cargo +nightly run --script integration_test.rs
```

## 📊 测试内容

该脚本执行以下测试：

1. **API Key 验证** - 测试 API key 是否有效
2. **消息发送** - 发送简单的测试消息并显示响应

## ⚠️ 安全注意事项

- ⚠️ **切勿将 `.env` 文件提交到 git**
- ✅ `.env` 已在 `.gitignore` 中
- ✅ 只提交 `.env.example` 作为模板
- ✅ API key 应该妥善保管，不要分享

## 🔧 兼容的 API

此测试兼容以下 Anthropic API：

- **Anthropic 官方 API** (https://api.anthropic.com)
- **BigModel API** (https://open.bigmodel.cn/api/anthropic)
- 其他 Anthropic 兼容的 API

### BigModel 配置示例

```bash
ANTHROPIC_API_KEY=your_bigmodel_api_key
ANTHROPIC_BASE_URL=https://open.bigmodel.cn/api/anthropic
ANTHROPIC_MODEL=glm-4.7
```

## 📝 故障排除

### 错误：ANTHROPIC_API_KEY must be set

**原因**: `.env` 文件不存在或未设置 `ANTHROPIC_API_KEY`

**解决方法**:
```bash
cp .env.example .env
# 编辑 .env 文件，添加你的 API key
```

### 错误：API key returned status: 401

**原因**: API key 无效或过期

**解决方法**:
- 检查 API key 是否正确
- 访问 [Anthropic Console](https://console.anthropic.com/) 验证

### 错误：Request failed: ...

**原因**: 网络问题或 Base URL 错误

**解决方法**:
- 检查网络连接
- 验证 `ANTHROPIC_BASE_URL` 是否正确

# 实现任务 / Implementation Tasks

## 1. 准备工作 / Preparation
- [ ] 1.1 阅读 `openspec/specs/message-model/spec.md` 中 Model Manager 相关需求
- [ ] 1.2 阅读 `openspec/changes/implement-model-switching/proposal.md`
- [ ] 1.3 阅读 TypeScript 参考：`/Users/gemini/Documents/backup/Kode-cli/src/services/modelAdapterFactory.ts`

## 2. Model Manager 实现 / Model Manager Implementation

- [ ] 2.1 定义 `ModelManager` 结构
  - [ ] 存储多个模型配置
  - [ ] 管理当前激活的模型
  - [ ] 提供模型查询接口

- [ ] 2.2 实现模型切换功能
  - [ ] `switch_model(model_name: &str)` 方法
  - [ ] 验证新模型配置
  - [ ] 返回切换结果
  - [ ] 处理不存在的模型

- [ ] 2.3 实现模型指针（Model Pointers）
  - [ ] 定义 `ModelPointers` 枚举或结构
  - [ ] 支持 default, task, architect 等指针类型
  - [ ] 指针动态切换
  - [ ] 指针配置持久化

- [ ] 2.4 实现模型配置验证
  - [ ] 验证必需字段（model_name, api_key）
  - [ ] 验证 base_url 格式
  - [ ] 验证 max_tokens 范围
  - [ ] 返回验证错误信息

- [ ] 2.5 实现 `get_adapter()` 方法
  - [ ] 根据模型名称获取适配器
  - [ ] 处理模型不存在的情况
  - [ ] 返回 `Result<Box<dyn ModelAdapter>>`

## 3. CLI 命令实现 / CLI Commands

- [ ] 3.1 实现 `/model switch <name>` 命令
  - [ ] 解析命令参数
  - [ ] 调用 ModelManager::switch_model()
  - [ ] 显示切换结果

- [ ] 3.2 实现 `/model list` 命令
  - [ ] 列出所有可用模型
  - [ ] 显示当前激活的模型
  - [ ] 显示模型指针状态

## 4. UI 更新 / UI Updates

- [ ] 4.1 更新状态栏显示当前模型
  - [ ] 修改 `crates/kode-ui/src/widgets/status.rs`
  - [ ] 显示模型名称和提供商

## 5. 测试 / Testing

- [ ] 5.1 单元测试
  - [ ] 测试模型切换
  - [ ] 测试模型指针
  - [ ] 测试配置验证
  - [ ] 测试 get_adapter()

- [ ] 5.2 集成测试
  - [ ] 测试 CLI 命令
  - [ ] 测试与 ModelAdapter 的集成

## 6. 文档 / Documentation

- [ ] 6.1 Rustdoc 注释
  - [ ] ModelManager 结构
  - [ ] ModelPointers 类型
  - [ ] 相关公共方法

## 7. 验证 / Validation

- [ ] 7.1 运行测试（cargo test）
- [ ] 7.2 运行 clippy（cargo clippy -- -D warnings）
- [ ] 7.3 运行 fmt（cargo fmt）
- [ ] 7.4 验证 openspec（openspec validate --strict）

# Delta Spec: Context Management

## ADDED Requirements

### Requirement: MessageContextManager / MessageContextManager
The system SHALL implement intelligent context window management.

系统应实现智能上下文窗口管理。

#### Scenario: Token 计数 / Token Counting
- **WHEN** 计算消息 token 时
- **THEN** 使用字符估算（简单实现）
- **AND** 或使用 tokenizer（高级实现）
- **AND** 返回准确 token 数量

- **WHEN** counting message tokens
- **THEN** uses character estimation (simple implementation)
- **AND** or uses tokenizer (advanced implementation)
- **AND** returns accurate token count

#### Scenario: 智能裁剪 / Smart Trimming
- **WHEN** 上下文超出限制时
- **THEN** 按优先级裁剪消息
- **AND** 保留系统消息
- **AND** 保留最近的消息
- **AND** 移除最早的工具结果

- **WHEN** context exceeds limit
- **THEN** trims messages by priority
- **AND** keeps system messages
- **AND** keeps recent messages
- **AND** removes earliest tool results

#### Scenario: 上下文优先级 / Context Priority
- **WHEN** 定义消息优先级时
- **THEN** 系统消息优先级最高
- **AND** 用户消息优先级较高
- **AND** 工具结果优先级较低

- **WHEN** defining message priority
- **THEN** system messages have highest priority
- **AND** user messages have higher priority
- **AND** tool results have lower priority

### Requirement: FileFreshnessService / FileFreshnessService
The system SHALL track file access timestamps and detect external modifications.

系统应追踪文件访问时间戳并检测外部修改。

#### Scenario: 文件读取追踪 / File Read Tracking
- **WHEN** 读取文件时
- **THEN** 记录读取时间戳
- **AND** 记录文件修改时间和大小
- **AND** 添加到会话文件列表

- **WHEN** reading a file
- **THEN** record read timestamp
- **AND** record file modification time and size
- **AND** add to session file list

#### Scenario: 文件编辑追踪 / File Edit Tracking
- **WHEN** 编辑文件时
- **THEN** 更新修改时间戳
- **AND** 记录 Agent 编辑时间
- **AND** 从编辑冲突列表中移除

- **WHEN** editing a file
- **THEN** update modification timestamp
- **AND** record Agent edit time
- **AND** remove from edit conflicts list

#### Scenario: 新鲜度检查 / Freshness Checking
- **WHEN** 检查文件新鲜度时
- **THEN** 比较当前修改时间与记录时间
- **AND** 标记是否为外部修改
- **AND** 返回新鲜度状态

- **WHEN** checking file freshness
- **THEN** compare current modification time with recorded time
- **AND** mark if externally modified
- **AND** return freshness status

#### Scenario: 重要文件获取 / Important Files Retrieval
- **WHEN** 获取重要文件列表时
- **THEN** 按最近访问时间排序
- **AND** 过滤依赖目录（node_modules, .git 等）
- **AND** 限制返回数量

- **WHEN** retrieving important files list
- **THEN** sort by recent access time
- **AND** filter dependency directories (node_modules, .git, etc.)
- **AND** limit returned count

### Requirement: 集成测试需求 / Integration Testing Requirements
The system SHALL include comprehensive integration tests to verify context management functionality.

系统应包含全面的集成测试以验证上下文管理功能。

#### Scenario: ModelAdapter 集成测试 / ModelAdapter Integration Tests
- **WHEN** 实现服务层后
- **THEN** 测试 MessageContextManager 与 ModelAdapter 的集成
- **AND** 验证消息在发送前正确归一化
- **AND** 验证 token 计数与实际 API 返回一致
- **AND** 测试自动压缩在达到阈值时触发

- **WHEN** service layer is implemented
- **THEN** test MessageContextManager integration with ModelAdapter
- **AND** verify messages are normalized before sending
- **AND** verify token counting matches actual API returns
- **AND** test auto-compact triggers when threshold reached

**测试前提条件 / Testing Prerequisites**:
- ✅ MessageContextManager 已实现
- ✅ TokenUsage 结构已添加到 Message 类型
- ⏳ ModelAdapter 服务层待实现
- ⏳ 实际 AI 模型 API 集成待完成

**测试用例 / Test Cases**:
1. 消息归一化流程测试
   - 验证 `normalize_messages_for_api()` 正确过滤和转换消息
   - 验证 ProgressMessage 被正确处理
   - 验证工具结果消息正确合并

2. Token 计数准确性测试
   - 使用实际 API 响应验证 `count_tokens_from_usage()`
   - 对比字符估算与真实 token 计数
   - 验证缓存 token 计数逻辑

3. 自动压缩端到端测试
   - 模拟长对话场景（超过 92% 阈值）
   - 验证压缩提示词正确发送到 LLM
   - 验证摘要消息正确创建并替换历史消息

4. 系统提示词格式化测试
   - 验证 `format_system_prompt_with_context()` 正确合并系统提示词
   - 验证上下文变量正确注入
   - 验证提醒逻辑正确工作

5. 文件新鲜度追踪集成测试
   - 模拟文件读取和编辑操作
   - 验证外部修改检测正确工作
   - 验证重要文件列表正确生成

**实现状态 / Implementation Status**:
- 单元测试: ✅ 完成（23 个测试全部通过）
- ModelAdapter 集成: ⏳ 等待服务层实现
- 完整流程测试: ⏳ 等待 ModelAdapter 实现

**备注 / Notes**:
这些集成测试依赖于 ModelAdapter 服务的完整实现。当前所有核心功能已在单元测试中验证，与服务层的集成测试将在 ModelAdapter 实现后添加。

These integration tests depend on the complete implementation of ModelAdapter service. All core functionality has been verified in unit tests. Integration tests with the service layer will be added after ModelAdapter is implemented.

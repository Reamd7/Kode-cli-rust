# Delta Spec: Model Switching

## MODIFIED Requirements

### Requirement: ModelManager / ModelManager
The system SHALL support runtime model switching.

系统应支持运行时模型切换。

#### Scenario: 模型指针 / Model Pointers
- **WHEN** 定义 ModelPointers 时
- **THEN** 包含 default 指针
- **AND** 包含 task 指针
- **AND** 包含 architect 指针
- **AND** 其他自定义指针

- **WHEN** defining ModelPointers
- **THEN** includes default pointer
- **AND** includes task pointer
- **AND** includes architect pointer
- **AND** other custom pointers

#### Scenario: 运行时切换 / Runtime Switching
- **WHEN** 用户执行 /model switch <name> 时
- **THEN** 更新当前激活的模型
- **AND** UI 显示新模型
- **AND** 后续请求使用新模型

- **WHEN** user executes /model switch <name>
- **THEN** updates currently active model
- **AND** UI shows new model
- **AND** subsequent requests use new model

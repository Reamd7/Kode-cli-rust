# TypeScript 配置系统 API 参考

> **文档说明 / Document Notice**
>
> 本文档是 [implement-config-loading](../) 变更的 API 参考文档，列出了 TypeScript 版本的全部 30 个公开 API 函数。
>
> This document is an API reference for the [implement-config-loading](../) change, listing all 30 public API functions from the TypeScript version.

---

## 源文件位置
- `/Users/gemini/Documents/backup/Kode-cli/src/utils/config.ts`
- `/Users/gemini/Documents/backup/Kode-cli/src/utils/env.ts`

## 核心 API 函数列表

### 1. 配置加载 (2 个)
- `getGlobalConfig(): GlobalConfig`
- `getCurrentProjectConfig(): ProjectConfig`

### 2. 配置保存 (2 个)
- `saveGlobalConfig(config: GlobalConfig): void`
- `saveCurrentProjectConfig(config: ProjectConfig): void`

### 3. 环境变量处理 (2 个)
- `getOpenAIApiKey(): string | undefined`
- `getAnthropicApiKey(): string`

### 4. 配置迁移 (1 个)
- `migrateModelProfilesRemoveId(config: GlobalConfig): GlobalConfig`

### 5. 模型系统 (2 个)
- `setAllPointersToModel(modelName: string): void`
- `setModelPointer(pointer: ModelPointerType, modelName: string): void`

### 6. CLI 工具 (4 个)
- `getConfigForCLI(key: string, global: boolean): unknown`
- `setConfigForCLI(key: string, value: unknown, global: boolean): void`
- `deleteConfigForCLI(key: string, global: boolean): void`
- `listConfigForCLI(global: boolean): object`

### 7. 配置键验证 (2 个)
- `isGlobalConfigKey(key: string): boolean`
- `isProjectConfigKey(key: string): boolean`

### 8. 工具函数 (6 个)
- `normalizeApiKeyForConfig(apiKey: string): string`
- `getCustomApiKeyStatus(truncatedApiKey: string): 'approved' | 'rejected' | 'new'`
- `isAutoUpdaterDisabled(): Promise<boolean>`
- `checkHasTrustDialogAccepted(): boolean`
- `getOrCreateUserID(): string`
- `enableConfigs(): void`

### 9. GPT-5 支持 (5 个)
- `isGPT5ModelName(modelName: string): boolean`
- `validateAndRepairGPT5Profile(profile: ModelProfile): ModelProfile`
- `validateAndRepairAllGPT5Profiles(): { repaired: number; total: number }`
- `getGPT5ConfigRecommendations(modelName: string): Partial<ModelProfile>`
- `createGPT5ModelProfile(...): ModelProfile`

### 10. MCP 支持 (4 个)
- `getMcprcConfig(): Record<string, McpServerConfig>`
- `clearMcprcConfigForTesting(): void`
- `addMcprcServerForTesting(name: string, server: McpServerConfig): void`
- `removeMcprcServerForTesting(name: string): void`

**总计**: 30 个公开 API 函数

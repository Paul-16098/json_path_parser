# json_path_parser

这是一个用于 JSON 路径检索和错误处理的 Rust 库。它提供了一个强大的 API，用于根据指定路径从 JSON 数据中提取值，同时确保结构化的错误处理。

## 技术栈

该项目使用了以下技术：

- **Rust**: 编程语言
- **nom**: 用于解析操作
- **serde** 和 **serde_json**: 用于 JSON 的序列化和反序列化
- **thiserror**: 用于结构化错误处理

## 项目架构

该库分为以下关键组件：

1. **`src/lib.rs`**:
   - 包含主要的库逻辑。
   - 提供 `get_json_value` 函数，用于根据路径从 JSON 中检索值。
   - 从 `src/erroe.rs` 重新导出错误类型（`AppError`，`AppResult`）。
2. **`src/erroe.rs`**:
   - 定义了用于结构化错误处理的 `AppError` 枚举。
   - 提供了 `AppResult` 类型别名，用于一致的结果处理。

## 快速开始

### 前置条件

- 安装 [Rust](https://www.rust-lang.org/tools/install)。

### 安装步骤

1. 克隆仓库：

   ```bash
   git clone <repository-url>
   ```

2. 进入项目目录：

   ```bash
   cd xxx
   ```

3. 构建项目：

   ```bash
   cargo build
   ```

### 运行测试

使用以下命令运行测试套件：

```bash
cargo test
```

## 项目结构

该项目遵循以下文件组织：

- `src/lib.rs`: 主要的库逻辑。
- `src/erroe.rs`: 错误处理定义。
- `Cargo.toml`: 依赖项和项目配置。

## 主要功能

- 根据路径从 JSON 中检索值。
- 为无效路径或数据类型不匹配的情况提供优雅的错误处理。
- 使用路径中的 `#` 符号检索 JSON 中数组的长度。

## 开发工作流

- **构建**: 使用 `cargo build` 编译项目。
- **测试**: 使用 `cargo test` 运行测试套件。

## 编码规范

- 对所有错误情况使用 `AppError`。
- 将外部错误（例如 `nom` 错误）转换为 `AppError` 变体。
- 将所有测试放在 `#[cfg(test)]` 模块中。
- 使用 `serde_json::json!` 构造测试数据。

## 测试

该库包含演示其用法的测试用例：

- 从 JSON 中检索数组的长度：

  ```rust
  get_json_value("a.b.#", json_data);
  ```

- 在路径无效或数据类型不匹配时优雅地处理错误。

## 贡献

- 遵循现有的错误处理和测试模式。
- 确保所有新函数都包含单元测试。
- 保持与现有 API 结构的兼容性。

有关进一步的说明，请参阅 `src/lib.rs` 中的测试用例。

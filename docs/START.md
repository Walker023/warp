# 🚀 Warp 项目启动指南

## 快速启动（3 步）

### 1. 安装依赖（首次运行）
```bash
cd /Users/walker/code/claude/warp
./script/bootstrap
```

这个脚本会自动安装所有需要的依赖，包括：
- Rust 工具链
- 平台特定的依赖
- 开发工具

### 2. 构建并运行项目
```bash
./script/run
```

这个命令会：
- 编译整个项目
- 启动 Warp 应用
- 自动打开终端窗口

### 3. 测试中文支持

一旦 Warp 启动，你可以：

#### 方式 1：在代码中临时切换语言
如果你想测试中文菜单，可以临时修改代码：

编辑 `app/src/lib.rs`，在初始化函数中添加：
```rust
// 在适当的初始化位置添加
use crate::i18n::switch_locale;
switch_locale("zh-CN");  // 切换到中文
```

然后重新运行：
```bash
./script/run
```

#### 方式 2：查看菜单栏
启动后，查看顶部菜单栏，如果切换了语言，你会看到：
- **英文**：File | Edit | View | Help
- **中文**：文件 | 编辑 | 视图 | 帮助

---

## 开发流程

### 运行测试
```bash
# 运行所有测试
./script/presubmit

# 只运行 i18n 测试
cargo test --package warp --lib i18n::tests
```

### 检查代码
```bash
# 格式化代码
cargo fmt

# 运行 Clippy 检查
cargo clippy --package warp --lib

# 完整的预提交检查
./script/presubmit
```

### 查看编译日志
```bash
# 只编译，不运行
cargo build --package warp

# 检查语法错误
cargo check --package warp --lib
```

---

## 测试中文翻译

### 1. 查看翻译文件
```bash
# 英文翻译
cat app/locales/en.yml

# 中文翻译
cat app/locales/zh-CN.yml
```

### 2. 添加新翻译并测试

编辑翻译文件：
```bash
# 用你喜欢的编辑器
vim app/locales/zh-CN.yml
# 或
code app/locales/zh-CN.yml
```

重新构建并运行：
```bash
./script/run
```

---

## 常见问题

### Q1: `./script/bootstrap` 失败了怎么办？
**A**: 确保你有网络连接，并且：
```bash
# macOS
xcode-select --install

# 检查 Rust 是否已安装
rustc --version
```

### Q2: 编译时间太长？
**A**: 首次编译会花 10-20 分钟，这是正常的。后续增量编译会快很多。

你可以：
```bash
# 使用更快的链接器（macOS）
brew install llvm

# 或者只编译你修改的部分
cargo check --package warp --lib
```

### Q3: 如何快速验证我的翻译改动？
**A**: 使用增量编译：
```bash
# 1. 修改翻译文件
vim app/locales/zh-CN.yml

# 2. 重新编译（只重编译改动部分）
cargo build --package warp

# 3. 运行
./target/debug/warp-oss
```

### Q4: 菜单没有显示中文？
**A**: 确保：
1. 翻译文件存在：`ls app/locales/`
2. 代码中调用了 `switch_locale("zh-CN")`
3. 重新编译了项目

---

## 项目结构

```
warp/
├── app/                    # 主应用代码
│   ├── src/
│   │   ├── i18n/          # i18n 模块
│   │   ├── app_menus.rs   # 菜单（已国际化）
│   │   └── lib.rs         # 主入口
│   └── locales/           # 翻译文件
│       ├── en.yml         # 英文
│       └── zh-CN.yml      # 中文
├── crates/                # 其他 crates
├── script/                # 构建脚本
│   ├── bootstrap          # 初始化脚本
│   ├── run                # 运行脚本
│   └── presubmit          # 测试脚本
└── docs/                  # 文档
    └── i18n-*.md          # i18n 文档
```

---

## 开发工作流

### 标准流程
```bash
# 1. 修改代码或翻译
vim app/locales/zh-CN.yml

# 2. 检查编译
cargo check --package warp --lib

# 3. 运行测试
cargo test --package warp --lib i18n::tests

# 4. 格式化
cargo fmt

# 5. 运行应用
./script/run

# 6. 提交更改
git add .
git commit -m "feat(i18n): 添加新的中文翻译"
```

### 快速测试流程
```bash
# 修改 -> 编译 -> 运行
vim app/locales/zh-CN.yml && cargo build --package warp && ./target/debug/warp-oss
```

---

## 性能优化

### 加速编译
```bash
# 1. 使用 release 模式（更慢的编译，更快的运行）
cargo build --release

# 2. 或者开发时使用 dev 模式（默认）
cargo build
```

### 减少编译时间
```bash
# 只检查语法，不生成二进制
cargo check

# 只编译库，不编译二进制
cargo build --lib
```

---

## 环境要求

### 必需
- macOS 10.15+ / Linux / Windows 10+
- Rust 1.70+
- 8GB+ RAM
- 10GB+ 可用磁盘空间

### 推荐
- 16GB+ RAM
- SSD 存储
- 多核 CPU（加速并行编译）

---

## 调试技巧

### 启用详细日志
```bash
# 设置日志级别
RUST_LOG=debug ./script/run

# 或针对特定模块
RUST_LOG=warp::i18n=trace ./script/run
```

### 使用 LLDB（macOS）
```bash
# 编译调试版本
cargo build

# 用 LLDB 启动
lldb ./target/debug/warp-oss
(lldb) run
```

---

## 有用的命令

```bash
# 查看项目依赖
cargo tree --package warp

# 清理构建缓存
cargo clean

# 更新依赖
cargo update

# 检查过时的依赖
cargo outdated

# 生成文档
cargo doc --open

# 运行特定测试
cargo test --package warp i18n

# 查看编译时间
cargo build --timings
```

---

## 下一步

1. ✅ 成功启动项目
2. 🔍 查看菜单栏（验证国际化工作）
3. 📝 阅读快速入门：`docs/i18n-quickstart.md`
4. 🛠️ 开始添加新翻译或完成剩余工作

---

## 获取帮助

- **文档**：查看 `docs/` 目录
- **问题**：查看项目 README.md
- **Slack**：加入 Warp 社区的 `#oss-contributors` 频道

---

**祝你开发愉快！** 🎉

如果遇到任何问题，请先检查：
1. Rust 工具链是否正确安装
2. 依赖是否完整（运行 `./script/bootstrap`）
3. 构建日志中的错误信息

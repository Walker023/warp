# Warp 中文支持 - 快速入门指南

## 🚀 5分钟快速入门

本指南帮助你快速了解和使用 Warp 的中文支持功能。

---

## 📖 核心概念

### 什么是 i18n？
i18n（internationalization）= 国际化，使应用支持多语言的过程。

### 如何工作？
```
代码中使用键名 → rust-i18n 查找翻译 → 返回当前语言的文本
    t!("menu.file")  →    查找 locales/   →   "文件" 或 "File"
```

---

## 💻 使用示例

### 1. 在代码中添加翻译

#### 步骤 1：添加翻译文本
编辑 `app/locales/en.yml`：
```yaml
my_feature:
  title: "My Feature"
  description: "This is my feature"
```

编辑 `app/locales/zh-CN.yml`：
```yaml
my_feature:
  title: "我的功能"
  description: "这是我的功能"
```

#### 步骤 2：在代码中使用
```rust
use crate::i18n::t;

fn render_title() -> String {
    t!("my_feature.title")
}

fn render_description() -> String {
    t!("my_feature.description")
}
```

#### 结果
- 英文环境：显示 "My Feature"
- 中文环境：显示 "我的功能"

---

### 2. 切换语言

#### 代码方式
```rust
use crate::i18n::switch_locale;

// 切换到中文
switch_locale("zh-CN");

// 切换到英文
switch_locale("en");
```

#### UI 方式（待实现）
在设置界面选择语言即可。

---

### 3. 带参数的翻译

#### 翻译文件
```yaml
messages:
  welcome: "Welcome, %{name}!"
  items_count: "You have %{count} items"
```

#### 代码使用
```rust
use crate::i18n::t;

let welcome = t!("messages.welcome", name = "Alice");
// 结果：英文 "Welcome, Alice!" / 中文 "欢迎，Alice！"

let count_msg = t!("messages.items_count", count = 5);
// 结果：英文 "You have 5 items" / 中文 "您有 5 个项目"
```

---

## 🎯 常见场景

### 场景 1：菜单项
```rust
// ❌ 旧方式
Menu::new("File", items)

// ✅ 新方式
use crate::i18n::t;
Menu::new(t!("menu.file"), items)
```

### 场景 2：按钮文本
```rust
// ❌ 旧方式
button.with_label("Save")

// ✅ 新方式
use crate::i18n::t;
button.with_label(&t!("common.save"))
```

### 场景 3：错误消息
```rust
// ❌ 旧方式
return Err("Network connection failed");

// ✅ 新方式
use crate::i18n::t;
return Err(t!("errors.network_error"));
```

### 场景 4：对话框
```rust
use crate::i18n::t;

dialog
    .with_title(&t!("dialog.confirm_title"))
    .with_message(&t!("dialog.confirm_message"))
    .with_buttons(vec![
        t!("common.yes"),
        t!("common.no"),
    ])
```

---

## 📋 翻译文件结构

### 推荐的组织方式
```yaml
# 按功能模块组织
menu:           # 菜单相关
  file: "文件"
  edit: "编辑"

settings:       # 设置相关
  general: "通用"
  appearance: "外观"

dialogs:        # 对话框
  confirm: "确认"
  cancel: "取消"

errors:         # 错误消息
  network: "网络错误"
  permission: "权限错误"

common:         # 通用文本
  save: "保存"
  delete: "删除"
  yes: "是"
  no: "否"
```

### 命名规范
- ✅ 使用点号分隔：`menu.file.new`
- ✅ 使用下划线：`new_window`
- ✅ 保持简洁：`save` 而非 `save_button_text`
- ❌ 避免驼峰：`newWindow`
- ❌ 避免过长：`settings.appearance.theme.dark_mode.description`

---

## 🔧 调试技巧

### 检查当前语言
```rust
use crate::i18n::current_locale;
println!("Current locale: {}", current_locale());
```

### 查看可用语言
```rust
use crate::i18n::available_locales;
println!("Available: {:?}", available_locales());
```

### 测试翻译
```rust
#[test]
fn test_translation() {
    use crate::i18n::{t, set_locale};
    
    set_locale("en");
    assert_eq!(t!("menu.file"), "File");
    
    set_locale("zh-CN");
    assert_eq!(t!("menu.file"), "文件");
}
```

---

## ⚠️ 常见错误

### 错误 1：在常量中使用 t!()
```rust
// ❌ 错误 - 编译失败
const TITLE: &str = t!("title");

// ✅ 正确 - 使用函数
fn title() -> String {
    t!("title")
}
```

### 错误 2：忘记添加翻译
```rust
// 代码中使用
let text = t!("new.feature");

// 但忘记在 zh-CN.yml 添加
// 结果：显示英文（fallback）
```

### 错误 3：参数名不匹配
```yaml
# 翻译文件
message: "Hello, %{username}!"
```
```rust
// ❌ 错误
t!("message", name = "Alice")  // 参数名不匹配

// ✅ 正确
t!("message", username = "Alice")
```

---

## 📚 速查表

### API 速查
| 函数 | 用途 | 示例 |
|------|------|------|
| `t!("key")` | 获取翻译 | `t!("menu.file")` |
| `t!("key", param = val)` | 带参数翻译 | `t!("msg", name = "Alice")` |
| `switch_locale("zh-CN")` | 切换语言 | `switch_locale("zh-CN")` |
| `current_locale()` | 获取当前语言 | `current_locale()` |
| `available_locales()` | 获取可用语言 | `available_locales()` |

### 语言代码
| 代码 | 语言 |
|------|------|
| `en` | English |
| `zh-CN` | 简体中文 |

---

## 🎓 进阶主题

### 复数形式
```yaml
items:
  one: "1 item"
  other: "%{count} items"
```

### 性别变化
```yaml
greeting:
  male: "Welcome, Mr. %{name}"
  female: "Welcome, Ms. %{name}"
```

### 日期格式
```yaml
date_format:
  short: "%Y-%m-%d"
  long: "%B %d, %Y"
```

---

## ✅ 检查清单

添加新功能的国际化时，确保：
- [ ] 在 `en.yml` 添加英文文本
- [ ] 在 `zh-CN.yml` 添加中文文本
- [ ] 代码中使用 `t!()` 而非硬编码
- [ ] 测试两种语言下的显示
- [ ] 检查参数名匹配
- [ ] 运行单元测试
- [ ] 手动验证 UI 显示

---

## 🆘 获取帮助

### 文档资源
1. **完整实施报告**：`docs/i18n-implementation-report.md`
2. **剩余工作指南**：`docs/i18n-remaining-work.md`
3. **实施计划**：`.claude/plans/encapsulated-coalescing-crane.md`

### 外部资源
- [rust-i18n GitHub](https://github.com/longbridge/rust-i18n)
- [ICU 消息格式](https://unicode-org.github.io/icu/userguide/format_parse/messages/)

### 测试命令
```bash
# 运行 i18n 单元测试
cargo test --package warp --lib i18n::tests

# 检查编译
cargo check --package warp --lib

# 格式化代码
./script/format

# 运行 Clippy
cargo clippy --package warp --lib
```

---

## 🎉 快速示例

### 完整的功能添加示例

#### 1. 添加翻译（`app/locales/en.yml`）
```yaml
my_dialog:
  title: "Confirm Action"
  message: "Are you sure you want to continue?"
  confirm: "Yes, Continue"
  cancel: "Cancel"
```

#### 2. 添加翻译（`app/locales/zh-CN.yml`）
```yaml
my_dialog:
  title: "确认操作"
  message: "您确定要继续吗？"
  confirm: "是的，继续"
  cancel: "取消"
```

#### 3. 使用翻译（代码）
```rust
use crate::i18n::t;

fn show_confirm_dialog() {
    let dialog = Dialog::new()
        .with_title(&t!("my_dialog.title"))
        .with_message(&t!("my_dialog.message"))
        .with_buttons(vec![
            Button::new(&t!("my_dialog.confirm"))
                .on_click(|_| confirm_action()),
            Button::new(&t!("my_dialog.cancel"))
                .on_click(|_| cancel_action()),
        ]);
    
    dialog.show();
}
```

#### 4. 结果
**英文界面**：
```
┌─────────────────────┐
│ Confirm Action      │
├─────────────────────┤
│ Are you sure you    │
│ want to continue?   │
├─────────────────────┤
│ [Yes, Continue] [Cancel] │
└─────────────────────┘
```

**中文界面**：
```
┌─────────────────────┐
│ 确认操作            │
├─────────────────────┤
│ 您确定要继续吗？    │
├─────────────────────┤
│ [是的，继续] [取消] │
└─────────────────────┘
```

---

## 🚦 现在开始！

1. **浏览现有翻译**：查看 `app/locales/en.yml` 和 `app/locales/zh-CN.yml`
2. **找个简单功能**：从一个小的 UI 组件开始
3. **添加翻译**：按照本指南添加翻译键
4. **使用 t!() 宏**：在代码中使用翻译
5. **测试验证**：切换语言查看效果

**恭喜！你现在可以为 Warp 添加中文支持了！** 🎊

---

**版本**：1.0  
**更新日期**：2026年6月16日

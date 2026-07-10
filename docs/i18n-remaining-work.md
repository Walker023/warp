# Warp 中文支持实施指南 - 剩余工作

本文档说明如何完成剩余的国际化工作。

## 已完成工作概述

✅ **第一阶段：基础设施**
- rust-i18n 依赖已添加
- i18n 模块已创建（`app/src/i18n/mod.rs`）
- 翻译文件已创建（`app/locales/en.yml`, `app/locales/zh-CN.yml`）
- LanguageSettings 和 Locale 枚举已添加

✅ **第二阶段：菜单栏国际化**
- `app/src/app_menus.rs` 已完全国际化
- 所有主菜单和子菜单已使用 `t!()` 宏
- 165+ 条翻译已添加到翻译文件

## 剩余工作清单

### 任务 8：创建语言切换 UI

**目标**：在设置界面添加语言选择下拉菜单

**实施步骤**：

#### 8.1 扩展翻译文件

在 `app/locales/en.yml` 中添加：
```yaml
settings:
  language:
    title: "Language"
    description: "Select your preferred language for the application interface"
    english: "English"
    simplified_chinese: "简体中文"
```

在 `app/locales/zh-CN.yml` 中添加：
```yaml
settings:
  language:
    title: "语言"
    description: "选择应用界面的首选语言"
    english: "English"
    simplified_chinese: "简体中文"
```

#### 8.2 在主设置页面添加语言选项

修改 `app/src/settings_view/main_page.rs` 或创建专门的语言设置组件：

```rust
use crate::i18n::{t, switch_locale, current_locale};
use crate::settings::Locale;

// 添加一个渲染语言选择器的函数
fn render_language_selector(ui_builder: &UiBuilder) -> Box<dyn Element> {
    let current = current_locale();
    
    // 创建下拉菜单或按钮组
    ui_builder
        .dropdown()
        .with_label(t!("settings.language.title"))
        .with_options(vec![
            ("en", t!("settings.language.english")),
            ("zh-CN", t!("settings.language.simplified_chinese")),
        ])
        .with_selected(current)
        .on_change(|locale| {
            switch_locale(locale);
            // 触发 UI 刷新
        })
        .build()
        .finish()
}
```

#### 8.3 使语言设置持久化

修改 `app/src/settings/mod.rs`，将 `LanguageSettings` 注册到设置系统：

```rust
// 在适当的位置添加
impl Setting for LanguageSettings {
    const STORAGE_KEY: &'static str = "Language";
    
    fn default() -> Self {
        Self {
            locale: Locale::En,
        }
    }
}
```

然后在应用启动时加载语言设置：
```rust
// 在 app/src/lib.rs 或主初始化函数中
use crate::i18n::switch_locale;
use crate::settings::LanguageSettings;

// 加载保存的语言设置
let language_settings = LanguageSettings::load();
switch_locale(language_settings.locale.to_str());
```

### 任务 9：国际化对话框和提示

**目标**：国际化常见对话框、提示信息和错误消息

#### 9.1 Reward View (奖励弹窗)

由于 `reward_view.rs` 使用常量，需要改为动态方式：

**修改 `app/src/reward_view.rs`**：

```rust
// 在文件顶部添加
use crate::i18n::t;

// 删除或注释掉这些常量：
// const TITLE: &str = "Congrats!";
// const SUBTITLE_SENT_REFERRAL: &str = "...";
// const SUBTITLE_RECEIVED_REFERRAL: &str = "...";
// const BUTTON_CTA: &str = "Try it out!";

// 修改方法：
fn render_title(&self, ui_builder: &UiBuilder) -> Box<dyn Element> {
    Align::new(
        ui_builder
            .span(&t!("reward.title"))  // 使用动态翻译
            .with_style(UiComponentStyles {
                font_size: Some(TITLE_FONT_SIZE),
                margin: Some(Coords {
                    bottom: TITLE_MARGIN_BOTTOM,
                    ..Default::default()
                }),
                ..Default::default()
            })
            .build()
            .finish(),
    )
    .finish()
}

fn subtitle(&self) -> String {  // 改为返回 String
    match self.kind {
        RewardKind::SentReferralTheme => t!("reward.subtitle_sent"),
        RewardKind::ReceivedReferralTheme => t!("reward.subtitle_received"),
    }
}

fn render_button(&self, ui_builder: &UiBuilder) -> Box<dyn Element> {
    Align::new(
        Container::new(
            ui_builder
                .button(ButtonVariant::Accent, self.cta_mouse_state.clone())
                .with_centered_text_label(t!("reward.button_cta"))  // 使用动态翻译
                .with_style(UiComponentStyles {
                    height: Some(BUTTON_HEIGHT),
                    width: Some(BUTTON_WIDTH),
                    font_size: Some(BUTTON_FONT_SIZE),
                    ..Default::default()
                })
                .build()
                .on_click(|ctx, _, _| ctx.dispatch_typed_action(RewardAction::OpenThemePicker))
                .finish(),
        )
        .with_margin_bottom(BUTTON_MARGIN_BOTTOM)
        .finish(),
    )
    .finish()
}
```

**扩展翻译文件**：

`app/locales/en.yml`:
```yaml
reward:
  title: "Congrats!"
  subtitle_sent: "You earned an exclusive Warp theme for referring someone to Warp."
  subtitle_received: "You earned an exclusive Warp theme for being referred to Warp."
  button_cta: "Try it out!"
  accessibility_help: "Press enter to open the theme chooser or escape to dismiss."
```

`app/locales/zh-CN.yml`:
```yaml
reward:
  title: "恭喜！"
  subtitle_sent: "您因推荐他人使用 Warp 而获得了专属主题。"
  subtitle_received: "您因被推荐使用 Warp 而获得了专属主题。"
  button_cta: "试试看！"
  accessibility_help: "按 Enter 打开主题选择器，按 Escape 关闭。"
```

#### 9.2 Tips 系统

修改 `app/src/tips/mod.rs` 或相关文件：

```rust
use crate::i18n::t;

// 将硬编码的提示文本替换为：
let tip_text = t!("tips.welcome");
let getting_started = t!("tips.getting_started");
```

#### 9.3 Banner 提示

修改 `app/src/banner/view.rs`：

```rust
use crate::i18n::t;

// 查找所有硬编码的 banner 文本并替换
let banner_message = t!("banner.message_key");
```

#### 9.4 错误消息

搜索常见的错误消息模式并国际化：

```bash
# 查找需要国际化的错误消息
rg "Error:|Failed to|Cannot" app/src --type rust
```

示例：
```rust
// 替换前
let error_msg = "Network connection failed";

// 替换后
use crate::i18n::t;
let error_msg = t!("errors.network_error");
```

### 通用国际化模式

#### 模式 1：简单文本替换
```rust
// 替换前
let text = "Hello World";

// 替换后
use crate::i18n::t;
let text = t!("hello_world");
```

#### 模式 2：带参数的文本
```yaml
# 在翻译文件中
messages:
  greeting: "Hello, %{name}!"
```

```rust
use crate::i18n::t;
let text = t!("messages.greeting", name = username);
```

#### 模式 3：常量转函数
```rust
// 替换前
const ERROR_MSG: &str = "Error occurred";

// 替换后
use crate::i18n::t;
fn error_msg() -> String {
    t!("errors.generic")
}
```

## 测试与验证

### 单元测试

运行 i18n 模块测试：
```bash
cargo test --package warp --lib i18n::tests
```

### 手动测试

1. **测试语言切换**：
```rust
// 在代码中临时添加
use crate::i18n::switch_locale;

// 切换到中文
switch_locale("zh-CN");

// 切换回英文
switch_locale("en");
```

2. **检查菜单显示**：
   - 启动应用
   - 查看菜单栏是否显示正确的语言
   - 打开子菜单验证所有文本

3. **检查对话框**：
   - 触发各种对话框
   - 验证按钮和消息文本正确

### 编译验证

```bash
# 检查语法错误
cargo check --package warp --lib

# 运行格式化
./script/format

# 运行 Clippy
cargo clippy --package warp --lib
```

## 优先级建议

### 高优先级（P0）
1. ✅ 菜单栏国际化（已完成）
2. 语言切换 UI（任务 8）
3. 应用启动时加载语言设置

### 中优先级（P1）
4. Reward View 国际化
5. 设置界面主要文本
6. 常见错误消息

### 低优先级（P2）
7. Tips 系统
8. Banner 提示
9. 不常见的错误消息
10. 调试相关文本

## 扩展到更多语言

完成中文支持后，添加其他语言很简单：

1. 在 `app/src/settings/mod.rs` 中添加新的 Locale 变体：
```rust
pub enum Locale {
    En,
    ZhCn,
    Ja,     // 日文
    Ko,     // 韩文
    De,     // 德文
    Fr,     // 法文
}
```

2. 创建新的翻译文件：
```bash
cp app/locales/en.yml app/locales/ja.yml
# 然后翻译内容
```

3. 更新 `to_str()` 和 `display_name()` 方法

## 常见问题

### Q1: 如何处理动态生成的文本？
**A**: 使用参数化翻译：
```yaml
messages:
  items_count: "You have %{count} items"
```
```rust
t!("messages.items_count", count = num)
```

### Q2: 如何处理复数形式？
**A**: rust-i18n 支持 ICU 格式：
```yaml
messages:
  items:
    one: "1 item"
    other: "%{count} items"
```

### Q3: 翻译缺失会怎样？
**A**: rust-i18n 会自动 fallback 到英文，并显示键名作为提示。

### Q4: 如何重新加载翻译文件？
**A**: 开发模式下修改翻译文件需要重新编译。生产环境下翻译是编译时嵌入的。

## 资源链接

- [rust-i18n 文档](https://github.com/longbridge/rust-i18n)
- [rust-i18n 示例](https://github.com/longbridge/rust-i18n/tree/main/examples)
- [ICU 消息格式](https://unicode-org.github.io/icu/userguide/format_parse/messages/)

## 总结

完成以上工作后，Warp 将拥有：
- ✅ 完整的菜单栏中英文支持
- ✅ i18n 基础设施
- ✅ 165+ 条翻译
- 语言切换 UI
- 常见对话框和提示的国际化
- 可扩展的多语言框架

下一步可以逐步扩展到应用的其他部分，或添加更多语言支持。

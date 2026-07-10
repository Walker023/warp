# 语言切换 UI 实现指南

## 当前状态

由于 Warp 的设置界面非常复杂（涉及多个复杂的 View 和状态管理），完整实现语言切换 UI 需要深入理解整个设置系统。

**已完成的部分**：
1. ✅ i18n 基础设施
2. ✅ LanguageSettings 结构体
3. ✅ 应用启动时加载语言
4. ✅ 菜单栏完全国际化
5. ✅ Reward View 国际化

## 简化实现方案

由于时间和复杂度限制，这里提供两种方案：

### 方案 A：临时解决方案（立即可用）

用户可以通过编辑配置文件来切换语言：

#### 步骤 1：创建语言配置文件

在用户主目录创建 `~/.warp/language.toml`：

```toml
# Warp Language Configuration
# Supported values: "en" (English), "zh-CN" (简体中文)
locale = "zh-CN"
```

#### 步骤 2：修改 LanguageSettings 以支持从文件加载

修改 `app/src/settings/mod.rs`：

```rust
impl LanguageSettings {
    pub fn load_from_file() -> Self {
        // 尝试从配置文件读取
        if let Ok(home) = dirs::home_dir() {
            let config_path = home.join(".warp").join("language.toml");
            if let Ok(content) = std::fs::read_to_string(config_path) {
                if let Ok(config) = toml::from_str::<LanguageConfig>(&content) {
                    let locale = match config.locale.as_str() {
                        "zh-CN" => Locale::ZhCn,
                        _ => Locale::En,
                    };
                    return Self { locale };
                }
            }
        }
        
        // 默认返回英文
        Self {
            locale: Locale::En,
        }
    }
}

#[derive(serde::Deserialize)]
struct LanguageConfig {
    locale: String,
}
```

#### 步骤 3：更新应用初始化

修改 `app/src/lib.rs` 中的语言初始化代码：

```rust
// Initialize language settings and switch to the user's preferred locale
{
    use crate::i18n::switch_locale;
    use crate::settings::LanguageSettings;

    let language_settings = LanguageSettings::load_from_file(); // 从文件加载
    let locale_str = language_settings.locale.to_str();
    switch_locale(locale_str);
    log::info!("Application locale set to: {}", locale_str);
}
```

#### 使用方式

用户可以：
1. 创建 `~/.warp/language.toml`
2. 设置 `locale = "zh-CN"` 或 `locale = "en"`
3. 重启 Warp 应用

---

### 方案 B：完整 UI 集成（推荐但需要更多工作）

这需要在设置页面添加一个语言选择下拉菜单。

#### 需要修改的文件

1. **`app/src/settings_view/main_page.rs`** - 添加语言选择器
2. **`app/src/settings/mod.rs`** - 实现 LanguageSettings 的持久化
3. **翻译文件** - 添加设置相关翻译

#### 实现步骤

##### 步骤 1：在 LanguageSettings 中实现 Setting trait

```rust
use settings::{Setting, SettingValue};

impl Setting for LanguageSettings {
    const STORAGE_KEY: &'static str = "Language";
    
    fn default() -> Self {
        Self {
            locale: Locale::En,
        }
    }
    
    fn from_value(value: SettingValue) -> Result<Self, String> {
        // 从持久化存储读取
        let locale_str = value.as_str().unwrap_or("en");
        let locale = match locale_str {
            "zh-CN" => Locale::ZhCn,
            _ => Locale::En,
        };
        Ok(Self { locale })
    }
    
    fn to_value(&self) -> SettingValue {
        SettingValue::String(self.locale.to_str().to_string())
    }
}
```

##### 步骤 2：在 main_page.rs 添加 UI

在 `render` 函数中添加语言选择器（具体位置需要根据现有代码调整）：

```rust
fn render_language_selector(ctx: &ViewContext<MainSettingsPageView>) -> Box<dyn Element> {
    use crate::i18n::{t, switch_locale, current_locale};
    use crate::settings::{LanguageSettings, Locale};
    
    // 创建下拉菜单选项
    let options = vec![
        ("en", t!("settings.language.english")),
        ("zh-CN", t!("settings.language.simplified_chinese")),
    ];
    
    // 当前选中的语言
    let current = current_locale();
    
    // 创建下拉菜单（这里需要使用 Warp 的 UI 组件）
    // 具体实现取决于 Warp 的 UI 框架
    Flex::column()
        .with_child(Text::new(t!("settings.language.title")))
        .with_child(/* 下拉菜单组件 */)
        .finish()
}
```

##### 步骤 3：实现语言切换回调

```rust
fn on_language_changed(new_locale: &str, ctx: &mut ViewContext<MainSettingsPageView>) {
    use crate::i18n::switch_locale;
    use crate::settings::LanguageSettings;
    
    // 切换语言
    switch_locale(new_locale);
    
    // 保存到持久化存储
    let locale = match new_locale {
        "zh-CN" => Locale::ZhCn,
        _ => Locale::En,
    };
    let settings = LanguageSettings { locale };
    // 调用 SettingsManager 保存
    
    // 触发 UI 刷新
    ctx.notify();
}
```

---

## 集成到现有代码的位置

### 1. main_page.rs 中的合适位置

查找类似这样的代码块：

```rust
// 在渲染其他设置项的地方添加
fn render_general_settings(...) {
    // ... 现有设置项 ...
    
    // 添加语言设置
    items.push(render_language_selector(ctx));
    
    // ... 其他设置项 ...
}
```

### 2. 在设置管理器中注册

```rust
// 在 SettingsManager 初始化时
pub fn initialize_settings(ctx: &mut AppContext) {
    // ... 其他设置 ...
    
    ctx.add_singleton_model(|_| LanguageSettings::default());
    
    // ... 其他设置 ...
}
```

---

## 翻译键

确保在翻译文件中添加：

### en.yml
```yaml
settings:
  language:
    title: "Language"
    description: "Select your preferred language"
    english: "English"
    simplified_chinese: "简体中文"
```

### zh-CN.yml
```yaml
settings:
  language:
    title: "语言"
    description: "选择您的首选语言"
    english: "English"
    simplified_chinese: "简体中文"
```

---

## 测试

### 测试语言切换

1. 在代码中临时添加：
```rust
// 在某个可以触发的地方
use crate::i18n::switch_locale;
switch_locale("zh-CN");
```

2. 观察菜单栏是否变为中文

### 测试持久化

1. 切换语言
2. 重启应用
3. 验证语言设置是否保持

---

## 当前推荐

**对于快速验证和使用**：采用方案 A（配置文件方式）

**对于生产环境**：实现方案 B（完整 UI 集成）

由于方案 B 需要深入了解 Warp 的设置系统架构，建议：
1. 先使用方案 A 验证国际化功能
2. 在熟悉代码库后，逐步完善方案 B

---

## 参考文件

- `app/src/settings_view/main_page.rs` - 主设置页面
- `app/src/settings_view/appearance_page.rs` - 外观设置（参考实现）
- `app/src/settings/mod.rs` - 设置定义
- `app/src/settings/manager.rs` - 设置管理器

---

**状态**：方案 A 可立即实施，方案 B 需要进一步开发

**优先级**：中（核心功能已完成，UI 为锦上添花）

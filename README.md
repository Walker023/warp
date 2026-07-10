**简体中文** | [English](README_EN.md)

<a href="https://www.warp.dev">
    <img width="1024" alt="Warp 智能开发环境预览" src="https://github.com/user-attachments/assets/9976b2da-2edd-4604-a36c-8fd53719c6d4" />
</a>
&nbsp;
<p align="center">
  <a href="https://www.warp.dev"><img height="20" alt="使用 Warp 构建" src="https://raw.githubusercontent.com/warpdotdev/brand-assets/main/Github/Built-With-Warp-Export@2x.png" /></a>
  &nbsp;
  <a href="https://oz.warp.dev"><img height="20" alt="由 Oz 驱动" src="https://raw.githubusercontent.com/warpdotdev/brand-assets/main/Github/Powered-By-Oz-Export@2x.png" /></a>
</p>

<p align="center">
  <a href="https://www.warp.dev">官网</a>
  ·
  <a href="https://www.warp.dev/code">代码</a>
  ·
  <a href="https://www.warp.dev/agents">智能体</a>
  ·
  <a href="https://www.warp.dev/terminal">终端</a>
  ·
  <a href="https://www.warp.dev/drive">云端空间</a>
  ·
  <a href="https://docs.warp.dev">文档</a>
  ·
  <a href="https://www.warp.dev/blog/how-warp-works">Warp 工作原理</a>
</p>

> [!NOTE]
> 本仓库是 [warpdotdev/warp](https://github.com/warpdotdev/warp) 的社区 fork，增加了简体中文本地化，并在首次启动时默认显示中文。已有语言设置会继续保留，用户仍可在设置中切换为英文。

<h1></h1>

## 关于 Warp

[Warp](https://www.warp.dev) 是一个从终端发展而来的智能开发环境。你可以使用 Warp 内置的编码智能体，也可以接入自己的 CLI 智能体，例如 Claude Code、Codex、Gemini CLI 等。

OpenAI 是新版开源 Warp 仓库的创始赞助商，新的智能化仓库管理工作流由 GPT 模型驱动。

## 安装

从本 fork 的 [GitHub Releases](https://github.com/Walker023/warp/releases) 下载社区构建：

- `WarpOss.dmg`：支持 Apple Silicon 和 Intel Mac 的 Universal 安装包。
- `WarpOssSetup.exe`：Windows x64 安装程序。
- `WarpOssSetup-arm64.exe`：Windows ARM64 安装程序。

这些是未使用 Warp 官方证书签名的社区构建。macOS Gatekeeper 或 Windows SmartScreen 可能在安装时显示安全警告。需要官方签名版本时，请前往 [Warp 官方下载页](https://www.warp.dev/download)。

## Warp 贡献概览

访问 [build.warp.dev](https://build.warp.dev) 可以：

- 查看数千个 Oz 智能体分类 Issue、编写规范、实现功能和审查 PR。
- 查看主要贡献者以及正在开发的功能。
- 使用 GitHub 登录后跟踪自己的 Issue。
- 在浏览器编译的 Warp 终端中查看正在运行的智能体会话。

## Oz for OSS

如果你在维护热门开源项目，可以[申请 Oz 额度](https://tally.so/r/LZWxqG)，了解 [Oz for OSS](https://github.com/warpdotdev/oz-for-oss)。

Oz for OSS 将 Warp 仓库使用的智能化开源管理工作流带给合作项目，包括 Issue 分类、PR 审查、社区管理和贡献者协作。

## 许可证

Warp UI 框架，即 `warpui_core` 和 `warpui` crate，使用 [MIT 许可证](LICENSE-MIT)。

仓库中的其余代码使用 [AGPL v3](LICENSE-AGPL)。

## 开源贡献

Warp 客户端代码已开源。社区贡献请遵循 [CONTRIBUTING.md](CONTRIBUTING.md) 中的完整流程。

> [!TIP]
> 可以加入 Warp Slack 社区的 [`#oss-contributors`](https://warpcommunity.slack.com/archives/C0B0LM8N4DB) 频道讨论设计、实现和贡献问题。首次加入请先访问 [Warp Slack 邀请页面](https://go.warp.dev/join-preview)。

### 从 Issue 到 PR

提交问题前，请先[搜索上游已有 Issue](https://github.com/warpdotdev/warp/issues?q=is%3Aissue+is%3Aopen+sort%3Areactions-%2B1-desc)。没有相同问题时，再使用上游模板[创建 Issue](https://github.com/warpdotdev/warp/issues/new/choose)。安全漏洞必须按照 [CONTRIBUTING.md](CONTRIBUTING.md#reporting-security-issues) 的说明私下报告。

Warp 维护者可能为 Issue 添加以下状态标签：

- [`ready-to-spec`](https://github.com/warpdotdev/warp/issues?q=is%3Aissue+is%3Aopen+label%3Aready-to-spec)：设计工作已开放。
- [`ready-to-implement`](https://github.com/warpdotdev/warp/issues?q=is%3Aissue+is%3Aopen+label%3Aready-to-implement)：实现方案已确定，可以开始编码。

### 本地构建

从源码构建并运行 Warp：

```bash
./script/bootstrap   # 安装当前平台所需依赖
./script/run         # 构建并运行 Warp
./script/presubmit   # 运行格式、Clippy 和测试检查
```

完整工程说明、代码风格、测试方法和平台注意事项请参阅 [AGENTS.md](AGENTS.md)。

## 加入 Warp 团队

查看 Warp 官方的[招聘职位](https://www.warp.dev/careers)。

## 支持与问题

1. 产品功能和使用方法请查阅 [Warp 官方文档](https://docs.warp.dev/)。
2. 社区交流请加入 [Warp Slack](https://go.warp.dev/join-preview) 的 [`#oss-contributors`](https://warpcommunity.slack.com/archives/C0B0LM8N4DB) 频道。
3. 本 fork 的中文本地化问题请在当前仓库提交 Issue。
4. 需要上游维护者处理的问题，可以在上游 Issue 中提及 **@oss-maintainers**。

## 行为准则

所有参与者都应保持尊重和同理心。Warp 遵循仓库中的[行为准则](CODE_OF_CONDUCT.md)。违规问题可以发送邮件至 warp-coc at warp.dev。

## 主要开源依赖

以下开源项目为 Warp 提供了重要支持，完整列表请参阅[依赖许可证页面](https://docs.warp.dev/help/licenses)：

- [Tokio](https://github.com/tokio-rs/tokio)
- [NuShell](https://github.com/nushell/nushell)
- [Fig Completion Specs](https://github.com/withfig/autocomplete)
- [Warp Server Framework](https://github.com/seanmonstar/warp)
- [Alacritty](https://github.com/alacritty/alacritty)
- [Hyper HTTP library](https://github.com/hyperium/hyper)
- [FontKit](https://github.com/servo/font-kit)
- [Core-foundation](https://github.com/servo/core-foundation-rs)
- [Smol](https://github.com/smol-rs/smol)

//! Small presentation helpers for the `warp-tui` front-end's TUI views.

use rust_i18n::t;
use warpui_core::elements::tui::{Modifier, TuiElement, TuiFlex, TuiStyle, TuiText};

/// Abbreviates a leading home-directory prefix of `path` to `~`.
pub(crate) fn abbreviate_home_prefix(path: &str) -> String {
    if let Some(home) = dirs::home_dir() {
        let home = home.to_string_lossy();
        if let Some(rest) = path.strip_prefix(&*home) {
            if rest.is_empty() || rest.starts_with('/') || rest.starts_with('\\') {
                return format!("~{rest}");
            }
        }
    }
    path.to_owned()
}

/// Vertically centers `content` by padding above and below with flex spacers.
pub(crate) fn centered(content: TuiFlex) -> Box<dyn TuiElement> {
    TuiFlex::column()
        .flex_child(TuiFlex::column().finish())
        .child(content.finish())
        .flex_child(TuiFlex::column().finish())
        .finish()
}

/// Placeholder shown while the user completes device-authorization login. The
/// verification URL/code are surfaced once known (the browser also auto-opens).
pub(crate) fn login_placeholder(
    verification_uri: Option<&str>,
    user_code: Option<&str>,
) -> Box<dyn TuiElement> {
    let dim = TuiStyle::default().add_modifier(Modifier::DIM);
    let mut content = TuiFlex::column().child(
        TuiText::new(t!("warp_tui.login.sign_in_to_continue").to_string())
            .truncate()
            .finish(),
    );
    match (verification_uri, user_code) {
        (Some(uri), Some(code)) => {
            content = content
                .child(
                    TuiText::new(t!("warp_tui.login.open_in_browser", uri = uri).to_string())
                        .with_style(dim)
                        .truncate()
                        .finish(),
                )
                .child(
                    TuiText::new(t!("warp_tui.login.enter_code", code = code).to_string())
                        .with_style(dim)
                        .truncate()
                        .finish(),
                );
        }
        (Some(uri), None) => {
            content = content.child(
                TuiText::new(t!("warp_tui.login.open_in_browser", uri = uri).to_string())
                    .with_style(dim)
                    .truncate()
                    .finish(),
            );
        }
        _ => {
            content = content.child(
                TuiText::new(t!("warp_tui.login.opening_browser").to_string())
                    .with_style(dim)
                    .truncate()
                    .finish(),
            );
        }
    }
    centered(content)
}

/// Placeholder shown between login completion and terminal session creation.
pub(crate) fn terminal_starting() -> Box<dyn TuiElement> {
    let dim = TuiStyle::default().add_modifier(Modifier::DIM);
    centered(
        TuiFlex::column().child(
            TuiText::new(t!("warp_tui.terminal.starting").to_string())
                .with_style(dim)
                .truncate()
                .finish(),
        ),
    )
}

/// Placeholder shown when login fails; the user can quit with `Ctrl-C`.
pub(crate) fn login_failed(message: &str) -> Box<dyn TuiElement> {
    let dim = TuiStyle::default().add_modifier(Modifier::DIM);
    let content = TuiFlex::column()
        .child(
            TuiText::new(t!("warp_tui.login.failed", message = message).to_string())
                .truncate()
                .finish(),
        )
        .child(
            TuiText::new(t!("warp_tui.login.press_ctrl_c_to_exit").to_string())
                .with_style(dim)
                .truncate()
                .finish(),
        );
    centered(content)
}

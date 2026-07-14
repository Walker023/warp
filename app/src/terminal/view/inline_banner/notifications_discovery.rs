use serde::Serialize;
use warpui::elements::MouseStateHandle;
use warpui::notification::RequestPermissionsOutcome;
use warpui::Element;

use super::{
    render_inline_block_list_banner, InlineBannerButtonState, InlineBannerCloseButton,
    InlineBannerContent, InlineBannerStyle, InlineBannerTextButton, InlineBannerTextButtonVariant,
};
use crate::appearance::Appearance;
use crate::i18n::t;
use crate::terminal::session_settings::NotificationsMode;
use crate::terminal::view::{InlineBannerId, NotificationsTrigger, TerminalAction};

#[derive(Clone, Copy, Debug, Serialize)]
pub enum NotificationsDiscoveryBannerAction {
    LearnMore,
    Troubleshoot,
    TurnOn(NotificationsTrigger),
    Configure,
    Close,
}

#[derive(Default)]
pub struct NotificationsDiscoveryBannerMouseStates {
    pub learn_more: MouseStateHandle,
    pub troubleshoot: MouseStateHandle,
    pub turn_on: MouseStateHandle,
    pub configure: MouseStateHandle,
    pub close: MouseStateHandle,
}

/// State necessary to render the (singleton) notifications discovery banner.
pub struct NotificationsDiscoveryBannerState {
    pub banner_id: InlineBannerId,
    pub mouse_states: NotificationsDiscoveryBannerMouseStates,
}

pub fn render_inline_notifications_discovery_banner(
    trigger: NotificationsTrigger,
    request_outcome: Option<RequestPermissionsOutcome>,
    state: &NotificationsDiscoveryBannerState,
    notifications_mode: NotificationsMode,
    appearance: &Appearance,
) -> Box<dyn Element> {
    let active_ui_text_color = appearance.theme().active_ui_text_color().into_solid();

    let learn_more_button = InlineBannerTextButton {
        text: t!("common.learn_more").to_string(),
        text_color: active_ui_text_color,
        button_state: InlineBannerButtonState {
            on_click_event: TerminalAction::NotificationsDiscoveryBanner(
                NotificationsDiscoveryBannerAction::LearnMore,
            ),
            mouse_state_handle: state.mouse_states.learn_more.clone(),
        },
        font: Default::default(),
        position_id: None,
        variant: InlineBannerTextButtonVariant::Secondary,
    };
    let troubleshoot_button = InlineBannerTextButton {
        text: t!("terminal_ui.inline_banner.notifications.troubleshoot").to_string(),
        text_color: active_ui_text_color,
        button_state: InlineBannerButtonState {
            on_click_event: TerminalAction::NotificationsDiscoveryBanner(
                NotificationsDiscoveryBannerAction::Troubleshoot,
            ),
            mouse_state_handle: state.mouse_states.troubleshoot.clone(),
        },
        font: Default::default(),
        position_id: None,
        variant: InlineBannerTextButtonVariant::Secondary,
    };

    let (title, buttons) = match notifications_mode {
        NotificationsMode::Dismissed => (
            t!("terminal_ui.inline_banner.notifications.dismissed").to_string(),
            vec![],
        ),
        NotificationsMode::Disabled => (
            t!("terminal_ui.inline_banner.notifications.disabled").to_string(),
            vec![],
        ),
        NotificationsMode::Unset => (
            trigger.discovery_banner_copy(),
            vec![
                learn_more_button,
                InlineBannerTextButton {
                    text: t!("common.enable").to_string(),
                    text_color: active_ui_text_color,
                    button_state: InlineBannerButtonState {
                        on_click_event: TerminalAction::NotificationsDiscoveryBanner(
                            NotificationsDiscoveryBannerAction::TurnOn(trigger),
                        ),
                        mouse_state_handle: state.mouse_states.turn_on.clone(),
                    },
                    font: Default::default(),
                    position_id: None,
                    variant: InlineBannerTextButtonVariant::Primary,
                },
            ],
        ),
        NotificationsMode::Enabled => {
            // Determine the messaging based on what the user's response was to the
            // permissions request (if any)
            let (title, docs_button) = match request_outcome {
                Some(request_outcome) => match request_outcome {
                    RequestPermissionsOutcome::Accepted => (
                        t!("terminal_ui.inline_banner.notifications.enabled").to_string(),
                        learn_more_button,
                    ),
                    RequestPermissionsOutcome::PermissionsDenied => (
                        t!("terminal_ui.inline_banner.notifications.permission_denied").to_string(),
                        troubleshoot_button,
                    ),
                    RequestPermissionsOutcome::OtherError { .. } => (
                        t!("terminal_ui.inline_banner.notifications.permission_error").to_string(),
                        troubleshoot_button,
                    ),
                },
                None => (
                    t!("terminal_ui.inline_banner.notifications.allow_reminder").to_string(),
                    learn_more_button,
                ),
            };

            (
                title,
                vec![
                    docs_button,
                    InlineBannerTextButton {
                        text: t!("terminal_ui.inline_banner.notifications.configure").to_string(),
                        text_color: active_ui_text_color,
                        button_state: InlineBannerButtonState {
                            on_click_event: TerminalAction::NotificationsDiscoveryBanner(
                                NotificationsDiscoveryBannerAction::Configure,
                            ),
                            mouse_state_handle: state.mouse_states.configure.clone(),
                        },
                        font: Default::default(),
                        position_id: None,
                        variant: InlineBannerTextButtonVariant::Secondary,
                    },
                ],
            )
        }
    };

    let close_button = InlineBannerCloseButton(InlineBannerButtonState {
        on_click_event: TerminalAction::NotificationsDiscoveryBanner(
            NotificationsDiscoveryBannerAction::Close,
        ),
        mouse_state_handle: state.mouse_states.close.clone(),
    });

    render_inline_block_list_banner(
        InlineBannerStyle::CallToAction,
        appearance,
        InlineBannerContent {
            title,
            buttons,
            close_button: Some(close_button),
            ..Default::default()
        },
    )
}

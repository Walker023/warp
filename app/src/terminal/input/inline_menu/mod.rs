//! Generic inline menu view for rendering search results with selection and navigation.
mod message_bar;
mod message_provider;
mod model;
pub(crate) mod positioning;
pub mod styles;
mod view;

pub use message_bar::{InlineMenuMessageArgs, InlineMenuMessageBarArgs};
pub use message_provider::{default_navigation_message_items, InlineMenuMessageProvider};
pub use model::{InlineMenuModel, InlineMenuModelEvent, InlineMenuTabConfig};
pub use positioning::InlineMenuPositioner;
use serde::{Deserialize, Serialize};
pub use view::{
    DetailsRenderConfig, InlineMenuAction, InlineMenuClickBehavior, InlineMenuEvent,
    InlineMenuHeaderConfig, InlineMenuRowAction, InlineMenuView, QueryResultRendererExt,
};

use super::{InputSuggestionsMode, UserQueryMenuAction};
use crate::i18n::t;

/// Identifies a specific inline menu type.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    schemars::JsonSchema,
    settings_value::SettingsValue,
)]
#[schemars(
    description = "Identifies a specific inline menu.",
    rename_all = "snake_case"
)]
pub enum InlineMenuType {
    SlashCommands,
    ModelSelector,
    ConversationMenu,
    ProfileSelector,
    PromptsMenu,
    SkillMenu,
    UserQueryMenu,
    RewindMenu,
    InlineHistoryMenu,
    IndexedReposMenu,
    PlanMenu,
}

impl InlineMenuType {
    fn display_label(&self) -> String {
        match self {
            InlineMenuType::SlashCommands => t!("terminal_ui.input.inline_menu.header.commands"),
            InlineMenuType::ModelSelector => t!("terminal_ui.input.inline_menu.header.model"),
            InlineMenuType::ConversationMenu => {
                t!("terminal_ui.input.inline_menu.header.conversations")
            }
            InlineMenuType::ProfileSelector => {
                t!("terminal_ui.input.inline_menu.header.profiles")
            }
            InlineMenuType::PromptsMenu => t!("terminal_ui.input.inline_menu.header.prompts"),
            InlineMenuType::SkillMenu => t!("terminal_ui.input.inline_menu.header.skills"),
            InlineMenuType::UserQueryMenu => t!("terminal_ui.input.inline_menu.header.fork"),
            InlineMenuType::RewindMenu => t!("terminal_ui.input.inline_menu.header.rewind"),
            InlineMenuType::InlineHistoryMenu => {
                t!("terminal_ui.input.inline_menu.header.history")
            }
            InlineMenuType::IndexedReposMenu => t!("terminal_ui.input.inline_menu.header.repos"),
            InlineMenuType::PlanMenu => t!("terminal_ui.input.inline_menu.header.plans"),
        }
        .to_string()
    }

    pub(crate) fn from_suggestions_mode(mode: &InputSuggestionsMode) -> Option<Self> {
        match mode {
            InputSuggestionsMode::SlashCommands => Some(InlineMenuType::SlashCommands),
            InputSuggestionsMode::ModelSelector => Some(InlineMenuType::ModelSelector),
            InputSuggestionsMode::ConversationMenu => Some(InlineMenuType::ConversationMenu),
            InputSuggestionsMode::ProfileSelector => Some(InlineMenuType::ProfileSelector),
            InputSuggestionsMode::PromptsMenu => Some(InlineMenuType::PromptsMenu),
            InputSuggestionsMode::SkillMenu => Some(InlineMenuType::SkillMenu),
            InputSuggestionsMode::UserQueryMenu {
                action: UserQueryMenuAction::ForkFrom,
                ..
            } => Some(InlineMenuType::UserQueryMenu),
            InputSuggestionsMode::UserQueryMenu {
                action: UserQueryMenuAction::Rewind,
                ..
            } => Some(InlineMenuType::RewindMenu),
            InputSuggestionsMode::InlineHistoryMenu { .. } => {
                Some(InlineMenuType::InlineHistoryMenu)
            }
            InputSuggestionsMode::IndexedReposMenu => Some(InlineMenuType::IndexedReposMenu),
            InputSuggestionsMode::PlanMenu { .. } => Some(InlineMenuType::PlanMenu),
            InputSuggestionsMode::Closed
            | InputSuggestionsMode::HistoryUp { .. }
            | InputSuggestionsMode::CompletionSuggestions { .. }
            | InputSuggestionsMode::StaticWorkflowEnumSuggestions { .. }
            | InputSuggestionsMode::DynamicWorkflowEnumSuggestions { .. }
            | InputSuggestionsMode::AIContextMenu { .. } => None,
        }
    }
}
